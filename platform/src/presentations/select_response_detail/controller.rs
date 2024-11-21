#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::api::v1::surveys::GetSurveyResponse;

//FIXME: move to model file
#[derive(Debug, Clone, PartialEq)]
pub struct PanelGroup {
    country: String,
    gender: String,
    age: String,
    occupation: String,
    value: u64,
}

//FIXME: move to model file
#[derive(Debug, Clone, PartialEq)]
pub struct Panel {
    country: String,
    gender: String,
    age: String,
    occupation: String,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Controller {
    response_count: Signal<String>,
    survey_response: Signal<GetSurveyResponse>,
    panel_groups: Signal<Vec<PanelGroup>>,
    panels: Signal<Vec<Panel>>,
    select_panel_groups: Signal<Vec<u64>>,
    select_panels: Signal<Vec<u64>>,
}

impl Controller {
    pub fn init() -> Self {
        let ctrl = Self {
            response_count: use_signal(|| "0".to_string()),
            survey_response: use_signal(|| GetSurveyResponse::default()),
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
            select_panel_groups: use_signal(|| vec![]),
            select_panels: use_signal(|| vec![]),
        };

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

    pub fn get_select_panel_groups(&self) -> Vec<u64> {
        (self.select_panel_groups)()
    }

    pub fn get_select_panels(&self) -> Vec<u64> {
        (self.select_panels)()
    }

    pub fn change_select_panel_groups(&mut self, ind: u64, value: bool) {
        let mut groups = self.get_select_panel_groups();
        if value {
            groups.push(ind);
        } else {
            let ind = groups.iter().position(|x| *x == ind).unwrap();
            groups.remove(ind);
        }
        self.select_panel_groups.set(groups);
    }

    pub fn change_select_panels(&mut self, ind: u64, value: bool) {
        let mut panels = self.get_select_panels();
        if value {
            panels.push(ind);
        } else {
            let ind = panels.iter().position(|x| *x == ind).unwrap();
            panels.remove(ind);
        }
        self.select_panels.set(panels);
    }

    pub fn get_panel_groups(&self) -> Vec<PanelGroup> {
        (self.panel_groups)()
    }

    pub fn get_panels(&self) -> Vec<Panel> {
        (self.panels)()
    }

    pub fn get_title(&self) -> String {
        self.get_survey().survey.title.clone()
    }

    pub fn get_survey(&self) -> GetSurveyResponse {
        (self.survey_response)()
    }

    pub fn get_response_count(&mut self) -> String {
        (self.response_count)()
    }

    pub fn set_response_count(&mut self, response_count: String) {
        self.response_count.set(response_count);
    }
}
