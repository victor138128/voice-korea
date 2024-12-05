#![allow(non_snake_case)]
use crate::{pages::dashboard::StatusButton, routes::Route};
use dioxus::prelude::*;
use models::prelude::SurveyDraftStatus;

use super::Language;

#[component]
pub fn DashboardCard(
    lang: Language,
    survey_id: String,
    draft_id: String,
    survey_sequence: SurveyDraftStatus,
    survey_type: String,
    title: String,
    update_date: String,
    response_count: u64,
    total_response_count: u64,
    draft_label: String,
    in_progress_label: String,
    complete_label: String,
    update_date_label: String,
    add_question_description: String,
    response: String,
    edit_survey: String,
    analysis_result: String,
) -> Element {
    let navigator = use_navigator();
    rsx! {
        div {
            class: "flex flex-col w-[380px] h-[420px] rounded-lg justify-between items-start bg-white m-9 p-7 border-[1px] border-[#d2d2d2]",
            div {
                class: "flex flex-col w-[380px]",
                StatusButton {
                    survey_type: survey_type.clone(),
                    draft_label,
                    in_progress_label,
                    complete_label
                }
                div {
                    class: "flex flex-col w-[340px]",
                    div {
                        class: "w-full text-[#4c4c4c] font-semibold text-[30px] mb-[14px] overflow-hidden truncate",
                        {title}
                    }
                }
                div {
                    class: "text-[20px] font-normal text-[#4c4c4c]",
                    "{update_date_label} {update_date}"
                }
            }
            div {
                class: "flex flex-col w-full",
                div {
                    class: "flex flex-row w-full justify-start items-center mb-[37px]",
                    img {
                        class: "mr-[10px] mb-[3px]",
                        src: asset!("/public/images/info.png"),
                        alt: "Info"
                    }
                    if survey_type.clone() == "draft" {
                        div {
                            class: "text-[#1e5eaf] font-normal text-[20px] mt-[1px]",
                            "{add_question_description}"
                        }
                    } else {
                        div {
                            class: "text-[#1e5eaf] font-normal text-[20px] mb-[2px]",
                            "{response_count}/{total_response_count}{response}"
                        }
                    }
                }
                if survey_type == "draft" {
                    div {
                        class: "flex flex-row w-full h-[55px] rounded-[8px] border-solid border-[3px] border-[#1e5eaf] bg-white items-center justify-center",
                        onclick: move |_| {
                            if survey_sequence == SurveyDraftStatus::Title {
                                navigator.push(
                                    Route::WriteTitlePage {
                                        lang, survey_id: draft_id.clone(),
                                    }
                                );
                            } else if survey_sequence == SurveyDraftStatus::Question {
                                navigator.push(
                                    Route::WriteQuestionPage { lang, survey_id: draft_id.clone() }
                                );
                            } else if survey_sequence == SurveyDraftStatus::Quotas {
                                navigator.push(
                                    Route::SelectResponsePage { lang, survey_id: draft_id.clone() }
                                );
                            } else {
                                navigator.push(Route::SurveySummaryPage { lang, survey_id: draft_id.clone() });
                            }
                        },
                        div {
                            class: "text-[20px] font-medium text-[#1e5eaf]",
                            "{edit_survey}"
                        }
                    }
                } else {
                    div {
                        class: "flex flex-row w-full h-[55px] rounded-[8px] border-solid border-[3px] border-[#1e5eaf] bg-white items-center justify-center",
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
