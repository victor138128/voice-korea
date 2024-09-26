use dioxus::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Props, Serialize, Deserialize)]
pub struct TotalQuestions {
    pub questions: Vec<QuestionSummary>,
}

#[derive(Debug, Clone, PartialEq, Props, Serialize, Deserialize)]
pub struct QuestionSummary {
    pub r#type: QuestionStatus,
    pub title: String,
    pub update_date: u64,
    pub response_count: u64,
    pub total_response_count: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum QuestionStatus {
    #[serde(rename = "draft")]
    Draft,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "finished")]
    Finished,
}
