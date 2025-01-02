#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct IconProps {
    #[props(default = "black".to_string())]
    stroke: String,
    #[props(default = "none".to_string())]
    fill: String,
    #[props(default = "24px".to_string())]
    width: String,
    #[props(default = "24px".to_string())]
    height: String,
    class: Option<String>,
}

pub fn Cancel(props: IconProps) -> Element {
    rsx! {
        svg {
            "viewBox": "0 0 24 23",
            fill: "{props.fill}",
            xmlns: "http://www.w3.org/2000/svg",
            width: "{props.width}",
            height: "{props.height}",
            path {
                d: "M23.5 11.5C23.5 17.5553 18.3715 22.5 12 22.5C5.62846 22.5 0.5 17.5553 0.5 11.5C0.5 5.44471 5.62846 0.5 12 0.5C18.3715 0.5 23.5 5.44471 23.5 11.5Z",
                fill: "#F7F7F7",
                stroke: "#AEAEAE",
            }
            path {
                "stroke-linejoin": "round",
                "stroke-linecap": "round",
                d: "M8 15.8645L11.9323 11.9323L15.8645 15.8645M15.8645 8L11.9315 11.9323L8 8",
                "stroke-width": "1.5",
                stroke: "{props.stroke}",
            }
        }
    }
}

#[component]
pub fn Search(width: String, height: String, color: String) -> Element {
    rsx! {
        svg {
            view_box: "0 0 21 20",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            width,
            height,
            path {
                d: "M19.7959 19L15.0383 14.4439M15.0383 14.4439C15.8019 13.7126 16.4076 12.8444 16.8209 11.8889C17.2342 10.9335 17.4469 9.90942 17.4469 8.87523C17.4469 7.84104 17.2342 6.81697 16.8209 5.86151C16.4076 4.90604 15.8019 4.03788 15.0383 3.3066C14.2747 2.57532 13.3681 1.99523 12.3704 1.59947C11.3727 1.2037 10.3034 1 9.22344 1C8.14352 1 7.07418 1.2037 6.07646 1.59947C5.07875 1.99523 4.17221 2.57532 3.40859 3.3066C1.8664 4.78349 1 6.78659 1 8.87523C1 10.9639 1.8664 12.967 3.40859 14.4439C4.95078 15.9207 7.04244 16.7505 9.22344 16.7505C11.4044 16.7505 13.4961 15.9207 15.0383 14.4439Z",
                stroke: color,
                "stroke-width": "2",
                "stroke-linecap": "round",
                "stroke-linejoin": "round",
            }
        }
    }
}

#[component]
pub fn Add(width: String, height: String, inner_color: String, color: String) -> Element {
    rsx! {
        svg {
            width: width.clone(),
            height,
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 21 21",
            fill: "none",
            circle {
                cx: "10.5",
                cy: "10.5",
                r: "8",
                fill: inner_color,
            }
            path {
                d: "M10.0625 0C7.40378 0.0322598 4.86304 1.10277 2.98291 2.98291C1.10277 4.86304 0.0322598 7.40378 0 10.0625C0.0322598 12.7212 1.10277 15.262 2.98291 17.1421C4.86304 19.0222 7.40378 20.0927 10.0625 20.125C12.7212 20.0927 15.262 19.0222 17.1421 17.1421C19.0222 15.262 20.0927 12.7212 20.125 10.0625C20.0927 7.40378 19.0222 4.86304 17.1421 2.98291C15.262 1.10277 12.7212 0.0322598 10.0625 0ZM15.8125 10.7812H10.7812V15.8125H9.34375V10.7812H4.3125V9.34375H9.34375V4.3125H10.7812V9.34375H15.8125V10.7812Z",
                fill: color,
            }

        }
    }
}

#[component]
pub fn Close(
    width: String,
    height: String,
    color: String,
    border_color: String,
    button_color: String,
) -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 24 23",
            width,
            height,
            fill: "none",
            path {
                stroke: border_color,
                d: "M23.5 11.5C23.5 17.5553 18.3715 22.5 12 22.5C5.62846 22.5 0.5 17.5553 0.5 11.5C0.5 5.44471 5.62846 0.5 12 0.5C18.3715 0.5 23.5 5.44471 23.5 11.5Z",
                fill: color,
            }
            path {
                "stroke-linejoin": "round",
                "stroke-width": "1.5",
                stroke: button_color,
                d: "M8 15.8645L11.9323 11.9323L15.8645 15.8645M15.8645 8L11.9315 11.9323L8 8",
                "stroke-linecap": "round",
            }
        }
    }
}

#[component]
pub fn BottomArrow(width: String, height: String) -> Element {
    rsx! {
        svg {
            fill: "none",
            width,
            xmlns: "http://www.w3.org/2000/svg",
            "viewBox": "0 0 14 15",
            height,
            path {
                stroke: "#9BAAE4",
                d: "M11.082 6.07585L7.7058 9.45207C7.31528 9.8426 6.68212 9.8426 6.29159 9.45207L2.91536 6.07585",
                "stroke-width": "1.5",
                "stroke-linecap": "round",
                "stroke-linejoin": "round",
            }
        }
    }
}

#[component]
pub fn Logo(width: String, height: String) -> Element {
    rsx! {
        svg {
            fill: "none",
            height,
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 31 32",
            width,
            path {
                fill: "#EBEFF5",
                d: "M11.0349 22.6203C17.1286 22.6203 22.0685 17.6803 22.0685 11.5866C22.0685 10.896 22.0051 10.2202 21.8837 9.56472C21.2282 9.44333 20.5524 9.37988 19.8618 9.37988C13.7681 9.37988 8.82812 14.3198 8.82812 20.4135C8.82812 21.1042 8.89158 21.78 9.01297 22.4354C9.66843 22.5568 10.3442 22.6203 11.0349 22.6203Z",
            }
            path {
                fill: "#85AEE2",
                d: "M9.01176 22.4352C3.88367 21.4855 0 16.9895 0 11.5864C0 5.49267 4.93994 0.552734 11.0337 0.552734C16.4368 0.552734 20.9328 4.4364 21.8825 9.5645C21.227 9.44311 20.5512 9.37966 19.8606 9.37966C13.7669 9.37966 8.82692 14.3196 8.82692 20.4133C8.82692 21.1039 8.89037 21.7797 9.01176 22.4352Z",
                "clip-rule": "evenodd",
                "fill-rule": "evenodd",
            }
            path {
                "clip-rule": "evenodd",
                fill: "#85AEE2",
                "fill-rule": "evenodd",
                d: "M11.0336 22.62C17.1273 22.62 22.0673 17.6801 22.0673 11.5863C22.0673 10.8957 22.0038 10.2199 21.8824 9.56445C27.0105 10.5141 30.8942 15.0102 30.8942 20.4133C30.8942 26.507 25.9543 31.4469 19.8605 31.4469C14.4574 31.4469 9.96142 27.5633 9.01172 22.4352C9.66719 22.5565 10.343 22.62 11.0336 22.62Z",
            }
        }
    }
}
