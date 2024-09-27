#![allow(non_snake_case)]
use crate::prelude::*;
use dioxus::prelude::*;

use super::controller;

#[derive(PartialEq, Props, Clone)]
pub struct StepFourProps {
    ctrl: controller::Controller,
    lang: Language,
    complete_join_membership_info: String,
    email_address: String,
    name_info: String,
    phone_info: String,
    company_name_info: String,
    complete: String,
    company_name_example: String,
}

#[component]
pub fn StepFourPage(props: StepFourProps) -> Element {
    let ctrl = props.ctrl;
    rsx! {
        div {
            class: "flex flex-col w-full h-full items-start justify-center px-[70px] py-[40px]",
            div {
                class: "text-[32px] font-bold text-black pb-[30px]",
                "{props.complete_join_membership_info}"
            }
            div {
                class: "flex flex-col w-full h-full items-start justify-center pb-[340px]",
                Row {
                    enable_bottom_border: false,
                    label: props.email_address,
                    element: rsx! {
                        div {
                            class: "flex flex-row w-full h-full justify-start items-center px-[20px]",
                            div {
                                class: "text-[16px] font-normal text-black",
                                {ctrl.get_email_address()}
                            }
                        }
                    }
                }
                Row {
                    enable_bottom_border: false,
                    label: props.name_info,
                    element: rsx! {
                        div {
                            class: "flex flex-row w-full h-full justify-start items-center px-[20px]",
                            div {
                                class: "text-[16px] font-normal text-black",
                                {ctrl.get_name()}
                            }
                        }
                    }
                }
                Row {
                    enable_bottom_border: false,
                    label: props.phone_info,
                    element: rsx! {
                        div {
                            class: "flex flex-row w-full h-full justify-start items-center px-[20px]",
                            div {
                                class: "text-[16px] font-normal text-black",
                                {ctrl.get_cellphone_number()}
                            }
                        }
                    }
                }
                Row {
                    enable_bottom_border: true,
                    label: props.company_name_info,
                    element: rsx! {
                        div {
                            class: "flex flex-row w-full h-full justify-start items-center px-[20px]",
                            div {
                                class: "text-[16px] font-normal text-black",
                                {props.company_name_example}
                            }
                        }
                    }
                }
            }
            div {
                class: "flex flex-row w-full justify-end min-w-[710px] items-end pt-[30px]",
                div {
                    class: "flex flex-row w-[300px] h-[60px] justify-end items-end bg-[#f5f5f5] border-solid border border-[#e0e0e0]",
                    div {
                        class: "flex flex-row w-full h-full justify-center items-center text-[24px] font-bold text-black",
                        {props.complete}
                    }
                }
            }
        }
    }
}

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

    rsx! {
        div {
            class: "flex flex-col w-full justify-start items-start",
                div {
                    class: "flex flex-row w-full min-w-[710px] {height} border-solid border border-t-[#e0e0e0] {bottom_border} border-l-[#e0e0e0] border-r-[#ffffff]",
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
