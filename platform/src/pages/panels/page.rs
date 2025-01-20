use dioxus::prelude::*;
use dioxus_translate::{translate, Language};
use models::prelude::{AttributeSummary, PanelSummary};

use crate::{
    components::icons::{ArrowLeft, ArrowRight, RowOption, Search, Switch},
    pages::panels::{
        controller::Controller,
        i18n::{AttributeListTranslate, PanelListTranslate, PanelTranslate},
    },
};

#[derive(Props, Clone, PartialEq)]
pub struct PanelProps {
    lang: Language,
}

#[component]
pub fn PanelPage(props: PanelProps) -> Element {
    let ctrl = Controller::init(props.lang);
    let panels = ctrl.get_panels();
    let attributes = ctrl.get_attributes();

    let translate: PanelTranslate = translate(&props.lang);

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "text-[#9b9b9b] font-medium text-[14px] mb-[10px]", "{translate.panel_title}" }
        }
        div { class: "text-[#3a3a3a] font-semibold text-[28px] mb-[25px]", "{translate.panel_title}" }
        div { class: "text-[#35343f] font-normal text-[14px] mb-[40px]",
            "{translate.panel_description}"
        }
        PanelList { lang: props.lang, panels }
        AttributeList { lang: props.lang, attributes }
    }
}

#[component]
pub fn AttributeList(lang: Language, attributes: Vec<AttributeSummary>) -> Element {
    let mut is_focused = use_signal(|| false);
    let mut attribute_name = use_signal(|| "".to_string());

    let translate: AttributeListTranslate = translate(&lang);
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start mb-[40px]",
            div { class: "font-bold text-[#222222] text-[16px] mb-[10px]", "{translate.attribute_list}" }
            div {
                class: "flex flex-col w-full justify-start items-start px-[20px] pt-[20px] pb-[30px] bg-white rounded-[8px]",
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
                            placeholder: translate.search_hint,
                            value: (attribute_name)(),
                            onfocus: move |_| {
                                is_focused.set(true);
                            },
                            onblur: move |_| {
                                is_focused.set(false);
                            },
                            oninput: move |event| {
                                attribute_name.set(event.value());
                            },
                        }
                        Search { width: "18", height: "18", color: "#7c8292" }
                    }
                    div { class: "flex flex-row gap-[10px]",
                        div { class: "w-[25px] h-[25px]",
                            ArrowLeft { width: "25", height: "25", color: "#555462" }
                        }
                        div { class: "w-[25px] h-[25px]",
                            ArrowRight { width: "25", height: "25", color: "#555462" }
                        }
                    }
                }
                div { class: "flex flex-col w-full justify-start items-start border rounded-lg border-[#bfc8d9]",
                    div { class: "flex flex-row w-full h-[55px] justify-start items-center",
                        div { class: "flex flex-row w-[185px] min-w-[185px] h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                "{translate.attribute_name}"
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row flex-1 h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                "{translate.attribute}"
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row w-[90px] min-w-[90px] h-full justify-center items-center gap-[10px]",
                            button { class: "flex flex-row w-[24px] h-[24px] justify-center items-center bg-[#d1d1d1] opacity-50 rounded-[4px] font-bold text-[#35343f] text-lg",
                                "+"
                            }
                        }
                    }
                    for attribute in attributes {
                        div { class: "flex flex-col w-full justify-start items-start",
                            div { class: "flex flex-row w-full h-[1px] bg-[#bfc8d9]" }
                            div { class: "flex flex-row w-full h-[55px]",
                                div { class: "flex flex-row w-[185px] min-w-[185px] h-full justify-center items-center",
                                    div { class: "font-medium text-[#222222] text-[14px]",
                                        "{attribute.name}"
                                    }
                                }
                                div { class: "flex flex-row flex-1 h-full justify-center items-center gap-[10px]",
                                    for attr in attribute.attribute {
                                        PanelLabel { label: attr.name }
                                    }
                                }
                                div { class: "flex flex-row w-[90px] min-w-[90px] h-full justify-center items-center",
                                    RowOption { width: "24", height: "24" }
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
pub fn PanelList(lang: Language, panels: Vec<PanelSummary>) -> Element {
    let mut is_focused = use_signal(|| false);
    let mut panel_name = use_signal(|| "".to_string());
    let translate: PanelListTranslate = translate(&lang);
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start mb-[40px]",
            div { class: "font-bold text-[#222222] text-[16px] mb-[10px]", "{translate.panel_list}" }
            div {
                class: "flex flex-col w-full justify-start items-start px-[20px] pt-[20px] pb-[30px] bg-white rounded-[8px]",
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
                            placeholder: translate.search_hint,
                            value: (panel_name)(),
                            onfocus: move |_| {
                                is_focused.set(true);
                            },
                            onblur: move |_| {
                                is_focused.set(false);
                            },
                            oninput: move |event| {
                                panel_name.set(event.value());
                            },
                        }
                        Search { width: "18", height: "18", color: "#7c8292" }
                    }
                    div { class: "flex flex-row gap-[10px]",
                        div { class: "w-[25px] h-[25px]",
                            ArrowLeft { width: "25", height: "25", color: "#555462" }
                        }
                        div { class: "w-[25px] h-[25px]",
                            ArrowRight { width: "25", height: "25", color: "#555462" }
                        }
                    }
                }
                div { class: "flex flex-col w-full justify-start items-start border rounded-lg border-[#bfc8d9]",
                    div { class: "flex flex-row w-full h-[55px] justify-start items-center",
                        div { class: "flex flex-row flex-1 h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                "{translate.panel_name}"
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row w-[120px] min-w-[120px] h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                "{translate.personnel}"
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row flex-1 h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                "{translate.job}"
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row flex-1 h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                "{translate.gender}"
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row flex-1 h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                "{translate.age}"
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row flex-1 h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                "{translate.education}"
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row flex-1 h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                "{translate.residence}"
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row flex-1 h-full justify-center items-center gap-[10px]",
                            div { class: "text-[#555462] font-semibold text-[14px]",
                                "{translate.nationality}"
                            }
                            Switch { width: "19", height: "19" }
                        }
                        div { class: "flex flex-row w-[90px] min-w-[90px] h-full justify-center items-center gap-[10px]",
                            button { class: "flex flex-row w-[24px] h-[24px] justify-center items-center bg-[#d1d1d1] opacity-50 rounded-[4px] font-bold text-[#35343f] text-lg",
                                "+"
                            }
                        }
                    }
                    for panel in panels {
                        div { class: "flex flex-col w-full justify-start items-start",
                            div { class: "flex flex-row w-full h-[1px] bg-[#bfc8d9]" }
                            div { class: "flex flex-row w-full h-[55px]",
                                div { class: "flex flex-row flex-1 h-full justify-center items-center",
                                    div { class: "font-medium text-[#222222] text-[14px]",
                                        "{panel.name}"
                                    }
                                }
                                div { class: "flex flex-row w-[120px] min-w-[120px] h-full justify-center items-center",
                                    div { class: "font-medium text-[#222222] text-[14px]",
                                        "{panel.count}"
                                    }
                                }
                                for attribute in panel.attribute {
                                    div { class: "flex flex-row flex-1 h-full justify-center items-center gap-[5px]",
                                        for attr in attribute.attribute {
                                            PanelLabel { label: attr.name }
                                        }
                                    }
                                }
                                div { class: "flex flex-row w-[90px] min-w-[90px] h-full justify-center items-center",
                                    RowOption { width: "24", height: "24" }
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
pub fn PanelLabel(label: String) -> Element {
    rsx! {
        div { class: "flex flex-row h-[25px] justify-center items-center px-[8px] py-[3px] bg-[#35343f] rounded-[100px] font-semibold text-[14px] text-white",
            {label}
        }
    }
}
