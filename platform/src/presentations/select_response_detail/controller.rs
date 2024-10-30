#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Controller {
    response_count: Signal<String>,
}

impl Controller {
    pub fn init() -> Self {
        let ctrl = Self {
            response_count: use_signal(|| "0".to_string()),
        };

        ctrl
    }

    pub fn get_response_count(&mut self) -> String {
        (self.response_count)()
    }

    pub fn set_response_count(&mut self, response_count: String) {
        self.response_count.set(response_count);
    }
}
