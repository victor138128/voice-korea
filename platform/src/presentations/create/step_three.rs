#![allow(non_snake_case)]
use crate::{components::input::Input, prelude::*};
use dioxus::prelude::*;

use super::controller;

#[derive(PartialEq, Props, Clone)]
pub struct StepThreeProps {
    ctrl: controller::Controller,
    lang: Language,
    join_the_membership: String,
    email_address: String,
    send_authentication: String,
    authentication_number: String,
    authentication_descriptions: Vec<String>,
    company_info: String,
    company_example: String,
    name_info: String,
    name_example: String,
    phone_info: String,
    phone_example: String,
    address_info: String,
    search_address: String,
    check_title: String,
    check_membership_descriptions: Vec<String>,
    complete_join_membership: String,
}

#[component]
pub fn StepThreePage(props: StepThreeProps) -> Element {
    let mut ctrl = props.ctrl;
    rsx! {
        div {
            class: "flex flex-col w-full h-full items-start justify-center px-[70px] py-[40px]",
            div {
                class: "text-[32px] font-bold text-black pb-[30px]",
                "{props.join_the_membership}"
            }
            div {
                class: "flex flex-col w-full h-full items-start justify-center pb-[30px]",
                Row {
                    enable_bottom_border: false,
                    label: props.email_address,
                    element: rsx! {
                        div {
                            class: "flex flex-row w-full h-full justify-start items-center",
                            div {
                                class: "mx-[10px]",
                                Input {
                                    value: ctrl.get_email_address(),
                                    onchange: move |e| {
                                        ctrl.set_email_address(e);
                                    }
                                }
                            },
                            ButtonComponent {
                                label: props.send_authentication,
                                lang: props.lang,
                                onclick: move |_| {
                                    ctrl.set_click_send_authentication();
                                }
                            }
                        }
                    }
                }
                Row {
                    enable_bottom_border: false,
                    height: Some(135),
                    label: props.authentication_number,
                    element: rsx! {
                        div {
                            class: "flex flex-col w-full h-full justify-start items-start mt-[10px] ml-[10px]",
                            div {
                                class: "pb-[5px]",
                                Input {
                                    value: ctrl.get_authentication_number(),
                                    onchange: move |e| {
                                        ctrl.set_authentication_number(e);
                                    }
                                }
                            },
                            div {
                                class: "text-[16px] font-normal text-[#636363]",
                                "{props.authentication_descriptions[0]}"
                            }
                            div {
                                class: "text-[16px] font-normal text-[#636363]",
                                "{props.authentication_descriptions[1]}"
                            }
                        }
                    }
                }
                Row {
                    enable_bottom_border: false,
                    label: props.company_info,
                    element: rsx! {
                        div {
                            class: "flex flex-row w-full h-full items-center mx-[20px] text-black font-normal text-[16px]",
                            "{props.company_example}"
                        }
                    }
                }
                Row {
                    enable_bottom_border: false,
                    label: props.name_info,
                    element: rsx! {
                        div {
                            class: "flex flex-row w-full h-full justify-start items-center",
                            div {
                                class: "mx-[10px]",
                                Input {
                                    value: ctrl.get_name(),
                                    placeholder: props.name_example,
                                    onchange: move |e| {
                                        ctrl.set_name(e);
                                    }
                                }
                            },
                        }
                    }
                }
                Row {
                    enable_bottom_border: false,
                    label: props.phone_info,
                    element: rsx! {
                        div {
                            class: "flex flex-row w-full h-full justify-start items-center",
                            div {
                                class: "mx-[10px]",
                                Input {
                                    value: ctrl.get_cellphone_number(),
                                    placeholder: props.phone_example,
                                    onchange: move |e| {
                                        ctrl.set_cellphone_number(e);
                                    }
                                }
                            },
                        }
                    }
                }
                Row {
                    enable_bottom_border: true,
                    height: Some(135),
                    label: props.address_info,
                    element: rsx! {
                        div {
                            class: "flex flex-col w-full h-min justify-start items-start mt-[10px]",
                            div {
                                class: "flex flex-row w-full h-full justify-start items-center mb-[10px]",
                                div {
                                    class: "mx-[10px]",
                                    Input {
                                        value: ctrl.get_simple_address(),
                                        onchange: move |e| {
                                            ctrl.set_simple_address(e);
                                        }
                                    }
                                },
                                ButtonComponent {
                                    width: Some(130),
                                    label: props.search_address,
                                    lang: props.lang,
                                    onclick: move |_| {
                                        ctrl.set_click_search_address();
                                    }
                                }
                            }
                            div {
                                class: "flex flex-row w-full mx-[10px] pr-[20px]",
                                Input {
                                    value: ctrl.get_detail_address(),
                                    onchange: move |e| {
                                        ctrl.set_detail_address(e);
                                    },
                                    width: Some(-1)
                                }
                            },
                        }
                    }
                }
            }
            div {
                class: "flex flex-col w-full h-min min-w-[710px] border-solid border border-[#e0e0e0] px-[20px] py-[15px]",
                div {
                    class: "text-black text-[20px] font-normal pb-[20px]",
                    "{props.check_title}"
                }
                div {
                    class: "text-[#636363] text-[20px] font-normal pb-[5px]",
                    "{props.check_membership_descriptions[0]}"
                }
                div {
                    class: "text-[#636363] text-[20px] font-normal pb-[5px]",
                    "{props.check_membership_descriptions[1]}"
                }
                div {
                    class: "text-[#636363] text-[20px] font-normal",
                    "{props.check_membership_descriptions[2]}"
                }
            }
            div {
                class: "flex flex-row w-full justify-end min-w-[710px] items-end pt-[30px]",
                div {
                    onclick: move |_| {
                        ctrl.set_click_complete_join_membership();
                        ctrl.set_step(3);
                    },
                    class: "flex flex-row w-auto h-[60px] justify-end items-end bg-[#2168c3] px-[20px]",
                    div {
                        class: "flex flex-row w-full h-full justify-center items-center text-[24px] font-bold text-white",
                        "{props.complete_join_membership}"
                    }
                }
            }
        }
    }
}

#[component]
pub fn ButtonComponent(
    width: Option<i64>,
    label: String,
    lang: Language,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let width = match width {
        Some(w) => format!("w-[{}px]", w),
        None => {
            if lang == Language::En {
                "w-[270px]".to_string()
            } else {
                "w-[135px]".to_string()
            }
        }
    };

    rsx! {
        div {
            class: "flex flex-col {width} h-[35px] justify-start items-start",
            onclick: move |evt| { onclick.call(evt); },
            style: "border: 1px solid; border-color: rgba(33, 104, 195, 0.5); border-radius: 5px; background-clip: padding-box; background-color: rgba(33, 104, 195, 0.04);",
            div {
                class: "flex flex-row justify-center items-center w-full h-full text-[#2168c3] font-normal text-[15px]",
                {label}
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
