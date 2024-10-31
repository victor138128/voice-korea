#![allow(unused_imports)]
pub mod login;
pub mod reset;
pub mod signup;
pub mod verify;

use dioxus::prelude::{
    server_fn::codec::{GetUrl, Json, PostUrl},
    *,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct LoginUserRequest {
    pub email: String,
    pub password: String,
}

#[server(endpoint = "/v1/users", input = Json, output = Json)]
pub async fn login_user(req: LoginUserRequest) -> Result<(), ServerFnError> {
    dioxus_logger::tracing::debug!("/v1/users: {:?}", req);

    Err(ServerFnError::ServerError(
        "not implemented yet".to_string(),
    ))
}
