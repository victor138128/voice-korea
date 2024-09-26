#![allow(non_snake_case)]
use std::time::{Duration, UNIX_EPOCH};

use chrono::{self, DateTime, Local, Utc};
use dioxus::prelude::*;
use dioxus_logger::tracing;

use crate::{
    api::question::get_total_questions,
    models::question::{QuestionStatus, QuestionSummary, TotalQuestions},
};

#[derive(Debug, Clone, PartialEq)]
pub struct QuestionDashboards {
    pub question_type: String,
    pub title: String,
    pub update_date: String,
    pub response_count: u64,
    pub total_response_count: u64,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Controller {
    pub questions: Signal<Vec<QuestionDashboards>>,
    pub clicked_type: Signal<u64>, //0: type-1, 1: type-2
}

impl Controller {
    pub fn init() -> Self {
        let mut ctrl = Self {
            questions: use_signal(|| vec![]),
            clicked_type: use_signal(|| 0),
        };

        let _ = use_effect(move || {
            spawn(async move {
                match get_total_questions().await {
                    Ok(res) => {
                        let questions = res.questions;
                        let mut total_questions: Vec<QuestionDashboards> = vec![];

                        for i in 0..questions.len() {
                            let question = &questions[i];
                            let question_type = if question.r#type == QuestionStatus::Draft {
                                "draft"
                            } else if question.r#type == QuestionStatus::InProgress {
                                "in_progress"
                            } else {
                                "finished"
                            };

                            let d = UNIX_EPOCH + Duration::from_secs(questions[i].update_date);
                            let datetime = DateTime::<Local>::from(d);
                            let timestamp_str = datetime.format("%Y-%m-%d").to_string();
                            total_questions.push(QuestionDashboards {
                                question_type: question_type.to_string(),
                                title: question.title.clone(),
                                update_date: timestamp_str,
                                response_count: question.response_count,
                                total_response_count: question.total_response_count,
                            });
                        }
                        ctrl.questions.set(total_questions);
                    }
                    Err(e) => {
                        tracing::error!("Error: {:?}", e);
                    }
                }
            });
        });

        ctrl
    }

    pub fn get_clicked_type(&mut self) -> u64 {
        (self.clicked_type)()
    }

    pub fn set_clicked_type(&mut self, clicked_type: u64) {
        self.clicked_type.set(clicked_type);
    }

    pub fn get_total_questions(&mut self) -> Vec<QuestionDashboards> {
        (self.questions)()
    }
}
