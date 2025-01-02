#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::prelude::Language;

mod controller;
mod i18n;

#[derive(Props, Clone, PartialEq)]
pub struct MemberPageProps {
    lang: Language,
}

#[component]
pub fn MemberPage(props: MemberPageProps) -> Element {
    let _props = props;
    rsx! {
        div { "member" }
    }
}
