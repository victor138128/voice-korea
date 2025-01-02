#![allow(non_snake_case)]
use controller::{GroupMember, GroupProject, ProjectStatus, ProjectType};
use dioxus::prelude::*;

use crate::{
    components::{
        icons::{AddUser, ArrowLeft, ArrowRight, Expand, Plus, RowOption, Search, Switch},
        label::Label,
    },
    prelude::Language,
    routes::Route,
};
pub mod controller;
pub mod i18n;

#[derive(Props, Clone, PartialEq)]
pub struct GroupDetailPageProps {
    lang: Language,
    group_id: String,
}

#[derive(Props, Clone, PartialEq)]
pub struct GroupParticipantTranslate {
    group_team_member: String,
    add_member: String,
    name: String,
    group: String,
    role: String,
    project: String,
}

#[derive(Props, Clone, PartialEq)]
pub struct CommonProjectTranslate {
    common_project: String,
    add_project: String,

    item: String,
    project: String,
    panel: String,
    period: String,
    status: String,
}

#[component]
pub fn GroupDetailPage(props: GroupDetailPageProps) -> Element {
    let ctrl = controller::Controller::init(props.lang, props.group_id);
    let group = ctrl.get_group();
    let total_groups = ctrl.get_groups();
    let total_roles = ctrl.get_roles();

    let group_name = group.group.clone();

    let translates = i18n::translate(props.lang.clone());

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "text-[#9b9b9b] font-medium text-[14px] mb-[10px]",
                "{translates.organization_management} / {translates.group_management} / {translates.see_detail}"
            }
            div { class: "flex flex-row w-full justify-start items-center mb-[25px]",
                Link {
                    class: "mr-[6px]",
                    to: Route::GroupPage {
                        lang: props.lang,
                    },
                    ArrowLeft { width: "24", height: "24", color: "#3a3a3a" }
                }
                div { class: "text-[#3a3a3a] font-semibold text-[28px]", "{group_name}" }
            }
            div { class: "text-[#3a3a3a] font-normal text-[14px] mb-[35px]",
                "{translates.register_date} {group.register_date}"
            }
            div { class: "flex flex-col w-full gap-[40px] mb-[30px]",
                GroupParticipant {
                    members: ctrl.get_group().group_members,
                    total_groups,
                    total_roles,
                    i18n: GroupParticipantTranslate {
                        group_team_member: translates.group_team_member,
                        add_member: translates.add_member,
                        name: translates.name,
                        group: translates.group,
                        role: translates.role,
                        project: translates.project.clone(),
                    },
                }
                GroupCommonProject {
                    projects: ctrl.get_group().group_projects,
                    i18n: CommonProjectTranslate {
                        common_project: translates.common_project,
                        add_project: translates.add_project,
                        item: translates.item,
                        project: translates.project,
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
pub fn GroupCommonProject(projects: Vec<GroupProject>, i18n: CommonProjectTranslate) -> Element {
    let mut name = use_signal(|| "".to_string());
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "font-bold text-[#3a3a3a] text-[16px] mb-[10px]", {i18n.common_project} }
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
                    div { class: "flex flex-row gap-[40px] items-center",
                        div { class: "flex flex-row items-center w-[185px] h-[40px] bg-[#2a60d3] rounded-md px-[14px] py-[8px] gap-[5px]",
                            Plus { width: "18", height: "18", color: "#afc9ff" }
                            div { class: "text-white font-bold text-[16px]", {i18n.add_project} }
                        }
                        div { class: "flex flex-row gap-[10px]",
                            ArrowLeft { width: "25", height: "25", color: "#555462" }
                            ArrowRight { width: "25", height: "25", color: "#555462" }
                        }
                    }
                }
                //테이블 섹션
                div { class: "flex flex-col w-full h-full justify-start items-start bg-white border rounded-lg border-[#bfc8d9]",
                    div { class: "flex flex-row w-full h-[55px] justify-start items-center",
                        div { class: "flex flex-row w-[120px] min-w-[120px] h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                {i18n.item}
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row flex-1 h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                {i18n.project}
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row flex-1 h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                {i18n.panel}
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row flex-1 h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                {i18n.period}
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row w-[120px] min-w-[120px] h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                {i18n.status}
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row w-[90px] min-w-[90px] h-full justify-center items-center gap-[10px]" }
                    }
                    for project in projects {
                        div { class: "flex flex-col w-full justify-start items-start",
                            div { class: "flex flex-row w-full h-[1px] bg-[#bfc8d9]" }
                            div { class: "flex flex-row w-full h-[55px] justify-start items-center text-[#35343f] font-semibold text-[14px]",
                                div { class: "flex flex-row w-[120px] min-w-[120px] h-full justify-center items-center gap-[10px]",
                                    match project.project_type {
                                        ProjectType::Investigation => "조사",
                                        _ => "공론",
                                    }
                                }
                                div { class: "flex flex-row flex-1 h-full justify-center items-center gap-[10px]",
                                    {project.project_subject.clone()}
                                }
                                div { class: "flex flex-row flex-1 h-full justify-center items-center gap-[10px]",
                                    if project.panels.len() > 0 {
                                        Label {
                                            label_name: project.panels[0].clone(),
                                            label_color: "bg-[#35343f]",
                                            is_delete: false,
                                        }
                                    }
                                    Expand { width: "18", height: "18" }
                                }
                                div { class: "flex flex-row flex-1 h-full justify-center items-center gap-[10px]",
                                    {project.periods.clone()}
                                }
                                div { class: "flex flex-row w-[120px] min-w-[120px] h-full justify-center items-center gap-[10px]",
                                    match project.project_status {
                                        ProjectStatus::Ready => "준비",
                                        ProjectStatus::InProgress => "진행",
                                        _ => "마감",
                                    }
                                }
                                div { class: "flex flex-row w-[90px] min-w-[90px] h-full justify-center items-center gap-[10px]",
                                    RowOption { width: 24, height: 24 }
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
pub fn GroupParticipant(
    members: Vec<GroupMember>,
    total_groups: Vec<String>,
    total_roles: Vec<String>,

    i18n: GroupParticipantTranslate,
) -> Element {
    let mut name = use_signal(|| "".to_string());
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "font-bold text-[#3a3a3a] text-[16px] mb-[10px]", {i18n.group_team_member} }
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
                    div { class: "flex flex-row gap-[40px] items-center",
                        div { class: "flex flex-row w-[150px] h-[40px] bg-[#2a60d3] rounded-md px-[14px] py-[8px] gap-[5px]",
                            AddUser { width: "24", height: "24" }
                            div { class: "text-white font-bold text-[16px]", {i18n.add_member} }
                        }
                        div { class: "flex flex-row gap-[10px]",
                            ArrowLeft { width: "25", height: "25", color: "#555462" }
                            ArrowRight { width: "25", height: "25", color: "#555462" }
                        }
                    }
                }
                //테이블 섹션
                div { class: "flex flex-col w-full justify-start items-start bg-white border rounded-lg border-[#bfc8d9]",
                    div { class: "flex flex-row w-full h-[55px] justify-start items-center",
                        div { class: "flex flex-row w-[355px] min-w-[355px] h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                {i18n.name}
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row w-[310px] min-w-[310px] h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                {i18n.group}
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row w-[310px] min-w-[310px] h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                {i18n.role}
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row w-full h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                {i18n.project}
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row w-[90px] min-w-[90px] h-full justify-center items-center gap-[10px]" }
                    }
                    for member in members {
                        div { class: "flex flex-col w-full justify-start items-start",
                            div { class: "flex flex-row w-full h-[1px] bg-[#bfc8d9]" }
                            div { class: "flex flex-row w-full h-[55px] justify-start items-center text-[#3a3a3a] font-medium text-[14px]",
                                div { class: "flex flex-row w-[355px] min-w-[355px] h-full justify-center items-center gap-[10px]",
                                    div { class: "w-[36px] h-[36px] rounded-[40px] bg-[#9baae4] mr-[10px]" }
                                    div { class: "flex flex-col justify-start items-start",
                                        div { class: "text-[14px] font-medium text-[#3a3a3a] mb-[5px]",
                                            {member.profile_name.unwrap_or_default()}
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
                                        for group in total_groups.clone() {
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
                                        for role in total_roles.clone() {
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
        }
    }
}
