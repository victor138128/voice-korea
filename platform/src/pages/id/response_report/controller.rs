#![allow(non_snake_case)]
use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Controller {
    pub select_page: Signal<SelectPage>,
    pub panels: Signal<Vec<Response>>,
    pub clicked_index: Signal<usize>,
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
