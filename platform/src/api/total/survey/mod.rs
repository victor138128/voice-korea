use dioxus::prelude::{server, server_fn, ServerFnError};
use server_fn::codec::{GetUrl, Json};

use crate::models::survey::{SurveyStatus, SurveySummary, TotalSurveySummaries};

#[server(endpoint = "/total/survey", input=GetUrl, output=Json)]
pub async fn list_surveys() -> Result<TotalSurveySummaries, ServerFnError> {
    Ok(TotalSurveySummaries {
        surveys: vec![
            SurveySummary {
                r#type: SurveyStatus::Draft,
                title: "설문지 타이틀".to_string(),
                update_date: 1725548400,
                response_count: 0,
                total_response_count: 0,
            },
            SurveySummary {
                r#type: SurveyStatus::InProgress,
                title: "설문지 타이틀".to_string(),
                update_date: 1725548400,
                response_count: 1,
                total_response_count: 50,
            },
            SurveySummary {
                r#type: SurveyStatus::Finished,
                title: "설문지 타이틀".to_string(),
                update_date: 1725548400,
                response_count: 10,
                total_response_count: 50,
            },
            SurveySummary {
                r#type: SurveyStatus::Finished,
                title: "설문지 타이틀".to_string(),
                update_date: 1725548400,
                response_count: 10,
                total_response_count: 50,
            },
            SurveySummary {
                r#type: SurveyStatus::Draft,
                title: "설문지 타이틀".to_string(),
                update_date: 1725548400,
                response_count: 0,
                total_response_count: 0,
            },
            SurveySummary {
                r#type: SurveyStatus::InProgress,
                title: "설문지 타이틀".to_string(),
                update_date: 1725548400,
                response_count: 1,
                total_response_count: 50,
            },
            SurveySummary {
                r#type: SurveyStatus::Finished,
                title: "설문지 타이틀".to_string(),
                update_date: 1725548400,
                response_count: 10,
                total_response_count: 50,
            },
            SurveySummary {
                r#type: SurveyStatus::Finished,
                title: "설문지 타이틀".to_string(),
                update_date: 1725548400,
                response_count: 10,
                total_response_count: 50,
            },
        ],
    })
}
