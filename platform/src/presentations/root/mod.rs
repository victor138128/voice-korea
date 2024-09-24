#![allow(non_snake_case)]
use crate::prelude::*;
use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct RootProps {
    lang: Language,
}

#[component]
pub fn Root(props: RootProps) -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        div {
            h1 { "High-Five counter: {count}" }
            button { onclick: move |_| count += 1, "Up high!" }
            button { onclick: move |_| count -= 1, "Down low!" }
            button {
                "Get Server Data"
            }
        }
    }
}
