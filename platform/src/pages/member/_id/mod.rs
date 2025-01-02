#![allow(non_snake_case)]
use controller::{ProjectHistory, ProjectStatus, ProjectType};
use dioxus::prelude::*;

use crate::{
    components::icons::{ArrowLeft, ArrowRight, Expand, RowOption, Switch},
    prelude::Language,
    routes::Route,
};

mod controller;
mod i18n;

#[derive(Props, Clone, PartialEq)]
pub struct MemberDetailPageProps {
    lang: Language,
    member_id: String,
}

#[derive(Props, Clone, PartialEq)]
pub struct ProfileInfoTranslate {
    privacy: String,
    name: String,
    group: String,
    role: String,
    email: String,
    save: String,
    remove_team_member: String,
}

#[derive(Props, Clone, PartialEq)]
pub struct ProfileHistoryTranslate {
    participation_record: String,
    item: String,
    project: String,
    role: String,
    panel: String,
    period: String,
    status: String,
}

#[component]
pub fn MemberDetailPage(props: MemberDetailPageProps) -> Element {
    let ctrl = controller::Controller::init(props.lang, props.member_id);
    let translates = i18n::translate(props.lang.clone());

    let member = ctrl.get_member();
    let groups = ctrl.get_groups();
    let roles = ctrl.get_roles();

    let profile_name = member.profile_name.unwrap_or_default();
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "text-[#9b9b9b] font-medium text-[14px] mb-[10px]",
                "{translates.organization_management} / {translates.team_member_management} / {translates.see_detail}"
            }
            div { class: "flex flex-row w-full justify-start items-center mb-[25px]",
                Link {
                    class: "mr-[6px]",
                    to: Route::MemberPage {
                        lang: props.lang,
                    },
                    ArrowLeft { width: "24", height: "24", color: "#3a3a3a" }
                }
                div { class: "text-[#3a3a3a] font-semibold text-[28px]", "{profile_name}" }
            }
            div { class: "text-[#3a3a3a] font-normal text-[14px] mb-[35px]",
                "{translates.register_date} {member.register_date}"
            }
            div { class: "flex flex-row w-full h-full justify-start items-start",
                div { class: "mr-[15px]",
                    ProfileInfo {
                        profile_image: member.profile_image,
                        profile_name,
                        group: member.group,
                        role: member.role,
                        email: member.email,

                        total_groups: groups,
                        total_roles: roles,

                        i18n: ProfileInfoTranslate {
                            privacy: translates.privacy,
                            name: translates.name,
                            group: translates.group,
                            role: translates.role.clone(),
                            email: translates.email,
                            save: translates.save,
                            remove_team_member: translates.remove_team_member,
                        },
                    }
                }
                ProfileHistory {
                    histories: member.project_history,
                    i18n: ProfileHistoryTranslate {
                        participation_record: translates.participation_record,
                        item: translates.item,
                        project: translates.project,
                        role: translates.role.clone(),
                        panel: translates.panel,
                        period: translates.period,
                        status: translates.status,
                    },
                }
            }
        }
    }
}

#[component]
pub fn ProfileHistory(histories: Vec<ProjectHistory>, i18n: ProfileHistoryTranslate) -> Element {
    rsx! {
        div { class: "flex flex-col w-[1166px] justify-start items-start",
            div { class: "font-bold text-[#3a3a3a] text-[16px] mb-[10px]",
                "{i18n.participation_record}"
            }
            div {
                class: "flex flex-col w-full justify-start items-start bg-white rounded-lg shadow-lg p-[20px]",
                style: "box-shadow: 0 4px 6px rgba(53, 70, 177, 0.05);",
                div { class: "flex flex-col w-full justify-start items-start bg-white border rounded-lg border-[#bfc8d9] mb-[30px]",
                    div { class: "flex flex-row w-full h-[55px] justify-start items-center",
                        div { class: "flex flex-row w-[120px] min-w-[120px] h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                "{i18n.item}"
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row w-[200px] min-w-[200px] h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                "{i18n.project}"
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row w-[200px] min-w-[200px] h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                "{i18n.role}"
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row w-[200px] min-w-[200px] h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                "{i18n.panel}"
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row w-[200px] min-w-[200px] h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                "{i18n.period}"
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row w-[120px] min-w-[120px] h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                "{i18n.status}"
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row w-full min-w-[120px] h-full justify-center items-center gap-[10px]" }
                    }
                    for history in histories {
                        div { class: "flex flex-col w-full justify-start items-start",
                            div { class: "flex flex-row w-full h-[1px] bg-[#bfc8d9]" }
                            div { class: "flex flex-row w-full h-[55px] justify-start items-center text-[#35343f] font-semibold text-[14px]",
                                div { class: "flex flex-row w-[120px] min-w-[120px] h-full justify-center items-center gap-[10px]",
                                    match history.project_type {
                                        ProjectType::Investigation => "조사",
                                        _ => "공론",
                                    }
                                }
                                div { class: "flex flex-row w-[200px] min-w-[200px] h-full justify-center items-center gap-[10px]",
                                    {history.project_subject.clone()}
                                }
                                div { class: "flex flex-row w-[200px] min-w-[200px] h-full justify-center items-center gap-[10px]",
                                    {history.role.clone()}
                                }
                                div { class: "flex flex-row w-[200px] min-w-[200px] h-full justify-center items-center gap-[20px]",
                                    if history.panel.len() > 0 {
                                        Label {
                                            label_name: history.panel[0].clone(),
                                            label_color: "bg-[#35343f]",
                                        }
                                    }
                                    Expand { width: "18", height: "18" }
                                }
                                div { class: "flex flex-row w-[200px] min-w-[200px] h-full justify-center items-center gap-[10px]",
                                    {history.period.clone()}
                                }
                                div { class: "flex flex-row w-[120px] min-w-[120px] h-full justify-center items-center gap-[10px]",
                                    match history.project_status {
                                        ProjectStatus::Ready => "준비",
                                        ProjectStatus::InProgress => "진행",
                                        _ => "마감",
                                    }
                                }
                                div { class: "flex flex-row w-[120px] min-w-[120px] h-full justify-center items-center gap-[10px]",
                                    RowOption { width: 24, height: 24 }
                                }
                            }
                        }
                    }
                }
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
                "flex flex-row w-[50px] h-[25px] justify-center items-center {} rounded-[20px]",
                label_color,
            ),
            div { class: "text-white font-semibold text-[14px]", {label_name} }
        }
    }
}

#[component]
pub fn ProfileInfo(
    profile_image: Option<String>,
    profile_name: Option<String>,
    group: String,
    role: String,
    email: String,

    total_groups: Vec<String>,
    total_roles: Vec<String>,

    i18n: ProfileInfoTranslate,
) -> Element {
    let mut name = use_signal(|| profile_name.unwrap_or_default());
    let mut email = use_signal(|| email.clone());
    let mut select_group = use_signal(|| group.clone());
    let mut select_role = use_signal(|| role.clone());

    rsx! {
        div { class: "flex flex-col w-[370px] justify-start items-start",
            div { class: "font-bold text-[#3a3a3a] text-[16px] mb-[10px]", "{i18n.privacy}" }
            div {
                class: "flex flex-col w-full justify-start items-start bg-white rounded-lg shadow-lg px-[20px] py-[32px]",
                style: "box-shadow: 0 4px 6px rgba(53, 70, 177, 0.05);",
                div { class: "flex flex-row w-[68px] h-[68px] justify-center items-center bg-[#9baae4] rounded-[40px] text-white font-bold text-[28px] mb-[30px]",
                    "VK"
                }
                div { class: "flex flex-col w-full justify-start items-start font-normal text-[#7c8292] text-[14px]",
                    div { class: "flex flex-col w-full justify-start items-start mb-[20px]",
                        div { class: "mb-[8px]", "{i18n.name}" }
                        input {
                            class: "flex flex-row w-[214px] h-[40px] bg-[#f7f7f7] rounded-lg focus:outline-none px-[16px] py-[8px]",
                            r#type: "text",
                            placeholder: "Enter public name or email address".to_string(),
                            value: (name)(),
                            oninput: move |event| {
                                name.set(event.value());
                            },
                        }
                    }
                    div { class: "flex flex-col w-full justify-start items-start mb-[20px]",
                        div { class: "mb-[8px]", "{i18n.group}" }
                        select {
                            class: "flex flex-row w-[214px] h-[40px] bg-[#f7f7f7] rounded-lg focus:outline-none px-[16px] py-[8px]",
                            value: select_group,
                            onchange: move |evt| {
                                select_group.set(evt.value());
                            },
                            for group in total_groups {
                                option { value: group.clone(), "{group}" }
                            }
                        }
                    }
                    div { class: "flex flex-col w-full justify-start items-start mb-[20px]",
                        div { class: "mb-[8px]", "{i18n.role}" }
                        select {
                            class: "flex flex-row w-[214px] h-[40px] bg-[#f7f7f7] rounded-lg focus:outline-none px-[16px] py-[8px] mr-[8px]",
                            value: select_role,
                            onchange: move |evt| {
                                select_role.set(evt.value());
                            },
                            for role in total_roles {
                                option { value: role.clone(), "{role}" }
                            }
                        }
                    }
                    div { class: "flex flex-col w-full justify-start items-start mb-[20px]",
                        div { class: "mb-[8px]", "{i18n.email}" }
                        input {
                            class: "flex flex-row w-[214px] h-[40px] bg-[#f7f7f7] rounded-lg focus:outline-none px-[16px] py-[8px]",
                            r#type: "text",
                            placeholder: "Enter public name or email address".to_string(),
                            value: (email)(),
                            oninput: move |event| {
                                email.set(event.value());
                            },
                        }
                    }
                    div { class: "flex flex-row w-full justify-between items-end mt-[10px]",
                        div { class: "flex flex-row w-[85px] h-[40px] justify-center items-center bg-[#2a60d3] font-bold text-[16px] text-white rounded-md",
                            "{i18n.save}"
                        }
                        div { class: "font-bold text-[16px] text-[#3a3a3a] underline",
                            "{i18n.remove_team_member}"
                        }
                    }
                }
            }
        }
    }
}
