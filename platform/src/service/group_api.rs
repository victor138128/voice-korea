pub type Result<T> = std::result::Result<T, ServerFnError>;

use std::collections::HashMap;

use dioxus::prelude::*;
use models::prelude::{CreateGroupRequest, Group, GroupActionRequest};

use crate::{api::common::CommonQueryResponse, utils::api::ReqwestClient};

use super::login_service::LoginService;

#[derive(Debug, Clone, Copy)]
pub struct GroupApi {
    pub endpoint: Signal<String>,
    pub login_service: LoginService,
}

impl GroupApi {
    pub fn init() {
        let login_service: LoginService = use_context();
        let srv = Self {
            endpoint: use_signal(|| "http://localhost:3000".to_string()),
            login_service,
        };
        use_context_provider(|| srv);
    }

    pub async fn create_group(&self, req: CreateGroupRequest) -> Result<Group> {
        let token = self.get_token();
        let client = ReqwestClient::new()?;

        let res = client
            .post("/v1/groups")
            .header("Authorization", token)
            .json(&req)
            .send()
            .await?;

        Ok(res.json().await?)
    }

    pub async fn update_group_name(&self, group_id: String, group_name: String) -> Result<()> {
        let token = self.get_token();

        let client = ReqwestClient::new()?;

        let _res = client
            .post(format!("/v1/groups/{}", group_id).as_str())
            .header("Authorization", token)
            .json(&GroupActionRequest::UpdateName(group_name))
            .send()
            .await?;

        Ok(())
    }

    pub async fn remove_group(&self, group_id: String) -> Result<()> {
        let token = self.get_token();
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
        size: Option<i64>,
        bookmark: Option<String>,
    ) -> Result<CommonQueryResponse<Group>> {
        let token = self.get_token();

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

    pub async fn get_group(&self, group_id: String) -> Result<Group> {
        let token = self.get_token();

        let client = ReqwestClient::new()?;

        let res = client
            .get(format!("/v1/groups/{group_id}").as_str())
            .header("Authorization", token)
            .send()
            .await?;

        let res = res.error_for_status()?;

        let members = res.json().await?;
        Ok(members)
    }

    pub fn get_token(&self) -> String {
        let cookie = if cfg!(feature = "web") {
            self.login_service
                .get_cookie_value()
                .unwrap_or_else(|| "".to_string())
        } else {
            "".to_string()
        };

        let token = cookie.replace('"', "");
        let format_cookie = format!("token={token}");
        let token = format_cookie.replace("token=", "Bearer ").replace("\"", "");

        token
    }
}
