#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::{
    components::icons::{
        ArrowLeft, ArrowRight, ColOption, Expand, Folder, Plus, Remove, RowOption, Search, Switch,
    },
    prelude::Language,
    routes::Route,
};

pub mod _id;
mod controller;
mod i18n;

#[derive(Props, Clone, PartialEq)]
pub struct GroupPageProps {
    lang: Language,
}

#[component]
pub fn GroupPage(props: GroupPageProps) -> Element {
    let ctrl = controller::Controller::init(props.lang);
    let mut name = use_signal(|| "".to_string());
    let translates = i18n::translate(props.lang.clone());

    let groups = ctrl.get_groups();

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "text-[#9b9b9b] font-medium text-[14px] mb-[10px]",
                "{translates.organization_management} / {translates.group_management}"
            }
            div { class: "text-[#3a3a3a] font-semibold text-[28px] mb-[25px]",
                "{translates.group_management}"
            }
            div { class: "text-[#35343f] font-normal text-[14px] mb-[40px]",
                "Lörem ipsum plar ses tire. Krosm psykototal nesöng. Rosk ans. Nyr dystopi, antinde är speskapet. Mal neling medan rebel. "
            }

            div {
                class: "flex flex-col w-full justify-start items-start bg-white rounded-lg shadow-lg p-[20px]",
                style: "box-shadow: 0 4px 6px rgba(53, 70, 177, 0.05);",
                div { class: "flex flex-row w-full justify-between items-center pb-[20px]",
                    div { class: "flex flex-row w-[590px] h-[45px] justify-between items-center rounded-lg bg-[#f7f7f7] border border-[#7c8292] px-[11px] py-[13px]",
                        input {
                            class: "flex flex-row w-full h-full bg-transparent focus:outline-none",
                            r#type: "text",
                            placeholder: "Enter public name or email address".to_string(),
                            value: (name)(),
                            oninput: move |event| {
                                name.set(event.value());
                            },
                        }
                        Search { width: "18", height: "18", color: "#7c8292" }
                    }
                    div { class: "flex flex-row gap-[10px]",
                        div { class: "flex flex-row w-[140px] h-[40px] bg-[#2a60d3] rounded-md px-[14px] py-[8px] gap-[5px]",
                            Folder { width: "24", height: "24" }
                            div { class: "text-white font-bold text-[16px]",
                                "{translates.create_group}"
                            }
                        }
                        ColOption { width: "40", height: "40" }
                    }
                }
                //테이블 섹션
                div { class: "flex flex-col w-full justify-start items-start bg-white border rounded-lg border-[#bfc8d9] mb-[30px]",
                    div { class: "flex flex-row w-full h-[55px] justify-start items-center",
                        div { class: "flex flex-row w-[310px] min-w-[310px] h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                "{translates.group}"
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row w-[120px] min-w-[120px] h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                "{translates.personnel}"
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row w-full h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                "{translates.team_member}"
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "w-[90px] h-full justify-center items-center gap-[10px]" }
                    }
                    for group in groups {
                        div { class: "flex flex-col w-full justify-start items-start",
                            div { class: "flex flex-row w-full h-[1px] bg-[#bfc8d9]" }
                            Link {
                                class: "flex flex-row w-full",
                                to: Route::GroupDetailPage {
                                    lang: props.lang,
                                    group_id: group.group_id,
                                },
                                div { class: "flex flex-row w-full h-[55px] justify-start items-center text-[#3a3a3a] font-medium text-[14px]",
                                    div { class: "flex flex-row w-[310px] min-w-[310px] h-full justify-center items-center",
                                        "{group.group_name}"
                                    }
                                    div { class: "flex flex-row w-[120px] min-w-[120px] h-full justify-center items-center",
                                        "{group.member_count}"
                                    }
                                    div { class: "flex flex-row w-full h-full justify-center items-center",
                                        if group.member_list.len() > 0 {
                                            Label {
                                                label_name: group.member_list[0].clone(),
                                                label_color: "bg-[#35343f]",
                                            }
                                        }
                                        div { class: "flex flex-row w-[24px] h-[24px] justify-center items-center bg-[#d1d1d1] ml-[5px] opacity-50 rounded-lg",
                                            Plus { width: "10", height: "10" }
                                        }
                                        div { class: "pl-[20px]",
                                            Expand { width: "18", height: "18" }
                                        }
                                    }
                                    div { class: "flex flex-row w-[90px] h-full justify-center items-center",
                                        RowOption { width: 24, height: 24 }
                                    }
                                }
                            }
                        }
                    }
                }
                //페이지네이션
                div { class: "flex flex-row w-full justify-center items-center",
                    div { class: "mr-[20px]",
                        ArrowLeft { width: "24", height: "24" }
                    }
                    //FIXME: add pagination by variable(page, index)
                    for i in 0..10 {
                        if i == 0 {
                            div { class: "flex flex-row w-[40px] h-[40px] justify-center items-center bg-[#7c8292] rounded-lg text-white font-bold text-[15px] mr-[8px]",
                                "{i + 1}"
                            }
                        } else {
                            div { class: "flex flex-row w-[40px] h-[40px] justify-center items-center bg-white border border-[#dfdfdf] rounded-lg text-[#0d1732] font-bold text-[15px] mr-[8px]",
                                "{i + 1}"
                            }
                        }
                    }
                    div { class: "ml-[12px]",
                        ArrowRight { width: "24", height: "24" }
                    }
                }
            }
        }
    }
}

#[component]
pub fn Label(label_name: String, label_color: String) -> Element {
    rsx! {
        div {
            class: format!(
                "flex flex-row w-[100px] h-[25px] justify-between items-center {} rounded-[20px] px-[5px] py-[3px]",
                label_color,
            ),
            div { class: "text-white font-semibold text-[14px]", {label_name} }
            Remove { width: "18", height: "18" }
        }
    }
}
