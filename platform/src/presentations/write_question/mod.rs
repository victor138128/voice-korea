#![allow(non_snake_case)]
use crate::prelude::*;
use dioxus::prelude::*;

mod controller;
mod i18n;

#[derive(PartialEq, Props, Clone)]
pub struct WriteQuestionProps {
    lang: Language,
    title: String,
}

#[component]
pub fn WriteQuestionPage(props: WriteQuestionProps) -> Element {
    let mut ctrl = controller::Controller::init(props.title.clone());
    let translates = i18n::translate(props.lang.clone());
    let survey = ctrl.get_survey();

    //TODO: write question page
    rsx! {
        div {
            class: "flex flex-col w-full h-full justify-start items-center",
            div {
                class: "flex flex-col max-w-[1200px] min-w-[600px] w-full justify-start items-start mt-[15px] px-[50px]",
                div {
                    class: "flex flex-row w-full h-[110px] rounded-[10px] bg-white mb-[10px]",
                    div {
                        class: "flex flex-row w-full h-[110px] items-center justify-start text-[#2168c3] font-semibold text-[30px] pl-[30px]",
                        "{survey.title}"
                    }
                }
                div {
                    class: "flex flex-row w-full h-[90px] rounded-[10px] bg-white justify-center items-center mb-[30px]",
                    div {
                        class: "flex flex-row w-[200px] h-[50px] rounded-[20px] bg-[#d6d6d6] justify-center items-center",
                        img {
                            class: "flex flex-col pr-[10px]",
                            src: "/images/add.png",
                            alt: "add question",
                        }
                        div {
                            class: "text-[20px] font-medium text-black",
                            "{translates.add_question}"
                        }
                    }
                }
                div {
                    class: "flex flex-row w-full justify-end items-end",
                    div {
                        class: "flex flex-row justify-center items-center w-[115px] h-[50px] rounded-[10px] bg-[#434343] text-white font-medium text-[20px] mr-[20px]",
                        "{translates.back_label}"
                    }
                    div {
                        class: "flex flex-row justify-center items-center w-[115px] h-[50px] rounded-[10px] bg-[#2168c3] text-white font-medium text-[20px] mr-[20px]",
                        "{translates.save_label}"
                    }
                }
            }
        }
    }
}
