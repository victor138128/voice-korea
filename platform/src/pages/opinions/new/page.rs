#![allow(non_snake_case)]
use crate::{
    components::{icons::ArrowLeft, stepper::Stepper},
    pages::opinions::new::{
        composition_opinion::CompositionOpinion, controller::CurrentStep,
        input_opinion::InputOpinion,
    },
    routes::Route,
};

use super::i18n::OpinionNewTranslate;
use super::{composition_commitee::CompositionCommitee, controller::Controller};
use dioxus::prelude::*;
use dioxus_translate::{translate, Language};

#[derive(Props, Clone, PartialEq)]
pub struct OpinionProps {
    lang: Language,
}

#[component]
pub fn OpinionCreatePage(props: OpinionProps) -> Element {
    let translates: OpinionNewTranslate = translate(&props.lang.clone());
    let ctrl = Controller::init(props.lang, translates.clone());

    let step = ctrl.get_current_step();
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

            div { class: "flex flex-col w-full justify-start items-center mt-[20px] mb-[80px]",
                div { class: "flex flex-row w-[1400px] min-w-[1400px] justify-center items-center",
                    Stepper {
                        current_step: if step == CurrentStep::PublicOpinionComposition { 1 } else { 2 },
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


            if step == CurrentStep::PublicOpinionComposition {
                CompositionOpinion { lang: props.lang.clone() }
            } else if step == CurrentStep::InputInformation {
                InputOpinion { lang: props.lang.clone() }
            } else if step == CurrentStep::CommitteeComposition {
                CompositionCommitee { lang: props.lang.clone() }
            } else {
                div { "Hello" }
            }
        }
    }
}
