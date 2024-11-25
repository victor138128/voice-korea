#![allow(non_snake_case)]
use dioxus::prelude::*;
use models::prelude::{Age, Gender, Quota};

use crate::{
    api::v1::surveys::GetSurveyResponse, models::question::Question,
    service::login_service::use_login_service,
};

use chrono::{DateTime, Local, Utc};

use super::{Language, Route};

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Controller {
    survey: Resource<GetSurveyResponse>,
    summary_clicked: Signal<bool>,
    panel_clicked: Signal<bool>,
    survey_list_clicked: Signal<bool>,
    pub survey_id: Signal<String>,
    pub start_date: Signal<String>,
    pub end_date: Signal<String>,
    pub object_count: Signal<u64>,
    pub subject_count: Signal<u64>,
    pub total_count: Signal<u64>,

    pub total_panels: Signal<u64>,
    pub total_attributes: Signal<u64>,

    pub start_year: Signal<String>,
    pub start_month: Signal<String>,
    pub start_day: Signal<String>,

    pub end_year: Signal<String>,
    pub end_month: Signal<String>,
    pub end_day: Signal<String>,

    pub initialize: Signal<bool>,
}

impl Controller {
    pub fn init(lang: Language, id: String) -> Self {
        let navigator = use_navigator();
        let email: String = use_login_service().get_email().clone();

        if email.is_empty() {
            navigator.push(Route::LoginPage { lang });
        };

        let id_copy = id.clone();

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

        let mut ctrl = Self {
            survey: survey_response,
            summary_clicked: use_signal(|| false),
            panel_clicked: use_signal(|| false),
            survey_list_clicked: use_signal(|| false),
            survey_id: use_signal(|| "".to_string()),
            start_date: use_signal(|| "".to_string()),
            end_date: use_signal(|| "".to_string()),
            object_count: use_signal(|| 0),
            subject_count: use_signal(|| 0),
            total_count: use_signal(|| 0),

            total_attributes: use_signal(|| 0),
            total_panels: use_signal(|| 0),

            start_year: use_signal(|| "".to_string()),
            start_month: use_signal(|| "".to_string()),
            start_day: use_signal(|| "".to_string()),
            end_year: use_signal(|| "".to_string()),
            end_month: use_signal(|| "".to_string()),
            end_day: use_signal(|| "".to_string()),

            initialize: use_signal(|| false),
        };

        if !(ctrl.initialize)() {
            let current_date = Local::now().format("%Y-%m-%d").to_string();

            ctrl.set_date(Some(current_date.clone()), Some(current_date.clone()));

            ctrl.start_date.set(current_date.clone());
            ctrl.end_date.set(current_date.clone());

            ctrl.initialize.set(true);
        }

        let (object_count, subject_count, total_count) = ctrl.get_survey_count();
        let _ = ctrl.get_panel_info();
        ctrl.survey_id.set(id_copy.clone());
        ctrl.object_count.set(object_count);
        ctrl.subject_count.set(subject_count);
        ctrl.total_count.set(total_count);

        use_context_provider(|| ctrl);

        ctrl
    }

    pub fn set_date(&mut self, start_date: Option<String>, end_date: Option<String>) {
        match start_date {
            Some(s) => {
                let start: Vec<&str> = s.split("-").collect();
                self.start_year.set(start.get(0).unwrap().to_string());
                self.start_month.set(start.get(1).unwrap().to_string());
                self.start_day.set(start.get(2).unwrap().to_string());
            }
            None => {}
        };

        match end_date {
            Some(e) => {
                let end: Vec<&str> = e.split("-").collect();

                self.end_year.set(end.get(0).unwrap().to_string());
                self.end_month.set(end.get(1).unwrap().to_string());
                self.end_day.set(end.get(2).unwrap().to_string());
            }
            None => {}
        }
    }

    pub fn get_survey_status(&mut self) -> crate::models::survey::SurveyStatus {
        let survey: crate::models::survey::SurveyStatus = self.get_survey().survey.status;

        match survey {
            crate::models::survey::SurveyStatus::InProgress {
                started_at,
                ended_at,
            } => {
                let start_timestamp = DateTime::from_timestamp(started_at as i64, 0).unwrap();
                let end_timestamp = DateTime::from_timestamp(ended_at.unwrap() as i64, 0).unwrap();

                self.change_period(start_timestamp, end_timestamp);
            }
            crate::models::survey::SurveyStatus::Finished {
                started_at,
                ended_at,
            } => {
                let start_timestamp = DateTime::from_timestamp(started_at as i64, 0).unwrap();
                let end_timestamp = DateTime::from_timestamp(ended_at.unwrap() as i64, 0).unwrap();

                self.change_period(start_timestamp, end_timestamp);
            }
            _ => {}
        }

        survey
    }

    pub fn change_period(&mut self, start_timestamp: DateTime<Utc>, end_timestamp: DateTime<Utc>) {
        // yyyy-mm-dd 형식으로 변환
        let formatted_start_date = start_timestamp.format("%Y-%m-%d").to_string();
        let formatted_end_date = end_timestamp.format("%Y-%m-%d").to_string();

        let start: Vec<&str> = formatted_start_date.split("-").collect();
        self.start_year.set(start.get(0).unwrap().to_string());
        self.start_month.set(start.get(1).unwrap().to_string());
        self.start_day.set(start.get(2).unwrap().to_string());

        let end: Vec<&str> = formatted_end_date.split("-").collect();
        self.end_year.set(end.get(0).unwrap().to_string());
        self.end_month.set(end.get(1).unwrap().to_string());
        self.end_day.set(end.get(2).unwrap().to_string());
    }

    pub fn get_total_panels(&self) -> u64 {
        (self.total_panels)()
    }

    pub fn get_total_attributes(&self) -> u64 {
        (self.total_attributes)()
    }

    pub fn get_survey_count(&self) -> (u64, u64, u64) {
        let questions = self.get_survey().questions;
        let mut object_count: u64 = 0;
        let mut subject_count: u64 = 0;
        let mut total_count: u64 = 0;

        for question in questions {
            match question.options {
                Some(_) => {
                    object_count += 1;
                }
                None => {
                    subject_count += 1;
                }
            }
            total_count += 1;
        }
        (object_count, subject_count, total_count)
    }

    pub fn get_panel_info(&mut self) {
        let mut panels = 0;
        let mut attributes = 0;
        let mut attribute_vec = vec![0, 0, 0, 0];

        for panel in self.get_panels() {
            match panel {
                Quota::Attribute {
                    salary_tier,
                    region_code,
                    gender,
                    age,
                    quota,
                } => {
                    match salary_tier {
                        Some(_s) => {
                            attribute_vec[0] = attribute_vec[0] + 1;
                        }
                        None => {}
                    };

                    match region_code {
                        Some(_r) => {
                            attribute_vec[1] = attribute_vec[1] + 1;
                        }
                        None => {}
                    };

                    match gender {
                        Some(_g) => {
                            attribute_vec[2] = attribute_vec[2] + 1;
                        }
                        None => {}
                    };

                    match age {
                        Some(_a) => {
                            attribute_vec[3] = attribute_vec[3] + 1;
                        }
                        None => {}
                    };

                    panels += quota;
                }
                _ => {}
            }
        }

        for i in attribute_vec {
            if i != 0 {
                attributes += 1;
            }
        }

        self.total_panels.set(panels);
        self.total_attributes.set(attributes);
    }

    pub fn get_panels(&self) -> Vec<Quota> {
        vec![
            Quota::Attribute {
                salary_tier: None,
                region_code: Some(051),
                gender: Some(Gender::Male),
                age: Some(Age::Specific(30)),
                quota: 50,
            },
            Quota::Attribute {
                salary_tier: None,
                region_code: Some(051),
                gender: Some(Gender::Male),
                age: Some(Age::Range {
                    inclusive_min: 30,
                    inclusive_max: 40,
                }),
                quota: 50,
            },
            Quota::Attribute {
                salary_tier: None,
                region_code: Some(051),
                gender: Some(Gender::Male),
                age: Some(Age::Range {
                    inclusive_min: 30,
                    inclusive_max: 90,
                }),
                quota: 50,
            },
            Quota::Attribute {
                salary_tier: None,
                region_code: Some(051),
                gender: Some(Gender::Female),
                age: Some(Age::Specific(30)),
                quota: 50,
            },
            Quota::Attribute {
                salary_tier: None,
                region_code: Some(051),
                gender: Some(Gender::Female),
                age: Some(Age::Specific(30)),
                quota: 50,
            },
        ]
    }

    pub fn change_start_date(&mut self, start_date: String) {
        self.start_date.set(start_date.clone());
        self.set_date(Some(start_date.clone()), None);
    }

    pub fn change_end_date(&mut self, end_date: String) {
        self.end_date.set(end_date.clone());
        self.set_date(None, Some(end_date.clone()));
    }

    pub fn get_questions(&self) -> Vec<Question> {
        self.get_survey().questions
    }

    pub fn get_survey(&self) -> GetSurveyResponse {
        match (self.survey.value())() {
            Some(value) => value,
            None => GetSurveyResponse::default(),
        }
    }

    pub fn get_summary_clicked(&self) -> bool {
        (self.summary_clicked)()
    }

    pub fn get_panel_clicked(&self) -> bool {
        (self.panel_clicked)()
    }

    pub fn get_survey_list_clicked(&self) -> bool {
        (self.survey_list_clicked)()
    }

    pub fn change_summary_clicked(&mut self) {
        self.summary_clicked.set(!self.get_summary_clicked());
    }

    pub fn change_panel_clicked(&mut self) {
        self.panel_clicked.set(!self.get_panel_clicked());
    }

    pub fn change_survey_list_clicked(&mut self) {
        self.survey_list_clicked
            .set(!self.get_survey_list_clicked());
    }
}

#[allow(dead_code)]
pub fn use_controller() -> Controller {
    use_context()
}
