#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::{api::v1::surveys::GetSurveyResponse, service::login_service::use_login_service};

use super::{Language, Route};

//FIXME: move to model file
#[derive(Debug, Clone, PartialEq)]
pub struct PanelGroup {
    pub country: String,
    pub gender: String,
    pub age: String,
    pub occupation: String,
    pub value: u64,
}

//FIXME: move to model file
#[derive(Debug, Clone, PartialEq)]
pub struct Panel {
    pub country: String,
    pub gender: String,
    pub age: String,
    pub occupation: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SelectAttribute {
    pub id: usize,
    pub name: String,
    pub value: Vec<String>,
    pub initial_value: String,
    pub is_stored: bool,   //저장 되었는지 유무
    pub is_search: bool,   //검색 되었는지 유무
    pub is_selected: bool, //선택 되었는지 유무
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Controller {
    response_count: Signal<String>,
    pub bar_index: Signal<usize>,
    survey_response: Resource<GetSurveyResponse>,
    panel_groups: Signal<Vec<PanelGroup>>,
    panels: Signal<Vec<Panel>>,
    select_panel_groups: Signal<Vec<u64>>,
    select_panels: Signal<Vec<u64>>,
    total_select_count: Signal<u64>,
    click_total_check: Signal<bool>,
    show_add_attribute_modal: Signal<bool>,
    step: Signal<Step>,
    attributes: Signal<Vec<SelectAttribute>>,
    pub selected_attributes: Signal<Vec<SelectAttribute>>,
    pub search_attributes: Signal<Vec<SelectAttribute>>,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Step {
    Attribute,
    Panel,
}

impl Controller {
    pub fn init(lang: Language, id: String) -> Self {
        let navigator = use_navigator();
        let email: String = use_login_service().get_email().clone();

        if email.is_empty() {
            navigator.push(Route::LoginPage { lang });
        };

        let survey_response = use_resource(move || {
            let id_value = id.clone();
            let email_value = email.clone();
            async move {
                crate::utils::api::get::<GetSurveyResponse>(&format!(
                    "/v1/email/{}/surveys/{}",
                    email_value, id_value
                ))
                .await
            }
        });

        let ctrl = Self {
            response_count: use_signal(|| "0".to_string()),
            survey_response,
            panel_groups: use_signal(|| {
                vec![
                    PanelGroup {
                        country: "대한민국(Korea)".to_string(),
                        gender: "남성".to_string(),
                        age: "30대".to_string(),
                        occupation: "사무직".to_string(),
                        value: 50,
                    },
                    PanelGroup {
                        country: "대한민국(Korea)".to_string(),
                        gender: "남성".to_string(),
                        age: "30대".to_string(),
                        occupation: "자영업자".to_string(),
                        value: 50,
                    },
                    PanelGroup {
                        country: "대한민국(Korea)".to_string(),
                        gender: "남성".to_string(),
                        age: "30대".to_string(),
                        occupation: "무직".to_string(),
                        value: 50,
                    },
                    PanelGroup {
                        country: "대한민국(Korea)".to_string(),
                        gender: "남성".to_string(),
                        age: "30대".to_string(),
                        occupation: "전문직".to_string(),
                        value: 50,
                    },
                    PanelGroup {
                        country: "대한민국(Korea)".to_string(),
                        gender: "남성".to_string(),
                        age: "30대".to_string(),
                        occupation: "현장직".to_string(),
                        value: 50,
                    },
                ]
            }), //FIXME: fix to get api
            panels: use_signal(|| {
                vec![
                    Panel {
                        country: "대한민국(Korea)".to_string(),
                        gender: "남성".to_string(),
                        age: "30대".to_string(),
                        occupation: "사무직".to_string(),
                    },
                    Panel {
                        country: "대한민국(Korea)".to_string(),
                        gender: "남성".to_string(),
                        age: "30대".to_string(),
                        occupation: "자영업자".to_string(),
                    },
                    Panel {
                        country: "대한민국(Korea)".to_string(),
                        gender: "남성".to_string(),
                        age: "30대".to_string(),
                        occupation: "무직".to_string(),
                    },
                    Panel {
                        country: "대한민국(Korea)".to_string(),
                        gender: "남성".to_string(),
                        age: "30대".to_string(),
                        occupation: "전문직".to_string(),
                    },
                    Panel {
                        country: "대한민국(Korea)".to_string(),
                        gender: "남성".to_string(),
                        age: "30대".to_string(),
                        occupation: "현장직".to_string(),
                    },
                ]
            }), //FIXME: fix to get api
            attributes: use_signal(|| {
                vec![
                    SelectAttribute {
                        id: 0,
                        name: "연령".to_string(),
                        value: vec![
                            "10대".to_string(),
                            "20대".to_string(),
                            "30대".to_string(),
                            "40대".to_string(),
                            "50대".to_string(),
                            "60대 이상".to_string(),
                        ],
                        initial_value: "10대".to_string(),
                        is_stored: false,
                        is_search: false,
                        is_selected: false,
                    },
                    SelectAttribute {
                        id: 1,
                        name: "성별".to_string(),
                        value: vec!["남성".to_string(), "여성".to_string(), "중성".to_string()],
                        initial_value: "남성".to_string(),
                        is_stored: false,
                        is_search: false,
                        is_selected: false,
                    },
                    SelectAttribute {
                        id: 2,
                        name: "연봉".to_string(),
                        value: vec![
                            "2000만원 이하".to_string(),
                            "2000만원~4000만원".to_string(),
                            "4000만원~6000만원".to_string(),
                            "6000만원~8000만원".to_string(),
                            "8000만원 이상".to_string(),
                        ],
                        initial_value: "2000만원 이하".to_string(),
                        is_stored: false,
                        is_search: false,
                        is_selected: false,
                    },
                    SelectAttribute {
                        id: 3,
                        name: "지역".to_string(),
                        value: vec!["서울".to_string(), "부산".to_string(), "기타".to_string()],
                        initial_value: "서울".to_string(),
                        is_stored: false,
                        is_search: false,
                        is_selected: false,
                    },
                ]
            }),
            selected_attributes: use_signal(|| vec![]),
            search_attributes: use_signal(|| vec![]),
            select_panel_groups: use_signal(|| vec![]),
            select_panels: use_signal(|| vec![]),
            total_select_count: use_signal(|| 0),
            click_total_check: use_signal(|| false),
            show_add_attribute_modal: use_signal(|| false),
            step: use_signal(|| Step::Attribute),
            bar_index: use_signal(|| 0),
        };

        use_context_provider(|| ctrl);

        // let _ = use_effect(move || {
        //     spawn(async move {
        //         match get_survey().await {
        //             Ok(res) => {
        //                 ctrl.survey_response.set(res);
        //             }
        //             Err(e) => {
        //                 tracing::error!("Error: {:?}", e);
        //             }
        //         }
        //     });
        // });

        ctrl
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

    pub fn change_clicked_add_attribute(&mut self, clicked: bool) {
        self.show_add_attribute_modal.set(clicked);
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
                    initial_value: attribute.initial_value.clone(),
                    ..attribute
                });

                attributes.push(SelectAttribute {
                    id: attribute.id,
                    name,
                    value,
                    initial_value: attribute.initial_value.clone(),
                    is_stored: true,
                    is_search: false,
                    is_selected: false,
                });
            } else {
                attributes.push(SelectAttribute {
                    id: attribute.id,
                    name,
                    value,
                    initial_value: attribute.initial_value.clone(),
                    is_stored: attribute.is_stored,
                    is_search: false,
                    is_selected: false,
                });
            }
        }

        self.selected_attributes.set(vec![]);
        self.attributes.set(attributes);
        self.search_attributes.set(search_attributes);
        self.show_add_attribute_modal.set(false);
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
                initial_value: attribute.initial_value.clone(),
                is_stored: attribute.is_stored,
                is_search: false,
                is_selected: false,
            });
        }

        self.selected_attributes.set(vec![]);
        self.attributes.set(attributes);
        self.show_add_attribute_modal.set(false);
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

    pub fn get_clicked_add_attribute(&self) -> bool {
        (self.show_add_attribute_modal)()
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

    pub fn get_search_attributes(&self) -> Vec<SelectAttribute> {
        (self.search_attributes)()
    }

    // pub fn get_selected_attributes(&self) -> Vec<SelectAttribute> {
    //     (self.selected_attributes)()
    // }

    pub fn get_select_panel_groups(&self) -> Vec<u64> {
        (self.select_panel_groups)()
    }

    pub fn get_select_panels(&self) -> Vec<u64> {
        (self.select_panels)()
    }

    pub fn get_total_select_count(&self) -> u64 {
        (self.total_select_count)()
    }

    pub fn change_step(&mut self, step: Step) {
        self.step.set(step);
    }

    pub fn get_step(&self) -> Step {
        (self.step)()
    }

    pub fn change_select_panel_groups(&mut self, ind: u64, value: bool) {
        let mut groups = self.get_select_panel_groups();
        let panel_groups = self.get_panel_groups();
        if value {
            let panel = panel_groups.get(ind as usize).unwrap();
            groups.push(ind);
            self.total_select_count
                .set((self.total_select_count)() + panel.value);
        } else {
            let panel = panel_groups.get(ind as usize).unwrap();
            let ind = groups.iter().position(|x| *x == ind).unwrap();
            groups.remove(ind);
            self.total_select_count
                .set((self.total_select_count)() - panel.value);
        }
        self.select_panel_groups.set(groups);
    }

    pub fn clicked_panels_all(&mut self) {
        let check = (self.click_total_check)();
        let mut select_panels = vec![];
        let panels = self.get_panels();

        if !check {
            for (i, _panel) in panels.iter().enumerate() {
                select_panels.push(i as u64);
            }
        }

        self.select_panels.set(select_panels.clone());
        self.total_select_count
            .set(select_panels.clone().len() as u64);
        self.click_total_check.set(!check);
    }

    pub fn get_click_total_check(&self) -> bool {
        (self.click_total_check)()
    }

    pub fn change_select_panels(&mut self, ind: u64, value: bool) {
        let mut panels = self.get_select_panels();
        if value {
            panels.push(ind);
            self.total_select_count.set((self.total_select_count)() + 1);
        } else {
            let ind = panels.iter().position(|x| *x == ind).unwrap();
            panels.remove(ind);
            self.total_select_count.set((self.total_select_count)() - 1);
        }
        self.select_panels.set(panels);
    }

    pub fn get_panel_groups(&self) -> Vec<PanelGroup> {
        (self.panel_groups)()
    }

    pub fn get_panels(&self) -> Vec<Panel> {
        (self.panels)()
    }

    #[allow(dead_code)]
    pub fn get_title(&self) -> String {
        self.get_survey().survey.title.clone()
    }

    #[allow(dead_code)]
    pub fn get_survey(&self) -> GetSurveyResponse {
        match (self.survey_response.value())() {
            Some(value) => value,
            None => GetSurveyResponse::default(),
        }
    }

    pub fn get_response_count(&mut self) -> String {
        (self.response_count)()
    }

    pub fn set_response_count(&mut self, response_count: String) {
        self.response_count.set(response_count);
    }
}

#[allow(dead_code)]
pub fn use_controller() -> Controller {
    use_context()
}
