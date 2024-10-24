#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
    pub button_text: String,
    pub onclick: Option<EventHandler<MouseEvent>>,
    #[props(default = "".to_string())]
    pub class: String,
    #[props(default = "".to_string())]
    pub text_class: String,
}

pub fn Button(props: ButtonProps) -> Element {
    rsx! {
        button {
            class: "{props.class} rounded-xl justify-center items-center",
            onclick: move |event| {
                if let Some(onclick) = props.onclick {
                    onclick(event)
                }
            },
            div { class: "{props.text_class} text-xl font-normal text-white whitespace-nowrap",
                "{props.button_text}"
            }
        }
    }
}
