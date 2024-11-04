#![allow(non_snake_case)]
use crate::{presentations::dashboard::StatusButton, routes::Route};
use dioxus::prelude::*;

use super::Language;

#[component]
pub fn DashboardRow(
    lang: Language,
    survey_id: String,
    survey_sequence: String,
    survey_type: String,
    title: String,
    update_date: String,
    response_count: u64,
    total_response_count: u64,
    draft_label: String,
    in_progress_label: String,
    complete_label: String,
    add_question_description: String,
    edit_survey: String,
    analysis_result: String,
) -> Element {
    let navigator = use_navigator();
    let s: Vec<&str> = survey_id.split("-").collect();
    let survey_id = format!("{}", s[2]);
    rsx! {
        div {
            class: "flex flex-row w-full h-[110px] mb-[10px] justify-between items-center rounded-[5px] bg-white px-[30px]",
            div {
                class: "flex flex-row min-w-[500px]",
                div {
                    class: "text-[#4c4c4c] font-semibold text-[30px] mr-[50px]",
                    {title}
                }
                if survey_type == "draft" {
                    div {
                        class: "flex flex-row items-center",
                        div {
                            class: "w-[20px] h-[20px] mr-[10px]",
                            img {
                                src: "/images/info.png",
                                width: 20,
                                height: 20,
                                alt: "Info"
                            }
                        }
                        div {
                            class: "text-[#1e5eaf] font-normal text-[20px] mt-[1px]",
                            "{add_question_description}"
                        }
                    }
                }

            }
            div {
                class: "flex flex-row justify-start items-center",
                div {
                    class: "flex flex-row items-center justify-center w-[80px] text-[#696969] font-normal text-[20px] mr-[50px]",
                    "{response_count}/{total_response_count}"
                }
                div {
                    class: "flex flex-row items-center justify-center w-[210px] text-[#696969] font-normal text-[20px] mr-[50px]",
                    "{update_date}"
                }
                div {
                    class: "flex flex-col items-center justify-center w-[110px] text-[#696969] font-normal text-[20px] mr-[50px]",
                    div {
                        class: "flex flex-col w-full justify-center items-center mt-[25px]",
                        StatusButton {
                            survey_type: survey_type.clone(),
                            draft_label,
                            in_progress_label,
                            complete_label,
                        }
                    }
                }
                div {
                    class: "flex flex-row w-[265px] items-center justify-center",
                    if survey_type == "draft" {
                        div {
                            class: "flex flex-row w-[200px] h-[55px] rounded-[8px] border-solid border border-[#b0b0b0] bg-white items-center justify-center",
                            onclick: move |_| {
                                if survey_sequence == "title" {
                                    navigator.push(
                                        Route::WriteTitlePage {
                                            lang, id: survey_id.clone()
                                        }
                                    );
                                }
                            },
                            div {
                                class: "text-[20px] font-medium text-[#1e5eaf]",
                                "{edit_survey}"
                            }
                        }
                    } else {
                        div {
                            class: "flex flex-row w-[200px] h-[55px] rounded-[8px] border-solid border border-[#b0b0b0] bg-white items-center justify-center",
                            div {
                                class: "text-[20px] font-medium text-[#1e5eaf]",
                                "{analysis_result}"
                            }
                        }
                    }
                }
            }
        }
    }
}
