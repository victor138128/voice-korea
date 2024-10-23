#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::prelude::Language;

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
        Some(-1) => "h-full".to_string(),
        Some(h) => format!("h-[{}px]", h),
        None => "h-[40px]".to_string(),
    };
    let width = match width {
        Some(-1) => "w-full".to_string(),
        Some(w) => format!("w-[{}px]", w),
        None => "w-[300px]".to_string(),
    };
    let font_size = match font_size {
        Some(fs) => format!("text-[{}px]", fs),
        None => "text-[16px]".to_string(),
    };
    rsx! {
        input {
            class: "border-[#E0E0E0] text-medium border-[1px] {height} {font_size} {width}",
            style: "padding: 5px",
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
