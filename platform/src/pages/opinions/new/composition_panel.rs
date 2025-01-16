use dioxus::prelude::*;
use dioxus_translate::Language;

use crate::components::icons::{BottomDropdownArrow, Checked, TopDropdownArrow, UnChecked};

#[derive(Props, Clone, PartialEq)]
pub struct CompositionPanelProps {
    lang: Language,
}

#[component]
pub fn CompositionPanel(props: CompositionPanelProps) -> Element {
    let _props = props.clone();
    let mut selected_option = use_signal(move || "공평한 인원수 배정".to_string());
    let mut total_members = use_signal(move || "0".to_string());
    let mut add_panel_clicked = use_signal(|| false);
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "font-medium text-[16px] text-[#222222] mb-[10px]", "참여자 패널 구성" }
            div { class: "flex flex-col w-full justify-start items-start rounded-lg bg-white px-[40px] py-[24px]",
                div { class: "font-bold text-[#222222] text-lg mb-[3px]", "전체 패널 설정" }
                div { class: "font-normal text-[#6d6d6d] text-sm mb-[20px]",
                    "공론위원회는 다양한 의견을 수렴하고 합의된 결정을 도출하는 역할을 합니다. 각 역할의 담당자를 선정해주세요."
                }

                div { class: "flex flex-row w-full justify-between items-center mb-[10px]",
                    div { class: "flex flex-row ",
                        div { class: "flex flex-row w-[180px] mr-[50px] font-medium text-black text-[15px]",
                            "전체 패널"
                        }

                        div { class: "flex items-center space-x-4",
                            button {
                                onclick: move |_| {
                                    selected_option.set("공평한 인원수 배정".to_string());
                                },

                                if selected_option() == "공평한 인원수 배정" {
                                    Checked { width: "18", height: "18" }
                                } else {
                                    UnChecked { width: "18", height: "18" }
                                }
                            }
                            div { class: "ml-[10px] font-normal text-[#222222] text-[15px] mr-[50px]",
                                "공평한 인원수 배정"
                            }
                            button {
                                onclick: move |_| {
                                    selected_option.set("인원수 비례 배정".to_string());
                                },
                                if selected_option() == "인원수 비례 배정" {
                                    Checked { width: "18", height: "18" }
                                } else {
                                    UnChecked { width: "18", height: "18" }
                                }
                            }
                            div { class: "ml-[10px] font-normal text-[#222222] text-[15px]",
                                "인원수 비례 배정"
                            }
                        }
                    }
                    div { class: "flex flex-row justify-start items-center",
                        div { class: "flex flex-row w-[215px] focus:outline-none h-[55px] justify-start items-center bg-[#f7f7f7] rounded-[4px] px-[15px] mr-[10px]",
                            input {
                                class: "flex flex-row w-full justify-start items-center bg-transparent focus:outline-none",
                                r#type: "text",
                                placeholder: "총 인원",
                                value: total_members(),
                                onkeydown: move |e: KeyboardEvent| {
                                    let key = e.key();
                                    if key != Key::Backspace && key != Key::Delete {
                                        let s = match key {
                                            Key::Character(c) => c,
                                            _ => "".to_string(),
                                        };
                                        if !s.chars().all(|c| c.is_ascii_digit()) {
                                            e.prevent_default();
                                        }
                                    }
                                },
                                oninput: move |e: FormEvent| {
                                    let new_value = e.value().clone();
                                    if new_value.chars().all(|c| c.is_ascii_digit()) {
                                        total_members.set(new_value);
                                    } else {
                                        e.prevent_default();
                                    }
                                },
                            }
                        }
                        div { class: "font-normal text-black text-[15px]", "명" }
                    }
                }

                div { class: "flex flex-row w-full justify-start items-center",
                    div { class: "flex flex-row w-[180px] mr-[50px] font-medium text-black text-[15px]",
                        "패널 선택"
                    }
                    div { class: "flex flex-row w-full justify-start items-center p-[15px] rounded-[4px] bg-[#f7f7f7] font-medium text-[#b4b4b4] text-[15px]",
                        "패널 선택"
                    }
                }
            }

            div { class: "flex flex-row w-full justify-between items-center px-[40px] py-[24px] bg-white rounded-lg mt-[20px]",
                div { class: "flex flex-col w-full justify-start items-start",
                    div { class: "font-bold text-[#222222] text-lg mb-[3px]", "패널 직접 추가" }
                    div { class: "font-normal text-[#6d6d6d] text-sm",
                        "패널별 속성을 직접 작성하여 추가합니다. 이는 패널&속성관리 페이지에 자동으로 반영됩니다."
                    }
                }
                div {
                    onclick: move |_| {
                        let clicked = add_panel_clicked();
                        add_panel_clicked.set(!clicked);
                    },
                    div { class: "cursor-pointer",
                        if add_panel_clicked() {
                            TopDropdownArrow { width: "24", height: "24" }
                        } else {
                            BottomDropdownArrow { width: "24", height: "24" }
                        }
                    }
                }
            }

            div { class: "flex flex-row w-full justify-end items-end mt-[40px] mb-[50px]",
                div {
                    class: "flex flex-row w-[70px] h-[55px] rounded-[4px] justify-center items-center bg-white border border-[#bfc8d9] font-semibold text-[16px] text-[#555462] mr-[20px]",
                    onclick: move |_| {},
                    "뒤로"
                }
                div {
                    class: "flex flex-row w-[105px] h-[55px] rounded-[4px] justify-center items-center bg-white border border-[#bfc8d9] font-semibold text-[16px] text-[#555462] mr-[20px]",
                    onclick: move |_| {},
                    "임시저장"
                }
                div {
                    class: "cursor-pointer flex flex-row w-[110px] h-[55px] rounded-[4px] justify-center items-center bg-[#2a60d3] font-semibold text-[16px] text-white",
                    onclick: move |_| {},
                    "다음으로"
                }
            }
        }
    }
}
