#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::prelude::Language;

mod controller;
mod i18n;

#[derive(Props, Clone, PartialEq)]
pub struct GroupPageProps {
    lang: Language,
}

#[component]
pub fn GroupPage(props: GroupPageProps) -> Element {
    let _props = props;
    rsx! {
        div { "group" }
    }
}
