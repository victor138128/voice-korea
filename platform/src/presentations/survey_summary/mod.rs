#![allow(non_snake_case)]
use crate::{
    models::question::{Question, QuestionType},
    prelude::*,
};
use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct SurveySummaryProps {
    lang: Language,
    title: String,
}

pub struct QuestionModel {
    title: String,
    questions: Vec<String>,
}

pub mod controller;
pub mod i18n;

#[component]
pub fn SurveySummaryPage(props: SurveySummaryProps) -> Element {
    let ctrl = controller::Controller::init();
    let translates = i18n::translate(props.lang.clone());
    rsx! {
        div {
            class: "flex flex-col w-full h-full justify-start items-center",
            div {
                class: "flex flex-col max-w-[1200px] min-w-[600px] w-full justify-end items-end mt-[15px] px-[50px]",
                div {
                    class: "flex flex-row w-[250px] h-[55px] mt-[55px] rounded-[8px] bg-[#2168c3] justify-center items-center text-[21px] font-semibold text-white",
                    {translates.start_survey}
                }
            }
            div {
                class: "flex flex-col max-w-[1200px] min-w-[600px] w-full justify-start items-start mt-[15px] px-[50px]",
                div {
                    class: "flex flex-col w-full justify-center items-start max-w-[1200px] h-[100px] rounded-[10px] bg-white px-[30px]",
                    div {
                        class: "text-[#2168c3] font-semibold text-[30px] mb-[20px]",
                        "설문지 제목입니다."
                    },
                }
            }
            div {
                class: "flex flex-col max-w-[1200px] min-w-[600px] w-full justify-start items-start mt-[15px] px-[50px]",
                div {
                    class: "flex flex-col w-full justify-center items-start max-w-[1200px] h-[140px] rounded-[10px] bg-white px-[30px]",
                    div {
                        class: "text-black font-semibold text-[24px] mb-[10px]",
                        {translates.set_response_date}
                    },
                    div {
                        class: "flex flex-row w-min h-[30px] justify-start items-start border border-b-[1px] border-b-black border-l-transparent border-r-transparent border-t-transparent",
                        div {
                            class: "text-[#2168c3] font-semibold text-[20px]",
                            "2024"
                        }
                        div {
                            class: "text-[#000000] font-semibold text-[20px] mr-[10px]",
                            {translates.year.clone()}
                        }
                        div {
                            class: "text-[#2168c3] font-semibold text-[20px]",
                            "10"
                        }
                        div {
                            class: "text-[#000000] font-semibold text-[20px] mr-[10px]",
                            {translates.month.clone()}
                        }
                        div {
                            class: "text-[#2168c3] font-semibold text-[20px]",
                            "01"
                        }
                        div {
                            class: "text-[#000000] font-semibold text-[20px] mr-[10px]",
                            {translates.day.clone()}
                        }
                        div {
                            class: "text-[#000000] font-semibold text-[20px] mr-[10px]",
                            "~"
                        }
                        div {
                            class: "text-[#2168c3] font-semibold text-[20px]",
                            "2024"
                        }
                        div {
                            class: "text-[#000000] font-semibold text-[20px] mr-[10px]",
                            {translates.year}
                        }
                        div {
                            class: "text-[#2168c3] font-semibold text-[20px]",
                            "10"
                        }
                        div {
                            class: "text-[#000000] font-semibold text-[20px] mr-[10px]",
                            {translates.month}
                        }
                        div {
                            class: "text-[#2168c3] font-semibold text-[20px]",
                            "10"
                        }
                        div {
                            class: "text-[#000000] font-semibold text-[20px] mr-[10px]",
                            {translates.day}
                        }
                    }
                }
            }
            div {
                class: "flex flex-col max-w-[1200px] min-w-[600px] w-full justify-start items-start mt-[15px] px-[50px]",
                div {
                    class: "flex flex-row w-full justify-between items-center max-w-[1200px] h-[100px] rounded-[10px] bg-white px-[30px]",
                    div {
                        class: "text-black font-semibold text-[24px] mb-[20px]",
                        {translates.survey_summary}
                    },
                    img {
                        src: asset!("public/images/arrow-down-black.png"),
                        class: "w-[35px] h-[35px]",
                    }
                }
            }
            div {
                class: "flex flex-col max-w-[1200px] min-w-[600px] w-full justify-start items-start mt-[15px] px-[50px]",
                div {
                    class: "flex flex-row w-full justify-between items-center max-w-[1200px] h-[150px] rounded-[10px] bg-white px-[30px]",
                    div {
                        class: "flex flex-col w-full justify-center items-start",
                        div {
                            class: "text-black font-semibold text-[24px] mb-[10px]",
                            {translates.select_panel_and_attribute}
                        },
                        div {
                            class: "text-[#5e5e5e] font-normal text-[22px]",
                            {format!("{:?}{:?}", 250, translates.response_info).replace('"', "")},
                        },
                    }
                    img {
                        src: asset!("public/images/arrow-down-black.png"),
                        class: "w-[35px] h-[35px]",
                    }
                }
            }
            ListSurvey {
                ctrl,
                question_count: ctrl.get_questions().len() as u64,
                question_list: ctrl.get_questions(),
                response_list: translates.response_list,
                total: translates.total,
                num_of_detail: translates.num_of_detail,
                num_of: translates.num_of,
            }
            div {
                class: "flex flex-col max-w-[1200px] min-w-[600px] w-full justify-end items-end mt-[15px] px-[50px]",
                Link {
                    to: Route::SelectResponsePage {
                        lang: props.lang.clone(),
                        title: props.title.clone()
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
pub fn ListSurvey(
    ctrl: controller::Controller,
    question_count: u64,
    question_list: Vec<Question>,
    response_list: String,
    total: String,
    num_of_detail: String,
    num_of: String,
) -> Element {
    let survey_list_clicked = ctrl.get_survey_list_clicked();

    let mut questions: Vec<QuestionModel> = vec![];

    for question in question_list {
        match question.question.clone() {
            QuestionType::LongText(long_text_title) => {
                questions.push(QuestionModel {
                    title: long_text_title.unwrap_or("".to_string()).clone(),
                    questions: vec![],
                });
            }
            QuestionType::SingleChoice { question, options } => {
                questions.push(QuestionModel {
                    title: question.unwrap_or("".to_string()).clone(),
                    questions: options,
                });
            }
            QuestionType::Text(text_title) => {
                questions.push(QuestionModel {
                    title: text_title.unwrap_or("".to_string()).clone(),
                    questions: vec![],
                });
            }
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
                            src: asset!("public/images/arrow-down-black.png"),
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
                            src: asset!("public/images/arrow-down-black.png"),
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
