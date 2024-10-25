#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_logger::tracing;

use crate::api::v1::surveys::{create_survey, CreateSurveyRequest};

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Controller {
    pub survey_title: Signal<String>,
}

impl Controller {
    pub fn init() -> Self {
        let ctrl = Self {
            survey_title: use_signal(|| "".to_string()),
        };

        use_context_provider(|| ctrl);

        ctrl
    }

    pub fn get_survey_title(&self) -> String {
        (self.survey_title)()
    }

    pub fn set_survey_title(&mut self, title: String) {
        self.survey_title.set(title);
    }

    pub async fn write_survey_title(&mut self) {
        tracing::info!("write survey title button clicked");
        let _ = create_survey(CreateSurveyRequest {}).await;
    }
}
