#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::models::pi::PiChart;

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Controller {
    pub select_page: Signal<SelectPage>,
    pub panels: Signal<Vec<Response>>,
    pub clicked_index: Signal<usize>,
    pub attributes: Signal<Vec<Attributes>>,
    pub surveys: Signal<Vec<Surveys>>,
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
    pub response_status: ResponseStatus,
    pub final_update_date: String,
    pub inprogress_time: String,
    pub attribute: Vec<String>,
}

impl Controller {
    pub fn init() -> Self {
        let mut ctrl = Self {
            select_page: use_signal(|| SelectPage::Summary),
            clicked_index: use_signal(|| 0),
            attributes: use_signal(|| {
                vec![
                    Attributes {
                        label: "연봉".to_string(),
                        chart_datas: vec![
                            PiChart {
                                label: "2000만원 이하",
                                percentage: 0.4,
                                color: "#5778a3",
                            },
                            PiChart {
                                label: "2000만원~4000만원",
                                percentage: 0.2,
                                color: "#a8c9e5",
                            },
                            PiChart {
                                label: "4000만원~6000만원",
                                percentage: 0.1,
                                color: "#e49343",
                            },
                            PiChart {
                                label: "6000만원~8000만원",
                                percentage: 0.1,
                                color: "#f5c086",
                            },
                            PiChart {
                                label: "8000만원 이상",
                                percentage: 0.2,
                                color: "#6b9f59",
                            },
                            // PiChart {
                            //     label: "1억 이상",
                            //     percentage: 0.1,
                            //     color: "#9ccf85",
                            // },
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
                                label: "기타",
                                percentage: 0.2,
                                color: "#e49343",
                            },
                        ],
                    },
                    Attributes {
                        label: "연령".to_string(),
                        chart_datas: vec![
                            PiChart {
                                label: "20대",
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
            }),
            surveys: use_signal(|| {
                vec![
                    Surveys {
                        title: "해당 서비스를 얼마나 자주 이용하시나요?".to_string(),
                        answer: 5,
                        skipped_answer: 0,
                        labels: vec![
                            "매우 만족".to_string(),
                            "만족".to_string(),
                            "보통".to_string(),
                            "나쁨".to_string(),
                            "매우 나쁨".to_string(),
                        ],
                        value_percents: vec![20.0, 80.0, 0.0, 0.0, 0.0],
                        colors: vec![
                            "#34D399".to_string(),
                            "#3B82F6".to_string(),
                            "#FBBF24".to_string(),
                            "#F87171".to_string(),
                            "#EF4444".to_string(),
                        ],
                        value_counts: vec![1, 4, 0, 0, 0],
                    },
                    Surveys {
                        title: "서비스의 접근성은 만족스러웠나요?".to_string(),
                        answer: 5,
                        skipped_answer: 0,
                        labels: vec![
                            "매우 만족".to_string(),
                            "만족".to_string(),
                            "보통".to_string(),
                            "나쁨".to_string(),
                            "매우 나쁨".to_string(),
                        ],
                        value_percents: vec![0.0, 60.0, 40.0, 0.0, 0.0],
                        colors: vec![
                            "#34D399".to_string(),
                            "#3B82F6".to_string(),
                            "#FBBF24".to_string(),
                            "#F87171".to_string(),
                            "#EF4444".to_string(),
                        ],
                        value_counts: vec![0, 3, 2, 0, 0],
                    },
                    Surveys {
                        title: "서비스를 이용하는 과정에서 불편함을 겪으셨나요?".to_string(),
                        answer: 5,
                        skipped_answer: 0,
                        labels: vec![
                            "매우 만족".to_string(),
                            "만족".to_string(),
                            "보통".to_string(),
                            "나쁨".to_string(),
                            "매우 나쁨".to_string(),
                        ],
                        value_percents: vec![20.0, 40.0, 40.0, 0.0, 0.0],
                        colors: vec![
                            "#34D399".to_string(),
                            "#3B82F6".to_string(),
                            "#FBBF24".to_string(),
                            "#F87171".to_string(),
                            "#EF4444".to_string(),
                        ],
                        value_counts: vec![1, 2, 2, 0, 0],
                    },
                ]
            }),
            panels: use_signal(|| {
                vec![
                    Response {
                        response_type: ResponseType::AttributeResponse,
                        response_status: ResponseStatus::NotProgress,
                        final_update_date: "2024-09-09".to_string(),
                        inprogress_time: "10m30s".to_string(),
                        attribute: vec![
                            "2000만원 이하".to_string(),
                            "서울".to_string(),
                            "남성".to_string(),
                            "20대".to_string(),
                        ],
                    },
                    Response {
                        response_type: ResponseType::AttributeResponse,
                        response_status: ResponseStatus::NotProgress,
                        final_update_date: "2024-09-09".to_string(),
                        inprogress_time: "10m30s".to_string(),
                        attribute: vec![
                            "2000만원 이하".to_string(),
                            "부산".to_string(),
                            "여성".to_string(),
                            "30대".to_string(),
                        ],
                    },
                    Response {
                        response_type: ResponseType::AttributeResponse,
                        response_status: ResponseStatus::InProgress,
                        final_update_date: "2024-09-09".to_string(),
                        inprogress_time: "10m30s".to_string(),
                        attribute: vec![
                            "2000만원~4000만원".to_string(),
                            "부산".to_string(),
                            "남성".to_string(),
                            "40대".to_string(),
                        ],
                    },
                    Response {
                        response_type: ResponseType::AttributeResponse,
                        response_status: ResponseStatus::InProgress,
                        final_update_date: "2024-09-09".to_string(),
                        inprogress_time: "10m30s".to_string(),
                        attribute: vec![
                            "2000만원~4000만원".to_string(),
                            "서울".to_string(),
                            "남성".to_string(),
                            "20대".to_string(),
                        ],
                    },
                    Response {
                        response_type: ResponseType::AttributeResponse,
                        response_status: ResponseStatus::InProgress,
                        final_update_date: "2024-09-09".to_string(),
                        inprogress_time: "10m30s".to_string(),
                        attribute: vec![
                            "8000만원 이상".to_string(),
                            "기타".to_string(),
                            "여성".to_string(),
                            "40대".to_string(),
                        ],
                    },
                    Response {
                        response_type: ResponseType::AttributeResponse,
                        response_status: ResponseStatus::Finished,
                        final_update_date: "2024-09-09".to_string(),
                        inprogress_time: "10m30s".to_string(),
                        attribute: vec![
                            "4000만원~6000만원".to_string(),
                            "기타".to_string(),
                            "남성".to_string(),
                            "20대".to_string(),
                        ],
                    },
                    Response {
                        response_type: ResponseType::AttributeResponse,
                        response_status: ResponseStatus::Finished,
                        final_update_date: "2024-09-09".to_string(),
                        inprogress_time: "10m30s".to_string(),
                        attribute: vec![
                            "6000만원~8000만원".to_string(),
                            "서울".to_string(),
                            "남성".to_string(),
                            "40대".to_string(),
                        ],
                    },
                    Response {
                        response_type: ResponseType::AttributeResponse,
                        response_status: ResponseStatus::Finished,
                        final_update_date: "2024-09-09".to_string(),
                        inprogress_time: "10m30s".to_string(),
                        attribute: vec![
                            "8000만원 이상".to_string(),
                            "서울".to_string(),
                            "남성".to_string(),
                            "20대".to_string(),
                        ],
                    },
                    Response {
                        response_type: ResponseType::AttributeResponse,
                        response_status: ResponseStatus::Finished,
                        final_update_date: "2024-09-09".to_string(),
                        inprogress_time: "10m30s".to_string(),
                        attribute: vec![
                            "2000만원 이하".to_string(),
                            "서울".to_string(),
                            "남성".to_string(),
                            "20대".to_string(),
                        ],
                    },
                    Response {
                        response_type: ResponseType::AttributeResponse,
                        response_status: ResponseStatus::Finished,
                        final_update_date: "2024-09-09".to_string(),
                        inprogress_time: "10m30s".to_string(),
                        attribute: vec![
                            "2000만원 이하".to_string(),
                            "서울".to_string(),
                            "남성".to_string(),
                            "20대".to_string(),
                        ],
                    },
                ]
            }),
        };
        ctrl.clicked_index.set(ctrl.get_panels().len());
        use_context_provider(|| ctrl);
        ctrl
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
