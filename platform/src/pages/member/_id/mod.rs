#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::prelude::Language;

#[derive(Props, Clone, PartialEq)]
pub struct MemberDetailPageProps {
    lang: Language,
    member_id: String,
}

#[component]
pub fn MemberDetailPage(props: MemberDetailPageProps) -> Element {
    let _props = props;
    rsx! { "Member Detail" }
}
