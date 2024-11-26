#![allow(non_snake_case)]
use dioxus::prelude::*;

use crate::models::pi::PiChart;

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Controller {
    pub select_page: Signal<SelectPage>,
    pub panels: Signal<Vec<Response>>,
    pub clicked_index: Signal<usize>,
    pub attributes: Signal<Vec<Attributes>>,
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
                        label: "연령".to_string(),
                        chart_datas: vec![
                            PiChart {
                                label: "20~30대",
                                percentage: 0.6,
                                color: "#5778a3",
                            },
                            PiChart {
                                label: "30~40대",
                                percentage: 0.13,
                                color: "#a8c9e5",
                            },
                            PiChart {
                                label: "40~50대",
                                percentage: 0.13,
                                color: "#e49343",
                            },
                            PiChart {
                                label: "50~60대",
                                percentage: 0.06,
                                color: "#f5c086",
                            },
                            PiChart {
                                label: "60~70대",
                                percentage: 0.03,
                                color: "#6b9f59",
                            },
                            PiChart {
                                label: "70대 이상",
                                percentage: 0.03,
                                color: "#9ccf85",
                            },
                        ],
                    },
                    Attributes {
                        label: "성별".to_string(),
                        chart_datas: vec![
                            PiChart {
                                label: "남성",
                                percentage: 0.5,
                                color: "#5778a3",
                            },
                            PiChart {
                                label: "여성",
                                percentage: 0.3,
                                color: "#a8c9e5",
                            },
                            PiChart {
                                label: "모름",
                                percentage: 0.2,
                                color: "#e49343",
                            },
                        ],
                    },
                    Attributes {
                        label: "직업".to_string(),
                        chart_datas: vec![
                            PiChart {
                                label: "자영업",
                                percentage: 0.6,
                                color: "#5778a3",
                            },
                            PiChart {
                                label: "사무직",
                                percentage: 0.13,
                                color: "#a8c9e5",
                            },
                            PiChart {
                                label: "전문직",
                                percentage: 0.13,
                                color: "#e49343",
                            },
                            PiChart {
                                label: "현장직",
                                percentage: 0.06,
                                color: "#f5c086",
                            },
                            PiChart {
                                label: "무직",
                                percentage: 0.03,
                                color: "#6b9f59",
                            },
                            PiChart {
                                label: "기타",
                                percentage: 0.03,
                                color: "#9ccf85",
                            },
                        ],
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
                            "대한민국".to_string(),
                            "남성".to_string(),
                            "30대".to_string(),
                            "자영업".to_string(),
                        ],
                    },
                    Response {
                        response_type: ResponseType::AttributeResponse,
                        response_status: ResponseStatus::NotProgress,
                        final_update_date: "2024-09-09".to_string(),
                        inprogress_time: "10m30s".to_string(),
                        attribute: vec![
                            "대한민국".to_string(),
                            "남성".to_string(),
                            "30대".to_string(),
                            "자영업".to_string(),
                        ],
                    },
                    Response {
                        response_type: ResponseType::AttributeResponse,
                        response_status: ResponseStatus::InProgress,
                        final_update_date: "2024-09-09".to_string(),
                        inprogress_time: "10m30s".to_string(),
                        attribute: vec![
                            "대한민국".to_string(),
                            "남성".to_string(),
                            "30대".to_string(),
                            "자영업".to_string(),
                        ],
                    },
                    Response {
                        response_type: ResponseType::AttributeResponse,
                        response_status: ResponseStatus::InProgress,
                        final_update_date: "2024-09-09".to_string(),
                        inprogress_time: "10m30s".to_string(),
                        attribute: vec![
                            "대한민국".to_string(),
                            "남성".to_string(),
                            "30대".to_string(),
                            "자영업".to_string(),
                        ],
                    },
                    Response {
                        response_type: ResponseType::AttributeResponse,
                        response_status: ResponseStatus::InProgress,
                        final_update_date: "2024-09-09".to_string(),
                        inprogress_time: "10m30s".to_string(),
                        attribute: vec![
                            "대한민국".to_string(),
                            "남성".to_string(),
                            "30대".to_string(),
                            "자영업".to_string(),
                        ],
                    },
                    Response {
                        response_type: ResponseType::AttributeResponse,
                        response_status: ResponseStatus::Finished,
                        final_update_date: "2024-09-09".to_string(),
                        inprogress_time: "10m30s".to_string(),
                        attribute: vec![
                            "대한민국".to_string(),
                            "남성".to_string(),
                            "30대".to_string(),
                            "자영업".to_string(),
                        ],
                    },
                    Response {
                        response_type: ResponseType::AttributeResponse,
                        response_status: ResponseStatus::Finished,
                        final_update_date: "2024-09-09".to_string(),
                        inprogress_time: "10m30s".to_string(),
                        attribute: vec![
                            "대한민국".to_string(),
                            "남성".to_string(),
                            "30대".to_string(),
                            "자영업".to_string(),
                        ],
                    },
                    Response {
                        response_type: ResponseType::AttributeResponse,
                        response_status: ResponseStatus::Finished,
                        final_update_date: "2024-09-09".to_string(),
                        inprogress_time: "10m30s".to_string(),
                        attribute: vec![
                            "대한민국".to_string(),
                            "남성".to_string(),
                            "30대".to_string(),
                            "자영업".to_string(),
                        ],
                    },
                    Response {
                        response_type: ResponseType::AttributeResponse,
                        response_status: ResponseStatus::Finished,
                        final_update_date: "2024-09-09".to_string(),
                        inprogress_time: "10m30s".to_string(),
                        attribute: vec![
                            "대한민국".to_string(),
                            "남성".to_string(),
                            "30대".to_string(),
                            "자영업".to_string(),
                        ],
                    },
                    Response {
                        response_type: ResponseType::AttributeResponse,
                        response_status: ResponseStatus::Finished,
                        final_update_date: "2024-09-09".to_string(),
                        inprogress_time: "10m30s".to_string(),
                        attribute: vec![
                            "대한민국".to_string(),
                            "남성".to_string(),
                            "30대".to_string(),
                            "자영업".to_string(),
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
