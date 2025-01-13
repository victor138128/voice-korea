use by_axum::axum::routing::post;
use by_axum::axum::{Json, Router};
use by_axum::{axum::extract::State, log::root};
use models::prelude::{Organization, OrganizationMember};
use models::{
    prelude::{CreateMemberRequest, Member},
    User,
};
use serde::Deserialize;
use slog::o;

use super::super::verification::email::{verify_handler, EmailVerifyParams};
use crate::common::CommonQueryResponse;
use crate::utils::{error::ApiError, hash::get_hash_string};

#[derive(Deserialize, Clone, Debug)]
pub struct SignUpParams {
    pub auth_id: String,
    pub auth_value: String,
    pub email: String,
    pub password: String,
}

#[derive(Clone, Debug)]
pub struct SignupControllerV1 {
    log: slog::Logger,
    db: std::sync::Arc<easy_dynamodb::Client>,
}

impl SignupControllerV1 {
    pub fn router(db: std::sync::Arc<easy_dynamodb::Client>) -> Router {
        let log = root().new(o!("api-controller" => "SignupControllerV1"));
        let ctrl = SignupControllerV1 { db, log };

        Router::new()
            .route("/", post(Self::signup))
            .with_state(ctrl.clone())
    }

    pub async fn signup(
        State(ctrl): State<SignupControllerV1>,
        Json(body): Json<SignUpParams>,
    ) -> Result<(), ApiError> {
        let log = ctrl.log.new(o!("api" => "signup"));
        slog::debug!(log, "signup {:?}", body);
        let auth_doc_id = verify_handler(
            State(ctrl.db.clone()),
            Json(EmailVerifyParams {
                id: body.auth_id.clone(),
                value: body.auth_value.clone(),
            }),
        )
        .await?;
        let hashed_pw = get_hash_string(body.password.as_bytes());
        let user = User::new(
            uuid::Uuid::new_v4().to_string(),
            body.email.clone(),
            hashed_pw,
        );

        let result: Result<
            (Option<Vec<models::User>>, Option<String>),
            easy_dynamodb::error::DynamoException,
        > = ctrl
            .db
            .find(
                "gsi1-index",
                None,
                Some(1),
                vec![("gsi1", User::gsi1(user.email.clone()))],
            )
            .await;
        match result {
            Ok((Some(docs), _)) => {
                if docs.len() > 0 {
                    return Err(ApiError::DuplicateUser);
                }
            }
            _ => (),
        };
        let _ = ctrl.db.delete(&auth_doc_id);
        let _ = ctrl
            .db
            .create(user.clone())
            .await
            .map_err(|e| ApiError::DynamoCreateException(e.to_string()))?;

        let _ = SignupControllerV1::create_organization(user.id.clone(), body.clone()).await?;

        let _ = SignupControllerV1::create_member(body).await?; //FIXME: add to organization

        Ok(())
    }

    async fn create_organization(user_id: String, body: SignUpParams) -> Result<(), ApiError> {
        let log = root();
        let cli = easy_dynamodb::get_client(&log);

        let id = uuid::Uuid::new_v4().to_string();

        let organization: Organization =
            Organization::new(id.clone(), user_id.clone(), body.email.clone());
        let _ = cli
            .upsert(organization)
            .await
            .map_err(|e| ApiError::DynamoCreateException(e.to_string()))?;

        let organization_member_id = uuid::Uuid::new_v4().to_string();
        let organization_member: OrganizationMember =
            OrganizationMember::new(organization_member_id, user_id.clone(), id.clone());
        let _ = cli
            .upsert(organization_member)
            .await
            .map_err(|e| ApiError::DynamoCreateException(e.to_string()))?;

        Ok(())
    }

    async fn create_member(body: SignUpParams) -> Result<(), ApiError> {
        let log = root();
        let cli = easy_dynamodb::get_client(&log);

        let res: CommonQueryResponse<Member> = CommonQueryResponse::query(
            &log,
            "gsi1-index",
            None,
            Some(1),
            vec![("gsi1", Member::get_gsi1(body.email.clone()))],
        )
        .await?;

        if res.items.len() != 0 {
            let item = res.items.first().unwrap();

            if item.deleted_at.is_none() {
                return Ok(()); //already invited member
            }
        }

        let id = uuid::Uuid::new_v4().to_string();

        let member: Member = (
            CreateMemberRequest {
                email: body.email.clone(),
                name: None,
                group: None,
                role: None,
            },
            id,
        )
            .into();

        match cli.upsert(member).await {
            Ok(()) => Ok(()),
            Err(e) => {
                slog::error!(log, "Create Member Failed {e:?}");
                Err(ApiError::DynamoCreateException(e.to_string()))
            }
        }
    }
}
