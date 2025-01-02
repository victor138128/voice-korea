#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_logger::tracing;

use crate::components::icons::ModalCancel;

#[derive(Debug, Clone, Copy, Default)]
pub struct PopupService {
    pub data: Signal<Option<Element>>,
    pub title: Signal<String>,
}

impl PopupService {
    pub fn init() {
        let srv = PopupService::default();
        use_context_provider(|| srv);
    }

    pub fn render(&self) -> Element {
        (self.data)().clone().unwrap_or(rsx! {
            Default {}
        })
    }

    pub fn is_opened(&self) -> bool {
        (self.data)().is_some()
    }

    pub fn get_title(&self) -> String {
        (self.title)()
    }

    pub fn open(&mut self, title: String, popup: Element) {
        tracing::debug!("open popup");
        (self.data).set(Some(popup));
        (self.title).set(title);
    }

    pub fn close(&mut self) {
        (self.data).set(None);
        (self.title).set("".to_string());
    }

    pub fn use_popup_service() -> PopupService {
        use_context()
    }
}

#[component]
pub fn Default() -> Element {
    rsx! {}
}

#[component]
pub fn PopupZone() -> Element {
    let mut popup: PopupService = use_context();

    rsx! {
        div {
            class: format!(
                "{}",
                match popup.is_opened() {
                    true => {
                        "fixed top-0 left-0 w-screen h-screen bg-black bg-opacity-50 flex justify-center items-center backdrop-blur-[10px] bg-[#21344C]/30 z-[101]"
                    }
                    false => "hidden",
                },
            ),
            onclick: move |_| {
                popup.close();
            },
            if popup.is_opened() {
                div {
                    class: "relative bg-white rounded-lg p-[40px] min-w-[600px]",
                    onclick: move |e| {
                        e.stop_propagation();
                    },
                    if popup.get_title() != "" {
                        div { class: "absolute top-0 left-0 m-[40px] font-semibold text-[#3a3a3a] text-[20px]",
                            {popup.get_title()}
                        }
                    }
                    div {
                        class: "absolute top-0 right-0 m-[40px] cursor-pointer",
                        onclick: move |_| {
                            popup.close();
                        },
                        ModalCancel { width: "24", height: "24" }
                    }
                    div { class: "mt-[30px]", {popup.render()} }
                }
            }
        }
    }
}
