#![allow(non_snake_case)]
use crate::prelude::*;
use dioxus::prelude::*;
use header::Header;
use side_bar::{SelectedMenu, SideBar};

pub mod header;
pub mod i18n;
pub mod side_bar;

#[component]
pub fn RootLayout(lang: Language) -> Element {
    use dioxus_logger::tracing;
    let translates = i18n::translate(lang.clone());
    rsx! {
        div {
            class: "flex flex-col w-screen min-h-screen bg-white text-black",
            Header {
                logout: translates.logout,
                lang,
            }
            div {
                class: "flex flex-row min-w-full max-w-full grow",
                SideBar {
                    onselected: |selected_menu: SelectedMenu| {
                        tracing::info!("selected menu {selected_menu:?}");
                    },
                    lang,
                    overview: translates.overview,
                    search_project: translates.search_project,
                    import_project: translates.import_project,
                    survey_management: translates.survey_management,
                    questionnaire_management: translates.questionnaire_management,
                    question_bank: translates.question_bank,
                    property_management: translates.property_management,
                    property_status: translates.property_status,
                    user_settings: translates.user_settings,
                }
                div {
                    class: "flex flex-col grow w-full bg-[#f0f2fc]",
                    Outlet::<Route> {}
                }
            }
        }
    }
}
