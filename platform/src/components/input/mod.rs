#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn Input(
    input_type: Option<String>,
    placeholder: Option<String>,
    onchange: Option<EventHandler<String>>,
    value: Option<String>,
    height: Option<i64>,
    width: Option<i64>,
    font_size: Option<u64>,
) -> Element {
    let height = match height {
        Some(-1) => "100%".to_string(),
        Some(h) => format!("{h}px"),
        None => "40px".to_string(),
    };
    let width = match width {
        Some(-1) => "100%".to_string(),
        Some(w) => format!("{w}px"),
        None => "300px".to_string(),
    };
    let font_size = match font_size {
        Some(fs) => format!("{fs}px"),
        None => "16px".to_string(),
    };
    rsx! {
        input {
            class: "border-[#E0E0E0] text-medium border-[1px]",
            style: "padding: 5px; height: {height}; width: {width}; font-size: {font_size};",
            "type": input_type.unwrap_or("text".to_string()),
            placeholder,
            value: value.clone().unwrap_or_default(),
            onchange: move |e| {
                if let Some(onchange) = onchange {
                    onchange(e.value());
                }
            },
        }
    }
}
