#![allow(non_snake_case)]
#[allow(unused_imports)]
use std::collections::HashMap;

use dioxus::prelude::*;
#[allow(unused_imports)]
use dioxus_logger::tracing;
use models::prelude::SurveyResultDocument;

use crate::{api::v2::survey::get_survey_result, models::pi::PiChart, prelude::Language};

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Controller {
    pub select_page: Signal<SelectPage>,
    pub panels: Signal<Vec<Response>>,
    pub clicked_index: Signal<usize>,
    pub attributes: Signal<Vec<Attributes>>,
    pub surveys: Signal<Vec<Surveys>>,

    pub survey_response: Signal<Resource<models::prelude::SurveyResultDocument>>,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum SelectPage {
    Summary,
    Response,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum ResponseType {
    AttributeResponse,
    NoneResponse,
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum ResponseStatus {
    NotProgress,
    InProgress,
    Finished,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Attributes {
    pub label: String,
    pub chart_datas: Vec<PiChart>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Surveys {
    pub title: String,
    pub answer: u64,
    pub skipped_answer: u64,
    pub labels: Vec<String>,
    pub value_percents: Vec<f32>,
    pub colors: Vec<String>,
    pub value_counts: Vec<u64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Response {
    pub response_type: ResponseType,
    pub response_status: Option<ResponseStatus>,
    pub final_update_date: String,
    pub inprogress_time: String,
    pub attribute: Vec<String>,
}

impl Controller {
    #[allow(unused_variables)]
    pub fn init(lang: Language, survey_id: String) -> Self {
        #[cfg(feature = "web")]
        {
            use crate::routes::Route;
            use crate::service::login_service::use_login_service;

            let navigator = use_navigator();
            let token = use_login_service().get_cookie_value();
            if token.is_none() {
                navigator.push(Route::LoginPage { lang });
            }
        }

        let res = use_resource(move || {
            let value = survey_id.clone();
            async move {
                match get_survey_result(value).await {
                    Ok(res) => {
                        tracing::debug!("this line come: {:?}", res);
                        res
                    }
                    Err(e) => {
                        tracing::error!("get survey failed: {:?}", e);
                        SurveyResultDocument::default()
                    }
                }
            }
        });

        let mut ctrl = Self {
            select_page: use_signal(|| SelectPage::Summary),
            clicked_index: use_signal(|| 0),
            attributes: use_signal(|| vec![]),
            surveys: use_signal(|| vec![]),
            panels: use_signal(|| vec![]),
            survey_response: use_signal(|| res),
        };

        ctrl.survey_response.set(res);

        let attributes = ctrl.get_attributes_chart();
        let surveys = ctrl.get_surveys_chart();
        let panels = ctrl.get_panels_chart();
        ctrl.attributes.set(attributes);
        ctrl.surveys.set(surveys);
        ctrl.panels.set(panels);
        ctrl.clicked_index.set(ctrl.get_panels().len());

        use_context_provider(|| ctrl);
        ctrl
    }

    pub fn get_survey_response(&self) -> Option<SurveyResultDocument> {
        ((self.survey_response)().value())()
    }

    pub fn get_panels_chart(&self) -> Vec<Response> {
        vec![
            Response {
                response_type: ResponseType::AttributeResponse,
                response_status: None,
                final_update_date: "2024-09-09".to_string(),
                inprogress_time: "10m30s".to_string(),
                attribute: vec![
                    "2400만원 이하".to_string(),
                    "서울".to_string(),
                    "남성".to_string(),
                    "18~29세".to_string(),
                ],
            },
            Response {
                response_type: ResponseType::AttributeResponse,
                response_status: None,
                final_update_date: "2024-09-09".to_string(),
                inprogress_time: "10m30s".to_string(),
                attribute: vec![
                    "2400만원 이하".to_string(),
                    "부산".to_string(),
                    "여성".to_string(),
                    "30대".to_string(),
                ],
            },
            Response {
                response_type: ResponseType::AttributeResponse,
                response_status: None,
                final_update_date: "2024-09-09".to_string(),
                inprogress_time: "10m30s".to_string(),
                attribute: vec![
                    "2400만원~5000만원".to_string(),
                    "부산".to_string(),
                    "남성".to_string(),
                    "40대".to_string(),
                ],
            },
            Response {
                response_type: ResponseType::AttributeResponse,
                response_status: None,
                final_update_date: "2024-09-09".to_string(),
                inprogress_time: "10m30s".to_string(),
                attribute: vec![
                    "2400만원~5000만원".to_string(),
                    "서울".to_string(),
                    "남성".to_string(),
                    "18~29세".to_string(),
                ],
            },
            Response {
                response_type: ResponseType::AttributeResponse,
                response_status: None,
                final_update_date: "2024-09-09".to_string(),
                inprogress_time: "10m30s".to_string(),
                attribute: vec![
                    "10000만원 이상".to_string(),
                    "대구".to_string(),
                    "여성".to_string(),
                    "40대".to_string(),
                ],
            },
            Response {
                response_type: ResponseType::AttributeResponse,
                response_status: None,
                final_update_date: "2024-09-09".to_string(),
                inprogress_time: "10m30s".to_string(),
                attribute: vec![
                    "5000만원~8000만원".to_string(),
                    "대구".to_string(),
                    "남성".to_string(),
                    "18~29세".to_string(),
                ],
            },
            Response {
                response_type: ResponseType::AttributeResponse,
                response_status: None,
                final_update_date: "2024-09-09".to_string(),
                inprogress_time: "10m30s".to_string(),
                attribute: vec![
                    "5000만원~8000만원".to_string(),
                    "서울".to_string(),
                    "남성".to_string(),
                    "40대".to_string(),
                ],
            },
            Response {
                response_type: ResponseType::AttributeResponse,
                response_status: None,
                final_update_date: "2024-09-09".to_string(),
                inprogress_time: "10m30s".to_string(),
                attribute: vec![
                    "10000만원 이상".to_string(),
                    "서울".to_string(),
                    "남성".to_string(),
                    "18~29세".to_string(),
                ],
            },
            Response {
                response_type: ResponseType::AttributeResponse,
                response_status: None,
                final_update_date: "2024-09-09".to_string(),
                inprogress_time: "10m30s".to_string(),
                attribute: vec![
                    "2400만원 이하".to_string(),
                    "서울".to_string(),
                    "남성".to_string(),
                    "18~29세".to_string(),
                ],
            },
            Response {
                response_type: ResponseType::AttributeResponse,
                response_status: None,
                final_update_date: "2024-09-09".to_string(),
                inprogress_time: "10m30s".to_string(),
                attribute: vec![
                    "2400만원 이하".to_string(),
                    "서울".to_string(),
                    "남성".to_string(),
                    "18~29세".to_string(),
                ],
            },
        ]
    }

    pub fn get_surveys_chart(&self) -> Vec<Surveys> {
        let default_colors = vec![
            "#34D399".to_string(),
            "#3B82F6".to_string(),
            "#FBBF24".to_string(),
            "#F87171".to_string(),
            "#EF4444".to_string(),
        ];
        let mut surveys: Vec<Surveys> = vec![];
        let survey = self.get_survey_response();

        if survey.is_none() {
            return vec![];
        }

        let questions = survey.clone().unwrap().questions;

        for (key, value) in questions {
            let s = survey.clone().unwrap().clone();
            let survey_response_by_question = s.survey_responses_by_question.get(&key);
            let mut values = vec![];
            let mut value_percent: Vec<f32> = vec![];
            let mut colors: Vec<String> = vec![];

            for _i in 0..value.options.clone().unwrap_or(vec![]).len() {
                values.push(0);
                value_percent.push(0.0);
            }

            let mut total_response = 0;

            if let Some(survey_response) = survey_response_by_question {
                for (i, option) in value.options.clone().unwrap_or(vec![]).iter().enumerate() {
                    if !survey_response_by_question.clone().is_none() {
                        let option_count = survey_response.get(option).copied().unwrap_or_default();

                        if let Some(count) = (option_count).try_into().ok() {
                            values[i] = count;
                            total_response += count;
                        }
                    }
                }

                if total_response == 0 {
                    values.clear();
                    value_percent.clear();
                }

                for (i, value) in values.iter().enumerate() {
                    if total_response == 0 {
                        value_percent[i] = 0.0;
                    } else {
                        value_percent[i] = (*value as f32) / (total_response as f32) * 100.0;
                    }

                    let ind = i % 5;
                    let color = default_colors[ind].clone();
                    colors.push(color);
                }

                surveys.push(Surveys {
                    title: value.title,
                    answer: if survey_response_by_question.is_some() {
                        if total_response == 0 {
                            survey_response_by_question.unwrap().keys().len() as u64
                        } else {
                            total_response
                        }
                    } else {
                        0
                    },
                    skipped_answer: 0,
                    labels: if !value.options.clone().is_none() {
                        value.options.unwrap()
                    } else {
                        survey_response_by_question
                            .map(|map| map.keys().cloned().collect::<Vec<_>>()) // Some인 경우 키를 벡터로 수집
                            .unwrap_or_else(Vec::new)
                    },
                    value_percents: value_percent,
                    colors,
                    value_counts: values,
                });
            } else {
                surveys.push(Surveys {
                    title: value.title,
                    answer: 0,
                    skipped_answer: 0,
                    labels: if !value.options.clone().is_none() {
                        value.options.unwrap()
                    } else {
                        vec![]
                    },
                    value_percents: value_percent,
                    colors,
                    value_counts: values,
                });
            }
        }

        surveys
    }

    pub fn get_attributes_chart(&self) -> Vec<Attributes> {
        vec![
            Attributes {
                label: "연봉".to_string(),
                chart_datas: vec![
                    PiChart {
                        label: "2400만원 이하",
                        percentage: 0.4,
                        color: "#5778a3",
                    },
                    PiChart {
                        label: "2400만원~5000만원",
                        percentage: 0.2,
                        color: "#a8c9e5",
                    },
                    PiChart {
                        label: "5000만원~8000만원",
                        percentage: 0.1,
                        color: "#e49343",
                    },
                    PiChart {
                        label: "8000만원~10000만원",
                        percentage: 0.1,
                        color: "#f5c086",
                    },
                    PiChart {
                        label: "10000만원 이상",
                        percentage: 0.2,
                        color: "#6b9f59",
                    },
                ],
            },
            Attributes {
                label: "성별".to_string(),
                chart_datas: vec![
                    PiChart {
                        label: "남성",
                        percentage: 0.8,
                        color: "#5778a3",
                    },
                    PiChart {
                        label: "여성",
                        percentage: 0.2,
                        color: "#a8c9e5",
                    },
                ],
            },
            Attributes {
                label: "지역".to_string(),
                chart_datas: vec![
                    PiChart {
                        label: "서울",
                        percentage: 0.6,
                        color: "#5778a3",
                    },
                    PiChart {
                        label: "부산",
                        percentage: 0.2,
                        color: "#a8c9e5",
                    },
                    PiChart {
                        label: "대구",
                        percentage: 0.2,
                        color: "#e49343",
                    },
                ],
            },
            Attributes {
                label: "연령".to_string(),
                chart_datas: vec![
                    PiChart {
                        label: "18~29세",
                        percentage: 0.6,
                        color: "#5778a3",
                    },
                    PiChart {
                        label: "30대",
                        percentage: 0.1,
                        color: "#a8c9e5",
                    },
                    PiChart {
                        label: "40대",
                        percentage: 0.3,
                        color: "#e49343",
                    },
                ],
            },
        ]
    }

    pub fn get_attributes(&self) -> Vec<Attributes> {
        (self.attributes)()
    }

    pub fn get_surveys(&self) -> Vec<Surveys> {
        (self.surveys)()
    }

    pub fn get_clicked_index(&self) -> usize {
        (self.clicked_index)()
    }

    pub fn change_clicked_index(&mut self, index: usize) {
        self.clicked_index.set(index);
    }

    pub fn get_panels(&self) -> Vec<Response> {
        (self.panels)()
    }

    pub fn change_select_page(&mut self, select_page: SelectPage) {
        self.select_page.set(select_page);
    }

    pub fn get_select_page(&self) -> SelectPage {
        (self.select_page)()
    }
}

#[allow(dead_code)]
pub fn use_controller() -> Controller {
    use_context()
}
