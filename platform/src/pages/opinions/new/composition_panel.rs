use dioxus::prelude::*;
use dioxus_translate::{translate, Language};
use models::prelude::CompositionPanelInfo;

use crate::{
    components::icons::{BottomDropdownArrow, Checked, Clear, Remove, TopDropdownArrow, UnChecked},
    pages::opinions::new::i18n::{
        CompositionPanelTranslate, DirectedAddPanelTranslate, SettingTotalPanelTranslate,
    },
};

#[derive(Props, Clone, PartialEq)]
pub struct CompositionPanelProps {
    lang: Language,
}

#[component]
pub fn CompositionPanel(props: CompositionPanelProps) -> Element {
    let translates: CompositionPanelTranslate = translate(&props.lang);
    let selected_option = use_signal(move || translates.faired_people_allocated.to_string());
    let total_members = use_signal(move || "0".to_string());
    let panels: Signal<Vec<CompositionPanelInfo>> = use_signal(|| vec![]);
    let directed_add_panels: Signal<Vec<CompositionPanelInfo>> = use_signal(|| vec![]);

    rsx! {
        div { class: "flex flex-col w-full justify-start items-start",
            div { class: "font-medium text-[16px] text-[#222222] mb-[10px]",
                "{translates.participant_panel_composition}"
            }
            SettingTotalPanel {
                lang: props.lang,
                selected_option,
                total_members,
                panels,
                directed_add_panels,
            }
            DirectedAddPanel { lang: props.lang, panels, directed_add_panels }

            div { class: "flex flex-row w-full justify-end items-end mt-[40px] mb-[50px]",
                div {
                    class: "flex flex-row w-[70px] h-[55px] rounded-[4px] justify-center items-center bg-white border border-[#bfc8d9] font-semibold text-[16px] text-[#555462] mr-[20px]",
                    onclick: move |_| {},
                    "{translates.backward}"
                }
                div {
                    class: "flex flex-row w-[105px] h-[55px] rounded-[4px] justify-center items-center bg-white border border-[#bfc8d9] font-semibold text-[16px] text-[#555462] mr-[20px]",
                    onclick: move |_| {},
                    "{translates.temporary_save}"
                }
                div {
                    class: "cursor-pointer flex flex-row w-[110px] h-[55px] rounded-[4px] justify-center items-center bg-[#2a60d3] font-semibold text-[16px] text-white",
                    onclick: move |_| {},
                    "{translates.next}"
                }
            }
        }
    }
}

#[component]
pub fn DirectedAddPanel(
    lang: Language,
    panels: Signal<Vec<CompositionPanelInfo>>,
    directed_add_panels: Signal<Vec<CompositionPanelInfo>>,
) -> Element {
    let mut add_panel_clicked = use_signal(|| false);
    let translates: DirectedAddPanelTranslate = translate(&lang);

    rsx! {
        div { class: "flex flex-col w-full",
            div { class: "flex flex-col w-full justify-between items-center px-[40px] py-[24px] bg-white rounded-lg mt-[20px]",
                div { class: "flex flex-row w-full justify-between items-center ",
                    div { class: "flex flex-col w-full justify-start items-start",
                        div { class: "font-bold text-[#222222] text-lg mb-[3px]",
                            "{translates.directed_add_panel_title}"
                        }
                        div { class: "font-normal text-[#6d6d6d] text-sm",
                            "{translates.directed_add_panel_description}"
                        }
                    }
                    div {
                        onclick: move |_| {
                            let clicked = add_panel_clicked();
                            add_panel_clicked.set(!clicked);
                        },
                        div { class: "cursor-pointer",
                            if add_panel_clicked() {
                                TopDropdownArrow { width: "24", height: "24" }
                            } else {
                                BottomDropdownArrow { width: "24", height: "24" }
                            }
                        }
                    }
                }
                div { class: "flex flex-col w-full",
                    for (i , panel) in directed_add_panels().into_iter().enumerate() {
                        div { class: "flex flex-row w-full justify-start items-center mt-[20px]",
                            div { class: "flex flex-row w-[215px] focus:outline-none h-[55px] justify-start items-center bg-[#f7f7f7] rounded-[4px] px-[15px] mr-[20px]",
                                input {
                                    class: "flex flex-row w-full justify-start items-center bg-transparent focus:outline-none",
                                    r#type: "text",
                                    placeholder: "{translates.enter_panel_name}",
                                    value: panel.clone().name.clone(),
                                    oninput: move |e: FormEvent| {
                                        let mut pn = panel.clone();
                                        pn.name = e.value();
                                        let mut aps = directed_add_panels();
                                        aps[i] = pn.clone();
                                        directed_add_panels.set(aps);
                                        let mut ps = panels();
                                        let ind = panels().len() - directed_add_panels().len();
                                        ps[ind + i] = pn;
                                        panels.set(ps);
                                    },
                                }
                            }
                            div { class: "flex flex-row w-full h-[55px] justify-start items-center bg-[#f7f7f7] rounded-[4px] px-[15px] font-medium text-[15px] text-[#b4b4b4]",
                                "{translates.select_attribute}"
                            }
                        }
                    }
                }
                div { class: "relative w-full flex items-center justify-center mt-[40px] mb-[24px]",
                    div { class: "border-t border-dashed border-gray-300 w-full" }
                    button {
                        class: "absolute bg-[#f7f7f7] border border-[#bfc8d9] rounded-[100px] w-[43px] h-[43px] flex items-center justify-center shadow",
                        onclick: move |_| {
                            let mut ps = panels();
                            ps.push(CompositionPanelInfo {
                                panel_id: None,
                                name: "".to_string(),
                            });
                            panels.set(ps);
                            let mut aps = directed_add_panels();
                            aps.push(CompositionPanelInfo {
                                panel_id: None,
                                name: "".to_string(),
                            });
                            directed_add_panels.set(aps);
                        },
                        "+"
                    }
                }
            }
        }
    }
}

#[component]
pub fn SettingTotalPanel(
    lang: Language,
    selected_option: Signal<String>,
    total_members: Signal<String>,
    panels: Signal<Vec<CompositionPanelInfo>>,
    directed_add_panels: Signal<Vec<CompositionPanelInfo>>,
) -> Element {
    let translates: SettingTotalPanelTranslate = translate(&lang);
    rsx! {
        div { class: "flex flex-col w-full justify-start items-start rounded-lg bg-white px-[40px] py-[24px]",
            div { class: "font-bold text-[#222222] text-lg mb-[3px]",
                "{translates.setting_total_panel_title}"
            }
            div { class: "font-normal text-[#6d6d6d] text-sm mb-[20px]",
                "{translates.setting_total_panel_description}"
            }

            div { class: "flex flex-row w-full justify-between items-center mb-[10px]",
                div { class: "flex flex-row ",
                    div { class: "flex flex-row w-[180px] mr-[50px] font-medium text-black text-[15px]",
                        "{translates.total_panel}"
                    }

                    div { class: "flex items-center space-x-4",
                        button {
                            onclick: move |_| {
                                selected_option.set(translates.faired_people_allocated.to_string());
                            },

                            if selected_option() == translates.faired_people_allocated {
                                Checked { width: "18", height: "18" }
                            } else {
                                UnChecked { width: "18", height: "18" }
                            }
                        }
                        div { class: "ml-[10px] font-normal text-[#222222] text-[15px] mr-[50px]",
                            "{translates.faired_people_allocated}"
                        }
                        button {
                            onclick: move |_| {
                                selected_option.set(translates.proportional_people_allocated.to_string());
                            },
                            if selected_option() == translates.proportional_people_allocated {
                                Checked { width: "18", height: "18" }
                            } else {
                                UnChecked { width: "18", height: "18" }
                            }
                        }
                        div { class: "ml-[10px] font-normal text-[#222222] text-[15px]",
                            "{translates.proportional_people_allocated}"
                        }
                    }
                }
                div { class: "flex flex-row justify-start items-center",
                    div { class: "flex flex-row w-[215px] focus:outline-none h-[55px] justify-start items-center bg-[#f7f7f7] rounded-[4px] px-[15px] mr-[10px]",
                        input {
                            class: "flex flex-row w-full justify-start items-center bg-transparent focus:outline-none",
                            r#type: "text",
                            placeholder: translates.total_members,
                            value: total_members(),
                            onkeydown: move |e: KeyboardEvent| {
                                let key = e.key();
                                if key != Key::Backspace && key != Key::Delete {
                                    let s = match key {
                                        Key::Character(c) => c,
                                        _ => "".to_string(),
                                    };
                                    if !s.chars().all(|c| c.is_ascii_digit()) {
                                        e.prevent_default();
                                    }
                                }
                            },
                            oninput: move |e: FormEvent| {
                                let new_value = e.value().clone();
                                if new_value.chars().all(|c| c.is_ascii_digit()) {
                                    total_members.set(new_value);
                                } else {
                                    e.prevent_default();
                                }
                            },
                        }
                    }
                    div { class: "font-normal text-black text-[15px]", "명" }
                }
            }

            div { class: "flex flex-row w-full justify-start items-center",
                div { class: "flex flex-row w-[180px] mr-[50px] font-medium text-black text-[15px]",
                    "{translates.select_panel}"
                }
                div { class: "flex flex-between w-full h-[55px] justify-start items-center p-[15px] rounded-[4px] bg-[#f7f7f7]",
                    if panels.len() == 0 {
                        div { class: "font-medium text-[#b4b4b4] text-[15px]",
                            "{translates.select_panel}"
                        }
                    } else {
                        div { class: "flex flex-wrap w-full justify-start items-center gap-[5px]",
                            for (i , panel) in panels().iter().enumerate() {
                                div {
                                    Label {
                                        label: panel.name.clone(),
                                        clicked_label: move |_e: MouseEvent| {
                                            let mut dps = directed_add_panels();
                                            if panels().len() - dps.len() <= i {
                                                dps.remove(i - (panels().len() - dps.len()));
                                                directed_add_panels.set(dps);
                                            }
                                            let mut ps = panels();
                                            ps.remove(i);
                                            panels.set(ps);
                                        },
                                    }
                                }
                            }
                        }
                        button {
                            onclick: move |_| {
                                panels.set(vec![]);
                                directed_add_panels.set(vec![]);
                            },
                            Remove { width: "20", height: "20", fill: "#555462" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn Label(label: String, clicked_label: EventHandler<MouseEvent>) -> Element {
    rsx! {
        div { class: "flex flex-row w-[80px] h-[25px] justify-between items-center pl-[8px] bg-[#35343f] rounded-[4px]",
            div { class: "font-semibold text-[14px] text-white", {label} }
            button {
                onclick: move |e: MouseEvent| {
                    clicked_label.call(e);
                },
                Clear { width: "24", height: "24" }
            }
        }
    }
}
