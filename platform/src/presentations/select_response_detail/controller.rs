#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_logger::tracing;

use crate::api::v1::surveys::{get_survey, GetSurveyResponse};

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Controller {
    response_count: Signal<String>,
    survey_response: Signal<GetSurveyResponse>,
}

impl Controller {
    pub fn init() -> Self {
        let mut ctrl = Self {
            response_count: use_signal(|| "0".to_string()),
            survey_response: use_signal(|| GetSurveyResponse::default()),
        };

        let _ = use_effect(move || {
            spawn(async move {
                match get_survey().await {
                    Ok(res) => {
                        ctrl.survey_response.set(res);
                    }
                    Err(e) => {
                        tracing::error!("Error: {:?}", e);
                    }
                }
            });
        });

        ctrl
    }

    pub fn get_title(&self) -> String {
        self.get_survey().survey.title.clone()
    }

    pub fn get_survey(&self) -> GetSurveyResponse {
        (self.survey_response)()
    }

    pub fn get_response_count(&mut self) -> String {
        (self.response_count)()
    }

    pub fn set_response_count(&mut self, response_count: String) {
        self.response_count.set(response_count);
    }
}
