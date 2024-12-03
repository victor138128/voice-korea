use std::sync::Arc;

use by_axum::axum::extract::State;
use by_axum::axum::Json;
use easy_dynamodb::Client;
use models::User;
use serde::Deserialize;

use super::super::verification::email::{verify_handler, EmailVerifyParams};
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
            id: body.auth_id,
            value: body.auth_value,
        }),
    )
    .await?;
    let hashed_pw = get_hash_string(body.password.as_bytes());
    let user = User::new(uuid::Uuid::new_v4().to_string(), body.email, hashed_pw);

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

    Ok(())
}
