use dioxus::prelude::{server, server_fn, ServerFnError};
use server_fn::codec::{GetUrl, Json};

use crate::models::question::{QuestionStatus, QuestionSummary, TotalQuestions};

#[server(endpoint = "/total/question", input=GetUrl, output=Json)]
pub async fn get_total_questions() -> Result<TotalQuestions, ServerFnError> {
    Ok(TotalQuestions {
        questions: vec![
            QuestionSummary {
                r#type: QuestionStatus::Draft,
                title: "설문지 타이틀".to_string(),
                update_date: 1725548400,
                response_count: 0,
                total_response_count: 0,
            },
            QuestionSummary {
                r#type: QuestionStatus::InProgress,
                title: "설문지 타이틀".to_string(),
                update_date: 1725548400,
                response_count: 1,
                total_response_count: 50,
            },
            QuestionSummary {
                r#type: QuestionStatus::Finished,
                title: "설문지 타이틀".to_string(),
                update_date: 1725548400,
                response_count: 10,
                total_response_count: 50,
            },
            QuestionSummary {
                r#type: QuestionStatus::Finished,
                title: "설문지 타이틀".to_string(),
                update_date: 1725548400,
                response_count: 10,
                total_response_count: 50,
            },
            QuestionSummary {
                r#type: QuestionStatus::Draft,
                title: "설문지 타이틀".to_string(),
                update_date: 1725548400,
                response_count: 0,
                total_response_count: 0,
            },
            QuestionSummary {
                r#type: QuestionStatus::InProgress,
                title: "설문지 타이틀".to_string(),
                update_date: 1725548400,
                response_count: 1,
                total_response_count: 50,
            },
            QuestionSummary {
                r#type: QuestionStatus::Finished,
                title: "설문지 타이틀".to_string(),
                update_date: 1725548400,
                response_count: 10,
                total_response_count: 50,
            },
            QuestionSummary {
                r#type: QuestionStatus::Finished,
                title: "설문지 타이틀".to_string(),
                update_date: 1725548400,
                response_count: 10,
                total_response_count: 50,
            },
        ],
    })
}
