#![allow(non_snake_case)]
use crate::prelude::*;
use dioxus::prelude::*;

use super::controller;

#[derive(PartialEq, Props, Clone)]
pub struct StepThreeProps {
    ctrl: controller::Controller,
    lang: Language,
}

#[component]
pub fn StepThreePage(props: StepThreeProps) -> Element {
    rsx! {
        div {
            class: "text-[16px] text-black font-normal",
            "step 3"
        }
    }
}
