#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::prelude::Language;

#[derive(Props, Clone, PartialEq)]
pub struct GroupDetailPageProps {
    lang: Language,
    group_id: String,
}

#[component]
pub fn GroupDetailPage(props: GroupDetailPageProps) -> Element {
    let _props = props;
    rsx! { "Group Detail" }
}
