use dioxus::prelude::{
    server_fn::codec::{GetUrl, Json},
    ServerFnError, *,
};

use crate::api::common::CommonQueryResponse;

#[server(endpoint = "/v1/search", input = GetUrl, output = Json)]
pub async fn search(
    q: String,
) -> Result<CommonQueryResponse<models::prelude::SearchResult>, ServerFnError> {
    dioxus_logger::tracing::debug!("/v1/search: {:?}", q);
    let url = if let Some(url) = option_env!("API_URL") {
        format!("{}/v1/search?query={}", url, q)
    } else {
        return Err(ServerFnError::new("\"API URL\" Not found"));
    };
    Ok(reqwest::get(url).await?.json().await?)
}
