use serde::{Deserialize, Serialize};

use crate::group::MemberInfo;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct UpsertOpinionRequest {
    pub id: Option<String>,
    pub status: Option<OpinionDraftStatus>,
    pub opinions: Option<Vec<OpinionInfo>>,
    pub informations: Option<OpinionInformation>,
    pub committees: Option<Vec<Vec<MemberInfo>>>,
    pub panels: Option<UpsertPanelInfo>,
    pub discussions: Option<DiscussionInfo>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct DiscussionInfo {
    pub groups: DiscussionGroupInfo,
    pub meetings: Vec<MeetingInfo>,
    pub schedules: Vec<ScheduleInfo>,
    pub documents: Vec<Document>,
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OpinionActionRequest {
    Delete,
    UpdateProjectType(OpinionType),
    UpdatePanels(Vec<PanelInfo>),
    UpdateStatus(ProjectStatus),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ScheduleInfo {
    pub title: String,
    pub schedules: Vec<ScheduleDetailInfo>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ScheduleDetailInfo {
    pub start_date: u64,
    pub end_date: u64,
    pub title: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MeetingInfo {
    pub meeting_type: MeetingType,
    pub title: String,
    pub start_date: u64,
    pub end_date: u64,
    pub discussion_group: Vec<DiscussionGroupInfo>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum MeetingType {
    #[default]
    Offline,
    Online,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct DiscussionGroupInfo {
    pub panels: Vec<PanelInfo>,
    pub panel_count: u64,
    pub groups: Vec<DiscussionGroupDetailInfo>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct DiscussionGroupDetailInfo {
    pub name: String,
    pub discussion_count: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct UpsertPanelInfo {
    pub totals: u64,
    pub allocation_method: AllocationMethod,
    pub panels: Vec<PanelAttribute>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct PanelAttribute {
    pub panel_count: u64,
    pub attributes: Vec<AttributeInfo>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct AttributeInfo {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum AllocationMethod {
    #[default]
    FairAllocated,
    ProportionalAllocation,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct OpinionInformation {
    pub opinion_type: OpinionType,
    pub title: String,
    pub description: String,
    pub documents: Vec<Document>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Document {
    pub url: String,
    pub name: String,
    pub volume: Option<String>, //etc. 3.5 MB
    pub projects: Option<ProjectInfo>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ProjectInfo {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct OpinionInfo {
    pub name: String,
    pub start_date: u64,
    pub end_date: u64,
    pub public_opinion_type: PublicOpinionType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum PublicOpinionType {
    #[default]
    General,
    Video,
    Post,
    Vote,
    Report,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
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

impl std::fmt::Display for ProjectStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProjectStatus::Ready => write!(f, "준비"),
            ProjectStatus::InProgress => write!(f, "진행"),
            ProjectStatus::Finish => write!(f, "마감"),
        }
    }
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

impl std::fmt::Display for OpinionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpinionType::Economy => write!(f, "경제"),
            OpinionType::Society => write!(f, "사회"),
            OpinionType::Environment => write!(f, "환경"),
            OpinionType::Education => write!(f, "교육"),
            OpinionType::Culture => write!(f, "문화"),
            OpinionType::Labor => write!(f, "노동"),
            OpinionType::City => write!(f, "도시"),
            OpinionType::Technology => write!(f, "기술"),
            OpinionType::Health => write!(f, "보건"),
            OpinionType::Politics => write!(f, "정치"),
        }
    }
}
