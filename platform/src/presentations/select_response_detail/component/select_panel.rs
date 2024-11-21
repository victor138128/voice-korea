#![allow(non_snake_case)]
use crate::{
    prelude::*,
    presentations::select_response_detail::controller::{self, Panel, PanelGroup},
};
use dioxus::prelude::*;
use dioxus_logger::tracing;

#[derive(PartialEq, Props, Clone)]
pub struct SelectPanelProps {
    ctrl: controller::Controller,
    lang: Language,
    select_type: String,
    panel_groups: Vec<PanelGroup>,
    panels: Vec<Panel>,
    select_panel_groups: Vec<u64>,
    select_panels: Vec<u64>,
}

#[component]
pub fn SelectPanelPage(props: SelectPanelProps) -> Element {
    let ctrl = props.ctrl;

    tracing::info!("select type: {}", props.select_type);
    rsx! {
        div {
            class: "flex flex-col w-full h-full justify-start items-center",
            div {
                class: "flex flex-col max-w-[1200px] min-w-[600px] w-full justify-end items-end mt-[15px] px-[50px]",
                div {
                    class: "flex flex-row w-[250px] h-[55px] mt-[55px] rounded-[8px] bg-[#2168c3] justify-center items-center text-[21px] font-semibold text-white",
                    "임시 저장"
                }
            }
            div {
                class: "flex flex-col max-w-[1200px] min-w-[600px] w-full justify-start items-start mt-[15px] px-[50px]",
                div {
                    class: "flex flex-col w-full rounded-[10px] bg-white mb-[10px] p-[30px]",
                    div {
                        class: "text-[#2168c3] font-semibold text-[30px] mb-[20px]",
                        "응답 패널 선택"
                    }
                    div {
                        class: "text-black font-semibold text-[22px] mb-[40px]",
                        "설문조사를 제공할 패널 선택"
                    }
                    if props.select_type == "attribute" {
                        div {
                            AttributeTable {
                                ctrl,
                                panel_groups: props.panel_groups,
                                select_panel_groups: props.select_panel_groups,
                            }
                        }
                    } else {

                    }
                }
            }


        }
    }
}

#[component]
pub fn AttributeTable(
    ctrl: controller::Controller,
    panel_groups: Vec<PanelGroup>,
    select_panel_groups: Vec<u64>,
) -> Element {
    let mut ctrl = ctrl.clone();
    rsx! {
        div {
            class: "flex flex-col justify-start items-start w-full",
            div {
                class: "flex flex-row w-full h-full justify-end items-end",
                div {
                    class: "text-xl font-normal text-[#5e5e5e] mr-[3px]",
                    "현재"
                }
                div {
                    class: "text-xl font-semibold text-[#2168c3] mr-[3px]",
                    "100명"
                }
                div {
                    class: "text-xl font-normal text-[#5e5e5e]",
                    "선택 중"
                }
            }
            div {
                class: "flex flex-col w-full h-min justify-start items-start",
                for (i, d) in panel_groups.iter().enumerate() {
                    if i % 2 == 0 {
                        div {
                            class: "flex flex-row w-full h-[80px] justify-between items-start p-[30px] bg-[#f9f9f9]",
                            div {
                                class: "pr-[20px]",
                                onclick: move |_| {
                                    let value = ctrl.get_select_panel_groups().contains(&(i as u64));
                                    ctrl.change_select_panel_groups(i as u64, !value);
                                },
                                img {
                                    class: "pr-[10px]",
                                    src: if ctrl.get_select_panel_groups().contains(&(i as u64)) {asset!("public/images/enable_check.png")} else {asset!("public/images/not_enable_check.png")},
                                    alt: "check"
                                }
                            }
                            div {
                                class: "text-black font-semibold text-[20px] pr-[20px]",
                                {format!("선택 패널 모집군 {}", i + 1)}
                            }
                            div {
                                class: "text-[#5e5e5e] font-normal text-[20px] pr-[20px]",
                                "대한민국(Korea)"
                            }
                            div {
                                class: "text-[#5e5e5e] font-normal text-[20px] pr-[20px]",
                                "남성"
                            }
                            div {
                                class: "text-[#5e5e5e] font-normal text-[20px] pr-[20px]",
                                "30대"
                            }
                            div {
                                class: "text-[#5e5e5e] font-normal text-[20px] pr-[20px]",
                                "사무직"
                            }
                        }
                    } else {
                        div {
                            class: "flex flex-row w-full h-[80px] justify-between items-start p-[30px] bg-[#ffffff]",
                            div {
                                class: "pr-[20px]",
                                onclick: move |_| {
                                    let value = ctrl.get_select_panel_groups().contains(&(i as u64));
                                    ctrl.change_select_panel_groups(i as u64, !value);
                                },
                                img {
                                    class: "pr-[10px]",
                                    src: if ctrl.get_select_panel_groups().contains(&(i as u64)) {asset!("public/images/enable_check.png")} else {asset!("public/images/not_enable_check.png")},
                                    alt: "check"
                                }
                            }
                            div {
                                class: "text-black font-semibold text-[20px] pr-[20px]",
                                {format!("선택 패널 모집군 {}", i + 1)}
                            }
                            div {
                                class: "text-[#5e5e5e] font-normal text-[20px] pr-[20px]",
                                "대한민국(Korea)"
                            }
                            div {
                                class: "text-[#5e5e5e] font-normal text-[20px] pr-[20px]",
                                "남성"
                            }
                            div {
                                class: "text-[#5e5e5e] font-normal text-[20px] pr-[20px]",
                                "30대"
                            }
                            div {
                                class: "text-[#5e5e5e] font-normal text-[20px] pr-[20px]",
                                "사무직"
                            }
                        }
                    }
                }
            }
        }
    }
}
