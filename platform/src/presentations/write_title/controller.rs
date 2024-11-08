#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_logger::tracing;

use crate::{
    api::v1::surveys::{
        upsert_survey::{upsert_survey, SurveyUpdateItem},
        GetSurveyResponse,
    },
    models::survey::StatusType,
    service::login_service::use_login_service,
};

use super::{Language, Route};

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Controller {
    survey: Signal<GetSurveyResponse>,
    pub survey_title: Signal<String>,
    pub survey_id: Signal<String>,
}

impl Controller {
    pub fn init(lang: Language, id: String) -> Self {
        let navigator = use_navigator();
        let email: String = use_login_service().get_email().clone();

        if email.is_empty() {
            navigator.push(Route::LoginPage { lang });
        };

        let mut ctrl = Self {
            survey: use_signal(|| GetSurveyResponse::default()),
            survey_title: use_signal(|| "".to_string()),
            survey_id: use_signal(|| "".to_string()),
        };

        ctrl.survey_id.set(id.clone());

        tracing::debug!("url: /v1/email/{}/surveys/{}", id, email);

        let _ = use_effect(move || {
            let id_value = id.clone();
            let email_value = email.clone();
            spawn(async move {
                let res = async move {
                    crate::utils::api::get::<GetSurveyResponse>(&format!(
                        "/v1/email/{}/surveys/{}",
                        email_value, id_value
                    ))
                    .await
                }
                .await;

                tracing::debug!("survey title: {}", res.clone().survey.title);

                ctrl.survey_title.set(res.clone().survey.title);
                ctrl.survey.set(res)
            });
        });

        use_context_provider(|| ctrl);

        ctrl
    }

    // pub fn get_survey(&self) -> GetSurveyResponse {
    //     (self.survey)()
    // }

    pub fn get_survey_title(&self) -> String {
        (self.survey_title)()
    }

    pub fn set_survey_title(&mut self, title: String) {
        self.survey_title.set(title);
    }

    pub fn get_survey_id(&self) -> String {
        (self.survey_id)()
    }

    pub async fn write_survey_title(&mut self, status: StatusType, title: String) {
        tracing::info!("write survey title button clicked {title}");
        let email = use_login_service().get_email().clone();
        let _ = upsert_survey(
            email,
            self.get_survey_id(),
            status,
            SurveyUpdateItem::Title(title),
        )
        .await;
    }
}
