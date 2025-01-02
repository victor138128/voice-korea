#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::{
    components::{
        icons::{ArrowLeft, ArrowRight, Expand, Folder, Plus, RowOption, Search, Switch},
        label::Label,
    },
    prelude::Language,
    routes::Route,
};

pub mod _id;
mod controller;
mod i18n;

#[derive(Props, Clone, PartialEq)]
pub struct MemberPageProps {
    lang: Language,
}

#[component]
pub fn MemberPage(props: MemberPageProps) -> Element {
    let ctrl = controller::Controller::init(props.lang);
    let mut name = use_signal(|| "".to_string());
    let translates = i18n::translate(props.lang.clone());

    let member_summary = ctrl.get_members();
    let groups = ctrl.get_groups();
    let roles = ctrl.get_roles();
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "text-[#9b9b9b] font-medium text-[14px] mb-[10px]",
                "{translates.organization_management} / {translates.team_member_management}"
            }
            div { class: "text-[#3a3a3a] font-semibold text-[28px] mb-[25px]",
                "{translates.team_member_management}"
            }
            div { class: "text-[#35343f] font-normal text-[14px] mb-[40px]",
                "Lörem ipsum plar ses tire. Krosm psykototal nesöng. Rosk ans. Nyr dystopi, antinde är speskapet. Mal neling medan rebel. "
            }
            div { class: "flex flex-row w-full justify-start items-start mb-[10px]",
                MemberCountCard {
                    label_name: translates.total,
                    label_count: member_summary.role_counts.get(0).unwrap_or(&0).clone(),
                }
                MemberCountCard {
                    label_name: translates.manager,
                    label_count: member_summary.role_counts.get(1).unwrap_or(&0).clone(),
                }
                MemberCountCard {
                    label_name: translates.public_opinion_manager,
                    label_count: member_summary.role_counts.get(2).unwrap_or(&0).clone(),
                }
                MemberCountCard {
                    label_name: translates.analyst,
                    label_count: member_summary.role_counts.get(3).unwrap_or(&0).clone(),
                }
                MemberCountCard {
                    label_name: translates.repeater,
                    label_count: member_summary.role_counts.get(4).unwrap_or(&0).clone(),
                }
                MemberCountCard {
                    label_name: translates.lecturer,
                    label_count: member_summary.role_counts.get(5).unwrap_or(&0).clone(),
                }
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
                        div { class: "flex flex-row w-[150px] h-[40px] bg-[#2a60d3] rounded-md px-[14px] py-[8px] gap-[5px]",
                            Folder { width: "24", height: "24" }
                            div { class: "text-white font-bold text-[16px]",
                                "{translates.add_team_member}"
                            }
                        }
                    }
                }
                div { class: "flex flex-col w-full justify-start items-start bg-white border rounded-lg border-[#bfc8d9] mb-[30px]",
                    div { class: "flex flex-row w-full h-[55px] justify-start items-center",
                        div { class: "flex flex-row w-[355px] min-w-[355px] h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                "{translates.name}"
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row w-[310px] min-w-[310px] h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                "{translates.group}"
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row w-[310px] min-w-[310px] h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                "{translates.role}"
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row w-full h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                "{translates.project}"
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "w-[90px] h-full justify-center items-center gap-[10px]" }
                    }
                    for member in member_summary.members {
                        div { class: "flex flex-col w-full justify-start items-start",
                            div { class: "flex flex-row w-full h-[1px] bg-[#bfc8d9]" }
                            Link {
                                class: "flex flex-row w-full",
                                to: Route::MemberDetailPage {
                                    lang: props.lang,
                                    member_id: member.member_id,
                                },
                                div { class: "flex flex-row w-full h-[55px] justify-start items-center text-[#3a3a3a] font-medium text-[14px]",
                                    div { class: "flex flex-row w-[355px] min-w-[355px] h-full justify-center items-center gap-[10px]",
                                        div { class: "w-[36px] h-[36px] rounded-[40px] bg-[#9baae4] mr-[10px]" }
                                        div { class: "flex flex-col justify-start items-start",
                                            div { class: "text-[14px] font-medium text-[#3a3a3a] mb-[5px]",
                                                {member.profile_name}
                                            }
                                            div { class: "text-[14px] font-normal text-[#7c8292]",
                                                {member.email}
                                            }
                                        }
                                    }
                                    div { class: "flex flex-row w-[310px] min-w-[310px] h-full justify-center items-center gap-[10px]",
                                        select {
                                            class: "bg-transparent focus:outline-none",
                                            value: member.group,
                                            //TODO: update member group
                                            onchange: |_evt| {},
                                            for group in groups.clone() {
                                                option { value: group.clone(), "{group}" }
                                            }
                                        }
                                    }
                                    div { class: "flex flex-row w-[310px] min-w-[310px] h-full justify-center items-center gap-[10px]",
                                        select {
                                            class: "bg-transparent focus:outline-none",
                                            value: member.role,
                                            //TODO: update member role
                                            onchange: |_evt| {},
                                            for role in roles.clone() {
                                                option { value: role.clone(), "{role}" }
                                            }
                                        }
                                    }
                                    div { class: "flex flex-row w-full h-full justify-center items-center gap-[10px]",
                                        if member.projects.len() > 0 {
                                            Label {
                                                label_name: member.projects[0].clone(),
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
pub fn MemberCountCard(label_name: String, label_count: u64) -> Element {
    rsx! {
        div { class: "flex flex-col w-[85px] h-[96px] justify-center items-center py-[18px] mr-[10px] bg-white rounded-lg",
            div { class: "font-semibold text-[#35343f] text-[15px] mb-[17px]", "{label_name}" }
            div { class: "font-bold text-[#435393] text-[24px]", "{label_count}" }
        }
    }
}
