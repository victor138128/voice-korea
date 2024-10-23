#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_logger::tracing;

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Controller {
    email: Signal<String>,
    name: Signal<String>,
    phone_number: Signal<String>,
    authentication_number: Signal<String>,
    step: Signal<u64>,
    new_password: Signal<String>,
    new_password_check: Signal<String>,
}

impl Controller {
    pub fn init() -> Self {
        let ctrl = Self {
            email: use_signal(|| "".to_string()),
            name: use_signal(|| "".to_string()),
            phone_number: use_signal(|| "".to_string()),
            authentication_number: use_signal(|| "".to_string()),
            step: use_signal(|| 0),
            new_password: use_signal(|| "".to_string()),
            new_password_check: use_signal(|| "".to_string()),
        };

        use_context_provider(|| ctrl);

        ctrl
    }

    pub fn get_email(&mut self) -> String {
        (self.email)()
    }

    pub fn get_name(&mut self) -> String {
        (self.name)()
    }

    pub fn get_phone_number(&mut self) -> String {
        (self.phone_number)()
    }

    pub fn get_authentication_number(&mut self) -> String {
        (self.authentication_number)()
    }

    pub fn get_step(&mut self) -> u64 {
        (self.step)()
    }

    pub fn get_new_password(&mut self) -> String {
        (self.new_password)()
    }

    pub fn get_new_password_check(&mut self) -> String {
        (self.new_password_check)()
    }

    pub fn set_step(&mut self, step: u64) {
        self.step.set(step);
    }

    pub fn set_email(&mut self, email: String) {
        self.email.set(email);
    }

    pub fn set_name(&mut self, name: String) {
        self.name.set(name);
    }

    pub fn set_phone_number(&mut self, phone_number: String) {
        self.phone_number.set(phone_number);
    }

    pub fn set_authentication_number(&mut self, authentication_number: String) {
        self.authentication_number.set(authentication_number);
    }

    pub fn set_new_password(&mut self, new_password: String) {
        self.new_password.set(new_password);
    }

    pub fn set_new_password_check(&mut self, new_password_check: String) {
        self.new_password_check.set(new_password_check);
    }

    pub fn set_click_send_authentication(&mut self) {
        tracing::info!("send authentication button clicked");
    }

    //TODO: connect api
    pub fn clicked_email_authentication(&mut self) {
        tracing::info!("email authentication button clicked");
        self.step.set(1);
    }

    pub fn clicked_reset_new_password(&mut self) {
        tracing::info!("reset new password button clicked");
        self.step.set(2);
    }
}
