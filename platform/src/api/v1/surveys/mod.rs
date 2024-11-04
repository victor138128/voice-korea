#![allow(unused_imports)]
pub mod upsert_survey;

use dioxus::prelude::{
    server_fn::codec::{GetUrl, Json, PostUrl},
    *,
};
use dioxus_logger::tracing;
use serde::{Deserialize, Serialize};

use crate::{
    api::common::CommonQueryResponse,
    models::{
        question::{Question, QuestionAnswer, QuestionType},
        survey::{Age, Gender, ProofId, QuestionSequence, Quota, SurveyStatus, SurveySummary},
    },
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
pub enum Status {
    #[serde(rename = "draft")]
    Draft,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "finished")]
    Finished,
}

#[server(endpoint = "/v1/surveys/lists", input = GetUrl, output = Json)]
pub async fn list_surveys(
    email: String,
    _size: Option<i32>,
    _bookmark: Option<String>,
    _status: Option<Status>,
) -> Result<CommonQueryResponse<SurveySummary>, ServerFnError> {
    dioxus_logger::tracing::debug!("/v1/surveys/lists",);

    use crate::api::aws::ses::AuthKeyModel;
    use easy_dynamodb::error::DynamoException;

    let log = crate::utils::logger::new_api("GET", &format!("/v1/surveys/lists"));

    CommonQueryResponse::query(&log, "gsi1-index", None, Some(100), vec![("gsi1", email)]).await
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetSurveyResponse {
    pub survey: SurveySummary,
    pub responders: Vec<ProofId>,
    pub questions: Vec<Question>,
}

#[server(endpoint = "/v1/surveys/:survey-id", input = GetUrl, output = Json)]
pub async fn get_survey() -> Result<GetSurveyResponse, ServerFnError> {
    // dioxus_logger::tracing::debug!("/v1/surveys/:survey-id:");
    // use axum::extract::Path;

    // let Path(survey_id): Path<u64> = extract().await?;
    // let log = crate::utils::logger::new_api("GET", &format!("/v1/surveys/{:?}", survey_id));
    // let cli = crate::utils::db::get(&log);

    // let res: Result<Option<SurveySummary>, easy_dynamodb::error::DynamoException> =
    //     cli.get(format!("survey-{:?}", survey_id).as_str()).await;

    // let surveys = match res {
    //     Ok(v) => {
    //         match v {
    //             Some(survey) => Ok(survey),
    //             None =>
    //         }
    //     },
    //     Err(_e) => {
    //         vec![]
    //     }
    // };

    Ok(GetSurveyResponse {
        survey: SurveySummary {
            id: "survey-id3".to_string(),
            title: "설문지 타이틀2".to_string(),
            status: SurveyStatus::InProgress {
                started_at: 1725548400,
                ended_at: Some(1825548400),
            },
            updated_at: 1725548400,
            created_at: 1725548400,
            responses: Some(100),
            expected_responses: Some(200),
            questions: 3,
            quotas: Some(vec![
                Quota::Attribute {
                    salary_tier: Some(1),
                    region_code: Some(02),
                    gender: Some(Gender::Female),
                    age: Some(Age::Specific(20)),
                    quota: 100,
                },
                Quota::Attribute {
                    salary_tier: Some(1),
                    region_code: Some(02),
                    gender: Some(Gender::Male),
                    age: Some(Age::Range {
                        inclusive_min: 20,
                        inclusive_max: 30,
                    }),
                    quota: 100,
                },
                Quota::Panel("proof-id".to_string()),
            ]),
            r#type: "survey".to_string(),
            gsi1: "account-id".to_string(),
            gsi2: "status".to_string(),
        },
        responders: vec!["proof-id".to_string()],
        questions: vec![
            Question {
                id: "question-id".to_string(),
                survey_id: "survey-id3".to_string(),
                title: "질문 타이틀".to_string(),
                question: QuestionType::Text(Some("질문 내용".to_string())),
                gsi1: "survey-id3".to_string(),
            },
            Question {
                id: "question-id".to_string(),
                survey_id: "survey-id3".to_string(),
                title: "질문 타이틀".to_string(),
                question: QuestionType::Text(None),
                gsi1: "survey-id3".to_string(),
            },
            Question {
                id: "question-id".to_string(),
                survey_id: "survey-id3".to_string(),
                title: "질문 타이틀".to_string(),
                question: QuestionType::LongText(Some("질문 내용".to_string())),
                gsi1: "survey-id3".to_string(),
            },
            Question {
                id: "question-id".to_string(),
                survey_id: "survey-id3".to_string(),
                title: "질문 타이틀".to_string(),
                question: QuestionType::LongText(None),
                gsi1: "survey-id3".to_string(),
            },
            Question {
                id: "question-id".to_string(),
                survey_id: "survey-id3".to_string(),
                title: "질문 타이틀".to_string(),
                question: QuestionType::SingleChoice {
                    question: Some("질문 내용".to_string()),
                    options: vec!["선택지1".to_string(), "선택지2".to_string()],
                },
                gsi1: "survey-id3".to_string(),
            },
            Question {
                id: "question-id".to_string(),
                survey_id: "survey-id3".to_string(),
                title: "질문 타이틀".to_string(),
                question: QuestionType::SingleChoice {
                    question: None,
                    options: vec!["선택지1".to_string(), "선택지2".to_string()],
                },
                gsi1: "survey-id3".to_string(),
            },
        ],
    })
}

#[server(endpoint = "/v1/surveys/answer", input = Json, output = Json)]
pub async fn answer_survey(
    survey_id: String,
    answers: Vec<QuestionAnswer>,
) -> Result<(), ServerFnError> {
    dioxus_logger::tracing::debug!("/v1/surveys/:survey-id: {} {:?}", survey_id, answers);

    Ok(())
}
