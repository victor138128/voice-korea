use dioxus::prelude::{server_fn::codec::Json, ServerFnError, *};

use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct SendNotificationParams {
    pub email: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct VerifyAuthenticationParams {
    pub id: String,
    pub value: String,
}

#[server(endpoint = "/v1/verification/email/send", input = Json, output = Json)]
pub async fn send_notification(req: SendNotificationParams) -> Result<String, ServerFnError> {
    use reqwest::Client;
    let client = Client::new();
    dioxus_logger::tracing::debug!("/v1/verification/email/send: {:?}", req.email);
    let url = if let Some(url) = option_env!("API_URL") {
        format!("{}/v1/verification/email/send", url)
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

#[server(endpoint = "/v1/verification/email/verify", input = Json, output = Json)]
pub async fn verify_authentication(
    req: VerifyAuthenticationParams,
) -> Result<String, ServerFnError> {
    use reqwest::Client;
    let client = Client::new();
    dioxus_logger::tracing::debug!(
        "/v1/verification/email/verify: {:?} {:?}",
        req.id,
        req.value
    );
    let url = if let Some(url) = option_env!("API_URL") {
        format!("{}/v1/verification/email/verify", url)
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
