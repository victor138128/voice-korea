#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_logger::tracing;

use crate::api::v1::surveys::{get_survey, GetSurveyResponse};

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Controller {
    survey_response: Signal<GetSurveyResponse>,
}

impl Controller {
    pub fn init(_title: String) -> Self {
        let mut ctrl = Self {
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
}
