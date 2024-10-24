#![allow(unused_imports)]
use dioxus::prelude::{
    server_fn::codec::{GetUrl, Json, PostUrl},
    *,
};
use serde::{Deserialize, Serialize};

use crate::models::{question::QuestionType, survey::Quota};

#[server(endpoint = "/v1/surveys", input = Json, output = Json)]
pub async fn upsert_survey(survey_id: String, item: SurveyUpdateItem) -> Result<(), ServerFnError> {
    dioxus_logger::tracing::debug!("/v1/surveys: {:?} {:?}", survey_id, item);

    Ok(())
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
pub enum SurveyUpdateItem {
    Title(String),
    AddQuestion {
        title: String,
        question: QuestionType,
    },
    UpdateQuestion {
        id: String,
        title: Option<String>,
        question: Option<QuestionType>,
    },
    // RemoveRequestion requires question id to be removed.
    RemoveQuestion(String),

    AddResponder(Quota),
    // RemoveResponder requires quota id to be removed.
    RemoveResponder(String),

    // SetPeriod requires epoch timestamp in seconds for start and end time.
    SetPeriod(u64, u64),
}
