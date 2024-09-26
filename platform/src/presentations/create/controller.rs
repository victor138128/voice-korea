#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Controller {
    authorize_type: Signal<u64>, //0: 개인, 1: 법인
    step: Signal<u64>,
    company: Signal<String>,
    business_number: Signal<String>,

    extend_first: Signal<bool>,
    extend_second: Signal<bool>,
    extend_third: Signal<bool>,
    click_first: Signal<bool>,
    click_second: Signal<bool>,
    click_third: Signal<bool>,
}

impl Controller {
    pub fn init() -> Self {
        let ctrl = Self {
            authorize_type: use_signal(|| 0),
            step: use_signal(|| 0),
            company: use_signal(|| "".to_string()),
            business_number: use_signal(|| "".to_string()),

            extend_first: use_signal(|| false),
            extend_second: use_signal(|| false),
            extend_third: use_signal(|| false),

            click_first: use_signal(|| false),
            click_second: use_signal(|| false),
            click_third: use_signal(|| false),
        };

        use_context_provider(|| ctrl);

        ctrl
    }

    pub fn get_authorize_type(&self) -> u64 {
        (self.authorize_type)()
    }

    pub fn get_step(&self) -> u64 {
        (self.step)()
    }

    pub fn get_company(&self) -> String {
        (self.company)()
    }

    pub fn get_business_number(&self) -> String {
        (self.business_number)()
    }

    pub fn get_extend_first(&self) -> bool {
        (self.extend_first)()
    }

    pub fn get_extend_second(&self) -> bool {
        (self.extend_second)()
    }

    pub fn get_extend_third(&self) -> bool {
        (self.extend_third)()
    }

    pub fn get_click_first(&self) -> bool {
        (self.click_first)()
    }

    pub fn get_click_second(&self) -> bool {
        (self.click_second)()
    }

    pub fn get_click_third(&self) -> bool {
        (self.click_third)()
    }

    pub fn set_authorize_type(&mut self, authorize_type: u64) {
        self.authorize_type.set(authorize_type);
    }

    pub fn set_step(&mut self, step: u64) {
        self.step.set(step);
    }

    pub fn set_company(&mut self, company: String) {
        self.company.set(company);
    }

    pub fn set_business_number(&mut self, business_number: String) {
        self.business_number.set(business_number);
    }

    pub fn set_extend_first_terms(&mut self, extend: bool) {
        self.extend_first.set(extend);
    }

    pub fn set_extend_second_terms(&mut self, extend: bool) {
        self.extend_second.set(extend);
    }

    pub fn set_extend_third_terms(&mut self, extend: bool) {
        self.extend_third.set(extend);
    }

    pub fn set_click_first_terms(&mut self, clicked: bool) {
        self.click_first.set(clicked);
        self.check_and_update_terms_agreement();
    }

    pub fn set_click_second_terms(&mut self, clicked: bool) {
        self.click_second.set(clicked);
        self.check_and_update_terms_agreement();
    }

    pub fn set_click_third_terms(&mut self, clicked: bool) {
        self.click_third.set(clicked);
        self.check_and_update_terms_agreement();
    }

    fn check_and_update_terms_agreement(&mut self) {
        if self.get_click_first() && self.get_click_second() && self.get_click_third() {
            self.set_step(2);
            self.click_first.set(false);
            self.click_second.set(false);
            self.click_third.set(false);
        }
    }
}
