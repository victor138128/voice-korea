pub type Result<T> = std::result::Result<T, ServerFnError>;

use std::collections::HashMap;

use dioxus::prelude::*;
use models::prelude::{
    Field, OpinionActionRequest, OpinionResponse, PanelInfo, ProjectStatus, UpsertOpinionRequest,
};

use crate::{api::common::CommonQueryResponse, utils::api::ReqwestClient};

use super::{login_service::LoginService, organization_api::OrganizationApi};

#[derive(Debug, Clone, Copy)]
pub struct OpinionApi {
    pub endpoint: Signal<String>,
    pub login_service: LoginService,
    pub organization_service: OrganizationApi,
}

impl OpinionApi {
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

    pub async fn remove_opinion(&self, project_id: String) -> Result<()> {
        let token = self.get_token();
        let id = self.get_organization_id();
        let client = ReqwestClient::new()?;

        let _res = client
            .post(&format!(
                "/v1/opinions/organizations/{id}/projects/{project_id}"
            ))
            .header("Authorization", token)
            .json(&OpinionActionRequest::Delete)
            .send()
            .await?;
        Ok(())
    }

    pub async fn update_project_status(
        &self,
        project_id: String,
        status: ProjectStatus,
    ) -> Result<()> {
        let token = self.get_token();
        let id = self.get_organization_id();
        let client = ReqwestClient::new()?;

        let _res = client
            .post(&format!(
                "/v1/opinions/organizations/{id}/projects/{project_id}"
            ))
            .header("Authorization", token)
            .json(&OpinionActionRequest::UpdateStatus(status))
            .send()
            .await?;
        Ok(())
    }

    pub async fn update_panels(&self, project_id: String, panels: Vec<PanelInfo>) -> Result<()> {
        let token = self.get_token();
        let id = self.get_organization_id();
        let client = ReqwestClient::new()?;

        let _res = client
            .post(&format!(
                "/v1/opinions/organizations/{id}/projects/{project_id}"
            ))
            .header("Authorization", token)
            .json(&OpinionActionRequest::UpdatePanels(panels))
            .send()
            .await?;
        Ok(())
    }

    pub async fn update_project_type(&self, project_id: String, project_type: Field) -> Result<()> {
        let token = self.get_token();
        let id = self.get_organization_id();
        let client = ReqwestClient::new()?;

        let _res = client
            .post(&format!(
                "/v1/opinions/organizations/{id}/projects/{project_id}"
            ))
            .header("Authorization", token)
            .json(&OpinionActionRequest::UpdateProjectType(project_type))
            .send()
            .await?;
        Ok(())
    }

    pub async fn upsert_opinion(&self, req: UpsertOpinionRequest) -> Result<UpsertOpinionRequest> {
        let token = self.get_token();
        let id = self.get_organization_id();
        let client = ReqwestClient::new()?;

        let res = client
            .post(&format!("/v1/opinions/organizations/{id}"))
            .header("Authorization", token)
            .json(&req)
            .send()
            .await?;

        Ok(res.json().await?)
    }

    pub async fn search_opinion(
        &self,
        keyword: String,
    ) -> Result<CommonQueryResponse<OpinionResponse>> {
        let token = self.get_token();
        let id = self.get_organization_id();
        let client = ReqwestClient::new()?;
        let mut params = HashMap::new();
        params.insert("keyword", keyword);

        let res = client
            .get(&format!("/v1/opinions/organizations/{id}/opinions"))
            .header("Authorization", token)
            .query(&params)
            .send()
            .await?;

        Ok(res.json().await?)
    }

    pub async fn get_opinion(&self, project_id: String) -> Result<OpinionResponse> {
        let token = self.get_token();
        let id = self.get_organization_id();
        let client = ReqwestClient::new()?;

        let res = client
            .get(&format!(
                "/v1/opinions/organizations/{id}/projects/{project_id}"
            ))
            .header("Authorization", token)
            .send()
            .await?;

        Ok(res.json().await?)
    }

    pub async fn list_opinions(
        &self,
        size: Option<i64>,
        bookmark: Option<String>,
    ) -> Result<CommonQueryResponse<OpinionResponse>> {
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
            .get(&format!("/v1/opinions/organizations/{id}"))
            .query(&params)
            .header("Authorization", token)
            .send()
            .await?;

        let res = res.error_for_status()?;

        let opinions = res.json().await?;
        Ok(opinions)
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
