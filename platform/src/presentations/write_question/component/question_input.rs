#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::{
    components::{button::Button, select::Select},
    presentations::write_question::controller::Controller,
};

#[derive(Props, Clone, PartialEq)]
pub struct QuestionInputProps {
    ctrl: Controller,
    temporary_save: String,
    input_question: String,
    next_question: String,
    save_label: String,
    cancel_label: String,
}

pub fn QuestionInput(props: QuestionInputProps) -> Element {
    let mut ctrl = props.ctrl;
    let survey = ctrl.get_survey();
    let questions = ctrl.get_question_types();
    let selected_question = ctrl.get_selected_question();
    rsx! {
        Fragment {
            div {
                class: "flex flex-row w-full justify-end items-end mb-[20px]",
                Button {
                    button_text: props.temporary_save.clone(),
                    onclick: move |_| {},
                    class: "flex flex-row w-[200px] h-[50px] bg-[#1e5eaf]",
                }
            }
            div {
                class: "flex flex-row w-full h-[110px] rounded-[10px] bg-white mb-[10px]",
                div {
                    class: "flex flex-row w-full h-[110px] items-center justify-start text-[#2168c3] font-semibold text-[30px] pl-[30px]",
                    "{survey.survey.title}"
                }
            }
            div {
                class: "flex flex-col w-full rounded-[10px] bg-white mb-[10px] p-[30px]",
                div {
                    class: "flex flex-row w-full",
                    input {
                        class: "flex flex-1 text-[21px] text-[#8a8a8a] font-normal",
                        "type": "text",
                        style: "border:0px; padding: 5px; border-color: transparent; outline-style: none; box-shadow: none; border-bottom: 1px solid #9f9f9f;",
                        placeholder: props.input_question.clone(),
                        onchange: move |_e| {
                        },
                    }
                    Select {
                        class: "w-[205px] h-[50px] rounded-lg font-normal text-[16px] text-black pt-[10px] pb-[10px] pl-[13px] pr-[13px] ml-[10px]",
                        border: "1px solid #9f9f9f",
                        value: questions
                            .get(selected_question as usize)
                            .unwrap()
                            .clone()
                            .value,
                        onchange: move |evt: Event<FormData>| {
                            let data = evt.clone().value();
                            ctrl.change_selected_question(data.parse::<u64>().unwrap_or(0));
                        },
                        component: rsx! {
                            {questions.iter().map(|opt| {
                                rsx! {
                                    option { label: opt.label.clone(), value: opt.value.clone(), color: "#000000" }
                                }
                            })}
                        }
                    }
                }
                div {
                    class: "flex flex-row w-full justify-between items-center mt-[30px]",
                    Button {
                        button_text: props.next_question,
                        onclick: move |_| {},
                        class: "flex flex-row w-[120px] h-[50px] bg-[#3a94ff]",
                    }
                    div {
                        class: "flex flex-row justify-center items-center",
                        Button {
                            button_text: props.cancel_label,
                            onclick: move |_| {},
                            class: "flex flex-row w-[80px] h-[50px] bg-[#434343] mr-[10px]",
                        }
                        Button {
                            button_text: props.save_label,
                            onclick: move |_| {},
                            class: "flex flex-row w-[80px] h-[50px] bg-[#2168c3]",
                        }
                    }
                }
            }

        }
    }
}
