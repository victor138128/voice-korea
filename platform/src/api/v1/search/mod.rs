use dioxus::prelude::{
    server_fn::codec::{GetUrl, Json},
    ServerFnError, *,
};

use crate::api::common::CommonQueryResponse;

#[server(endpoint = "/v1/search", input = GetUrl, output = Json)]
pub async fn search(
    q: String,
) -> Result<CommonQueryResponse<models::prelude::SearchResult>, ServerFnError> {
    use urlencoding::encode;
    let encoded_q = encode(&q);
    dioxus_logger::tracing::debug!("/v1/search: {:?}", encoded_q);
    let url = if let Some(url) = option_env!("API_URL") {
        format!("{}/v1/search?query={}", url, encoded_q)
    } else {
        return Err(ServerFnError::new("\"API URL\" Not found"));
    };
    Ok(reqwest::get(url).await?.json().await?)
}
