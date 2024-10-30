#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_logger::tracing;

use crate::api::v1::surveys::{
    get_survey,
    upsert_survey::{upsert_survey, SurveyUpdateItem},
    GetSurveyResponse,
};

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Controller {
    survey: Signal<GetSurveyResponse>,
    pub survey_title: Signal<String>,
}

impl Controller {
    pub fn init() -> Self {
        let mut ctrl = Self {
            survey: use_signal(|| GetSurveyResponse::default()),
            survey_title: use_signal(|| "".to_string()),
        };

        let _ = use_effect(move || {
            spawn(async move {
                match get_survey().await {
                    Ok(res) => {
                        ctrl.survey.set(res);
                    }
                    Err(e) => {
                        tracing::error!("Error: {:?}", e);
                    }
                }
            });
        });

        use_context_provider(|| ctrl);

        ctrl
    }

    pub fn get_survey(&self) -> GetSurveyResponse {
        (self.survey)()
    }

    pub fn get_survey_title(&self) -> String {
        (self.survey_title)()
    }

    pub fn set_survey_title(&mut self, title: String) {
        self.survey_title.set(title);
    }

    pub async fn write_survey_title(&mut self, title: String) {
        tracing::info!("write survey title button clicked");
        let survey = self.get_survey();
        let _ = upsert_survey(survey.survey.id, SurveyUpdateItem::Title(title)).await;
    }
}
