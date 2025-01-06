pub type Result<T> = std::result::Result<T, ServerFnError>;
use std::collections::HashMap;

use dioxus::prelude::*;
use models::prelude::{
    CreateMemberRequest, InviteMemberRequest, Member, MemberActionRequest, UpdateMemberRequest,
};

use crate::{api::common::CommonQueryResponse, utils::api::ReqwestClient};

#[derive(Debug, Clone, Copy, Default)]
pub struct MemberApi {
    pub endpoint: Signal<String>,
}

impl MemberApi {
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

    pub async fn create_member(&self, cookie: String, req: CreateMemberRequest) -> Result<Member> {
        let format_cookie = format!("{:?}", cookie);
        let token = format_cookie.replace("token=", "Bearer ").replace("\"", "");

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

    pub async fn update_member(
        &self,
        cookie: String,
        user_id: String,
        req: UpdateMemberRequest,
    ) -> Result<()> {
        let format_cookie = format!("{:?}", cookie);
        let token = format_cookie.replace("token=", "Bearer ").replace("\"", "");

        let client = ReqwestClient::new()?;

        let _res = client
            .post(format!("/v1/members/{}", user_id).as_str())
            .header("Authorization", token)
            .json(&MemberActionRequest::Update(req))
            .send()
            .await?;

        Ok(())
    }

    pub async fn remove_member(&self, cookie: String, user_id: String) -> Result<()> {
        let format_cookie = format!("{:?}", cookie);
        let token = format_cookie.replace("token=", "Bearer ").replace("\"", "");

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
        cookie: String,
        size: Option<i64>,
        bookmark: Option<String>,
    ) -> Result<CommonQueryResponse<Member>> {
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
            .get("/v1/members")
            .query(&params)
            .header("Authorization", token)
            .send()
            .await?;

        let res = res.error_for_status()?;

        let members = res.json().await?;
        Ok(members)
    }

    pub async fn get_member(&self, cookie: String, user_id: String) -> Result<Member> {
        let format_cookie = format!("{:?}", cookie);
        let token = format_cookie.replace("token=", "Bearer ").replace("\"", "");

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

    pub async fn invite_member(&self, cookie: String, req: InviteMemberRequest) -> Result<()> {
        let format_cookie = format!("{:?}", cookie);
        let token = format_cookie.replace("token=", "Bearer ").replace("\"", "");

        let client = ReqwestClient::new()?;

        let _res = client
            .post("/v1/members/invite")
            .header("Authorization", token)
            .json(&req)
            .send()
            .await?;

        Ok(())
    }
}
