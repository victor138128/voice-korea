#![allow(non_snake_case)]
use crate::{
    models::{question::Question, survey::SurveyStatus},
    prelude::*,
};
use controller::use_controller;
use dioxus::prelude::*;
use dioxus_logger::tracing;

#[derive(PartialEq, Props, Clone)]
pub struct SurveySummaryProps {
    lang: Language,
    survey_id: String,
}

pub struct QuestionModel {
    title: String,
    questions: Vec<String>,
}

mod controller;
mod i18n;
use models::prelude::{Age, Gender, Quota};
#[cfg(feature = "web")]
use wasm_bindgen::prelude::*;

#[component]
pub fn SurveySummaryPage(props: SurveySummaryProps) -> Element {
    let mut ctrl = controller::Controller::init(props.lang, props.survey_id.clone());
    let translates = i18n::translate(props.lang.clone());
    let survey_status = ctrl.get_survey_status();

    rsx! {
        div {
            class: "flex flex-col w-full h-full justify-start items-center",
            div {
                class: "flex flex-col max-w-[1200px] min-w-[600px] w-full justify-end items-end mt-[15px] px-[50px]",
                match survey_status {
                    SurveyStatus::Draft => {
                        rsx! {
                            div {
                                class: "flex flex-row w-[250px] h-[55px] mt-[55px] rounded-[8px] bg-[#2168c3] justify-center items-center text-[21px] font-semibold text-white",
                                {translates.start_survey}
                            }
                        }
                    },
                    SurveyStatus::InProgress { started_at: _, ended_at: _ } => {
                        rsx! {
                            div {
                                class: "flex flex-row w-[250px] h-[55px] mt-[55px] rounded-[8px] bg-[#8e8e8e] justify-center items-center text-[21px] font-semibold text-white",
                                {translates.inprogress_survey}
                            }
                        }
                    },
                    _ => {
                        rsx! {
                            div {
                                class: "flex flex-row w-[250px] h-[55px] mt-[55px] rounded-[8px] bg-[#8e8e8e] justify-center items-center text-[21px] font-semibold text-white",
                                {translates.finish_survey}
                            }
                        }
                    }
                }
            }
            div {
                class: "flex flex-col max-w-[1200px] min-w-[600px] w-full justify-start items-start mt-[15px] px-[50px]",
                div {
                    class: "flex flex-col w-full justify-center items-start max-w-[1200px] h-[100px] rounded-[10px] bg-white px-[30px]",
                    div {
                        class: "text-[#2168c3] font-semibold text-[30px] mb-[20px]",
                        {ctrl.get_survey().survey.title}
                    },
                }
            }
            if let SurveyStatus::Draft = survey_status {
                div {
                    class: "flex flex-col max-w-[1200px] min-w-[600px] w-full justify-start items-start mt-[15px] px-[50px]",
                    div {
                        class: "flex flex-col w-full justify-center items-start max-w-[1200px] h-[140px] rounded-[10px] bg-white px-[30px]",
                        div {
                            class: "text-black font-semibold text-[24px] mb-[10px]",
                            {translates.set_response_date}
                        },
                        div {
                            class: "flex flex-row w-min h-[35px] justify-start items-start border border-b-[1px] border-b-black border-l-transparent border-r-transparent border-t-transparent",
                            input {
                                id: "date-start-input",
                                r#type: "date",
                                value: (ctrl.start_date)().clone(),
                                onchange: move |e: FormEvent| {
                                    ctrl.change_start_date(e.value().clone());
                                    tracing::debug!("start date: {}", (ctrl.start_date)());
                                },
                            }
                            div {
                                class: "text-[#000000] font-semibold text-[20px] mr-[10px]",
                                "~"
                            }
                            input {
                                id: "date-end-input",
                                r#type: "date",
                                value: (ctrl.end_date)().clone(),
                                onchange: move |e: FormEvent| {
                                    ctrl.change_end_date(e.value().clone());
                                },
                            }
                        }
                    }
                }
            }
            Summary {
                survey_summary: translates.survey_summary,
                survey_progress_period: translates.survey_progress_period,
                select_panel_and_attribute: translates.select_panel_and_attribute.clone(),
                written_question: translates.written_question,
                panel: translates.panel,
                attribute: translates.attribute,
                objective: translates.objective,
                subjective: translates.subjective,
                total_question: translates.total_question,
            }
            SelectPanel {
                select_panel_and_attribute: translates.select_panel_and_attribute,
                unknown: translates.unknown,
            }
            ListSurvey {
                question_count: ctrl.get_questions().len() as u64,
                question_list: ctrl.get_questions(),
                response_list: translates.response_list,
                total: translates.total,
                num_of_detail: translates.num_of_detail,
                num_of: translates.num_of,
            }
            if let SurveyStatus::InProgress { started_at: _, ended_at: _ } = survey_status {
                div {
                    class: "flex flex-col max-w-[1200px] min-w-[600px] w-full justify-start items-start mt-[15px] px-[50px]",
                    div {
                        class: "flex flex-row w-full justify-start items-center max-w-[1200px] min-h-[150px] h-min rounded-[10px] bg-white px-[30px]",
                        div {
                            class: "flex flex-col w-full justify-start items-start",
                            div {
                                class: "text-black font-semibold text-[24px] mb-[10px]",
                                {translates.analysis_result}
                            },
                            div {
                                class: "text-[#5e5e5e] font-medium text-[22px]",
                                {translates.analysis_inprogress_info}
                            }
                        }
                    }
                }
            } else if let SurveyStatus::Finished { started_at: _, ended_at: _ } = survey_status {
                div {
                    class: "flex flex-col max-w-[1200px] min-w-[600px] w-full justify-start items-start mt-[15px] px-[50px]",
                    div {
                        class: "flex flex-row w-full justify-start items-center max-w-[1200px] min-h-[300px] h-min rounded-[10px] bg-white px-[30px]",
                        div {
                            class: "flex flex-col w-full justify-start items-start",
                            div {
                                class: "text-black font-semibold text-[24px] mb-[10px]",
                                {translates.analysis_result}
                            },
                            div {
                                class: "text-[#5e5e5e] font-medium text-[22px] mb-[15px]",
                                {translates.analysis_result_info}
                            },
                            div {
                                class: "flex flex-row w-full h-[140px] justify-between items-center rounded-lg bg-[#f6f6f6] px-[30px]",
                                div {
                                    class: "flex flex-row w-[90px] h-[40px] justify-center items-center bg-[#2168c3] rounded-lg text-white font-medium text-[20px] mr-[50px]",
                                    {translates.complete}
                                }
                                div {
                                    class: "flex flex-col w-full h-min justify-start items-start",
                                    div {
                                        class: "text-[#4c4c4c] font-semibold text-[30px] mb-[15px]",
                                        {ctrl.get_survey().survey.title}
                                    }
                                    div {
                                        class: "text-[#4c4c4c] font-normal text-[20px]",
                                        {format!("{}: {}.{}.{}", translates.survey_complete_date, ctrl.end_year, ctrl.end_month, ctrl.end_day)}
                                    }
                                }
                                div {
                                    class: "flex flex-row w-[220px] h-[50px] justify-center items-center bg-[#2168c3] rounded-lg text-white font-medium text-[20px]",
                                    {translates.check_response_report}
                                }
                            }
                        }
                    }
                }
            }
            div {
                class: "flex flex-col max-w-[1200px] min-w-[600px] w-full justify-end items-end mt-[15px] px-[50px]",
                Link {
                    to: Route::SelectResponsePage {
                        lang: props.lang.clone(),
                        survey_id: props.survey_id.clone()
                    },
                    div {
                        class: "flex flex-row justify-center items-center w-[115px] h-[50px] rounded-[10px] bg-[#434343] text-white font-medium text-[20px]",
                        {translates.back}
                    }
                }
            }
        }
    }
}

#[component]
pub fn Summary(
    survey_summary: String,
    survey_progress_period: String,
    select_panel_and_attribute: String,
    written_question: String,
    panel: String,
    attribute: String,
    objective: String,
    subjective: String,
    total_question: String,
) -> Element {
    let mut ctrl = use_controller();
    rsx! {
        if ctrl.get_summary_clicked() {
            div {
                class: "flex flex-col max-w-[1200px] min-w-[600px] w-full justify-start items-start mt-[15px] px-[50px]",
                div {
                    class: "flex flex-col w-full justify-start items-center max-w-[1200px] rounded-[10px] bg-white px-[30px] py-[40px]",
                    div {
                        class: "flex flex-row w-full justify-between items-center mb-[30px]",
                        div {
                            class: "text-black font-semibold text-[24px] mb-[20px]",
                            {survey_summary}
                        },
                        img {
                            src: asset!("/public/images/arrow-down-black.png"),
                            class: "w-[35px] h-[35px]",
                            style: "transform: scaleY(-1);",
                            onclick: move |_e| {
                                ctrl.change_summary_clicked();
                            },
                        }
                    }
                    div {
                        class: "flex flex-col w-full justify-start items-start mb-[40px]",
                        div {
                            class: "flex flex-col w-full justify-start items-start",
                            div {
                                class: "text-[22px] font-semibold text-[#090909] mb-[20px]",
                                {survey_progress_period}
                            }
                            div {
                                class: "flex flex-row w-full justify-start items-start text-[20px] font-medium text-[#000000] ml-[10px]",
                                div {
                                    class: "font-semibold text-[#2168c3]",
                                    {(ctrl.start_year)()}
                                }
                                div {
                                    class: "mr-[8px]",
                                    "년"
                                }
                                div {
                                    class: "font-semibold text-[#2168c3]",
                                    {(ctrl.start_month)()}
                                }
                                div {
                                    class: "mr-[8px]",
                                    "월"
                                }
                                div {
                                    class: "font-semibold text-[#2168c3]",
                                    {(ctrl.start_day)()}
                                }
                                div {
                                    class: "mr-[8px]",
                                    "일"
                                }
                                div {
                                    class: "mr-[8px]",
                                    "~"
                                }
                                div {
                                    class: "font-semibold text-[#2168c3]",
                                    {(ctrl.end_year)()}
                                }
                                div {
                                    class: "mr-[8px]",
                                    "년"
                                }
                                div {
                                    class: "font-semibold text-[#2168c3]",
                                    {(ctrl.end_month)()}
                                }
                                div {
                                    class: "mr-[8px]",
                                    "월"
                                }
                                div {
                                    class: "font-semibold text-[#2168c3]",
                                    {(ctrl.end_day)()}
                                }
                                div {
                                    class: "mr-[8px]",
                                    "일"
                                }
                            }
                        }
                    }
                    div {
                        class: "flex flex-col w-full justify-start items-start mb-[30px]",
                        div {
                            class: "flex flex-col w-full justify-start items-start",
                            div {
                                class: "text-[22px] font-semibold text-[#090909] mb-[20px]",
                                {select_panel_and_attribute}
                            }
                            div {
                                class: "flex flex-col w-full justify-start items-start ml-[10px]",
                                div {
                                    class: "flex flex-row justify-start items-start text-[20px] mb-[10px]",
                                    div {
                                        class: "font-normal text-[#5e5e5e] mr-[7px]",
                                        {format!("{} :", panel)}
                                    }
                                    div {
                                        class: "font-semibold text-[#2168c3]",
                                        {format!("{}명", ctrl.get_total_panels())}
                                    }
                                }
                                div {
                                    class: "flex flex-row justify-start items-start text-[20px] mb-[10px]",
                                    div {
                                        class: "font-normal text-[#5e5e5e] mr-[7px]",
                                        {format!("{} :", attribute)}
                                    }
                                    div {
                                        class: "font-semibold text-[#2168c3]",
                                        {format!("{}개", ctrl.get_total_attributes())}
                                    }
                                }
                            }
                        }
                    }
                    div {
                        class: "flex flex-col w-full justify-start items-start mb-[30px]",
                        div {
                            class: "flex flex-col w-full justify-start items-start",
                            div {
                                class: "text-[22px] font-semibold text-[#090909] mb-[20px]",
                                {written_question}
                            }
                            div {
                                class: "flex flex-col w-full justify-start items-start ml-[10px]",
                                div {
                                    class: "flex flex-row justify-start items-start text-[20px] mb-[10px]",
                                    div {
                                        class: "font-normal text-[#5e5e5e] mr-[7px]",
                                        {format!("{} :", objective)}
                                    }
                                    div {
                                        class: "font-semibold text-[#2168c3]",
                                        {format!("{}개", ((ctrl.object_count)()))}
                                    }
                                }
                                div {
                                    class: "flex flex-row justify-start items-start text-[20px] mb-[10px]",
                                    div {
                                        class: "font-normal text-[#5e5e5e] mr-[7px]",
                                        {format!("{} :", subjective)}
                                    }
                                    div {
                                        class: "font-semibold text-[#2168c3]",
                                        {format!("{}개", ((ctrl.subject_count)()))}
                                    }
                                }
                                div {
                                    class: "flex flex-row justify-start items-start text-[20px] mb-[10px]",
                                    div {
                                        class: "font-normal text-[#5e5e5e] mr-[7px]",
                                        {format!("{} :", total_question)}
                                    }
                                    div {
                                        class: "font-semibold text-[#2168c3]",
                                        {format!("{}개", ((ctrl.total_count)()))}
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            div {
                class: "flex flex-col max-w-[1200px] min-w-[600px] w-full justify-start items-start mt-[15px] px-[50px]",
                div {
                    class: "flex flex-row w-full justify-start items-center max-w-[1200px] min-h-[150px] h-min rounded-[10px] bg-white px-[30px]",
                    div {
                        class: "flex flex-row w-full justify-between items-center mb-[20px]",
                        div {
                            class: "text-black font-semibold text-[24px] mb-[20px]",
                            {survey_summary}
                        },
                        img {
                            src: asset!("/public/images/arrow-down-black.png"),
                            class: "w-[35px] h-[35px]",
                            onclick: move |_e| {
                                ctrl.change_summary_clicked();
                            },
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn SelectPanel(select_panel_and_attribute: String, unknown: String) -> Element {
    let mut ctrl = use_controller();
    rsx! {
        if ctrl.get_panel_clicked() {
            div {
                class: "flex flex-col max-w-[1200px] min-w-[600px] w-full justify-start items-start mt-[15px] px-[50px]",
                div {
                    class: "flex flex-col w-full justify-start items-center max-w-[1200px] rounded-[10px] bg-white px-[30px] py-[40px]",
                    div {
                        class: "flex flex-row w-full justify-between items-center",
                        div {
                            class: "text-black font-semibold text-[24px] mb-[10px]",
                            {select_panel_and_attribute}
                        },
                        img {
                            src: asset!("/public/images/arrow-down-black.png"),
                            class: "w-[35px] h-[35px]",
                            style: "transform: scaleY(-1);",
                            onclick: move |_e| {
                                ctrl.change_panel_clicked();
                            },
                        }
                    }
                    div {
                        class: "flex flex-col w-full justify-start items-start",
                        div {
                            class: "flex flex-row w-full justify-start items-start text-[22px] mb-[15px]",
                            div {
                                class: "text-[#5e5e5e] font-normal mr-[5px]",
                                "총"
                            },
                            div {
                                class: "text-[#2168c3] font-semibold",
                                {format!("{}", {ctrl.get_total_panels()})}
                            }
                            div {
                                class: "text-[#2168c3] font-semibold",
                                "명"
                            }
                        }
                        for panel in ctrl.get_panels() {
                            match panel {
                                Quota::Attribute {
                                    salary_tier: _,
                                    region_code: _,
                                    gender,
                                    age,
                                    quota,
                                } => {
                                    rsx! {
                                        div {
                                            class: "flex flex-row w-full h-[60px] justify-between items-center odd:bg-white even:bg-[#f9f9f9] px-[20px]",
                                            //FIXME: fix to real api data
                                            div {
                                                class: "flex flex-row w-min justify-center items-center text-[20px] font-medium text-[#5e5e5e]",
                                                div {
                                                    class: "w-[180px]",
                                                    "대한민국(Korea)"
                                                }
                                                div {
                                                    class: "w-[70px]",
                                                    match gender {
                                                        Some(g) => match g {
                                                            Gender::Male => rsx! {
                                                                div {
                                                                    class: "text-[20px] font-medium text-[#5e5e5e]",
                                                                    "남성"
                                                                }
                                                            },
                                                            _ => rsx! {
                                                                div {
                                                                    class: "text-[20px] font-medium text-[#5e5e5e]",
                                                                    "여성"
                                                                }
                                                            }
                                                        },
                                                        None => rsx! {
                                                            div {
                                                                class: "text-[20px] font-medium text-[#5e5e5e]",
                                                                {unknown.clone()}
                                                            }
                                                        }
                                                    }
                                                }
                                                div {
                                                    class: "w-[100px]",
                                                    match age {
                                                        Some(a) => match a {
                                                            Age::Range { inclusive_min, inclusive_max } => rsx! {
                                                                div {
                                                                    class: "text-[20px] font-medium text-[#5e5e5e]",
                                                                    {format!("{}-{}대", inclusive_min, inclusive_max)}
                                                                }
                                                            },
                                                            Age::Specific(v) => rsx! {
                                                                div {
                                                                    class: "text-[20px] font-medium text-[#5e5e5e]",
                                                                    {format!("{}대", v)}
                                                                }
                                                            }
                                                        },
                                                        None => rsx! {
                                                            div {
                                                                class: "text-[20px] font-medium text-[#5e5e5e]",
                                                                {unknown.clone()}
                                                            }
                                                        }
                                                    }
                                                }
                                                div {
                                                    class: "w-[150px]",
                                                    "사무직"
                                                }
                                            }
                                            div {
                                                class: "flex flex-row w-min justify-center items-center text-[20px] font-medium text-[#000000]",
                                                div {
                                                    class: "mr-[7px]",
                                                    "총"
                                                }
                                                div {
                                                    class: "text-[#1e5eaf]",
                                                    {format!("{}", {quota})}
                                                }
                                                div {
                                                    "명"
                                                }
                                            }
                                        }
                                    }
                                },
                                _ => {
                                    rsx! {}
                                }
                            }
                        }
                    }
                }
            }
        } else {
            div {
                class: "flex flex-col max-w-[1200px] min-w-[600px] w-full justify-start items-start mt-[15px] px-[50px]",
                div {
                    class: "flex flex-row w-full justify-start items-center max-w-[1200px] min-h-[150px] h-min rounded-[10px] bg-white px-[30px]",
                    div {
                        class: "flex flex-row w-full justify-between items-center",
                        div {
                            class: "text-black font-semibold text-[24px] mb-[10px]",
                            {select_panel_and_attribute}
                        },
                        img {
                            src: asset!("/public/images/arrow-down-black.png"),
                            class: "w-[35px] h-[35px]",
                            onclick: move |_e| {
                                ctrl.change_panel_clicked();
                            },
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn ListSurvey(
    question_count: u64,
    question_list: Vec<Question>,
    response_list: String,
    total: String,
    num_of_detail: String,
    num_of: String,
) -> Element {
    let mut ctrl = use_controller();
    let survey_list_clicked = ctrl.get_survey_list_clicked();

    let mut questions: Vec<QuestionModel> = vec![];

    for question in question_list {
        let question_title = question.question.clone();
        let options = question.options.clone();

        let gsi2: Vec<&str> = question.gsi2.split("#").collect();
        let survey_type = gsi2[3];

        if survey_type == "single-choice" {
            questions.push(QuestionModel {
                title: question_title.clone(),
                questions: options.unwrap(),
            });
        } else if survey_type == "text" {
            questions.push(QuestionModel {
                title: question_title.clone(),
                questions: vec![],
            });
        } else {
            questions.push(QuestionModel {
                title: question_title.clone(),
                questions: vec![],
            });
        }
    }

    rsx! {
        if survey_list_clicked {
            div {
                class: "flex flex-col max-w-[1200px] min-w-[600px] w-full justify-start items-start mt-[15px] px-[50px]",
                div {
                    class: "flex flex-col w-full justify-start items-start max-w-[1200px] min-h-[150px] h-min rounded-[10px] bg-white p-[30px]",
                    div {
                        class: "flex flex-row w-full justify-between items-center mb-[10px]",
                        div {
                            class: "text-black font-semibold text-[24px]",
                            {response_list}
                        },
                        img {
                            src: asset!("/public/images/arrow-down-black.png"),
                            class: "w-[35px] h-[35px]",
                            style: "transform: scaleY(-1);",
                            onclick: move |_e| {
                                ctrl.change_survey_list_clicked();
                            },
                        }
                    }
                    div {
                        class: "flex flex-row w-full justify-start items-start mb-[10px]",
                        div {
                            class: "text-[#5e5e5e] font-normal text-[20px] mr-[10px]",
                            {total}
                        },
                        div {
                            class: "text-[#2168c3] font-semibold text-[20px]",
                            {format!("{:?}{:?}", question_count, num_of).replace('"', "")}
                        }
                    },
                    for i in 0..question_count {
                        div {
                            class: "flex flex-row w-full min-h-[190px] h-min justify-start items-start odd:bg-[#f9f9f9] even:bg-white px-[20px] py-[10px] border border-b-[#9f9f9f] border-t-transparent border-l-transparent border-r-transparent",
                            div {
                                class: "text-black font-semibold text-[20px] mr-[20px]",
                                {format!("Q{:?}", i + 1)}
                            }
                            div {
                                class: "flex flex-col w-full justify-start items-start",
                                div {
                                    class: "text-[#5e5e5e] font-normal text-[20px]",
                                    {questions.get(i as usize).unwrap().title.clone()}
                                }
                                div {
                                    class: "flex flex-col w-full justify-start items-start pl-[30px] mt-[20px]",
                                    for j in 0..questions.get(i as usize).unwrap().questions.len() {
                                        div {
                                            class: "text-[#5e5e5e] font-normal text-[20px] mb-[10px]",
                                            {format!("{:?}. {:?}", j + 1, questions.get(i as usize).unwrap().questions.get(j as usize).unwrap().clone()).replace('"', "")}
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            div {
                class: "flex flex-col max-w-[1200px] min-w-[600px] w-full justify-start items-start mt-[15px] px-[50px]",
                div {
                    class: "flex flex-row w-full justify-start items-center max-w-[1200px] min-h-[150px] h-min rounded-[10px] bg-white px-[30px]",
                    div {
                        class: "flex flex-row w-full justify-between items-center",
                        div {
                            class: "flex flex-col w-full justify-center items-start",
                            div {
                                class: "text-black font-semibold text-[24px] mb-[10px]",
                                {response_list}
                            },
                            div {
                                class: "text-[#5e5e5e] font-normal text-[22px]",
                                {format!("{:?} {:?}{:?}", total, question_count, num_of_detail).replace('"', "")},
                            },
                        }
                        img {
                            src: asset!("/public/images/arrow-down-black.png"),
                            onclick: move |_e| {
                                ctrl.change_survey_list_clicked();
                            },
                            class: "w-[35px] h-[35px]",
                        }
                    }
                }
            }
        }

    }
}
