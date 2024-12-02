#![allow(non_snake_case)]
use crate::{
    api::v1::surveys::upsert_survey::{upsert_survey, SurveyUpdateItem},
    models::survey::StatusType,
};
use dioxus::prelude::*;

use crate::{api::v1::surveys::GetSurveyResponse, service::login_service::use_login_service};

use super::{Language, Route};

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Controller {
    survey_response: Resource<GetSurveyResponse>,
}

impl Controller {
    pub fn init(lang: Language, id: String) -> Self {
        let navigator = use_navigator();
        let email: String = use_login_service().get_email().clone();

        if email.is_empty() {
            navigator.push(Route::LoginPage { lang });
        };

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

        Self { survey_response }
    }

    pub async fn back_button_clicked(&mut self) {
        let email: String = use_login_service().get_email().clone();
        let survey = self.get_survey();

        let _ = upsert_survey(
            email.clone(),
            survey.survey.id.clone(),
            StatusType::Back,
            SurveyUpdateItem::AddResponder(crate::models::survey::Quota::Panel("".to_string())),
        )
        .await;
    }

    pub fn get_title(&self) -> String {
        self.get_survey().survey.title.clone()
    }

    pub fn get_survey(&self) -> GetSurveyResponse {
        match (self.survey_response.value())() {
            Some(value) => value,
            None => GetSurveyResponse::default(),
        }
    }
}
