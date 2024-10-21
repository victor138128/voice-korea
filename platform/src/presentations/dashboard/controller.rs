#![allow(non_snake_case)]
use std::time::{Duration, UNIX_EPOCH};

use chrono::{self, DateTime, Local};
use dioxus::prelude::*;
use dioxus_logger::tracing;

use crate::api::v1::surveys::list_surveys;

#[derive(Debug, Clone, PartialEq)]
pub struct Survey {
    pub survey_type: String,
    pub title: String,
    pub update_date: String,
    pub response_count: u64,
    pub total_response_count: u64,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Controller {
    pub surveys: Signal<Vec<Survey>>,
    pub clicked_type: Signal<u64>, //0: type-1, 1: type-2
}

impl Controller {
    pub fn init() -> Self {
        let mut ctrl = Self {
            surveys: use_signal(|| vec![]),
            clicked_type: use_signal(|| 0),
        };

        let _ = use_effect(move || {
            spawn(async move {
                match list_surveys(None, None, None).await {
                    Ok(res) => {
                        let surveys = res.items;

                        let total_surveys: Vec<Survey> = surveys
                            .into_iter()
                            .map(|survey| Survey {
                                survey_type: survey.r#type.to_string(),
                                title: survey.title,
                                update_date: Self::format_date(survey.updated_at),
                                response_count: survey.responses.unwrap_or_default(),
                                total_response_count: survey.expected_responses.unwrap_or_default(),
                            })
                            .collect();
                        ctrl.surveys.set(total_surveys);
                    }
                    Err(e) => {
                        tracing::error!("Error: {:?}", e);
                    }
                }
            });
        });

        ctrl
    }

    pub fn format_date(timestamp: u64) -> String {
        let d = UNIX_EPOCH + Duration::from_secs(timestamp);
        let datetime = DateTime::<Local>::from(d);
        datetime.format("%Y.%m.%d").to_string()
    }

    pub fn get_clicked_type(&mut self) -> u64 {
        (self.clicked_type)()
    }

    pub fn set_clicked_type(&mut self, clicked_type: u64) {
        self.clicked_type.set(clicked_type);
    }

    pub fn get_total_questions(&mut self) -> Vec<Survey> {
        (self.surveys)()
    }
}
