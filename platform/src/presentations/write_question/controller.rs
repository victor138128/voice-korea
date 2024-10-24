#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_logger::tracing;

use crate::api::v1::surveys::{get_survey, GetSurveyResponse};

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Controller {
    survey: Signal<GetSurveyResponse>,
    step: Signal<u64>,
    question_types: Signal<Vec<QuestionOption>>,
    selected_question_types: Signal<u64>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct QuestionOption {
    pub value: i64,
    pub label: String,
}

impl Controller {
    pub fn init(_title: String) -> Self {
        let mut ctrl = Self {
            survey: use_signal(|| GetSurveyResponse::default()),
            step: use_signal(|| 0),
            question_types: use_signal(|| {
                vec![
                    QuestionOption {
                        value: 0,
                        label: "객관식 질문".to_string(),
                    },
                    QuestionOption {
                        value: 1,
                        label: "주관식 질문(단답형)".to_string(),
                    },
                    QuestionOption {
                        value: 2,
                        label: "주관식 질문(서술형)".to_string(),
                    },
                ]
            }),
            selected_question_types: use_signal(|| 0),
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

    pub fn get_question_types(&mut self) -> Vec<QuestionOption> {
        (self.question_types)()
    }

    pub fn get_selected_question(&mut self) -> u64 {
        (self.selected_question_types)()
    }

    pub fn get_survey(&mut self) -> GetSurveyResponse {
        (self.survey)()
    }

    pub fn get_step(&mut self) -> u64 {
        (self.step)()
    }

    pub fn change_step(&mut self, step: u64) {
        self.step.set(step);
    }

    pub fn change_selected_question(&mut self, selected: u64) {
        self.selected_question_types.set(selected);
    }
}
