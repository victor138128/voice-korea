use std::sync::Arc;

use by_axum::axum::Json;
use by_axum::{axum::extract::State, log::root};
use easy_dynamodb::Client;
use models::{
    prelude::{CreateMemberRequest, Member},
    User,
};
use serde::Deserialize;

use super::super::verification::email::{verify_handler, EmailVerifyParams};
use crate::common::CommonQueryResponse;
use crate::utils::{error::ApiError, hash::get_hash_string};

#[derive(Deserialize)]
pub struct SignUpParams {
    pub auth_id: String,
    pub auth_value: String,
    pub email: String,
    pub password: String,
}

pub async fn handler(
    State(db): State<Arc<Client>>,
    Json(body): Json<SignUpParams>,
) -> Result<(), ApiError> {
    let auth_doc_id = verify_handler(
        State(db.clone()),
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
    > = db
        .find(
            "gsi1-index",
            None,
            Some(1),
            vec![("gsi1", User::gsi1(user.email.clone()))],
        )
        .await;
    match result {
        Ok((Some(docs), Some(_))) => {
            if docs.len() > 0 {
                return Err(ApiError::DuplicateUser);
            }
        }
        _ => (),
    };
    let _ = db.delete(&auth_doc_id);
    let _ = db.create(user).await;

    let _ = create_member(db, body).await;

    Ok(())
}

async fn create_member(_db: Arc<Client>, body: SignUpParams) -> Result<(), ApiError> {
    let log = root();
    let cli = easy_dynamodb::get_client(log.clone());

    let res: CommonQueryResponse<Member> = CommonQueryResponse::query(
        &log,
        "gsi1-index",
        None,
        Some(1),
        vec![("gsi1", Member::get_gsi1(body.email.clone()))],
    )
    .await?;

    slog::debug!(log, "This line come1 {:?}", res.items);

    if res.items.len() != 0 {
        let item = res.items.first().unwrap();
        slog::debug!(log, "This line come2 {:?}", item);

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
    slog::debug!(log, "This line come3 {:?}", member);

    match cli.upsert(member.clone()).await {
        Ok(()) => Ok(()),
        Err(e) => {
            slog::error!(log, "Create Member Failed {e:?}");
            Err(ApiError::DynamoCreateException(e.to_string()))
        }
    }
}
