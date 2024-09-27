#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct HeaderProps {
    logout: String,
}

#[component]
pub fn Header(props: HeaderProps) -> Element {
    let logo_path = "/images/logo.png";
    rsx! {
        div {
            class: "flex flex-row justify-between items-center w-full min-h-[70px] max-h-[70px]",
            div {
                class: "flex min-h-[70px] justify-center items-center w-[250px]",
                div { class: "mr-[7px]",
                    img {
                        src: "{logo_path}",
                        width: 42,
                        height: 42
                    }
                }
                div { class: "text-[24px] font-bold text-[#2168C3]", "VOICE KOREA" }
            }
            div {
                class: "flex flex-row w-[105px] h-[45px] rounded-[8px] cursor-pointer hover:bg-[#9a9a9a] bg-[#b0b0b0] mr-[45px]",
                div {
                    class: "flex flex-row w-full h-full justify-center items-center text-[16px] font-bold text-white",
                    "{props.logout}"
                }
            }
        }
    }
}
