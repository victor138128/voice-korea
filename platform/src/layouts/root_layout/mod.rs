#![allow(non_snake_case)]
use crate::prelude::*;
use dioxus::prelude::*;
use side_bar::{SelectedMenu, SideBar};

pub mod header;
pub mod i18n;
pub mod side_bar;

#[component]
pub fn RootLayout(lang: Language) -> Element {
    let mut selected_menu = use_signal(|| "프로젝트 검색".to_string());
    use dioxus_logger::tracing;
    // let translates = i18n::translate(lang.clone());
    rsx! {
        div { class: "flex flex-col w-screen min-h-screen bg-white text-black",
            // Header {
            //     logout: translates.logout,
            //     lang,
            // }
            div { class: "flex flex-row min-w-full max-w-full grow",
                SideBar {
                    onselected: move |selected: SelectedMenu| {
                        tracing::info!("selected menu {selected:?}");
                        selected_menu.set(selected.menu);
                    },
                    selected_menu: (selected_menu)(),
                    lang,
                }
                div { class: "flex flex-col grow w-full bg-[#f0f2fc]", Outlet::<Route> {} }
            }
        }
    }
}
