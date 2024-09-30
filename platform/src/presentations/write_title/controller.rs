#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Controller {
    pub survey_title: Signal<String>,
}

impl Controller {
    pub fn init() -> Self {
        let ctrl = Self {
            survey_title: use_signal(|| "".to_string()),
        };

        use_context_provider(|| ctrl);

        ctrl
    }

    pub fn get_survey_title(&self) -> String {
        (self.survey_title)()
    }

    pub fn set_survey_title(&mut self, title: String) {
        self.survey_title.set(title);
    }
}
