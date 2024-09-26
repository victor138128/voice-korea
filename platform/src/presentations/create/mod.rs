#![allow(non_snake_case)]
use crate::prelude::*;
use dioxus::prelude::*;
use step_four::StepFourPage;
use step_one::StepOnePage;
use step_three::StepThreePage;
use step_two::StepTwoPage;

#[derive(PartialEq, Props, Clone)]
pub struct CreatePageProps {
    lang: Language,
}

#[derive(PartialEq, Props, Clone)]
pub struct StepOneProps {
    lang: Language,
}

pub mod controller;
pub mod i18n;
pub mod step_four;
pub mod step_one;
pub mod step_three;
pub mod step_two;

#[component]
pub fn CreatePage(props: CreatePageProps) -> Element {
    let mut ctrl = controller::Controller::init();
    let translates = i18n::translate(props.lang.clone());
    let num_step = 4;

    rsx! {
        div {
            class: "flex flex-col w-full h-full px-[120px] pt-[55px]",
            div {
                class: "flex flex-row w-full px-[10px] pb-[40px]",
                Link {
                    to: Route::LoginPage {
                        lang: props.lang.clone(),
                    },
                    img {
                        src: "/images/close.png",
                        alt: "Close Button"
                    }
                }
                div {
                    class: "flex flex-row items-center justify-center w-min pl-[40px]",
                    for i in 0..num_step {
                        if i == ctrl.get_step() {
                            div {
                                onclick: move |_| {
                                    ctrl.set_step(i);
                                },
                                img {
                                    src: "/images/current-step.png",
                                    alt: "Current Step"
                                }
                            }
                        } else if i < ctrl.get_step() {
                            div {
                                onclick: move |_| {
                                    ctrl.set_step(i);
                                },
                                img {
                                    src: "/images/prev-step.png",
                                    alt: "Prev Step"
                                }
                            }
                        } else {
                            div {
                                img {
                                    src: "/images/not-current-step.png",
                                    alt: "Next Step"
                                }
                            }
                        }

                        if i != num_step - 1 {
                            if i < ctrl.get_step() {
                                div {
                                    class: "flex items-center justify-center w-[80px] h-[1px] bg-[#2168c3]"
                                }
                            } else {
                                div {
                                    class: "flex items-center justify-center w-[80px] h-[1px] bg-[#e0e0e0]"
                                }
                            }
                        }
                    }
                }
            }
            if ctrl.get_step() == 0 {
                StepOnePage {
                    ctrl: ctrl,
                    lang: props.lang,
                    authorization: translates.authorization,
                    individual: translates.individual,
                    company: translates.company,
                    individual_description: translates.individual_description,
                    phone: translates.phone,
                    phone_description: translates.phone_description,
                    check_title: translates.check_title,
                    check_description_1: translates.check_description_1,
                    check_description_2: translates.check_description_2,
                    check_description_3: translates.check_description_3,
                    company_name: translates.company_name,
                    business_register_number: translates.business_register_number,
                    company_name_example: translates.company_name_example,
                    business_register_number_example: translates.business_register_number_example,
                    next: translates.next,
                }
            } else if ctrl.get_step() == 1 {
                StepTwoPage {
                    ctrl: ctrl,
                    lang: props.lang,
                    agree_terms: translates.agree_terms,
                    agree_membership_terms: translates.agree_membership_terms,
                    agree_privacy_policy: translates.agree_privacy_policy,
                    entrust_personal_information: translates.entrust_personal_information,
                    essential: translates.essential,
                }
            } else if ctrl.get_step() == 2 {
                StepThreePage {
                    ctrl: ctrl,
                    lang: props.lang,
                }
            } else {
                StepFourPage {
                    ctrl: ctrl,
                    lang: props.lang,
                }
            }
        }
    }
}
