#![allow(unused_imports)]
use dioxus::prelude::{
    server_fn::codec::{GetUrl, Json, PostUrl},
    *,
};

use serde::{Deserialize, Serialize};

use crate::{models::user::User, utils::hash::get_hash_string};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

pub async fn login_user(req: LoginRequest) -> Result<String, ServerFnError> {
    use dioxus_logger::tracing;
    use reqwest::Client;
    let client = Client::new();
    dioxus_logger::tracing::debug!("/v1/auth/login: {:?}", req.email);
    let url = if let Some(url) = option_env!("API_URL") {
        format!("{}/v1/auth/login", url)
    } else {
        return Err(ServerFnError::new("\"API URL\" Not found"));
    };

    let data = req;

    let res = client.post(url).json(&data).send().await?;

    if res.status().is_success() {
        let body = res.text().await?;
        Ok(body)
    } else {
        let body = res.text().await?;
        return Err(dioxus::prelude::ServerFnError::ServerError(body));
    }
}
