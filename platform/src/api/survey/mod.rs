use dioxus::prelude::{server, server_fn, ServerFnError};
use server_fn::codec::{GetUrl, Json};

use crate::models::survey::{Survey, SurveyQuestion, SurveyQuestionType};

#[server(endpoint = "/survey", input=GetUrl, output=Json)]
pub async fn get_survey(title: String) -> Result<Survey, ServerFnError> {
    Ok(Survey {
        title,
        questions: vec![
            SurveyQuestion {
                question_type: SurveyQuestionType::MultipleChoice,
                question_id: 1,
                question: "테스트 질문".to_string(),
                answers: vec!["테스트 선지 1".to_string(), "테스트 선지 2".to_string()],
            },
            SurveyQuestion {
                question_type: SurveyQuestionType::Subjective,
                question_id: 2,
                question: "테스트 질문".to_string(),
                answers: vec![],
            },
            SurveyQuestion {
                question_type: SurveyQuestionType::MultipleChoice,
                question_id: 3,
                question: "테스트 질문".to_string(),
                answers: vec!["테스트 선지 1".to_string(), "테스트 선지 2".to_string()],
            },
            SurveyQuestion {
                question_type: SurveyQuestionType::MultipleChoice,
                question_id: 4,
                question: "테스트 질문".to_string(),
                answers: vec![],
            },
        ],
    })
}
