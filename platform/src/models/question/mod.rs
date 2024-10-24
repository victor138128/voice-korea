use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Question {
    pub id: String,
    pub survey_id: String,
    pub title: String,
    pub question: QuestionType,

    // list questions by survey id
    #[serde(skip)]
    pub gsi1: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuestionType {
    Text(Option<String>),
    LongText(Option<String>),
    SingleChoice {
        // question is a description of the question
        question: Option<String>,
        options: Vec<String>,
    },
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuestionAnswer {
    Text(String),
    LongText(String),
    SingleChoice(usize),
}
