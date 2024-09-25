#![allow(non_snake_case)]
use crate::prelude::*;
use dioxus::prelude::*;

#[component]
pub fn RootLayout(lang: Language) -> Element {
    let logo_path = "/images/logo.png";
    rsx! {
        div { class: "bg-white dark:bg-black w-screen min-h-screen flex flex-col",
            div {
                class: "flex flex-row w-full justify-start items-center px-[30px] py-[3px]",
                div { class: "mr-[7px]",
                    img {
                        src: "{logo_path}",
                        width: 42,
                        height: 42
                    }
                }
                div { class: "text-[24px] font-bold text-[#2168C3]", "VOICE KOREA" }
            }
            Outlet::<Route> {}
        }
    }
}
