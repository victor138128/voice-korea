#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::{
    components::pi_graph::PiGraph, pages::id::response_report::controller::use_controller,
};

#[derive(Props, Clone, PartialEq)]
pub struct ResponseSummaryProps {
    pub response_report: String,
    pub response_download: String,
    pub response_summary: String,
    pub total_number_of_responses: String,
    pub completion_rate: String,
    pub normal_time_required: String,
    pub most_skipped_questions: String,
    pub response_attribute: String,
    pub survey_summary: String,
}

#[component]
pub fn ResponseSummary(props: ResponseSummaryProps) -> Element {
    let ctrl = use_controller();
    rsx! {
        div {
            class: "flex flex-col w-full justify-start items-start",
            div {
                class: "text-[#2168c3] font-semibold text-[30px]",
                {props.response_report}
            }
            div {
                class: "flex flex-row w-full justify-end items-end mb-[20px]",
                div {
                    class: "flex flex-row w-[135px] h-[45px] justify-center items-center mt-[30px] rounded-lg bg-[#1e5eaf] text-white font-bold text-[16px]",
                    {props.response_download}
                }
            }
            div {
                class: "flex flex-col w-full justify-start items-start mb-[30px]",
                div {
                    class: "text-black font-semibold text-[24px] mb-[20px]",
                    {props.response_summary}
                }
                div {
                    class: "flex flex-wrap gap-5 justify-start items-start px-[10px]",
                    SummaryInfo {
                        label: props.total_number_of_responses,
                        //FIXME: fix to real api logic
                        value: "10".to_string()
                    }
                    SummaryInfo {
                        label: props.completion_rate,
                        //FIXME: fix to real api logic
                        value: "100%".to_string()
                    }
                    SummaryInfo {
                        label: props.normal_time_required,
                        //FIXME: fix to real api logic
                        value: "11s".to_string()
                    }
                    SummaryInfo {
                        label: props.most_skipped_questions,
                        //FIXME: fix to real api logic
                        value: "없음".to_string()
                    }
                }
            }
            div {
                class: "flex flex-col w-full justify-start items-start mb-[30px]",
                div {
                    class: "text-black font-semibold text-[24px] mb-[40px]",
                    {props.response_attribute}
                }
                for (index, attribute) in ctrl.get_attributes().iter().enumerate() {
                    div {
                        class: "flex flex-col w-full justify-start items-start mb-[40px]",
                        div {
                            class: "text-black font-semibold text-[24px] mb-[20px]",
                            {format!("{}. {}", index + 1, attribute.label.clone())}
                        }
                        PiGraph {
                            chart_data: attribute.chart_datas.clone(),
                        }
                    }
                }
                // pi_graph {
                //     width: 45,
                //     height: 45,
                //     pi_data: vec![
                //         PiData {
                //             name: "Test1".to_string(),
                //             value: 25,
                //         },
                //         PiData {
                //             name: "Test2".to_string(),
                //             value: 50,
                //         },
                //         PiData {
                //             name: "Test3".to_string(),
                //             value: 25,
                //         },
                //     ]
                // }
            }
            div {
                class: "flex flex-col w-full justify-start items-start mb-[30px]",
                div {
                    class: "text-black font-semibold text-[24px] mb-[20px]",
                    {props.survey_summary}
                }
            }
        }
    }
}

#[component]
pub fn SummaryInfo(label: String, value: String) -> Element {
    rsx! {
        div {
            class: "flex flex-col w-[170px] h-[160px] justify-start items-start px-[10px] py-[30px] bg-[#f9f9f9] rounded-lg",
            div {
                class: "text-[16px] font-bold text-[#696969] pb-[10px]",
                {label},
            }
            div {
                class: "text-[20px] font-medium text-[#2168c3]",
                {value}
            }
        }
    }
}
