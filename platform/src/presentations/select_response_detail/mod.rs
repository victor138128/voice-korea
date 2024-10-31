#![allow(non_snake_case)]
use crate::prelude::*;
use component::select_attribute::SelectAttributePage;
use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct SelectResponseDetailProps {
    lang: Language,
    title: String,
    select_type: String,
}

pub mod controller;
pub mod i18n;
pub mod component {
    pub mod select_attribute;
}

#[component]
pub fn SelectResponseDetailPage(props: SelectResponseDetailProps) -> Element {
    let ctrl = controller::Controller::init();
    let translates = i18n::translate(props.lang.clone());

    rsx! {
        SelectAttributePage {
            ctrl,
            lang: props.lang,
            title: props.title,
            select_type: props.select_type,

            temporary_save: translates.temporary_save,
            attribute_title: translates.attribute_title,
            attribute_description: translates.attribute_description,
            attribute_select_label: translates.attribute_select_label,
            nation: translates.nation,
            gender: translates.gender,
            age: translates.age,
            add_attribute: translates.add_attribute,
            cancel: translates.cancel,
            save: translates.save,
        }
    }
}
