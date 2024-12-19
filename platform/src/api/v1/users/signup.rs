#![allow(unused_imports)]
use dioxus::prelude::*;
use dioxus_logger::tracing;

use serde::{Deserialize, Serialize};

use crate::{api::common::TypeField, models::user::User, utils::hash::get_hash_string};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct SignupRequest {
    pub auth_id: String,
    pub auth_value: String,
    pub email: String,
    pub password: String,
}

pub async fn signup_user(req: SignupRequest) -> Result<(), ServerFnError> {
    use dioxus_logger::tracing;
    use reqwest::Client;
    let client = Client::new();
    dioxus_logger::tracing::debug!("/v1/auth/signup: {:?}", req.email);
    let url = if let Some(url) = option_env!("API_URL") {
        format!("{}/v1/auth/signup", url)
    } else {
        return Err(ServerFnError::new("\"API URL\" Not found"));
    };

    let data = req;

    let res = client.post(url).json(&data).send().await?;

    if res.status().is_success() {
        Ok(())
    } else {
        let body = res.text().await?;
        return Err(dioxus::prelude::ServerFnError::ServerError(body));
    }
}
