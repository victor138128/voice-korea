#![allow(non_snake_case)]
use crate::{
    models::question::{QuestionStatus, QuestionSummary},
    prelude::*,
};
use controller::QuestionDashboards;
use dioxus::prelude::*;

pub mod controller;

#[derive(PartialEq, Props, Clone)]
pub struct DashboardPageProps {
    lang: Language,
}

#[derive(PartialEq, Props, Clone)]
pub struct TypeOneProps {
    questions: Vec<QuestionDashboards>,
}

#[derive(PartialEq, Props, Clone)]
pub struct TypeTwoProps {
    questions: Vec<QuestionDashboards>,
}

#[component]
pub fn DashboardPage(props: DashboardPageProps) -> Element {
    let mut ctrl = controller::Controller::init();
    rsx! {
        div {
            class: "flex flex-col w-full h-full pt-[45px] pr-[45px] pl-[35px] items-start justify-start",
            div {
                class: "flex flex-row w-full items-start justify-between",
                div {
                    class: "flex flex-row mr-[10px]",
                    div {
                        class: "flex flex-row w-[450px] h-[55px] items-center bg-white border-solid border border-[#e0e0e0] rounded-[8px] pl-[15px] mr-[10px]",
                        img {
                            class: "mr-[5px]",
                            src: "/images/search.png",
                            width: 24,
                            height: 24
                        }
                        input {
                            class: "flex flex-1 text-[21px] text-[#8a8a8a] font-normal",
                            "type": "text",
                            style: "border:0px; padding: 5px; border-color: transparent; outline-style: none; box-shadow: none",
                            placeholder: "Search",
                            min: 0,
                        }
                    }
                    div {
                        class: "flex flex-row rounded-tl-[8px] rounded-bl-[8px] w-[55px] h-[55px] bg-[#4c4c4c] items-center justify-center mr-[2px]",
                        onclick: move |_| ctrl.set_clicked_type(0),
                        img {
                            src: "/images/menu_1.png",
                            width: 27,
                            height: 27
                        }
                    }
                    div {
                        class: "flex flex-row rounded-tr-[8px] rounded-br-[8px] w-[55px] h-[55px] bg-[#c8c8c8] items-center justify-center",
                        onclick: move |_| ctrl.set_clicked_type(1),
                        img {
                            src: "/images/menu_2.png",
                            width: 27,
                            height: 27
                        }
                    }
                }
                div {
                    class: "flex flex-row w-[200px] h-[50px] justify-end items-end bg-[#2168c3] rounded-[8px]",
                    div {
                        class: "flex flex-row w-full h-full justify-center items-center text-[21px] font-semibold text-white",
                        "설문 만들기"
                    }
                }
            }
            if ctrl.get_clicked_type() == 0 {
                TypeOneQuestionComponent {
                    questions: ctrl.get_total_questions()
                }
            } else {
                TypeTwoQuestionComponent {
                    questions: ctrl.get_total_questions()
                }
            }
        }
    }
}

pub fn TypeOneQuestionComponent(props: TypeOneProps) -> Element {
    let questions = props.questions;
    rsx! {
        div {
            "type-1"
        }
    }
}

pub fn TypeTwoQuestionComponent(props: TypeTwoProps) -> Element {
    rsx! {
        div {
            "type-2"
        }
    }
}
