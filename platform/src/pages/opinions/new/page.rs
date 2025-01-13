#![allow(non_snake_case)]
use crate::{
    components::{icons::ArrowLeft, stepper::Stepper},
    routes::Route,
};

use super::controller::Controller;
use super::i18n::OpinionNewTranslate;
use dioxus::prelude::*;
use dioxus_translate::{translate, Language};

#[derive(Props, Clone, PartialEq)]
pub struct OpinionProps {
    lang: Language,
}

#[component]
pub fn OpinionCreatePage(props: OpinionProps) -> Element {
    let _ctrl = Controller::init(props.lang);
    let translates: OpinionNewTranslate = translate(&props.lang.clone());
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "text-[#9b9b9b] font-medium text-[14px] mb-[10px]",
                "{translates.organization_management} / {translates.public_opinion_management}"
            }
            div { class: "flex flex-row w-full justify-start items-center mb-[25px]",
                Link {
                    class: "mr-[6px]",
                    to: Route::OpinionPage {
                        lang: props.lang,
                    },
                    ArrowLeft { width: "24", height: "24", color: "#3a3a3a" }
                }
                div { class: "text-[#3a3a3a] font-semibold text-[28px] mr-[20px]",
                    "{translates.start_public_opinion}"
                }
            }

            div { class: "flex flex-col w-full justify-start items-center mt-[20px]",
                div { class: "flex flex-row w-[1400px] min-w-[1400px] justify-center items-center",
                    Stepper {
                        current_step: 2,
                        steps: vec![
                            "공론 구성 및 기간".to_string(),
                            "필수정보 입력".to_string(),
                            "공론 위원회 구성".to_string(),
                            "참여자 패널 구성".to_string(),
                            "토론 설정".to_string(),
                            "전체 미리보기".to_string(),
                        ],
                    }
                }
            }
        }
    }
}
