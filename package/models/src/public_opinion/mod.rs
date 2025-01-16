use serde::{Deserialize, Serialize};

use crate::{field::Field, group::MemberInfo};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
pub struct CreateOpinionRequest {
    pub status: Option<OpinionDraftStatus>,
    pub opinions: Option<Vec<OpinionInfo>>,
    pub informations: Option<OpinionInformation>,
    pub committees: Option<Vec<Vec<MemberInfo>>>,
    pub panels: Option<UpsertPanelInfo>,
    pub discussions: Option<DiscussionInfo>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
pub struct UpdateOpinionRequest {
    pub status: Option<OpinionDraftStatus>,
    pub opinions: Option<Vec<OpinionInfo>>,
    pub informations: Option<OpinionInformation>,
    pub committees: Option<Vec<Vec<MemberInfo>>>,
    pub panels: Option<UpsertPanelInfo>,
    pub discussions: Option<DiscussionInfo>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
pub struct DiscussionInfo {
    pub groups: DiscussionGroupInfo,
    pub meetings: Vec<MeetingInfo>,
    pub schedules: Vec<ScheduleInfo>,
    pub documents: Vec<Document>,
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OpinionActionRequest {
    Create(CreateOpinionRequest),
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OpinionByIdActionRequest {
    Delete,
    Update(UpdateOpinionRequest),
    UpdateProjectType(Field),
    UpdatePanels(Vec<PanelInfo>),
    UpdateStatus(ProjectStatus),
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
pub struct ScheduleInfo {
    pub title: String,
    pub schedules: Vec<ScheduleDetailInfo>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
pub struct ScheduleDetailInfo {
    pub start_date: u64,
    pub end_date: u64,
    pub title: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
pub struct MeetingInfo {
    pub meeting_type: MeetingType,
    pub title: String,
    pub start_date: u64,
    pub end_date: u64,
    pub discussion_group: Vec<DiscussionGroupInfo>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
pub enum MeetingType {
    #[default]
    Offline,
    Online,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
pub struct DiscussionGroupInfo {
    pub panels: Vec<PanelInfo>,
    pub panel_count: u64,
    pub groups: Vec<DiscussionGroupDetailInfo>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
pub struct DiscussionGroupDetailInfo {
    pub name: String,
    pub discussion_count: u64,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
pub struct UpsertPanelInfo {
    pub totals: u64,
    pub allocation_method: AllocationMethod,
    pub panels: Vec<PanelAttribute>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
pub struct PanelAttribute {
    pub panel_count: u64,
    pub attributes: Vec<AttributeInfo>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
pub struct AttributeInfo {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
pub enum AllocationMethod {
    #[default]
    FairAllocated,
    ProportionalAllocation,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
pub struct OpinionInformation {
    pub opinion_type: Option<Field>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub documents: Vec<Document>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
pub struct Document {
    pub url: String,
    pub name: String,
    pub volume: Option<String>, //etc. 3.5 MB
    pub projects: Option<ProjectInfo>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
pub struct ProjectInfo {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
pub struct OpinionInfo {
    pub name: String,
    pub start_date: Option<u64>,
    pub end_date: Option<u64>,
    pub public_opinion_type: Option<PublicOpinionType>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
pub enum PublicOpinionType {
    #[default]
    General,
    Video,
    Post,
    Vote,
    Report,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
pub enum OpinionDraftStatus {
    #[default]
    Init,
    PublicOpinionComposition,
    InputInformation,
    CommitteeComposition,
    PanelComposition,
    DiscussionSetting,
    Finish,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct OpinionResponse {
    pub project_id: String,
    pub opinion_type: Field,
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

impl std::fmt::Display for ProjectStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProjectStatus::Ready => write!(f, "Ready"),
            ProjectStatus::InProgress => write!(f, "In Progress"),
            ProjectStatus::Finish => write!(f, "Finish"),
        }
    }
}
