#![allow(non_snake_case)]
use crate::{pages::id::select_response::response_type::controller, prelude::*};
use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct SelectAttributeProps {
    ctrl: controller::Controller,
    lang: Language,
    title: String,
    select_type: String,

    temporary_save: String,
    attribute_title: String,
    attribute_description: String,
    attribute_select_label: String,
    nation: String,
    gender: String,
    age: String,
    add_attribute: String,
    cancel: String,
    save: String,
}

#[component]
pub fn SelectAttributePage(props: SelectAttributeProps) -> Element {
    let mut ctrl = props.ctrl.clone();

    rsx! {
        div {
            class: "flex flex-col w-full h-full justify-start items-center",
            div {
                class: "flex flex-col max-w-[1200px] min-w-[600px] w-full justify-end items-end mt-[15px] px-[50px]",
                div {
                    class: "flex flex-row w-[250px] h-[55px] mt-[55px] rounded-[8px] bg-[#2168c3] justify-center items-center text-[21px] font-semibold text-white",
                    {props.temporary_save}
                }
            }
            div {
                class: "flex flex-col max-w-[1200px] min-w-[600px] w-full justify-start items-start mt-[15px] px-[50px]",
                div {
                    class: "flex flex-col w-full rounded-[10px] bg-white mb-[10px] p-[30px]",
                    div {
                        class: "text-[#2168c3] font-semibold text-[30px] mb-[20px]",
                        {props.attribute_title}
                    }
                    div {
                        class: "text-black font-semibold text-[22px] mb-[40px]",
                        {props.attribute_description}
                    }
                    div {
                        class: "flex flex-row w-full justify-start items-center mb-[50px]",
                        div {
                            class: "flex flex-row w-[138px] h-[50px] rounded-lg border-black border-[1px] justify-center items-center text-black font-semibold text-[22px] mr-[30px]",
                            {ctrl.get_response_count()}
                        }
                        div {
                            class: "flex flex-col w-full justify-start items-start",
                            input {
                                class: "flex flex-row w-full justify-start items-start bg-e3e3e3",
                                style: "accent-color: #3a94ff;",
                                "type": "range",
                                min: 0,
                                max: 5000,
                                value: ctrl.get_response_count(),
                                list: "attribute_response_value",
                                onchange: move |e| {
                                    ctrl.set_response_count(e.value());
                                },
                            }
                            datalist {
                                class: "flex flex-row w-full justify-between items-start",
                                id: "attribute_response_value",
                                option {
                                    value: 0,
                                    label: "0"
                                }
                                option {
                                    value: 50,
                                }
                                option {
                                    value: 100,
                                }
                                option {
                                    value: 250,
                                }
                                option {
                                    value: 500,
                                }
                                option {
                                    value: 1000,
                                }
                                option {
                                    value: 3000,
                                }
                                option {
                                    value: 5000,
                                    label: "5,000"
                                }
                            }
                        }
                    }
                    div {
                        class: "flex flex-col w-full justify-start items-start",
                        div {
                            class: "text-black font-semibold text-[22px] mb-[20px]",
                            {props.attribute_select_label}
                        }
                        Attribute {
                            label_image: asset!("public/images/national.png"),
                            label_name: props.nation,
                            label_value: "대한민국(Korea)".to_string()
                        }
                        Attribute {
                            label_image: asset!("public/images/gender.png"),
                            label_name: props.gender,
                            label_value: "남성".to_string()
                        }
                        Attribute {
                            label_image: asset!("public/images/age.png"),
                            label_name: props.age,
                            label_value: "10-99+".to_string()
                        }
                        div {
                            class: "flex flex-row w-full justify-center items-center mb-[40px]",
                            div {
                                class: "flex flex-row w-[200px] h-[50px] rounded-[20px] bg-[#d6d6d6] justify-center items-center mt-[20px]",
                                img {
                                    class: "flex flex-col pr-[10px]",
                                    src: asset!("public/images/add.png"),
                                    alt: "add question",
                                }
                                div {
                                    class: "text-[20px] font-medium text-black",
                                    onclick: move |_| {
                                    },
                                    {props.add_attribute}
                                }
                            }
                        }
                        div {
                            class: "flex flex-row w-full justify-end items-end mb-[30px]",
                            Link {
                                to: Route::SelectResponsePage {
                                    lang: props.lang.clone(),
                                    survey_id: props.title.clone()
                                },
                                div {
                                    class: "flex flex-row justify-center items-center w-[115px] h-[50px] rounded-[10px] bg-[#434343] text-white font-medium text-[20px] mr-[20px]",
                                    {props.cancel}
                                }
                            }
                            div {
                                class: "flex flex-row justify-center items-center w-[115px] h-[50px] rounded-[10px] bg-[#2168c3] text-white font-medium text-[20px] mr-[20px]",
                                onclick: move |_| {
                                    ctrl.change_step(2);
                                },
                                {props.save}
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn Attribute(label_image: Asset, label_name: String, label_value: String) -> Element {
    rsx! {
        div {
            class: "flex flex-row w-full justify-start items-start mb-[10px]",
            div {
                class: "flex flex-row w-full justify-start items-center",
                div {
                    class: "flex flex-row w-[245px] h-[80px] justify-start items-center pl-[20px] rounded-l-lg bg-[#2168c3]",
                    img {
                        class: "pr-[15px]",
                        src: label_image,
                        width: 35,
                        height: 35,
                        alt: label_name.clone()
                    }
                    div {
                        class: "text-white font-normal text-[20px]",
                        {label_name.clone()}
                    }
                }
                div {
                    class: "flex flex-row w-full h-[80px] justify-between items-center bg-[#f9f9f9] rounded-r-lg pl-[30px] pr-[15px]",
                    div {
                        class: "text-black font-normal text-[20px]",
                        {label_value.clone()}
                    }
                    img {
                        src: asset!("public/images/right-arrow.png"),
                        width: 20,
                        height: 20,
                        alt: "right arrow"
                    }
                }
            }
        }
    }
}
