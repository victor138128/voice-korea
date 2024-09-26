#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_logger::tracing;

use crate::{
    api::question::get_total_questions,
    models::question::{QuestionSummary, TotalQuestions},
};

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Controller {
    pub questions: Signal<Vec<QuestionSummary>>,
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
                        ctrl.questions.set(res.questions);
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
}
