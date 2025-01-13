#![allow(non_snake_case)]
use super::controller::Controller;
use super::i18n::OpinionNewTranslate;
use dioxus::prelude::*;
use dioxus_translate::{translate, Language};

#[derive(Props, Clone, PartialEq)]
pub struct OpinionProps {
    lang: Language,
}

#[component]
pub fn OpinionCreatePage(props: OpinionProps) -> Element {
    let _ctrl = Controller::init(props.lang);
    let _translates: OpinionNewTranslate = translate(&props.lang.clone());
    rsx! { "Hello 1" }
}
