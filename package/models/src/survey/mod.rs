use core::fmt;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Survey {
    pub id: u32,
    pub draft_id: Option<String>,
    pub draft_status: Option<SurveyDraftStatus>,
    pub title: String,
    pub description: Option<String>,
    pub status: SurveyStatus,
    #[serde(rename = "startedAt")]
    pub created_at: String,
    #[serde(rename = "endedAt")]
    pub ended_at: String,
    #[serde(rename = "rewardPoints")]
    pub reward_points: u32,
    #[serde(rename = "questionCount")]
    pub question_count: u32,
    participated: bool,
    #[serde(rename = "estimatedMinutes")]
    pub estimated_minutes: u32,
    pub quotas: Vec<Quota>,
    pub questions: Vec<SurveyQuestion>,
}

impl Survey {
    pub fn new() -> Survey {
        Survey::default()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum SurveyStatus {
    #[default]
    #[serde(rename = "draft")]
    Draft,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "finished")]
    Finished,
}

impl fmt::Display for SurveyStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SurveyStatus::Draft => write!(f, "draft"),
            SurveyStatus::InProgress {} => {
                write!(f, "in_progress")
            }
            SurveyStatus::Finished {} => write!(f, "finished"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SurveyQuestion {
    #[serde(rename = "id")]
    pub question_id: Option<u64>,
    pub title: String,
    pub description: Option<String>,
    #[serde(rename = "answerType")]
    pub answer_type: SurveyQuestionType,
    pub options: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SurveyQuestionType {
    #[serde(rename = "MULTIPLE_CHOICE")]
    MultipleChoice,
    #[serde(rename = "SINGLE_CHOICE")]
    SingleChoice,
    #[serde(rename = "LONG_ANSWER")]
    LongAnswer,
    #[serde(rename = "SHORT_ANSWER")]
    ShortAnswer,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum QuestionSequence {
    #[default]
    #[serde(rename = "title")]
    Title,
    #[serde(rename = "add_question")]
    AddQuestion,
    #[serde(rename = "select_response")]
    SelectResponse,
    #[serde(rename = "summary")]
    Summary,
}

impl fmt::Display for QuestionSequence {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            QuestionSequence::Title => write!(f, "title"),
            QuestionSequence::AddQuestion => write!(f, "add_question"),
            QuestionSequence::SelectResponse => write!(f, "select_response"),
            QuestionSequence::Summary => write!(f, "summary"),
        }
    }
}

// #[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
// #[serde(rename_all = "snake_case")]
// pub enum Quota {
//     Attribute {
//         // e.g. 1, 2, 3, 4, 5
//         salary_tier: Option<SalaryTier>,
//         // e.g. 02(Seoul), 051(Busan) and so on.
//         region_code: Option<RegionCode>,
//         gender: Option<Gender>,
//         age: Option<Age>,
//         quota: u64,
//     },
//     Panel(ProofId),
// }

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Quota {
    pub attribute: Option<Attribute>,
    pub panel: Option<Panel>,
    pub quota: u64,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Attribute {
    // e.g. 1, 2, 3, 4, 5
    pub salary_tier: Option<SalaryTier>,
    // e.g. 02(Seoul), 051(Busan) and so on.
    pub region_code: Option<RegionCode>,
    pub gender: Option<Gender>,
    pub age: Option<Age>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Panel {
    panel: Vec<ProofId>,
}

// SalaryTier means the annual salary range of the respondent.
// 0: 0 ~ 9,999,999
// 1: 10,000,000 ~ 19,999,999
// 2: 20,000,000 ~ 29,999,999
// ..
pub type SalaryTier = u16;
pub type RegionCode = u16;

pub type ProofId = String;

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Gender {
    Male,
    Female,
    Others,
}

impl<'de> Deserialize<'de> for Gender {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;
        match s {
            "남자" => Ok(Gender::Male),
            "여자" => Ok(Gender::Female),
            _ => Ok(Gender::Others),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Age {
    Specific(u8),
    Range {
        #[serde(rename = "inclusiveMin")]
        inclusive_min: Option<u8>,
        #[serde(rename = "inclusiveMax")]
        inclusive_max: Option<u8>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct SurveySummary {
    pub id: u32,
    pub draft_id: Option<String>,
    pub draft_status: Option<SurveyDraftStatus>,
    pub title: String,
    pub description: Option<String>,
    pub status: SurveyStatus,
    #[serde(rename = "startedAt")]
    pub started_at: String,
    #[serde(rename = "endedAt")]
    pub ended_at: String,
    #[serde(rename = "rewardPoints")]
    pub reward_points: u32,
    #[serde(rename = "questionCount")]
    pub question_count: u32,
    participated: bool,
    #[serde(rename = "estimatedMinutes")]
    pub estimated_minutes: u32,
    //FIXME: remove serde(default)
    #[serde(default)]
    pub updated_at: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct SurveyDocument {
    pub id: String,
    pub r#type: String,
    pub gsi1: String,
    pub created_at: i64,
    pub updated_at: i64,
    pub status: SurveyDraftStatus,

    pub title: String,
    pub quotas: Vec<Quota>,
    pub questions: Vec<SurveyQuestion>,
}
impl SurveyDocument {
    pub fn new(id: String) -> Self {
        let mut survey = SurveyDocument::default();
        survey.gsi1 = SurveyDocument::gsi1(id.clone());
        survey.id = id;
        survey.r#type = SurveyDocument::get_type();
        survey.created_at = chrono::Utc::now().timestamp_millis();
        survey.updated_at = chrono::Utc::now().timestamp_millis();
        survey
    }
    pub fn gsi1(id: String) -> String {
        format!("{}/{}", Self::get_type(), id)
    }
    pub fn get_type() -> String {
        "survey_draft".to_string()
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSurveyRequest {
    pub draft_id: String,
    pub started_at: u64, //TIMESTAMP
    pub ended_at: u64,   //TIMESTAMP
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSurveyResponse {
    pub survey_id: u32,
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListSurveyDraftRequest {
    pub size: Option<i32>,
    pub bookmark: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListSurveyDraftResponse {
    pub bookmark: Option<String>,
    pub survey: Vec<SurveySummary>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct CreateSurveyDto {
    pub custom_id: String,
    pub status: SurveyStatus,
    pub started_at: u64,
    pub ended_at: u64,
    pub title: String,
    pub quotas: Vec<Quota>,
    pub questions: Vec<SurveyQuestion>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum SurveyDraftStatus {
    #[default]
    Title,
    Quotas,
    Question,
    Complete,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct UpsertSurveyDraftRequest {
    pub id: Option<String>,
    pub status: Option<SurveyDraftStatus>,
    pub title: Option<String>,
    pub quotas: Option<Vec<Quota>>,
    pub questions: Option<Vec<SurveyQuestion>>,
}
