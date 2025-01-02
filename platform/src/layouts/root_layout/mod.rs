#![allow(non_snake_case)]
use crate::{components::icons::Logout, prelude::*, service::popup_service::PopupZone};
use dioxus::prelude::*;
use side_bar::{SelectedMenu, SideBar};

pub mod header;
pub mod i18n;
pub mod side_bar;

#[component]
pub fn RootLayout(lang: Language) -> Element {
    let mut selected_menu = use_signal(|| "".to_string());
    use dioxus_logger::tracing;

    let route: Route = use_route();
    let current_path = format!("{route}");

    if selected_menu() == "" {
        if current_path.contains("/group") {
            selected_menu.set("그룹 관리".to_string());
        } else if current_path.contains("/member") {
            selected_menu.set("팀원 관리".to_string());
        } else {
            selected_menu.set("프로젝트 검색".to_string());
        }
    }
    // let translates = i18n::translate(lang.clone());
    rsx! {
        div { class: "flex flex-col w-screen min-h-screen bg-white text-black",
            // Header {
            //     logout: translates.logout,
            //     lang,
            // }
            PopupZone {}
            div { class: "flex flex-row min-w-full max-w-full grow",
                SideBar {
                    onselected: move |selected: SelectedMenu| {
                        tracing::info!("selected menu {selected:?}");
                        selected_menu.set(selected.menu);
                    },
                    selected_menu: (selected_menu)(),
                    lang,
                }
                div { class: "flex flex-col grow w-full bg-[#f0f2fc] px-[60px] pt-[25px]",
                    Link { to: Route::LoginPage { lang },
                        div { class: "flex flex-row w-full justify-end items-end gap-[5px]",
                            Logout { width: "20", height: "20" }
                            div { class: "font-bold text-[#555462] text-[15px]", "logout" }
                        }
                    }
                    Outlet::<Route> {}
                }
            }
        }
    }
}
