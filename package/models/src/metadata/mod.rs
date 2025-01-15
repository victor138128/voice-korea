use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct GetPutObjectUriRequest {
    pub file_name: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct GetPutObjectUriResponse {
    pub presigned_uris: Vec<String>,
    pub uris: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct UpsertMetadataRequest {
    pub id: Option<String>,
    pub name: String,
    pub urls: Vec<String>,
    pub metadata_type: Option<MetadataType>,
    pub metadata_field: Option<MetadataField>,
    pub metadata_purpose: Option<MetadataPurpose>,
    pub metadata_source: Option<MetadataSource>,
    pub metadata_authority: Option<MetadataAuthority>,

    pub public_opinion_projects: Option<Vec<PublicOpinion>>,
    pub public_survey_projects: Option<Vec<PublicSurvey>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct MetadataSummary {
    pub id: String,
    pub name: String,
    pub urls: Vec<String>,
    pub metadata_type: Option<MetadataType>,
    pub metadata_field: Option<MetadataField>,
    pub metadata_purpose: Option<MetadataPurpose>,
    pub metadata_source: Option<MetadataSource>,
    pub metadata_authority: Option<MetadataAuthority>,

    pub public_opinion_projects: Option<Vec<PublicOpinion>>,
    pub public_survey_projects: Option<Vec<PublicSurvey>>,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MetadataActionRequest {
    Delete,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct PublicOpinion {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct PublicSurvey {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum MetadataAuthority {
    #[default]
    Public,
    Private,
    Restricted,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum MetadataSource {
    #[default]
    Internal,
    External,
    Goverment,
    Company,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum MetadataPurpose {
    #[default]
    DevelopmentPolicy,
    AcademicResearch,
    PublicDiscussion,
    Education,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum MetadataType {
    #[default]
    Report,
    Statistics,
    Survey,
    Thesis,
    Presentation,
    Media,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum MetadataField {
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
