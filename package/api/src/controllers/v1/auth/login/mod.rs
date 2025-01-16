use by_axum::axum::extract::State;
use by_axum::axum::http::header::SET_COOKIE;
use by_axum::axum::response::Response;
use by_axum::axum::routing::post;
use by_axum::axum::{Json, Router};
use by_axum::log::root;
use models::prelude::{CreateMemberRequest, Member};
use serde::Deserialize;
use slog::o;

use crate::common::CommonQueryResponse;
use crate::utils::error::ApiError;
use crate::utils::hash::get_hash_string;
use crate::utils::jwt::generate_jwt;

#[derive(Deserialize, Debug)]
pub struct LoginParams {
    email: String,
    password: String,
}

#[derive(Clone, Debug)]
pub struct LoginControllerV1 {
    log: slog::Logger,
}

impl LoginControllerV1 {
    pub fn router(_db: std::sync::Arc<easy_dynamodb::Client>) -> Router {
        let log = root().new(o!("api-controller" => "LoginControllerV1"));
        let ctrl = LoginControllerV1 { log };

        Router::new()
            .route("/", post(Self::login))
            .with_state(ctrl.clone())
    }

    pub async fn login(
        State(ctrl): State<LoginControllerV1>,
        Json(body): Json<LoginParams>,
    ) -> Result<Response<String>, ApiError> {
        let log = ctrl.log.new(o!("api" => "login"));
        slog::debug!(log, "login {:?}", body);
        let email = body.email.clone();
        let users = CommonQueryResponse::<models::User>::query(
            &log,
            "gsi1-index",
            None,
            Some(1),
            vec![("gsi1", models::User::gsi1(body.email.clone()))],
        )
        .await?;

        if users.items.len() == 0 {
            return Err(ApiError::InvalidCredentials(email));
        }
        let user = users.items.first().unwrap();

        let hashed_password = get_hash_string(body.password.as_bytes());
        slog::debug!(
            log,
            "user_password: {} hashed_password: {}",
            user.password,
            hashed_password
        );

        if user.password != hashed_password {
            return Err(ApiError::InvalidCredentials(email));
        }

        let jwt = generate_jwt(&user.id, &user.email)
            .map_err(|e| ApiError::JWTGenerationFail(e.to_string()))?;

        // If the member list has not been added, add the member list
        let _ = LoginControllerV1::create_member(body)
            .await
            .map_err(|e| ApiError::DynamoCreateException(e.to_string()))?;

        Ok(Response::builder()
            .status(200)
            .header(
                SET_COOKIE,
                format!("token={}; HttpOnly; Secure; SameSite=None; Path=/", jwt),
            )
            .body(jwt)
            .map_err(|e| ApiError::ValidationError(e.to_string()))?)
    }

    // FIXME: deprecated
    async fn create_member(body: LoginParams) -> Result<(), ApiError> {
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
                return Ok(()); //already member
            }
        }

        let id = uuid::Uuid::new_v4().to_string();

        let member: Member = (
            CreateMemberRequest {
                user_id: id.clone(),
                org_id: "default".to_string(),
                name: None,
                group: None,
                role: None,
            },
            id.clone(),
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
