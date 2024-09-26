#![allow(non_snake_case)]
use crate::prelude::*;
use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct DashboardPageProps {
    lang: Language,
}

#[component]
pub fn DashboardPage(props: DashboardPageProps) -> Element {
    rsx! {
        div {
            class: "flex flex-row w-full h-full justify-start items-start text-black font-bold text-[16px]",
            "dashboard page"
        }
    }
}
