use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct CreateOpinionRequest {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct OpinionResponse {
    pub project_id: String,
    pub opinion_type: OpinionType,
    pub project_name: String,
    pub total_response_count: u64,
    pub response_count: u64,
    pub panels: Vec<PanelInfo>,
    pub start_date: u64,
    pub end_date: u64,
    pub status: ProjectStatus,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
pub struct PanelInfo {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ProjectStatus {
    #[default]
    Ready,
    InProgress,
    Finish,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum OpinionType {
    #[default]
    Economy,
    Society,
    Environment,
    Education,
    Culture,
    Labor,
    City,
    Technology,
    Health,
    Politics,
}
