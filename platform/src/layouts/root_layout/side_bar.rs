#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::routes::Route;

use super::Language;

#[derive(PartialEq, Props, Clone)]
pub struct SidebarProps {
    onselected: EventHandler<SelectedMenu>,
    lang: Language,
    overview: String,
    search_project: String,
    import_project: String,
    survey_management: String,
    questionnaire_management: String,
    question_bank: String,
    property_management: String,
    property_status: String,
    user_settings: String,
}

#[component]
pub fn SideBar(props: SidebarProps) -> Element {
    rsx! {
        div {
            class: "flex-col min-w-[250px] grow bg-[#2168c3] justify-between items-between",
            div {
                class: "flex flex-col w-full h-full justify-between items-between",
                div {
                    class: "flex flex-col grow items-center",
                    SectionMenus {
                        onselected: props.onselected,
                        title: props.overview,
                        menus: vec![
                            MenuItem {title: props.search_project, link: Route::DashboardPage { lang: props.lang }},
                            MenuItem {title: props.import_project, link: Route::DashboardPage { lang: props.lang }},
                        ]
                    }
                    SectionMenus {
                        onselected: props.onselected,
                        title: props.survey_management,
                        menus: vec![
                            MenuItem {title: {props.questionnaire_management}, link: Route::DashboardPage { lang: props.lang }},
                            MenuItem {title: {props.question_bank}, link: Route::DashboardPage { lang: props.lang }},
                        ]
                    }
                    SectionMenus {
                        onselected: props.onselected,
                        title: props.property_management,
                        menus: vec![
                            MenuItem {title: {props.property_status}, link: Route::AttributePage { lang: props.lang }},
                        ]
                    }
                }
                div {
                    class: "flex flex-row w-full h-full justify-end items-end p-[10px]",
                    div {
                        class: "flex flex-row w-[110px]",
                        div {
                            class: "text-white text-[14px] font-normal pr-[10px]",
                            {props.user_settings}
                        }
                        img {
                            src: asset!("/public/images/config.png"),
                            class: "w-[24px] h-[24px]",
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Props)]
pub struct SelectedMenu {
    pub category: String,
    pub menu: String,
}

#[derive(Clone, PartialEq, Props)]
pub struct MenuItem {
    title: String,
    link: Route,
}

#[component]
pub fn SectionMenus(
    onselected: EventHandler<SelectedMenu>,
    title: String,
    menus: Vec<MenuItem>,
) -> Element {
    let mut expanded = use_signal(|| true);

    rsx! {
        div {
            class: "flex flex-col w-full",
            div {
                class: "flex flex-row h-[45px] w-full px-[10px] items-center justify-between",
                style: "background-color: rgb(0,0,0,0.2);",
                onclick: move |_| {
                    expanded.set(!expanded());
                },
                div { class: "font-bold text-[16px] text-white", "{title}"}
                if expanded() {
                    img {
                        src: asset!("/public/images/arrow-down.png"),
                        class: "w-[18px] h-[9px]",
                    }
                } else {
                    img {
                        src: asset!("/public/images/arrow-down.png"),
                        class: "w-[18px] h-[9px]",
                        style: "transform: scaleY(-1);"
                    }
                }
            }
            if expanded() {
                for menu in menus {
                    {
                        let title = title.clone();
                        let cm = menu.clone();

                        rsx!{
                            Link {
                                onclick: move |_evt|{
                                    onselected.call(SelectedMenu { category: title.clone(), menu: cm.title.clone() });
                                },
                                to: menu.link,
                                class: "flex flex-row h-[45px] w-full gap-[5px] px-[10px] items-center justify-start",
                                style: "background-color: rgb(0,0,0,0.1);",
                                div { class: "flex font-normal text-[14px] text-white", "{menu.title}"}
                            }
                        }
                    }
                }
            }
        }
    }
}
