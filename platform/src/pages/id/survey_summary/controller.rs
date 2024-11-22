#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::{
    api::v1::surveys::GetSurveyResponse, models::question::Question,
    service::login_service::use_login_service,
};

use super::{Language, Route};

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Controller {
    survey: Resource<GetSurveyResponse>,
    summary_clicked: Signal<bool>,
    panel_clicked: Signal<bool>,
    survey_list_clicked: Signal<bool>,
    pub survey_id: Signal<String>,
}

impl Controller {
    pub fn init(lang: Language, id: String) -> Self {
        let navigator = use_navigator();
        let email: String = use_login_service().get_email().clone();

        if email.is_empty() {
            navigator.push(Route::LoginPage { lang });
        };

        let id_copy = id.clone();

        let survey_response = use_resource(move || {
            let id_value = id.clone();
            let email_value = email.clone();
            async move {
                crate::utils::api::get::<GetSurveyResponse>(&format!(
                    "/v1/email/{}/surveys/{}",
                    email_value, id_value
                ))
                .await
            }
        });

        let mut ctrl = Self {
            survey: survey_response,
            summary_clicked: use_signal(|| false),
            panel_clicked: use_signal(|| false),
            survey_list_clicked: use_signal(|| false),
            survey_id: use_signal(|| "".to_string()),
        };

        ctrl.survey_id.set(id_copy.clone());

        ctrl
    }

    pub fn get_questions(&self) -> Vec<Question> {
        self.get_survey().questions
    }

    pub fn get_survey(&self) -> GetSurveyResponse {
        match (self.survey.value())() {
            Some(value) => value,
            None => GetSurveyResponse::default(),
        }
    }

    pub fn get_summary_clicked(&self) -> bool {
        (self.summary_clicked)()
    }

    pub fn get_panel_clicked(&self) -> bool {
        (self.panel_clicked)()
    }

    pub fn get_survey_list_clicked(&self) -> bool {
        (self.survey_list_clicked)()
    }

    pub fn change_summary_clicked(&mut self) {
        self.summary_clicked.set(!self.get_summary_clicked());
    }

    pub fn change_panel_clicked(&mut self) {
        self.panel_clicked.set(!self.get_panel_clicked());
    }

    pub fn change_survey_list_clicked(&mut self) {
        self.survey_list_clicked
            .set(!self.get_survey_list_clicked());
    }
}
