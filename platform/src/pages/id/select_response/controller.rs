#![allow(non_snake_case)]
use crate::api::v2::survey::{get_survey_draft, upsert_survey_draft};
use dioxus::prelude::*;
use models::prelude::{SurveyDraftStatus, UpsertSurveyDraftRequest};

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Controller {
    survey_response: Resource<models::prelude::Survey>,
    id: Signal<String>,
}

impl Controller {
    pub fn init(id: String) -> Self {
        let id_copy = id.clone();
        let survey_response: Resource<models::prelude::Survey> = use_resource(move || {
            let id_value = id.clone();
            async move {
                let survey = get_survey_draft(id_value).await;
                survey.unwrap_or_default()
            }
        });

        Self {
            survey_response,
            id: use_signal(|| id_copy.clone()),
        }
    }

    pub async fn back_button_clicked(&mut self) {
        let _ = upsert_survey_draft(UpsertSurveyDraftRequest {
            id: Some(self.get_survey_id()),
            status: Some(SurveyDraftStatus::Question),
            title: None,
            quotas: None,
            questions: None,
        })
        .await;
    }

    pub fn get_survey_id(&self) -> String {
        (self.id)()
    }

    pub fn get_title(&self) -> String {
        self.get_survey().title.clone()
    }

    pub fn get_survey(&self) -> models::prelude::Survey {
        match (self.survey_response.value())() {
            Some(value) => value,
            None => models::prelude::Survey::default(),
        }
    }
}
