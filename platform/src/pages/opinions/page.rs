#![allow(non_snake_case)]
use super::controller::Controller;
use super::i18n::OpinionTranslate;
use dioxus::prelude::*;
use dioxus_translate::{translate, Language};

#[derive(Props, Clone, PartialEq)]
pub struct OpinionProps {
    lang: Language,
}

#[component]
pub fn OpinionPage(props: OpinionProps) -> Element {
    let _ctrl = Controller::init(props.lang);
    let _translates: OpinionTranslate = translate(&props.lang.clone());
    rsx! { "Hello" }
}
