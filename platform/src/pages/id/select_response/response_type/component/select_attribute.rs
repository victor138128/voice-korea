#![allow(non_snake_case)]
use crate::{
    components::{
        alert::AlertModal,
        icons::{Add, Close, Search},
    },
    pages::{
        id::select_response::response_type::controller,
        id::select_response::response_type::controller::SelectAttribute,
    },
    prelude::*,
};
use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct SelectAttributeProps {
    lang: Language,
    survey_id: String,
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

    search_result: String,
    selected_attribute: String,
    search_hint: String,
}

#[derive(PartialEq, Props, Clone)]
pub struct ModalTranslates {
    search_hint: String,
    search_result: String,
    add_attribute: String,
    selected_attribute: String,
    cancel: String,
    save: String,
}

#[component]
pub fn SelectAttributePage(props: SelectAttributeProps) -> Element {
    let mut ctrl = controller::use_controller();

    rsx! {
        div {
            class: "flex flex-col",
            if ctrl.get_clicked_add_attribute() {
                AttributeAddModal {
                    translates: ModalTranslates {
                        search_hint: props.search_hint.clone(),
                        search_result: props.search_result.clone(),
                        add_attribute: props.add_attribute.clone(),
                        selected_attribute: props.selected_attribute.clone(),
                        cancel: props.cancel.clone(),
                        save: props.save.clone()
                    },
                    clicked_cancel_button: move |_e: MouseEvent| {
                        ctrl.clicked_cancel_button();
                    },
                    clicked_add_button: move |_e: MouseEvent| {
                        ctrl.clicked_add_button();
                    },
                    clicked_save_button: move |_e: MouseEvent| {
                        ctrl.clicked_save_button();
                    },
                    change_attribute_selected: move |(id, selected): (usize, bool)| {
                        ctrl.change_attribute_selected(id, selected);
                    },
                    total_attributes: ctrl.get_total_attributes(),
                    selected_attributes: (ctrl.selected_attributes)()
                }
            }
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
                            for attribute in ctrl.get_search_attributes() {
                                Attribute {
                                    label_image: if attribute.name == "국가" {
                                        asset!("/public/images/national.png")
                                    } else if attribute.name == "성별" {
                                        asset!("/public/images/gender.png")
                                    } else {
                                        asset!("/public/images/age.png")
                                    },
                                    label_name: attribute.name.clone(),
                                    label_value: attribute.initial_value.clone(),
                                }
                            }
                            div {
                                class: "flex flex-row w-full justify-center items-center mb-[40px]",
                                div {
                                    class: "flex flex-row w-[200px] h-[50px] rounded-[20px] bg-[#d6d6d6] justify-center items-center mt-[20px]",
                                    img {
                                        class: "flex flex-col pr-[10px]",
                                        src: asset!("/public/images/add.png"),
                                        alt: "add question",
                                    }
                                    div {
                                        class: "text-[20px] font-medium text-black",
                                        onclick: move |_| {
                                            ctrl.change_clicked_add_attribute(true);
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
                                        survey_id: props.survey_id.clone()
                                    },
                                    div {
                                        class: "flex flex-row justify-center items-center w-[115px] h-[50px] rounded-[10px] bg-[#434343] text-white font-medium text-[20px] mr-[20px]",
                                        {props.cancel.clone()}
                                    }
                                }
                                div {
                                    class: "flex flex-row justify-center items-center w-[115px] h-[50px] rounded-[10px] bg-[#2168c3] text-white font-medium text-[20px] mr-[20px]",
                                    onclick: move |_| {
                                        ctrl.change_step(controller::Step::Panel);
                                    },
                                    {props.save.clone()}
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn AttributeAddModal(
    translates: ModalTranslates,
    clicked_cancel_button: EventHandler<MouseEvent>,
    clicked_add_button: EventHandler<MouseEvent>,
    clicked_save_button: EventHandler<MouseEvent>,
    change_attribute_selected: EventHandler<(usize, bool)>,
    total_attributes: Vec<SelectAttribute>,
    selected_attributes: Vec<SelectAttribute>,
) -> Element {
    rsx! {
        div {
            class: "fixed flex flex-row w-screen h-screen backdrop-blur-sm justify-center items-center z-50",
            AlertModal {
                children: rsx! {
                    div {
                        class: "flex flex-col w-full h-min justify-start items-start",
                        div {
                            class: "flex flex-row w-full justify-end items-center mb-[30px]",
                            onclick: move |e| {
                                clicked_cancel_button.call(e);
                            },
                            Close {
                                width: "23",
                                height: "23",
                                color: "#f7f7f7",
                                border_color: "#aeaeae",
                                button_color: "#000000"
                            }
                        }
                        div {
                            class: "flex flex-row w-full h-[55px] justify-start items-center bg-white border-[1px] border-[#9f9f9f] rounded-xl px-[25px] mb-[40px]",
                            div {
                                class: "mr-[5px]",
                                Search {
                                    width: "21",
                                    height: "21",
                                    color: "#8a8a8a"
                                }
                            }
                            div {
                                class: "text-[#9f9f9f] font-normal text-[18px]",
                                {translates.search_hint}
                            }
                        }
                        div {
                            class: "flex flex-col w-full justify-start items-start",
                            div {
                                class: "text-black font-semibold text-[22px] mb-[18px]",
                                {translates.search_result.clone()}
                            }
                            div {
                                class: "flex flex-row w-full justify-start items-start",
                                for attribute in total_attributes {
                                    if attribute.is_selected {
                                        div {
                                            class: "flex flex-row w-[105px] h-[50px] justify-center items-center rounded-3xl bg-[#2168c3] mr-[5px]",
                                            onclick: move |_| {
                                                change_attribute_selected.call((attribute.id, false));
                                            },
                                            div {
                                                class: "mr-[5px]",
                                                Add {
                                                    width: "21",
                                                    height: "21",
                                                    inner_color: "#2168c3",
                                                    color: "#ffffff"
                                                }
                                            }
                                            div {
                                                class: "text-[20px] font-normal text-white",
                                                {attribute.name}
                                            }
                                        }
                                    } else {
                                        div {
                                            class: "flex flex-row w-[105px] h-[50px] justify-center items-center rounded-3xl bg-white border-[1px] border-[#9f9f9f] mr-[5px]",
                                            onclick: move |_| {
                                                change_attribute_selected.call((attribute.id, true));
                                            },
                                            div {
                                                class: "mr-[5px]",
                                                Add {
                                                    width: "21",
                                                    height: "21",
                                                    inner_color: "#ffffff",
                                                    color: "#000000"
                                                }
                                            }
                                            div {
                                                class: "text-[20px] font-normal text-black",
                                                {attribute.name}
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        div {
                            class: "flex flex-row w-full h-[50px] justify-center items-center rounded-lg bg-[#d6d6d6] border-[1px] border-[#c8c8c8] mt-[60px] mb-[70px]",
                            onclick: move |e| {
                                clicked_add_button.call(e);
                            },
                            div {
                                class: "font-medium text-[20px] text-black",
                                {translates.add_attribute.clone()}
                            }
                        }
                        div {
                            style: "width: 100%; height: 1px; background-color: #d3d3d3",
                        }
                        div {
                            class: "flex flex-col w-full justify-start items-start pt-[25px]",
                            div {
                                class: "text-black font-semibold text-[22px] mb-[18px]",
                                {translates.selected_attribute.clone()}
                            }
                            div {
                                class: "flex flex-row w-full justify-start items-start",
                                for attribute in selected_attributes {
                                    div {
                                        class: "flex flex-row w-[80px] h-[50px] justify-center items-center rounded-3xl bg-white border-[1px] border-[#9f9f9f] mr-[5px]",
                                        div {
                                            class: "text-[20px] font-normal text-black",
                                            {attribute.name}
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
                width: Some(460),
                max_width: Some(460),
                tail: rsx! {
                    div {
                        class: "flex flex-row w-full h-min justify-end items-end",
                        div {
                            class: "flex flex-row w-[85px] h-[45px] justify-center items-center rounded-[10px] bg-[#424242] text-white font-normal text-[20px] mr-[10px]",
                            onclick: move |e| {
                                clicked_cancel_button.call(e);
                            },
                            {translates.cancel}
                        }
                        div {
                            class: "flex flex-row w-[85px] h-[45px] justify-center items-center rounded-[10px] bg-[#2168c3] text-white font-normal text-[20px]",
                            onclick: move |e| {
                                clicked_save_button.call(e);
                            },
                            {translates.save}
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
                        src: asset!("/public/images/right-arrow.png"),
                        width: 20,
                        height: 20,
                        alt: "right arrow"
                    }
                }
            }
        }
    }
}
