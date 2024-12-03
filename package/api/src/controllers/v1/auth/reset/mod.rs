use std::sync::Arc;

use by_axum::axum::extract::State;
use by_axum::axum::Json;
use easy_dynamodb::Client;
use models::User;
use serde::Deserialize;

use super::super::verification::email::{verify_handler, EmailVerifyParams};
use crate::utils::{error::ApiError, hash::get_hash_string};

#[derive(Deserialize)]
pub struct ResetParams {
    pub auth_id: String,
    pub auth_value: String,
    pub email: String,
    pub password: String,
}

pub async fn handler(
    State(db): State<Arc<Client>>,
    Json(body): Json<ResetParams>,
) -> Result<(), ApiError> {
    verify_handler(
        State(db.clone()),
        Json(EmailVerifyParams {
            id: body.auth_id,
            value: body.auth_value,
        }),
    )
    .await?;
    let email = body.email.clone();

    let result: Result<(Option<Vec<User>>, Option<String>), easy_dynamodb::error::DynamoException> =
        db.find(
            "gsi1-index",
            None,
            Some(1),
            vec![("gsi1", User::gsi1(body.email))],
        )
        .await;

    let (docs, _) = match result {
        Ok((Some(docs), Some(_))) => (docs, ()),
        _ => return Err(ApiError::InvalidCredentials(email)),
    };
    let user = match docs.first() {
        Some(user) => user,
        None => return Err(ApiError::InvalidCredentials(email)),
    };
    let hashed_password = get_hash_string(body.password.as_bytes());
    let _ = db
        .update(&user.id, vec![("password", hashed_password)])
        .await;
    Ok(())
}
