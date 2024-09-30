#![allow(non_snake_case)]
use dioxus::prelude::*;

#[component]
pub fn Row(
    enable_bottom_border: bool,
    height: Option<u64>,
    label: String,
    element: Element,
) -> Element {
    let bottom_border = if enable_bottom_border {
        "border-b-[#e0e0e0]"
    } else {
        "border-b-[#ffffff]"
    };

    let height = match height {
        Some(h) => format!("h-[{}px]", h),
        None => "h-[70px]".to_string(),
    };

    let main_div_class = format!(
        "flex flex-row w-full min-w-[710px] {} border-solid border border-t-[#e0e0e0] {} border-l-[#e0e0e0] border-r-[#ffffff]",
        height, bottom_border
    );

    rsx! {
        div {
            class: "flex flex-col w-full justify-start items-start",
                div {
                    class: "{main_div_class}",
                    div {
                        class: "flex flex-row w-[200px] min-w-[200px] h-full justify-start items-start bg-[#2168c3]",
                        div {
                            class: "min-w-[200px] p-[20px] text-white text-[16px] font-normal",
                            "{label}"
                        }
                    }
                    {element}
                }
        }
    }
}
