use core::fmt;

use dioxus::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Props, Serialize, Deserialize)]
pub struct TotalSurveys {
    pub surveys: Vec<SurveySummary>,
}

#[derive(Debug, Clone, PartialEq, Props, Serialize, Deserialize)]
pub struct SurveySummary {
    pub r#type: SurveyStatus,
    pub title: String,
    pub update_date: u64,
    pub response_count: u64,
    pub total_response_count: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SurveyStatus {
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
            SurveyStatus::InProgress => write!(f, "in_progress"),
            SurveyStatus::Finished => write!(f, "finished"),
        }
    }
}
