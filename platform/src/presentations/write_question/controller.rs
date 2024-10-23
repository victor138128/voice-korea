#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_logger::tracing;

use crate::{
    api::v1::surveys::{get_survey, GetSurveyResponse},
    models::survey::Survey,
};

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Controller {
    survey: Signal<GetSurveyResponse>,
}

impl Controller {
    pub fn init(title: String) -> Self {
        let mut ctrl = Self {
            survey: use_signal(|| GetSurveyResponse::default()),
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

        ctrl
    }

    pub fn get_survey(&mut self) -> GetSurveyResponse {
        (self.survey)()
    }
}
