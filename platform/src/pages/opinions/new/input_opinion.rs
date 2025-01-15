use dioxus::prelude::*;
use dioxus_translate::Language;

use crate::{
    components::icons::{Search, Switch, UploadFile},
    pages::opinions::new::controller::Controller,
};

#[derive(Props, Clone, PartialEq)]
pub struct InputOpinionProps {
    lang: Language,
}

#[derive(Clone, PartialEq)]
pub enum DocumentTabType {
    DirectUpload,
    Import,
}

#[component]
pub fn InputOpinion(props: InputOpinionProps) -> Element {
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "font-medium text-[16px] text-[#000000] mb-[10px]", "공론 필수 정보" }
            InputIntroduction { lang: props.lang.clone() }
            UploadDocument {}
            ConnectProject {}

            div { class: "flex flex-row w-full justify-end items-end mt-[40px] mb-[50px]",
                div { class: "flex flex-row w-[70px] h-[55px] rounded-[4px] justify-center items-center bg-white border border-[#bfc8d9] font-semibold text-[16px] text-[#555462] mr-[20px]",
                    "뒤로"
                }
                div {
                    class: "flex flex-row w-[105px] h-[55px] rounded-[4px] justify-center items-center bg-white border border-[#bfc8d9] font-semibold text-[16px] text-[#555462] mr-[20px]",
                    onclick: move |_| {},
                    "임시저장"
                }
                div { class: "cursor-pointer flex flex-row w-[110px] h-[55px] rounded-[4px] justify-center items-center bg-[#b4b4b4] font-semibold text-[16px] text-white",
                    "다음으로"
                }
            }
        }
    }
}

#[component]
pub fn ConnectProject() -> Element {
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start rounded-lg bg-white px-[40px] py-[24px] mb-[100px]",
            div { class: "flex flex-col w-full mb-[10px]",
                div { class: "text-[18px] font-bold text-[#3a3a3a]", "조사 프로젝트 연동" }
                div { class: "text-[14px] font-medium text-[#6d6d6d]",
                    "해당 공론과 관련된 조사자료를 불러와주세요. (예. 여론조사, 설문조사, 기타조사 등)"
                }
            }
            div { class: "flex flex-row w-full h-[55px] justify-start items-center p-[15px] font-medium text-[15px] text-[#b4b4b4] bg-[#f7f7f7] rounded-[4px]",
                "조사 선택"
            }
        }
    }
}

#[component]
pub fn UploadDocument() -> Element {
    let mut tab_type = use_signal(|| DocumentTabType::DirectUpload);
    let mut is_focused = use_signal(|| false);
    let mut document_name = use_signal(|| "".to_string());
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start rounded-lg bg-white px-[40px] py-[24px] mb-[20px]",
            div { class: "flex flex-col w-full mb-[20px]",
                div { class: "text-[18px] font-bold text-[#3a3a3a]", "자료 업로드" }
                div { class: "text-[14px] font-medium text-[#6d6d6d]",
                    "해당 공론과 관련된 자료를 업로드해주세요. (예. 공론 소개, 참여 방법, 가이드라인)"
                }
            }

            div { class: "flex flex-col w-full justify-start items-start",
                div { class: "flex flex-row w-full",
                    button {
                        class: format!(
                            "flex flex-row w-[150px] h-[55px] justify-center items-center rounded-t-[4px] font-semibold text-[14px] mr-[10px] {}",
                            if tab_type() == DocumentTabType::DirectUpload {
                                "bg-[#2a60d3] text-white "
                            } else {
                                "bg-white border border-t-[#2a60d3] border-l-[#2a60d3] border-r-[#2a60d3] border-b-transparent text-[#2a60d3]"
                            },
                        ),
                        onclick: move |_| {
                            tab_type.set(DocumentTabType::DirectUpload);
                        },
                        "직접 업로드하기"
                    }
                    button {
                        class: format!(
                            "flex flex-row w-[170px] h-[55px] justify-center items-center rounded-t-[4px] font-semibold text-[14px] {}",
                            if tab_type() == DocumentTabType::Import {
                                "bg-[#2a60d3] text-white "
                            } else {
                                "bg-white border border-t-[#2a60d3] border-l-[#2a60d3] border-r-[#2a60d3] border-b-transparent text-[#2a60d3]"
                            },
                        ),
                        onclick: move |_| {
                            tab_type.set(DocumentTabType::Import);
                        },
                        "자료관리에서 불러오기"
                    }
                }

                if tab_type() == DocumentTabType::DirectUpload {
                    div { class: "flex flex-col w-full",
                        div { class: "flex flex-col w-full justify-center items-center p-[24px] rounded-[8px] border-[2px] border-dashed border-[#2a60d3] mb-[10px]",
                            div { class: "mb-[12px] w-[42px] h-[42px]",
                                UploadFile { width: "42", height: "42" }
                            }
                            div { class: "font-normal text-[#222222] text-sm mb-[8px]",
                                "업로드할 파일을 드래그해주세요."
                            }
                            div { class: "flex flex-row w-full justify-center items-center mb-[8px]",
                                div { class: "w-[80px] h-[1px] bg-[#e7e7e7] mr-[12px]" }
                                div { class: "font-normal text-[#6d6d6d] text-sm mr-[12px]",
                                    "OR"
                                }
                                div { class: "w-[80px] h-[1px] bg-[#e7e7e7] mr-[12px]" }
                            }
                            div { class: "flex flex-row w-[100px] h-[30px] justify-center items-center bg-white border border-[#1849d6] rounded-[4px] font-semibold text-[#1849d6] text-sm",
                                "파일 불러오기"
                            }
                        }

                        div { class: "font-normal text-[#6d6d6d] text-[14px]",
                            "jpg, .png, pdf, zip, word, excel, pptx 파일만 업로드 가능합니다."
                        }
                    }
                } else {
                    div { class: "flex flex-col w-full",
                        div { class: "flex flex-col w-full justify-start items-start p-[24px] border border-[#2a60d3] rounded-tr-lg rounded-b-lg mb-[20px]",
                            div {
                                class: format!(
                                    "flex flex-row w-full h-[45px] justify-start items-center rounded-lg  {} px-[11px] py-[13px] mb-[20px]",
                                    if (is_focused)() {
                                        "bg-[#ffffff] border border-[#2a60d3]"
                                    } else {
                                        "bg-[#f7f7f7] border border-[#7c8292]"
                                    },
                                ),
                                Search {
                                    width: "18",
                                    height: "18",
                                    color: "#7c8292",
                                }
                                input {
                                    class: "flex flex-row w-full h-full bg-transparent focus:outline-none ml-[10px]",
                                    r#type: "text",
                                    placeholder: "Enter public name or email address".to_string(),
                                    value: (document_name)(),
                                    onfocus: move |_| {
                                        is_focused.set(true);
                                    },
                                    onblur: move |_| {
                                        is_focused.set(false);
                                    },
                                    oninput: move |event| {
                                        document_name.set(event.value());
                                    },
                                }
                            }

                            //table
                            div { class: "flex flex-col w-full justify-start items-start bg-white",
                                div { class: "flex flex-row w-full h-[55px] justify-start items-center border border-t-[#bfc8d9] border-l-[#bfc8d9] border-r-[#bfc8d9] border-b-transparent rounded-[4px]",
                                    div { class: "flex flex-row w-[60px] min-w-[60px] h-full justify-center items-center gap-[10px]" }
                                    div { class: "flex flex-row flex-1 h-full justify-center items-center gap-[10px]",
                                        div { class: "text-[#7c8292] font-semibold text-[14px]",
                                            "제목"
                                        }
                                        Switch { width: "19", height: "19" }
                                    }
                                    div { class: "flex flex-row w-[100px] min-w-[100px] h-full justify-center items-center gap-[10px]",
                                        div { class: "text-[#7c8292] font-semibold text-[14px]",
                                            "유형"
                                        }
                                        Switch { width: "19", height: "19" }
                                    }
                                    div { class: "flex flex-row w-[100px] min-w-[100px] h-full justify-center items-center gap-[10px]",
                                        div { class: "text-[#7c8292] font-semibold text-[14px]",
                                            "분야"
                                        }
                                        Switch { width: "19", height: "19" }
                                    }
                                    div { class: "flex flex-row w-[100px] min-w-[100px] h-full justify-center items-center gap-[10px]",
                                        div { class: "text-[#7c8292] font-semibold text-[14px]",
                                            "활용 목적"
                                        }
                                        Switch { width: "19", height: "19" }
                                    }
                                    div { class: "flex flex-row w-[100px] min-w-[100px] h-full justify-center items-center gap-[10px]",
                                        div { class: "text-[#7c8292] font-semibold text-[14px]",
                                            "출처"
                                        }
                                        Switch { width: "19", height: "19" }
                                    }
                                    div { class: "flex flex-row w-[100px] min-w-[100px] h-full justify-center items-center gap-[10px]",
                                        div { class: "text-[#7c8292] font-semibold text-[14px]",
                                            "권한"
                                        }
                                        Switch { width: "19", height: "19" }
                                    }
                                    div { class: "flex flex-row w-[100px] min-w-[100px] h-full justify-center items-center gap-[10px]",
                                        div { class: "text-[#7c8292] font-semibold text-[14px]",
                                            "최종 수정일"
                                        }
                                        Switch { width: "19", height: "19" }
                                    }
                                    div { class: "flex flex-row w-[100px] min-w-[100px] h-full justify-center items-center gap-[10px]",
                                        div { class: "text-[#7c8292] font-semibold text-[14px]",
                                            "형식"
                                        }
                                        Switch { width: "19", height: "19" }
                                    }
                                }
                            }
                        }

                        //info
                        div { class: "font-normal text-[#6d6d6d] text-[14px]",
                            "jpg, .png, pdf, zip, word, excel, pptx 파일만 업로드 가능합니다."
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn InputIntroduction(lang: Language) -> Element {
    let mut ctrl: Controller = use_context();

    let information = ctrl.get_opinion_informations();
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start rounded-lg bg-white px-[40px] py-[24px] mb-[20px]",
            div { class: "flex flex-row w-full justify-start items-start",
                div { class: "text-[16px] font-bold text-[#eb5757] mt-[5px] mr-[2px]",
                    "*"
                }
                div { class: "text-[18px] font-bold text-[#3a3a3a]", "소개글 입력" }
            }
            div { class: "text-[14px] font-medium text-[#6d6d6d] mb-[10px]",
                "공론의 주제와 목적에 대해 설명해주세요. 참여자들이 더 쉽게 이해하고 적극적으로 참여할 수 있을 것입니다."
            }

            div { class: "flex flex-row w-full justify-start items-center",
                select {
                    class: "focus:outline-none w-[215px] h-[55px] justify-start items-start p-[15px] bg-[#f7f7f7] rounded-[4px] mr-[20px]",
                    value: if information.opinion_type.is_none() { "".to_string() } else { ctrl.opinion_field_type_translate(
                            lang.clone(),
                            information.opinion_type.clone().unwrap(),
                        )
                        .to_string() },
                    onchange: {
                        let information = information.clone();
                        move |e: Event<FormData>| {
                            let mut value = information.clone();
                            let opinion_field_type = ctrl.update_opinion_field_type_from_str(e.value());
                            value.opinion_type = opinion_field_type;
                            ctrl.update_opinion_information(value);
                        }
                    },
                    option {
                        value: "",
                        disabled: true,
                        selected: information.opinion_type.is_none(),
                        "분야 선택"
                    }
                    for field in ctrl.get_total_fields() {
                        option {
                            value: field.clone(),
                            selected: information.opinion_type.is_some()
                                && ctrl
                                    .opinion_field_type_translate(
                                        lang.clone(),
                                        information.opinion_type.clone().unwrap(),
                                    ) == field,
                            "{field}"
                        }
                    }
                }

                div { class: "flex flex-row w-full focus:outline-none h-[55px] justify-start items-center bg-[#f7f7f7] rounded-[4px] px-[15px]",
                    input {
                        class: "flex flex-row w-full justify-start items-center bg-transparent focus:outline-none",
                        r#type: "text",
                        placeholder: "제목을 입력해주세요.",
                        value: if information.title.is_none() { "" } else { information.title.clone().unwrap() },
                        oninput: {
                            let information = information.clone();
                            move |e: FormEvent| {
                                let mut value = information.clone();
                                value.title = Some(e.value());
                                ctrl.update_opinion_information(value);
                            }
                        },
                    }
                }
            }

            div { class: "flex flex-row w-full",
                div { class: "flex flex-row w-full h-[1px] bg-[#ebeff5] my-[10px]" }
            }

            div { class: "flex flex-row w-full focus:outline-none h-[55px] justify-start items-center px-[15px] border-b border-[#bfc8d9]",
                input {
                    class: "flex flex-row w-full justify-start items-center bg-transparent focus:outline-none",
                    r#type: "text",
                    placeholder: "내용을 입력해주세요.",
                    value: if information.description.is_none() { "" } else { information.description.clone().unwrap() },
                    oninput: {
                        let information = information.clone();
                        move |e: FormEvent| {
                            let mut value = information.clone();
                            value.description = Some(e.value());
                            ctrl.update_opinion_information(value);
                        }
                    },
                }
            }
        }
    }
}
