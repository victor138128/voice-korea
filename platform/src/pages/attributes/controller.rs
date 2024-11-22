#![allow(non_snake_case)]
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SelectAttribute {
    pub id: usize,
    pub name: String,
    pub value: String,
    pub is_stored: bool,   //저장 되었는지 유무
    pub is_search: bool,   //검색 되었는지 유무
    pub is_selected: bool, //선택 되었는지 유무
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Controller {
    pub attributes: Signal<Vec<SelectAttribute>>,
    pub selected_attributes: Signal<Vec<SelectAttribute>>,
    pub enable_modal: Signal<bool>,
    pub search_attributes: Signal<Vec<SelectAttribute>>,
}

impl Controller {
    pub fn init() -> Self {
        let ctrl = Self {
            attributes: use_signal(|| {
                vec![
                    SelectAttribute {
                        id: 0,
                        name: "연령".to_string(),
                        value: "10대, 20대, 30대, 40대, 50대, 60대".to_string(),
                        is_stored: false,
                        is_search: false,
                        is_selected: false,
                    },
                    SelectAttribute {
                        id: 1,
                        name: "성별".to_string(),
                        value: "남성, 여성".to_string(),
                        is_stored: false,
                        is_search: false,
                        is_selected: false,
                    },
                    SelectAttribute {
                        id: 2,
                        name: "소득".to_string(),
                        value: "200만원 이하, 300만원 이하, 400만원 이하, 500만원 이하, 이외"
                            .to_string(),
                        is_stored: false,
                        is_search: false,
                        is_selected: false,
                    },
                ]
            }),
            selected_attributes: use_signal(|| vec![]),
            search_attributes: use_signal(|| vec![]),
            enable_modal: use_signal(|| false),
        };

        ctrl
    }

    pub fn get_total_attributes(&self) -> Vec<SelectAttribute> {
        let mut attributes = vec![];

        for attribute in (self.attributes)() {
            if !attribute.is_search && !attribute.is_stored {
                attributes.push(attribute);
            }
        }

        attributes
    }

    pub fn show_modal(&mut self) {
        self.enable_modal.set(true);
    }

    pub fn cancel_modal(&mut self) {
        self.enable_modal.set(false);
    }

    pub fn clicked_cancel_button(&mut self) {
        let mut attributes = vec![];

        for attribute in (self.attributes)() {
            let name = attribute.name.clone();
            let value = attribute.value.clone();

            attributes.push(SelectAttribute {
                id: attribute.id,
                name: name.clone(),
                value: value.clone(),
                is_stored: attribute.is_stored,
                is_search: false,
                is_selected: false,
            });
        }

        self.selected_attributes.set(vec![]);
        self.attributes.set(attributes);
        self.cancel_modal();
    }

    pub fn clicked_save_button(&mut self) {
        let mut attributes = vec![];
        let mut search_attributes = (self.search_attributes)();

        for attribute in (self.attributes)() {
            let name = attribute.name.clone();
            let value = attribute.value.clone();

            if attribute.is_search {
                search_attributes.push(SelectAttribute {
                    is_search: false,
                    is_selected: false,
                    is_stored: true,
                    ..attribute
                });

                attributes.push(SelectAttribute {
                    id: attribute.id,
                    name,
                    value,
                    is_stored: true,
                    is_search: false,
                    is_selected: false,
                });
            } else {
                attributes.push(SelectAttribute {
                    id: attribute.id,
                    name,
                    value,
                    is_stored: attribute.is_stored,
                    is_search: false,
                    is_selected: false,
                });
            }
        }

        self.selected_attributes.set(vec![]);
        self.attributes.set(attributes);
        self.search_attributes.set(search_attributes);
        self.cancel_modal();
    }

    pub fn clicked_add_button(&mut self) {
        let mut selected_attributes = vec![];
        let mut attributes = vec![];

        for attribute in (self.attributes)() {
            let attr = if attribute.is_selected || attribute.is_search {
                let att = SelectAttribute {
                    is_search: true,
                    is_selected: false,
                    ..attribute
                };
                selected_attributes.push(att.clone());
                att
            } else {
                SelectAttribute { ..attribute }
            };

            attributes.push(attr);
        }

        self.selected_attributes.set(selected_attributes);
        self.attributes.set(attributes);
    }

    pub fn change_attribute_selected(&mut self, index: usize, selected: bool) {
        let mut attributes = (self.attributes)();
        let attribute = attributes.get(index).unwrap().clone();
        attributes[index] = SelectAttribute {
            is_selected: selected,
            ..attribute
        };
        self.attributes.set(attributes);
    }
}
