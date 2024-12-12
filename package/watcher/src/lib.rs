use reqwest::{header::HeaderMap, Client};
use std::error::Error;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub struct Watcher {
    pub endpoint: String,
    pub req_client: Client,
}

impl Watcher {
    pub fn new() -> Result<Self> {
        let endpoint = option_env!("VOICE_KOREA_API_ENDPOINT")
            .unwrap_or("http://localhost:3000")
            .to_string();
        let mut headers = HeaderMap::new();
        headers.insert(
            "DAGIT-SERVER-KEY",
            option_env!("INTERNAL_SERVER_KEY")
                .unwrap_or("server-key")
                .parse()?,
        );
        let req_client = Client::builder()
            .default_headers(headers)
            .timeout(std::time::Duration::from_secs(30))
            .build()?;
        Ok(Watcher {
            endpoint,
            req_client,
        })
    }

    pub async fn finalize_survey(&self) -> Result<u16> {
        self.req_client
            .request(reqwest::Method::PUT, format!("{}/m1/"))
    }
}
