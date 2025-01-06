pub type Result<T> = std::result::Result<T, ServerFnError>;

use std::collections::HashMap;

use dioxus::prelude::*;
use models::prelude::{CreateGroupRequest, Group, GroupActionRequest};

use crate::{api::common::CommonQueryResponse, utils::api::ReqwestClient};

#[derive(Debug, Clone, Copy, Default)]
pub struct GroupApi {
    pub endpoint: Signal<String>,
}

impl GroupApi {
    pub fn init() {
        let srv = Self {
            endpoint: use_signal(|| match option_env!("API_DOMAIN") {
                Some(endpoint) => endpoint.to_string(),
                None => format!(
                    "https://voice-korea-api.{}",
                    option_env!("SUBDOMAIN").unwrap_or("dev.biyard.co")
                ),
            }),
        };
        use_context_provider(|| srv);
    }

    pub async fn create_group(&self, cookie: String, req: CreateGroupRequest) -> Result<Group> {
        let format_cookie = format!("{:?}", cookie);
        let token = format_cookie.replace("token=", "Bearer ").replace("\"", "");

        let client = ReqwestClient::new()?;

        let res = client
            .post("/v1/groups")
            .header("Authorization", token)
            .json(&req)
            .send()
            .await?;

        let res = res.error_for_status()?;
        Ok(res.json().await?)
    }

    pub async fn update_group_name(
        &self,
        cookie: String,
        group_id: String,
        group_name: String,
    ) -> Result<()> {
        let format_cookie = format!("{:?}", cookie);
        let token = format_cookie.replace("token=", "Bearer ").replace("\"", "");

        let client = ReqwestClient::new()?;

        let _res = client
            .post(format!("/v1/groups/{}", group_id).as_str())
            .header("Authorization", token)
            .json(&GroupActionRequest::UpdateName(group_name))
            .send()
            .await?;

        Ok(())
    }

    pub async fn remove_group(&self, cookie: String, group_id: String) -> Result<()> {
        let format_cookie = format!("{:?}", cookie);
        let token = format_cookie.replace("token=", "Bearer ").replace("\"", "");

        let client = ReqwestClient::new()?;

        let _res = client
            .post(format!("/v1/groups/{}", group_id).as_str())
            .header("Authorization", token)
            .json(&GroupActionRequest::Delete)
            .send()
            .await?;

        Ok(())
    }

    pub async fn list_groups(
        &self,
        cookie: String,
        size: Option<i64>,
        bookmark: Option<String>,
    ) -> Result<CommonQueryResponse<Group>> {
        let format_cookie = format!("{:?}", cookie);
        let token = format_cookie.replace("token=", "Bearer ").replace("\"", "");

        let mut params = HashMap::new();
        if let Some(size) = size {
            params.insert("size", size.to_string());
        }
        if let Some(bookmark) = bookmark {
            params.insert("bookmark", bookmark);
        }

        let client = ReqwestClient::new()?;

        let res = client
            .get("/v1/groups")
            .query(&params)
            .header("Authorization", token)
            .send()
            .await?;

        let res = res.error_for_status()?;

        let groups = res.json().await?;
        Ok(groups)
    }

    pub async fn get_group(&self, cookie: String, group_id: String) -> Result<Group> {
        let format_cookie = format!("{:?}", cookie);
        let token = format_cookie.replace("token=", "Bearer ").replace("\"", "");

        let client = ReqwestClient::new()?;

        let res = client
            .get(format!("/v1/members/{group_id}").as_str())
            .header("Authorization", token)
            .send()
            .await?;

        let res = res.error_for_status()?;

        let members = res.json().await?;
        Ok(members)
    }
}
