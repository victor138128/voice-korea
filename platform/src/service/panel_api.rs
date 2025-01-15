pub type Result<T> = std::result::Result<T, ServerFnError>;

use std::collections::HashMap;

use dioxus::prelude::*;
use models::prelude::{PanelActionRequest, PanelSummary, UpsertPanelRequest};

use crate::{api::common::CommonQueryResponse, utils::api::ReqwestClient};

use super::{login_service::LoginService, organization_api::OrganizationApi};

#[derive(Debug, Clone, Copy)]
pub struct PanelApi {
    pub endpoint: Signal<String>,
    pub login_service: LoginService,
    pub organization_service: OrganizationApi,
}

impl PanelApi {
    pub fn init() {
        let login_service: LoginService = use_context();
        let organization_service: OrganizationApi = use_context();
        let srv = Self {
            endpoint: use_signal(|| {
                format!(
                    "{}",
                    option_env!("API_URL").unwrap_or("https://voice-korea-api.dev.biyard.co")
                )
            }),
            login_service,
            organization_service,
        };
        use_context_provider(|| srv);
    }

    pub async fn get_panel(&self, panel_id: String) -> Result<PanelSummary> {
        let token = self.get_token();
        let id = self.get_organization_id();

        let client = ReqwestClient::new()?;

        let res = client
            .get(&format!("/v1/panels/organizations/{id}/panels/{panel_id}"))
            .header("Authorization", token)
            .send()
            .await?;

        let res = res.error_for_status()?;

        let panel = res.json().await?;
        Ok(panel)
    }

    pub async fn upsert_panel(&self, req: UpsertPanelRequest) -> Result<UpsertPanelRequest> {
        let token = self.get_token();
        let id = self.get_organization_id();
        let client = ReqwestClient::new()?;

        let res = client
            .post(&format!("/v1/panels/organizations/{id}"))
            .header("Authorization", token)
            .json(&req)
            .send()
            .await?;

        Ok(res.json().await?)
    }

    pub async fn search_panel(&self, keyword: String) -> Result<CommonQueryResponse<PanelSummary>> {
        let token = self.get_token();
        let id = self.get_organization_id();

        let mut params = HashMap::new();
        params.insert("keyword", keyword.to_string());

        let client = ReqwestClient::new()?;

        let res = client
            .get(&format!("/v1/panels/organizations/{id}"))
            .query(&params)
            .header("Authorization", token)
            .send()
            .await?;

        let res = res.error_for_status()?;

        let panels = res.json().await?;
        Ok(panels)
    }

    pub async fn remove_panel(&self, panel_id: String) -> Result<()> {
        let token = self.get_token();
        let id = self.get_organization_id();
        let client = ReqwestClient::new()?;

        let _res = client
            .post(format!("/v1/panels/organizations/{}/panels/{}", id, panel_id).as_str())
            .header("Authorization", token)
            .json(&PanelActionRequest::Delete)
            .send()
            .await?;

        Ok(())
    }

    pub async fn list_panels(
        &self,
        size: Option<i64>,
        bookmark: Option<String>,
    ) -> Result<CommonQueryResponse<PanelSummary>> {
        let token = self.get_token();
        let id = self.get_organization_id();

        let mut params = HashMap::new();
        if let Some(size) = size {
            params.insert("size", size.to_string());
        }
        if let Some(bookmark) = bookmark {
            params.insert("bookmark", bookmark);
        }

        let client = ReqwestClient::new()?;

        let res = client
            .get(&format!("/v1/panels/organizations/{id}"))
            .query(&params)
            .header("Authorization", token)
            .send()
            .await?;

        let res = res.error_for_status()?;

        let panels = res.json().await?;
        Ok(panels)
    }

    pub fn get_organization_id(&self) -> String {
        let id = self.organization_service.get_selected_organization_id();
        id
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
