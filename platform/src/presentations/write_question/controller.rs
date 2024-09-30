#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_logger::tracing;

use crate::{api::survey::get_survey, models::survey::Survey};

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Controller {
    survey: Signal<Survey>,
}

impl Controller {
    pub fn init(title: String) -> Self {
        let mut ctrl = Self {
            survey: use_signal(|| Survey::new()),
        };

        let _ = use_effect(move || {
            let value = title.clone();
            spawn(async move {
                match get_survey(value).await {
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

    pub fn get_survey(&mut self) -> Survey {
        (self.survey)()
    }
}
