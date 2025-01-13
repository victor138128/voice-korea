#![allow(non_snake_case)]
use crate::components::{
    icons::{ArrowLeft, Expand, RowOption, Search, Switch},
    label::Label,
};

use super::controller::Controller;
use super::i18n::OpinionTranslate;
use dioxus::prelude::*;
use dioxus_translate::{translate, Language};

#[derive(Props, Clone, PartialEq)]
pub struct OpinionProps {
    lang: Language,
}

#[component]
pub fn OpinionPage(props: OpinionProps) -> Element {
    let ctrl = Controller::init(props.lang);
    let _translates: OpinionTranslate = translate(&props.lang.clone());
    let mut is_focused = use_signal(|| false);
    let mut search_text = use_signal(|| "".to_string());

    let project_types = ctrl.get_project_types();
    let project_statuses = ctrl.get_project_statuses();
    let opinions = ctrl.get_opinions();
    let mut clicked_opinion_type: Signal<i64> = use_signal(|| -1);
    let mut clicked_opinion_status: Signal<i64> = use_signal(|| -1);
    let mut clicked_panel: Signal<i64> = use_signal(|| -1);

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "text-[#9b9b9b] font-medium text-[14px] mb-[10px]",
                "조직 관리 / 공론관리"
            }
            div { class: "text-[#3a3a3a] font-semibold text-[28px] mb-[25px]", "공론관리" }
            div { class: "text-[#35343f] font-normal text-[14px] mb-[40px]",
                "공론 관리 페이지는 의견과 토론 내용을 체계적으로 관리하고 공유할 수 있도록 도와줍니다."
            }

            div {
                class: "flex flex-col w-full justify-start items-start bg-white rounded-lg shadow-lg p-[20px]",
                style: "box-shadow: 0 4px 6px rgba(53, 70, 177, 0.05);",
                div { class: "flex flex-row w-full justify-between items-center pb-[20px]",
                    div {
                        class: format!(
                            "flex flex-row w-[590px] h-[45px] justify-between items-center rounded-lg  {} px-[11px] py-[13px]",
                            if (is_focused)() {
                                "bg-[#ffffff] border border-[#2a60d3]"
                            } else {
                                "bg-[#f7f7f7] border border-[#7c8292]"
                            },
                        ),
                        input {
                            class: "flex flex-row w-full h-full bg-transparent focus:outline-none",
                            r#type: "text",
                            placeholder: "검색어를 입력해주세요.".to_string(),
                            value: (search_text)(),
                            onfocus: move |_| {
                                is_focused.set(true);
                            },
                            onblur: move |_| {
                                is_focused.set(false);
                            },
                            oninput: move |event| {
                                search_text.set(event.value());
                            },
                        }
                        Search { width: "18", height: "18", color: "#7c8292" }
                    }
                    div { class: "flex flex-row gap-[10px]",
                        div { class: "flex flex-row w-[130px] h-[40px] justify-center items-center bg-[#2a60d3] rounded-md gap-[5px]",
                            div {
                                class: "text-white font-semibold text-[16px]",
                                onclick: move |_| {},
                                "공론 시작하기"
                            }
                        }
                    }
                }
                //table section
                div { class: "flex flex-col w-full justify-start items-start bg-white border rounded-lg border-[#bfc8d9] mb-[30px]",
                    div { class: "flex flex-row w-full h-[55px] justify-start items-center",
                        div { class: "flex flex-row w-[120px] min-w-[120px] h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                "분야"
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row flex-1 h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                "프로젝트"
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row flex-1 h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                "응답율"
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row flex-1 h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                "패널"
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row flex-1 h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                "기간"
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row w-[120px] min-w-[120px] h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                "상태"
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row w-[120px] min-w-[120px] h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                "보기"
                            }
                        }
                        div { class: "w-[90px] h-full justify-center items-center gap-[10px]" }
                    }
                    for index in 0..opinions.len() {
                        div { class: "flex flex-col w-full h-[56px] justify-start items-start",
                            div { class: "flex flex-row w-full h-[1px] bg-[#bfc8d9]",
                                div { class: "flex flex-row w-full",
                                    div { class: "flex flex-row w-full h-[55px] justify-start items-center text-[#3a3a3a] font-medium text-[14px]",
                                        if clicked_opinion_type() != index as i64 {
                                            div {
                                                class: "flex flex-row w-[120px] min-w-[120px] cursor-pointer h-full justify-center items-center",
                                                onclick: move |_| {
                                                    clicked_opinion_type.set(index as i64);
                                                },
                                                {opinions[index].opinion_type.clone()}
                                            }
                                        } else {
                                            div { class: "flex flex-row w-[120px] h-[45px] justify-center items-center",
                                                select {
                                                    class: "flex flex-row w-[110px] h-[45px] justify-center items-center bg-[#f7f7f7] rounded-sm focus:outline-none focus:border focus:border-[#2a60d3] focus:bg-white px-[15px] text-[#222222]",
                                                    value: opinions[index].opinion_type.clone(),
                                                    onchange: move |_evt| {},
                                                    for project_type in project_types.clone() {
                                                        option { value: project_type.clone(),
                                                            "{project_type}"
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        div { class: "flex flex-row flex-1 h-full justify-center items-center gap-[10px]",
                                            {opinions[index].project_name.clone()}
                                        }
                                        div { class: "flex flex-row flex-1 h-full justify-center items-center",
                                            {
                                                format!(
                                                    "{}% ({}/{})",
                                                    opinions[index].response_count * 100 / opinions[index].total_response_count,
                                                    opinions[index].response_count,
                                                    opinions[index].total_response_count,
                                                )
                                            }
                                        }
                                        div {
                                            class: format!(
                                                "relative flex flex-row flex-1 h-full justify-center items-center {}",
                                                if opinions[index].status != "마감" { "cursor-pointer" } else { "" },
                                            ),
                                            onclick: {
                                                let status = opinions[index].status.clone();
                                                move |_| {
                                                    if status != "마감" {
                                                        clicked_panel.set(index as i64);
                                                    }
                                                }
                                            },
                                            div { class: "flex flex-row w-full justify-center items-center",
                                                if index < opinions.len() && opinions[index].panels.len() > 0 {
                                                    Label {
                                                        label_name: opinions[index].panels[0].name.clone(),
                                                        label_color: if opinions[index].status == "마감" { "bg-[#b4b4b4]".to_string() } else { "bg-[#35343f]".to_string() },
                                                        is_delete: false,
                                                    }
                                                }
                                                if clicked_panel() == index as i64 {
                                                    div {
                                                        class: "pl-[20px]",
                                                        onclick: move |e: MouseEvent| {
                                                            e.stop_propagation();
                                                            e.prevent_default();
                                                            clicked_panel.set(-1 as i64);
                                                        },
                                                        Expand {
                                                            width: "20px",
                                                            height: "20px",
                                                        }
                                                    }
                                                }
                                            }
                                            if clicked_panel() == index as i64 && index < opinions.len() {
                                                div { class: "absolute top-full bg-white border border-[#bfc8d9] shadow-lg rounded-lg w-full z-50 py-[20px] pl-[15px] pr-[10ㅔㅌ]",
                                                    div { class: "font-semibold text-[#7c8292] text-[14px] mb-[20px]",
                                                        "패널"
                                                    }
                                                    div { class: "inline-flex flex-wrap justify-start items-start gap-[10px] mr-[20px]",
                                                        for panel in opinions[index].panels.clone() {
                                                            Label {
                                                                label_name: panel.name.clone(),
                                                                label_color: if opinions[index].status == "마감" { "bg-[#b4b4b4]".to_string() } else { "bg-[#35343f]".to_string() },
                                                                is_delete: false,
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        div { class: "flex flex-row flex-1 h-full justify-center items-center",
                                            {
                                                format!(
                                                    "{} ~ {}",
                                                    opinions[index].start_date.to_string(),
                                                    opinions[index].end_date.to_string(),
                                                )
                                            }
                                        }
                                        if clicked_opinion_status() != index as i64 {
                                            div {
                                                class: format!(
                                                    "flex flex-row w-[120px] min-w-[120px] h-full justify-center items-center {}",
                                                    if opinions[index].status != "마감" { "cursor-pointer" } else { "" },
                                                ),
                                                onclick: {
                                                    let status = opinions[index].status.clone();
                                                    move |_| {
                                                        if status != "마감" {
                                                            clicked_opinion_status.set(index as i64);
                                                        }
                                                    }
                                                },
                                                {opinions[index].status.clone()}
                                            }
                                        } else {
                                            div { class: "flex flex-row w-[120px] h-[45px] justify-center items-center",
                                                select {
                                                    class: "flex flex-row w-[110px] h-[45px] justify-center items-center bg-[#f7f7f7] rounded-sm focus:outline-none focus:border focus:border-[#2a60d3] focus:bg-white px-[15px] text-[#222222]",
                                                    value: opinions[index].status.clone(),
                                                    onchange: move |_evt| {},
                                                    for project_status in project_statuses.clone() {
                                                        option { value: project_status.clone(),
                                                            "{project_status}"
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        div { class: "flex flex-row w-[120px] min-w-[120px] h-full justify-center items-center",
                                            if opinions[index].status == "준비" || opinions[index].status == "진행" {
                                                div { class: "font-semibold text-[14px] text-[#2a60d3]",
                                                    "자세히 보기"
                                                }
                                            } else {
                                                div { class: "font-semibold text-[14px] text-[#2a60d3]",
                                                    "결과 보기"
                                                }
                                            }
                                        }
                                        div { class: "flex flex-row w-[90px] h-full justify-center items-center",
                                            div { class: "group relative",
                                                button { onclick: move |_| {},
                                                    RowOption { width: 24, height: 24 }
                                                }
                                                nav {
                                                    tabindex: "0",
                                                    class: "border-2 bg-white invisible border-none shadow-lg rounded w-60 absolute right-0 top-full transition-all opacity-0 group-focus-within:visible group-focus-within:opacity-100 group-focus-within:translate-y-1 group-focus-within:z-20",
                                                    ul { class: "py-1",
                                                        li {
                                                            class: "p-3 text-sm text-gray-700 hover:bg-gray-100 cursor-pointer",
                                                            onclick: move |_| {},
                                                            "프로젝트 삭제하기"
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                //페이지네이션
                div { class: "flex flex-row w-full justify-center items-center mt-[20px]",
                    div { class: "mr-[20px] w-[24px] h-[24px]",
                        ArrowLeft { width: "24", height: "24" }
                    }
                    //FIXME: add pagination by variable(page, index)
                    for i in 0..10 {
                        if i == 0 {
                            div { class: "flex flex-row w-[40px] h-[40px] justify-center items-center bg-[#7c8292] rounded-lg text-white font-bold text-[15px] mr-[8px]",
                                "{i + 1}"
                            }
                        } else {
                            div { class: "flex flex-row w-[40px] h-[40px] justify-center items-center bg-white border border-[#dfdfdf] rounded-lg text-[#0d1732] font-bold text-[15px] mr-[8px]",
                                "{i + 1}"
                            }
                        }
                    }
                    div { class: "flex flex-row ml-[12px] w-[60px] h-[40px] justify-center items-center font-bold text-[15px] text-[#0d1732]",
                        "More"
                    }
                }
            }
        }
    }
}
