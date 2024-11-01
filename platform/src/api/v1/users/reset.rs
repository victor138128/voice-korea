#![allow(unused_imports)]
use dioxus::prelude::{
    server_fn::codec::{GetUrl, Json, PostUrl},
    *,
};

use dioxus_logger::tracing;
use serde::{Deserialize, Serialize};

use crate::{api::common::TypeField, models::user::User, utils::hash::get_hash_string};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResetPasswordRequest {
    pub email: String,
    pub password: String,
}

#[server(endpoint = "/v1/users/password/reset", input = Json, output = Json)]
pub async fn reset_password(req: ResetPasswordRequest) -> Result<(), ServerFnError> {
    use easy_dynamodb::error::DynamoException;
    let log = crate::utils::logger::new_api("POST", &format!("/v1/users/password/reset"));
    let cli = crate::utils::db::get(&log);

    let pw_hash = get_hash_string(req.password.as_bytes());

    match cli
        .update::<TypeField>(
            req.email.as_str(),
            vec![("hashed_password", TypeField::S(pw_hash))],
        )
        .await
    {
        Ok(()) => {
            tracing::debug!("DB update success",);
        }
        Err(_e) => {
            return Err(ServerFnError::ServerError(format!(
                "Password update failed"
            )));
        }
    };

    Ok(())
}
