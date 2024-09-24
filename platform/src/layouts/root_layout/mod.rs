#![allow(non_snake_case)]
use crate::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn RootLayout(lang: Language) -> Element {
    rsx! {
        div { class: "bg-white dark:bg-black w-screen min-h-screen flex flex-col",
            "Hello"
            Outlet::<Route> {}
        }
    }
}
