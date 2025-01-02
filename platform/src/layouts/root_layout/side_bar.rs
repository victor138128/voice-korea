#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::{
    components::icons::{BottomArrow, Logo},
    routes::Route,
};

use super::Language;

#[derive(PartialEq, Props, Clone)]
pub struct SidebarProps {
    onselected: EventHandler<SelectedMenu>,
    selected_menu: String,
    lang: Language,
}

#[component]
pub fn SideBar(props: SidebarProps) -> Element {
    rsx! {
        div { class: "flex-col min-w-[250px] grow bg-[#435393] justify-between items-between rounded-tr-lg rounded-br-lg",
            div { class: "flex flex-col w-full h-full justify-between items-between",
                div { class: "flex flex-col w-full justify-start items-start",
                    div { class: "flex flex-row w-full justify-start items-center mt-[33px] ml-[18px] mb-[40px]",
                        Logo { width: "31", height: "31" }
                        div { class: "font-bold text-[#b8d5fa] text-[18px] pl-[5px]",
                            "VOICE KOREA"
                        }
                    }
                    div { class: "flex flex-col grow items-center w-full",
                        SectionMenus {
                            onselected: props.onselected,
                            title: "Overview".to_string(),
                            menus: vec![
                                MenuItem {
                                    title: "프로젝트 검색".to_string(),
                                    is_selected: props.selected_menu == "프로젝트 검색",
                                    link: Route::DashboardPage {
                                        lang: props.lang,
                                    },
                                },
                                MenuItem {
                                    title: "기본 프로젝트 가져오기".to_string(),
                                    is_selected: props.selected_menu == "기본 프로젝트 가져오기",
                                    link: Route::DashboardPage {
                                        lang: props.lang,
                                    },
                                },
                            ],
                        }
                        SectionMenus {
                            onselected: props.onselected,
                            title: "조직 관리".to_string(),
                            menus: vec![
                                MenuItem {
                                    title: "팀원 관리".to_string(),
                                    is_selected: props.selected_menu == "팀원 관리",
                                    link: Route::DashboardPage {
                                        lang: props.lang,
                                    },
                                },
                                MenuItem {
                                    title: "그룹 관리".to_string(),
                                    is_selected: props.selected_menu == "그룹 관리",
                                    link: Route::DashboardPage {
                                        lang: props.lang,
                                    },
                                },
                            ],
                        }
                        SectionMenus {
                            onselected: props.onselected,
                            title: "조사 관리".to_string(),
                            menus: vec![
                                MenuItem {
                                    title: "조사 관리".to_string(),
                                    is_selected: props.selected_menu == "조사 관리",
                                    link: Route::DashboardPage {
                                        lang: props.lang,
                                    },
                                },
                                MenuItem {
                                    title: "질문 뱅크".to_string(),
                                    is_selected: props.selected_menu == "질문 뱅크",
                                    link: Route::DashboardPage {
                                        lang: props.lang,
                                    },
                                },
                            ],
                        }
                        SectionMenus {
                            onselected: props.onselected,
                            title: "공론 관리".to_string(),
                            menus: vec![
                                MenuItem {
                                    title: "공론 관리".to_string(),
                                    is_selected: props.selected_menu == "공론 관리",
                                    link: Route::DashboardPage {
                                        lang: props.lang,
                                    },
                                },
                            ],
                        }
                        SectionMenus {
                            onselected: props.onselected,
                            title: "속성 & 패널 관리".to_string(),
                            menus: vec![
                                MenuItem {
                                    title: "속성 & 패널 관리".to_string(),
                                    is_selected: props.selected_menu == "속성 & 패널 관리",
                                    link: Route::DashboardPage {
                                        lang: props.lang,
                                    },
                                },
                            ],
                        }
                        SectionMenus {
                            onselected: props.onselected,
                            title: "자료 관리".to_string(),
                            menus: vec![
                                MenuItem {
                                    title: "자료 관리".to_string(),
                                    is_selected: props.selected_menu == "자료 관리",
                                    link: Route::DashboardPage {
                                        lang: props.lang,
                                    },
                                },
                            ],
                        }
                    }
                }
                div { class: "flex flex-row w-full h-full justify-end items-end p-[10px]",
                    div { class: "flex flex-row w-[110px]",
                        div { class: "text-white text-[14px] font-normal pr-[10px]",
                            {"사용자 설정"}
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
    is_selected: bool,
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
        div { class: "flex flex-col w-full px-[18px] mb-[40px]",
            div {
                class: "flex flex-row h-[45px] w-full items-center justify-between cursor-pointer",
                onclick: move |_| {
                    expanded.set(!expanded());
                },
                div { class: "font-semibold text-[14px] text-white", "{title}" }
                if expanded() {
                    BottomArrow { width: "18".to_string(), height: "14".to_string() }
                } else {
                    div { style: "transform: scaleY(-1);",
                        BottomArrow { width: "18".to_string(), height: "14".to_string() }
                    }
                }
            }
            if expanded() {
                for menu in menus {
                    {
                        let title = title.clone();
                        let cm = menu.clone();
                        rsx! {
                            Link {
                                onclick: move |_evt| {
                                    onselected
                                        .call(SelectedMenu {
                                            category: title.clone(),
                                            menu: cm.title.clone(),
                                        });
                                },
                                to: menu.link,
                                class: format!(
                                    "flex flex-row h-[45px] w-full px-[16px] items-center justify-start {}",
                                    if menu.is_selected { "rounded-lg bg-[#182248]" } else { "" },
                                ),
                                div { class: "flex font-medium text-[14px] text-[#daeaff]", "{menu.title}" }
                            }
                        }
                    }
                }
            }
        }
    }
}
