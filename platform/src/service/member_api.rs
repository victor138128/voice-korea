pub type Result<T> = std::result::Result<T, ServerFnError>;
use std::collections::HashMap;

use dioxus::prelude::*;
use models::prelude::{
    CreateMemberRequest, InviteMemberRequest, Member, MemberActionRequest, UpdateMemberRequest,
};

use super::login_service::LoginService;
use crate::{api::common::CommonQueryResponse, utils::api::ReqwestClient};

#[derive(Debug, Clone, Copy)]
pub struct MemberApi {
    pub endpoint: Signal<String>,
    pub login_service: LoginService,
}

impl MemberApi {
    pub fn init() {
        let login_service: LoginService = use_context();
        let srv = Self {
            endpoint: use_signal(|| {
                format!(
                    "{}",
                    option_env!("API_URL").unwrap_or("https://voice-korea-api.dev.biyard.co")
                )
            }),
            login_service,
        };
        use_context_provider(|| srv);
    }

    pub async fn create_member(&self, req: CreateMemberRequest) -> Result<Member> {
        let token = self.get_token();

        let client = ReqwestClient::new()?;

        let res = client
            .post("/v1/members")
            .header("Authorization", token)
            .json(&req)
            .send()
            .await?;

        let res = res.error_for_status()?;
        Ok(res.json().await?)
    }

    pub async fn update_member(&self, user_id: String, req: UpdateMemberRequest) -> Result<()> {
        let token = self.get_token();

        let client = ReqwestClient::new()?;

        let _res = client
            .post(format!("/v1/members/{}", user_id).as_str())
            .header("Authorization", token)
            .json(&MemberActionRequest::Update(req))
            .send()
            .await?;

        Ok(())
    }

    pub async fn remove_member(&self, user_id: String) -> Result<()> {
        let token = self.get_token();

        let client = ReqwestClient::new()?;

        let _res = client
            .post(format!("/v1/members/{}", user_id).as_str())
            .header("Authorization", token)
            .json(&MemberActionRequest::Delete)
            .send()
            .await?;

        Ok(())
    }

    pub async fn list_members(
        &self,
        size: Option<i64>,
        bookmark: Option<String>,
    ) -> Result<CommonQueryResponse<Member>> {
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
            .get("/v1/members")
            .query(&params)
            .header("Authorization", token)
            .send()
            .await?;

        let res = res.error_for_status()?;

        let members = res.json().await?;
        Ok(members)
    }

    pub async fn get_member(&self, user_id: String) -> Result<Member> {
        let token = self.get_token();

        let client = ReqwestClient::new()?;

        let res = client
            .get(format!("/v1/members/{user_id}").as_str())
            .header("Authorization", token)
            .send()
            .await?;

        let res = res.error_for_status()?;

        let members = res.json().await?;
        Ok(members)
    }

    pub async fn invite_member(&self, req: InviteMemberRequest) -> Result<()> {
        let token = self.get_token();

        let client = ReqwestClient::new()?;

        let _res = client
            .post("/v1/members/invite")
            .header("Authorization", token)
            .json(&req)
            .send()
            .await?;

        Ok(())
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
