#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_logger::tracing;

use crate::{
    api::v1::surveys::{get_survey, GetSurveyResponse},
    models::{question::Question, survey::SurveySummary},
};

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Controller {
    survey_response: Signal<GetSurveyResponse>,
    summary_clicked: Signal<bool>,
    panel_clicked: Signal<bool>,
    survey_list_clicked: Signal<bool>,
}

impl Controller {
    pub fn init() -> Self {
        let mut ctrl = Self {
            survey_response: use_signal(|| GetSurveyResponse::default()),
            summary_clicked: use_signal(|| false),
            panel_clicked: use_signal(|| false),
            survey_list_clicked: use_signal(|| false),
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

    pub fn get_questions(&self) -> Vec<Question> {
        (self.survey_response)().questions
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
        tracing::info!("clciked");
        self.survey_list_clicked
            .set(!self.get_survey_list_clicked());
    }
}
