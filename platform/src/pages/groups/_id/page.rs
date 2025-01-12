#![allow(non_snake_case)]
use super::controller::{Controller, GroupProject, ProjectStatus, ProjectType};
use super::i18n::GroupDetailTranslate;
use dioxus::prelude::*;
use dioxus_translate::translate;
use dioxus_translate::Language;
use models::prelude::GroupMemberResponse;

use crate::components::icons::Plus;
use crate::{
    components::{
        icons::{AddUser, ArrowLeft, ArrowRight, ColOption, Expand, RowOption, Search, Switch},
        label::Label,
    },
    routes::Route,
    service::popup_service::PopupService,
};

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
    no_group: String,
    no_role: String,
    remove_team_member: String,
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

    investigation: String,
    public_opinion: String,
    ready: String,
    in_progress: String,
    finish: String,
    exclude_from_project: String,
}

#[derive(Props, Clone, PartialEq)]
pub struct RemoveProjectTranslate {
    remove_project_modal_title: String,
    remove_project_modal_info: String,
    remove: String,
    cancel: String,
}

#[derive(Props, Clone, PartialEq)]
pub struct RemoveMemberTranslate {
    remove_member_modal_title: String,
    remove_member_modal_info: String,
    remove: String,
    cancel: String,
}

#[derive(Props, Clone, PartialEq)]
pub struct UpdateGroupNameTranslate {
    update_group_name_modal_info: String,
    group_name: String,
    update_group_name_hint: String,
    update_group_name_warning: String,
    update: String,
    cancel: String,
}

#[derive(Props, Clone, PartialEq)]
pub struct RemoveGroupTranslate {
    remove_group_modal_title: String,
    remove_group_modal_info: String,
    remove: String,
    cancel: String,
}

#[derive(Props, Clone, PartialEq)]
pub struct AddMemberTranslate {
    necessary: String,
    input_email_address: String,
    input_email_address_hint: String,
    input_email_address_info: String,
    privacy: String,
    required_input: String,
    select_role: String,
    invite_project: String,
    invite: String,
    cancel: String,

    name: String,
    role: String,
    public_opinion: String,
    investigation: String,
}

#[derive(Clone, PartialEq)]
pub enum ModalType {
    None,
    UpdateGroupName,
    RemoveGroup,
    RemoveMember,
    RemoveProject,
    AddMember,
}

#[component]
pub fn GroupDetailPage(props: GroupDetailPageProps) -> Element {
    let mut ctrl = Controller::init(props.lang, props.group_id.clone());
    let group = ctrl.get_group();
    let total_groups = ctrl.get_groups();
    let total_roles = ctrl.get_roles();
    let group_id_copy1 = props.group_id.clone();
    let group_id_copy2 = props.group_id.clone();

    let group_name = group.group.clone();

    let translates: GroupDetailTranslate = translate(&props.lang);
    let mut modal_type = use_signal(|| ModalType::None);
    let mut popup: PopupService = use_context();

    let navigator = use_navigator();

    if ModalType::UpdateGroupName == modal_type() {
        popup
            .open(rsx! {
                UpdateGroupNameModal {
                    update_group_name: move |name: String| {
                        let group_id = group_id_copy1.clone();
                        async move {
                            ctrl.update_group_name(group_id, name).await;
                            modal_type.set(ModalType::None);
                        }
                    },
                    initialize_group_name: group_name.clone(),
                    onclose: move |_e: MouseEvent| {
                        modal_type.set(ModalType::None);
                    },
                    i18n: UpdateGroupNameTranslate {
                        update_group_name_modal_info: translates
                            .update_group_name_modal_info
                            .to_string(),
                        group_name: translates.group_name.to_string(),
                        update_group_name_hint: translates.update_group_name_hint.to_string(),
                        update_group_name_warning: translates.update_group_name_warning.to_string(),
                        update: translates.update.to_string(),
                        cancel: translates.cancel.to_string(),
                    },
                }
            })
            .with_id("update_group_name")
            .with_title(translates.update_group_name);
    } else if ModalType::RemoveGroup == modal_type() {
        popup
            .open(rsx! {
                RemoveGroupModal {
                    remove_group: move |_e: MouseEvent| {
                        let group_id = group_id_copy2.clone();
                        async move {
                            ctrl.remove_group(group_id).await;
                            modal_type.set(ModalType::None);
                            navigator
                                .push(Route::GroupPage {
                                    lang: props.lang,
                                });
                        }
                    },
                    onclose: move |_e: MouseEvent| {
                        modal_type.set(ModalType::None);
                    },
                    i18n: RemoveGroupTranslate {
                        remove_group_modal_title: translates.remove_group_modal_title.to_string(),
                        remove_group_modal_info: translates.remove_group_modal_info.to_string(),
                        remove: translates.remove.to_string(),
                        cancel: translates.cancel.to_string(),
                    },
                }
            })
            .with_id("remove_group")
            .with_title(translates.remove_group);
    } else if ModalType::RemoveMember == modal_type() {
        popup
            .open(rsx! {
                RemoveMemberModal {
                    onclose: move |_e: MouseEvent| {
                        modal_type.set(ModalType::None);
                    },
                    i18n: RemoveMemberTranslate {
                        remove_member_modal_title: translates.remove_member_modal_title.to_string(),
                        remove_member_modal_info: translates.remove_member_modal_info.to_string(),
                        remove: translates.remove.to_string(),
                        cancel: translates.cancel.to_string(),
                    },
                }
            })
            .with_id("remove_team_member")
            .with_title(translates.remove_team_member);
    } else if ModalType::AddMember == modal_type() {
        popup
            .open(rsx! {
                AddMemberModal {
                    onclose: move |_e: MouseEvent| {
                        modal_type.set(ModalType::None);
                    },
                    roles: total_roles.clone(),
                    i18n: AddMemberTranslate {
                        necessary: translates.necessary.to_string(),
                        input_email_address: translates.input_email_address.to_string(),
                        input_email_address_hint: translates.input_email_address_hint.to_string(),
                        input_email_address_info: translates.input_email_address_info.to_string(),
                        privacy: translates.privacy.to_string(),
                        required_input: translates.required_input.to_string(),
                        select_role: translates.select_role.to_string(),
                        invite_project: translates.invite_project.to_string(),
                        invite: translates.invite.to_string(),
                        cancel: translates.cancel.to_string(),
                        name: translates.name.to_string(),
                        role: translates.role.to_string(),
                        public_opinion: translates.public_opinion.to_string(),
                        investigation: translates.investigation.to_string(),
                    },
                }
            })
            .with_id("add_team_member")
            .with_title(translates.add_team_member);
    } else if ModalType::RemoveProject == modal_type() {
        popup
            .open(rsx! {
                RemoveProjectModal {
                    onclose: move |_e: MouseEvent| {
                        modal_type.set(ModalType::None);
                    },
                    i18n: RemoveProjectTranslate {
                        remove_project_modal_title: translates.remove_project_modal_title.to_string(),
                        remove_project_modal_info: translates.remove_project_modal_info.to_string(),
                        remove: translates.remove.to_string(),
                        cancel: translates.cancel.to_string(),
                    },
                }
            })
            .with_id("remove_project")
            .with_title(translates.remove_project);
    } else {
        popup.close();
    }

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
                div { class: "text-[#3a3a3a] font-semibold text-[28px] mr-[20px]", "{group_name}" }
                div { class: "group relative",
                    button { onclick: move |_| {},
                        div { class: "bg-transparent",
                            ColOption { width: "40", height: "40" }
                        }
                    }
                    nav {
                        tabindex: "0",
                        class: "border-2 bg-white invisible border-none shadow-lg rounded w-60 absolute left-0 top-full transition-all opacity-0 group-focus-within:visible group-focus-within:opacity-100 group-focus-within:translate-y-1 group-focus-within:z-20",
                        ul { class: "py-1",
                            li {
                                class: "p-3 text-sm text-gray-700 hover:bg-gray-100 cursor-pointer",
                                onclick: move |_| {
                                    modal_type.set(ModalType::RemoveGroup);
                                },
                                {translates.remove_group}
                            }
                            li {
                                class: "p-3 text-sm text-gray-700 hover:bg-gray-100 cursor-pointer",
                                onclick: move |_| {
                                    modal_type.set(ModalType::UpdateGroupName);
                                },
                                {translates.update_group_name}
                            }
                        }
                    }
                }
            }
            div { class: "text-[#3a3a3a] font-normal text-[14px] mb-[35px]",
                "{translates.register_date} {group.register_date}"
            }
            div { class: "flex flex-col w-full gap-[40px] mb-[30px]",
                GroupParticipant {
                    members: ctrl.get_group().group_members,
                    total_groups,
                    total_roles,
                    group_name,
                    i18n: GroupParticipantTranslate {
                        group_team_member: translates.group_team_member.to_string(),
                        add_member: translates.add_member.to_string(),
                        name: translates.name.to_string(),
                        group: translates.group.to_string(),
                        role: translates.role.to_string(),
                        project: translates.project.to_string(),
                        no_group: translates.no_group.to_string(),
                        no_role: translates.no_role.to_string(),
                        remove_team_member: translates.remove_team_member_li.to_string(),
                    },
                    change_popup_state: move |modal: String| {
                        if modal == "add_member" {
                            modal_type.set(ModalType::AddMember);
                        } else {
                            modal_type.set(ModalType::RemoveMember);
                        }
                    },
                }
                GroupCommonProject {
                    projects: ctrl.get_group().group_projects,
                    i18n: CommonProjectTranslate {
                        common_project: translates.common_project.to_string(),
                        add_project: translates.add_project.to_string(),
                        item: translates.item.to_string(),
                        project: translates.project.to_string(),
                        panel: translates.panel.to_string(),
                        period: translates.period.to_string(),
                        status: translates.status.to_string(),
                        investigation: translates.investigation.to_string(),
                        public_opinion: translates.public_opinion.to_string(),
                        ready: translates.ready.to_string(),
                        in_progress: translates.in_progress.to_string(),
                        finish: translates.finish.to_string(),
                        exclude_from_project: translates.exclude_from_project.to_string(),
                    },
                    change_popup_state: move |_e: MouseEvent| {
                        modal_type.set(ModalType::RemoveProject);
                    },
                }
            }
        }
    }
}

#[component]
pub fn GroupCommonProject(
    projects: Vec<GroupProject>,
    i18n: CommonProjectTranslate,
    change_popup_state: EventHandler<MouseEvent>,
) -> Element {
    let mut name = use_signal(|| "".to_string());
    let mut is_focused = use_signal(|| false);
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "font-bold text-[#3a3a3a] text-[16px] mb-[10px]", {i18n.common_project} }
            div {
                class: "flex flex-col w-full justify-start items-start bg-white rounded-lg shadow-lg p-[20px]",
                style: "box-shadow: 0 4px 6px rgba(53, 70, 177, 0.05);",
                div { class: "flex flex-row w-full justify-between items-center pb-[20px]",
                    div {
                        class: format!(
                            "flex flex-row w-[590px] h-[45px] justify-between items-center rounded-lg  {} px-[11px] py-[13px]",
                            if (is_focused)() {
                                "bg-[#ffffff] border border-[#2a60d3]"
                            } else {
                                "bg-[#f7f7f7] border border-[#7c8292]"
                            },
                        ),
                        input {
                            class: "flex flex-row w-full h-full bg-transparent focus:outline-none",
                            r#type: "text",
                            placeholder: "Enter public name or email address".to_string(),
                            value: (name)(),
                            onfocus: move |_| {
                                is_focused.set(true);
                            },
                            onblur: move |_| {
                                is_focused.set(false);
                            },
                            oninput: move |event| {
                                name.set(event.value());
                            },
                        }
                        Search { width: "18", height: "18", color: "#7c8292" }
                    }
                    div { class: "flex flex-row justify-end items-end",
                        div { class: "w-[25px] h-[25px]",
                            ArrowLeft { width: "25", height: "25", color: "#555462" }
                        }
                        div { class: "w-[25px] h-[25px]",
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
                            div { class: "w-[19px] h-[19px]",
                                Switch { width: "19", height: "19" }
                            }
                        }
                        div { class: "flex flex-row w-[90px] min-w-[90px] h-full justify-center items-center gap-[10px]" }
                    }
                    for project in projects {
                        div { class: "flex flex-col w-full justify-start items-start",
                            div { class: "flex flex-row w-full h-[1px] bg-[#bfc8d9]" }
                            div { class: "flex flex-row w-full h-[55px] justify-start items-center text-[#35343f] font-semibold text-[14px]",
                                div { class: "flex flex-row w-[120px] min-w-[120px] h-full justify-center items-center gap-[10px]",
                                    match project.project_type {
                                        ProjectType::Investigation => i18n.investigation.clone(),
                                        _ => i18n.public_opinion.clone(),
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
                                        ProjectStatus::Ready => i18n.ready.clone(),
                                        ProjectStatus::InProgress => i18n.in_progress.clone(),
                                        _ => i18n.finish.clone(),
                                    }
                                }
                                div { class: "group relative w-[90px] min-w-[90px] h-full justify-center items-center",
                                    button {
                                        class: "flex flex-row w-full h-full justify-center items-center",
                                        onclick: move |_| {},
                                        RowOption { width: 24, height: 24 }
                                    }
                                    nav {
                                        tabindex: "0",
                                        class: "border-2 bg-white invisible border-none shadow-lg rounded w-60 absolute right-0 top-full transition-all opacity-0 group-focus-within:visible group-focus-within:opacity-100 group-focus-within:translate-y-1 group-focus-within:z-20",
                                        ul { class: "py-1",
                                            li {
                                                class: "p-3 text-sm text-gray-700 hover:bg-gray-100 cursor-pointer",
                                                onclick: move |e: MouseEvent| {
                                                    change_popup_state.call(e);
                                                },
                                                {i18n.exclude_from_project.clone()}
                                            }
                                        }
                                    }
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
    members: Vec<GroupMemberResponse>,
    total_groups: Vec<String>,
    total_roles: Vec<String>,
    change_popup_state: EventHandler<String>,
    group_name: String,
    i18n: GroupParticipantTranslate,
) -> Element {
    let mut name = use_signal(|| "".to_string());
    let mut is_focused = use_signal(|| false);
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "font-bold text-[#3a3a3a] text-[16px] mb-[10px]", {i18n.group_team_member} }
            div {
                class: "flex flex-col w-full justify-start items-start bg-white rounded-lg shadow-lg p-[20px]",
                style: "box-shadow: 0 4px 6px rgba(53, 70, 177, 0.05);",
                div { class: "flex flex-row w-full justify-between items-center pb-[20px]",
                    div {
                        class: format!(
                            "flex flex-row w-[590px] h-[45px] justify-between items-center rounded-lg  {} px-[11px] py-[13px]",
                            if (is_focused)() {
                                "bg-[#ffffff] border border-[#2a60d3]"
                            } else {
                                "bg-[#f7f7f7] border border-[#7c8292]"
                            },
                        ),
                        input {
                            class: "flex flex-row w-full h-full bg-transparent focus:outline-none",
                            r#type: "text",
                            placeholder: "Enter public name or email address".to_string(),
                            value: (name)(),
                            onfocus: move |_| {
                                is_focused.set(true);
                            },
                            onblur: move |_| {
                                is_focused.set(false);
                            },
                            oninput: move |event| {
                                name.set(event.value());
                            },
                        }
                        Search { width: "18", height: "18", color: "#7c8292" }
                    }
                    div { class: "flex flex-row gap-[40px] items-center",
                        div {
                            class: "flex flex-row w-[150px] h-[40px] bg-[#2a60d3] rounded-md px-[14px] py-[8px] gap-[5px] cursor-pointer",
                            onclick: move |_| {
                                change_popup_state.call("add_member".to_string());
                            },
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
                                            {member.user_name.clone()}
                                        }
                                        div { class: "text-[14px] font-normal text-[#7c8292]",
                                            {member.user_email.clone()}
                                        }
                                    }
                                }
                                div { class: "flex flex-row w-[310px] min-w-[310px] h-full justify-center items-center gap-[10px]",
                                    select {
                                        class: "bg-transparent focus:outline-none",
                                        value: group_name.clone(),
                                        //TODO: update member group
                                        onchange: |_evt| {},
                                        option {
                                            value: "",
                                            selected: group_name.clone() == "".to_string(),
                                            {i18n.no_group.clone()}
                                        }
                                        for group in total_groups.clone() {
                                            option {
                                                value: group.clone(),
                                                selected: group_name.clone() == group,
                                                "{group}"
                                            }
                                        }
                                    }
                                }
                                div { class: "flex flex-row w-[310px] min-w-[310px] h-full justify-center items-center gap-[10px]",
                                    select {
                                        class: "bg-transparent focus:outline-none",
                                        value: member.role_name.clone(),
                                        //TODO: update member role
                                        onchange: |_evt| {},
                                        option {
                                            value: "",
                                            selected: member.role_name.is_none(),
                                            {i18n.no_role.clone()}
                                        }
                                        for role in total_roles.clone() {
                                            option {
                                                value: role.clone(),
                                                selected: !member.role_name.is_none() && member.role_name.clone().unwrap_or_default() == role,
                                                "{role}"
                                            }
                                        }
                                    }
                                }
                                div { class: "flex flex-row w-full h-full justify-center items-center gap-[10px]",
                                    // if member.projects.len() > 0 {
                                    //     Label {
                                    //         label_name: member.projects[0].clone(),
                                    //         label_color: "bg-[#35343f]",
                                    //     }
                                    // }
                                    div { class: "flex flex-row w-[24px] h-[24px] justify-center items-center bg-[#d1d1d1] ml-[5px] opacity-50 rounded-lg",
                                        Plus { width: "10", height: "10" }
                                    }
                                    div { class: "pl-[20px]",
                                        Expand { width: "18", height: "18" }
                                    }
                                }
                                div { class: "group relative w-[90px] min-w-[90px] h-full",
                                    button {
                                        class: "flex flex-row w-full h-full justify-center items-center",
                                        onclick: move |_| {},
                                        RowOption { width: 24, height: 24 }
                                    }
                                    nav {
                                        tabindex: "0",
                                        class: "border-2 bg-white invisible border-none shadow-lg rounded w-60 absolute right-0 top-full transition-all opacity-0 group-focus-within:visible group-focus-within:opacity-100 group-focus-within:translate-y-1 group-focus-within:z-20",
                                        ul { class: "py-1",
                                            li {
                                                class: "p-3 text-sm text-gray-700 hover:bg-gray-100 cursor-pointer",
                                                onclick: move |_| {
                                                    change_popup_state.call("remove_member".to_string());
                                                },
                                                {i18n.remove_team_member.clone()}
                                            }
                                        }
                                    }
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
pub fn RemoveProjectModal(
    onclose: EventHandler<MouseEvent>,
    i18n: RemoveProjectTranslate,
) -> Element {
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "flex flex-col text-[#222222] font-normal text-[14px] gap-[5px]",
                div { {i18n.remove_project_modal_title} }
                div { {i18n.remove_project_modal_info} }
            }
            div { class: "flex flex-row w-full justify-start items-start mt-[40px] gap-[20px]",
                div {
                    class: "flex flex-row w-[85px] h-[40px] justify-center items-center bg-[#2a60d3] rounded-md cursor-pointer",
                    onclick: move |_| {},
                    div { class: "text-white font-bold text-[16px]", {i18n.remove.clone()} }
                }
                div {
                    class: "flex flex-row w-[85px] h-[40px] font-semibold text-[16px] text-[#222222] justify-center items-center cursor-pointer",
                    onclick: move |e: MouseEvent| {
                        onclose.call(e);
                    },
                    {i18n.cancel.clone()}
                }
            }
        }
    }
}

#[component]
pub fn RemoveMemberModal(
    onclose: EventHandler<MouseEvent>,
    i18n: RemoveMemberTranslate,
) -> Element {
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "flex flex-col text-[#222222] font-normal text-[14px] gap-[5px]",
                div { {i18n.remove_member_modal_title} }
                div { {i18n.remove_member_modal_info} }
            }
            div { class: "flex flex-row w-full justify-start items-start mt-[40px] gap-[20px]",
                div {
                    class: "flex flex-row w-[85px] h-[40px] justify-center items-center bg-[#2a60d3] rounded-md cursor-pointer",
                    onclick: move |_| {},
                    div { class: "text-white font-bold text-[16px]", {i18n.remove.clone()} }
                }
                div {
                    class: "flex flex-row w-[85px] h-[40px] font-semibold text-[16px] text-[#222222] justify-center items-center cursor-pointer",
                    onclick: move |e: MouseEvent| {
                        onclose.call(e);
                    },
                    {i18n.cancel.clone()}
                }
            }
        }
    }
}

#[component]
pub fn UpdateGroupNameModal(
    onclose: EventHandler<MouseEvent>,
    update_group_name: EventHandler<String>,
    initialize_group_name: String,
    i18n: UpdateGroupNameTranslate,
) -> Element {
    let mut group_name = use_signal(|| initialize_group_name);
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "flex flex-col text-[#222222] font-normal text-[14px] gap-[5px] mb-[40px]",
                {i18n.update_group_name_modal_info}
            }
            div { class: "flex flex-col w-full justify-start items-start",
                div { class: "font-semibold text-[14px] text-[#222222] mb-[16px]", {i18n.group_name} }
                input {
                    class: "flex flex-row w-full h-[45px] bg-[#f7f7f7] rounded-sm focus:outline-none focus:border focus:border-[#2a60d3] focus:bg-white px-[15px] items-center mb-[5px] text-[#222222]",
                    r#type: "text",
                    placeholder: i18n.update_group_name_hint,
                    value: (group_name)(),
                    oninput: move |event| {
                        group_name.set(event.value());
                    },
                }
                div { class: "font-normal text-[13px] text-[#222222]",
                    {i18n.update_group_name_warning}
                }
            }
            div { class: "flex flex-row w-full justify-start items-start mt-[40px] gap-[20px]",
                div {
                    class: "flex flex-row w-[85px] h-[40px] justify-center items-center bg-[#2a60d3] rounded-md cursor-pointer",
                    onclick: move |_| {
                        update_group_name.call(group_name());
                    },
                    div { class: "text-white font-bold text-[16px]", {i18n.update} }
                }
                div {
                    class: "flex flex-row w-[85px] h-[40px] font-semibold text-[16px] text-[#222222] justify-center items-center cursor-pointer",
                    onclick: move |e: MouseEvent| {
                        onclose.call(e);
                    },
                    {i18n.cancel}
                }
            }
        }
    }
}

#[component]
pub fn RemoveGroupModal(
    onclose: EventHandler<MouseEvent>,
    remove_group: EventHandler<MouseEvent>,
    i18n: RemoveGroupTranslate,
) -> Element {
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "flex flex-col text-[#222222] font-normal text-[14px] gap-[5px]",
                div { {i18n.remove_group_modal_title} }
                div { {i18n.remove_group_modal_info} }
            }
            div { class: "flex flex-row w-full justify-start items-start mt-[40px] gap-[20px]",
                div {
                    class: "flex flex-row w-[85px] h-[40px] justify-center items-center bg-[#2a60d3] rounded-md cursor-pointer",
                    onclick: move |e: MouseEvent| {
                        remove_group.call(e);
                    },
                    div { class: "text-white font-bold text-[16px]", {i18n.remove} }
                }
                div {
                    class: "flex flex-row w-[85px] h-[40px] font-semibold text-[16px] text-[#222222] justify-center items-center cursor-pointer",
                    onclick: move |e: MouseEvent| {
                        onclose.call(e);
                    },
                    {i18n.cancel}
                }
            }
        }
    }
}

#[component]
pub fn AddMemberModal(
    roles: Vec<String>,
    onclose: EventHandler<MouseEvent>,
    i18n: AddMemberTranslate,
) -> Element {
    let mut email = use_signal(|| "".to_string());

    let mut name = use_signal(|| "".to_string());

    let mut select_role = use_signal(|| "".to_string());

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "flex flex-row w-full mb-[16px]",
                div { class: "text-[#eb5757] font-semibold text-[14px] mr-[5px]", {i18n.necessary} }
                div { class: "text-[#222222] font-semibold text-[14px]", {i18n.input_email_address} }
            }
            input {
                class: "flex flex-row w-full h-[45px] bg-[#f7f7f7] rounded-sm focus:outline-none focus:border focus:border-[#2a60d3] focus:bg-white px-[15px] items-center mb-[5px] text-[#222222]",
                r#type: "text",
                placeholder: i18n.input_email_address_hint,
                value: (email)(),
                oninput: move |event| {
                    email.set(event.value());
                },
            }
            div { class: "font-normal text-[#6f6f6f] text-[13px] mt-[5px] mb-[40px]",
                {i18n.input_email_address_info}
            }
            div { class: "flex flex-col w-full justify-start itmes-start",
                div { class: "font-medium text-[15px] text-[#222222] mb-[16px]", {i18n.privacy} }
                div { class: "flex flex-col w-full justify-start items-start border border-[#bfc8d9] rounded-lg p-[24px]",
                    div { class: "flex flex-row w-full justify-start items-center mb-[10px]",
                        div { class: "flex flex-row w-[60px]",
                            div { class: "text-[#eb5757] font-medium text-[15px] mr-[3px]",
                                "*"
                            }
                            div { class: "text-[#222222] font-medium text-[15px] mr-[3px] w-[40px]",
                                {i18n.name}
                            }
                        }
                        input {
                            class: "flex flex-row w-full h-[45px] bg-[#f7f7f7] rounded-sm focus:outline-none focus:border focus:border-[#2a60d3] focus:bg-white px-[15px] items-center mb-[5px] text-[#222222]",
                            r#type: "text",
                            placeholder: i18n.required_input,
                            value: (name)(),
                            oninput: move |event| {
                                name.set(event.value());
                            },
                        }
                    }
                    div { class: "flex flex-row w-full justify-start items-center mb-[10px]",
                        div { class: "text-[#222222] font-medium text-[15px] mr-[3px] w-[60px]",
                            {i18n.role}
                        }
                        select {
                            class: "flex flex-row w-full h-[45px] bg-[#f7f7f7] rounded-sm focus:outline-none focus:border focus:border-[#2a60d3] focus:bg-white px-[15px] items-center mb-[5px] text-[#222222]",
                            value: select_role(),
                            onchange: move |evt| {
                                select_role.set(evt.value());
                            },
                            option {
                                value: "",
                                disabled: true,
                                selected: select_role() == "",
                                hidden: select_role() != "",
                                {i18n.select_role}
                            }
                            for role in roles.clone() {
                                option {
                                    value: role.clone(),
                                    selected: role == select_role(),
                                    "{role}"
                                }
                            }
                        }
                    }
                }
            }
            div { class: "flex flex-col w-full justify-start items-start mt-[40px]",
                div { class: "font-medium text-[15px] text-[#222222] mb-[16px]",
                    {i18n.invite_project}
                }
                div { class: "flex flex-col w-full justify-start items-start border border-[#bfc8d9] rounded-lg p-[24px]",
                    div { class: "flex flex-row w-full justify-start items-center mb-[10px]",
                        div { class: "flex flex-row w-[60px]",
                            div { class: "text-[#222222] font-medium text-[15px] mr-[3px] w-[40px]",
                                {i18n.public_opinion}
                            }
                        }
                        div { class: "flex flex-row w-full h-[45px] justify-start items-center px-[11px] py-[13px] bg-[#f7f7f7] rounded-lg " }
                    }
                    div { class: "flex flex-row w-full justify-start items-center mb-[10px]",
                        div { class: "flex flex-row w-[60px]",
                            div { class: "text-[#222222] font-medium text-[15px] mr-[3px] w-[40px]",
                                {i18n.investigation}
                            }
                        }
                        div { class: "flex flex-row w-full h-[45px] justify-start items-center px-[11px] py-[13px] bg-[#f7f7f7] rounded-lg " }
                    }
                }
            }
            div { class: "flex flex-row w-full justify-start items-start mt-[40px] gap-[20px]",
                div {
                    class: "flex flex-row w-[120px] h-[40px] bg-[#2a60d3] rounded-md px-[14px] py-[8px] gap-[5px] cursor-pointer",
                    onclick: move |_| {},
                    AddUser { width: "24", height: "24" }
                    div { class: "text-white font-bold text-[16px]", {i18n.invite} }
                }
                div {
                    class: "flex flex-row w-[85px] h-[40px] font-semibold text-[16px] text-[#222222] justify-center items-center cursor-pointer",
                    onclick: move |e: MouseEvent| {
                        onclose.call(e);
                    },
                    {i18n.cancel}
                }
            }
        }
    }
}
