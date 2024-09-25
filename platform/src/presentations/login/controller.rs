#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Controller {
    pub email: Signal<String>,
    pub password: Signal<String>,
}

impl Controller {
    pub fn init() -> Self {
        let ctrl = Self {
            email: use_signal(|| "".to_string()),
            password: use_signal(|| "".to_string()),
        };

        use_context_provider(|| ctrl);

        ctrl
    }

    pub fn get_email(&self) -> String {
        (self.email)()
    }

    pub fn get_password(&self) -> String {
        (self.password)()
    }

    pub fn set_email(&mut self, email: String) {
        self.email.set(email);
    }

    pub fn set_password(&mut self, password: String) {
        self.password.set(password);
    }
}
