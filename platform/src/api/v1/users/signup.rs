#![allow(unused_imports)]
use dioxus::prelude::{
    server_fn::codec::{GetUrl, Json, PostUrl},
    *,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct SignupUserRequest {
    pub email: String,
    pub password: String,
    pub name: String,
    pub phone: String,
    pub address: String,
}

#[server(endpoint = "/v1/users/signup", input = Json, output = Json)]
pub async fn signup_user(req: SignupUserRequest) -> Result<(), ServerFnError> {
    dioxus_logger::tracing::debug!("/v1/users/signup: {:?}", req);

    Err(ServerFnError::ServerError(
        "not implemented yet".to_string(),
    ))
}
