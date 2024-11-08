#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_logger::tracing;

use crate::{
    api::v1::surveys::{
        upsert_survey::{upsert_survey, SurveyUpdateItem},
        GetSurveyResponse,
    },
    models::{question::QuestionType, survey::StatusType},
    service::login_service::use_login_service,
};

use super::{Language, Route};

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum QuestionStep {
    List,
    Input,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Controller {
    step: Signal<QuestionStep>,
    pub survey_response: Resource<GetSurveyResponse>,
    question_types: Signal<Vec<QuestionOption>>,
    question_title: Signal<String>,
    selected_question_types: Signal<u64>,
    objective_questions: Signal<Vec<ObjectiveQuestionOption>>,
    deleted_key_list: Signal<Vec<usize>>,
    update_key: Signal<usize>,
    update_button_clicked: Signal<bool>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct QuestionOption {
    pub value: i64,
    pub label: String,
}

#[derive(Clone, PartialEq, Debug)]
pub struct ObjectiveQuestionOption {
    pub hint: String,
    pub text_value: String,
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
            step: use_signal(|| QuestionStep::List),
            survey_response,
            question_title: use_signal(|| "".to_string()),
            question_types: use_signal(|| {
                vec![
                    QuestionOption {
                        value: 0,
                        label: "객관식 질문".to_string(),
                    },
                    QuestionOption {
                        value: 1,
                        label: "주관식 질문(단답형)".to_string(),
                    },
                    QuestionOption {
                        value: 2,
                        label: "주관식 질문(서술형)".to_string(),
                    },
                ]
            }),
            selected_question_types: use_signal(|| 0),
            objective_questions: use_signal(|| {
                vec![
                    ObjectiveQuestionOption {
                        hint: "옵션 1".to_string(),
                        text_value: "".to_string(),
                    },
                    ObjectiveQuestionOption {
                        hint: "옵션 2".to_string(),
                        text_value: "".to_string(),
                    },
                ]
            }),
            deleted_key_list: use_signal(|| vec![]),
            update_key: use_signal(|| 0),
            update_button_clicked: use_signal(|| false),
        };

        ctrl
    }

    pub fn clear_data(&mut self) {
        self.question_title.set("".to_string());
        self.selected_question_types.set(0);
        self.objective_questions.set(vec![
            ObjectiveQuestionOption {
                hint: "옵션 1".to_string(),
                text_value: "".to_string(),
            },
            ObjectiveQuestionOption {
                hint: "옵션 2".to_string(),
                text_value: "".to_string(),
            },
        ]);
        self.deleted_key_list.set(vec![]);
        self.update_button_clicked.set(false);
    }

    pub fn refresh_survey_response(&mut self) {
        (self.survey_response).restart();
    }

    pub fn get_question_title(&mut self) -> String {
        (self.question_title)()
    }

    pub fn get_update_key(&mut self) -> usize {
        (self.update_key)()
    }

    pub fn get_question_types(&mut self) -> Vec<QuestionOption> {
        (self.question_types)()
    }

    pub fn get_selected_question(&mut self) -> u64 {
        (self.selected_question_types)()
    }

    pub fn get_survey(&mut self) -> GetSurveyResponse {
        match (self.survey_response.value())() {
            Some(value) => value,
            None => GetSurveyResponse::default(),
        }
    }

    pub fn get_step(&mut self) -> QuestionStep {
        (self.step)()
    }

    pub fn get_objective_questions(&mut self) -> Vec<ObjectiveQuestionOption> {
        (self.objective_questions)()
    }

    pub fn delete_key_lists(&mut self) -> Vec<usize> {
        (self.deleted_key_list)()
    }

    pub fn change_question_title(&mut self, title: String) {
        self.question_title.set(title);
    }

    pub fn change_step(&mut self, step: QuestionStep) {
        self.step.set(step);
    }

    pub fn change_selected_question(&mut self, selected: u64) {
        self.selected_question_types.set(selected);
    }

    pub fn change_objective_question(&mut self, index: usize, text_value: String) {
        let mut objectives = self.get_objective_questions();
        objectives[index] = ObjectiveQuestionOption {
            hint: objectives[index].clone().hint,
            text_value,
        };
        self.objective_questions.set(objectives);
    }

    pub fn add_objective_question(&mut self) {
        let mut objectives = self.get_objective_questions();
        let objective_len = objectives.len();
        objectives.push(ObjectiveQuestionOption {
            hint: format!("옵션 {:?}", objective_len + 1),
            text_value: "".to_string(),
        });
        self.objective_questions.set(objectives);
    }

    pub fn remove_objective_question(&mut self, index: usize) {
        let mut objectives = self.get_objective_questions();
        objectives.remove(index);
        self.objective_questions.set(objectives);
    }

    pub async fn write_question(&mut self, status: StatusType) {
        let selected_question_type = self.get_selected_question();
        let question_title = self.get_question_title();
        let email: String = use_login_service().get_email().clone();
        let survey = self.get_survey();

        if (self.update_button_clicked)() {
            let questions = self.get_survey().questions;
            let question = questions.get((self.update_key)()).unwrap();
            if selected_question_type == 0 {
                //객관식
                let object_options = self.get_objective_questions();
                let mut options: Vec<String> = vec![];

                for d in object_options {
                    options.push(d.text_value);
                }
                let _ = upsert_survey(
                    email,
                    survey.survey.id,
                    status,
                    SurveyUpdateItem::UpdateQuestion {
                        id: question.id.clone(),
                        title: Some(survey.survey.title),
                        question: Some(QuestionType::SingleChoice {
                            question: Some(question_title),
                            options,
                        }),
                    },
                )
                .await;
            } else if selected_question_type == 1 {
                //단답형
                let _ = upsert_survey(
                    email,
                    survey.survey.id,
                    status,
                    SurveyUpdateItem::UpdateQuestion {
                        id: question.id.clone(),
                        title: Some(survey.survey.title),
                        question: Some(QuestionType::Text(Some(question_title))),
                    },
                )
                .await;
            } else {
                //서술형
                let _ = upsert_survey(
                    email,
                    survey.survey.id,
                    status,
                    SurveyUpdateItem::UpdateQuestion {
                        id: question.id.clone(),
                        title: Some(survey.survey.title),
                        question: Some(QuestionType::LongText(Some(question_title))),
                    },
                )
                .await;
            }
        } else {
            if selected_question_type == 0 {
                //객관식
                let object_options = self.get_objective_questions();
                let mut options: Vec<String> = vec![];

                for d in object_options {
                    options.push(d.text_value);
                }
                let _ = upsert_survey(
                    email,
                    survey.survey.id,
                    status,
                    SurveyUpdateItem::AddQuestion {
                        title: survey.survey.title,
                        question: QuestionType::SingleChoice {
                            question: Some(question_title),
                            options,
                        },
                    },
                )
                .await;
            } else if selected_question_type == 1 {
                //단답형
                let _ = upsert_survey(
                    email,
                    survey.survey.id,
                    status,
                    SurveyUpdateItem::AddQuestion {
                        title: survey.survey.title,
                        question: QuestionType::Text(Some(question_title)),
                    },
                )
                .await;
            } else {
                //서술형
                let _ = upsert_survey(
                    email,
                    survey.survey.id,
                    status,
                    SurveyUpdateItem::AddQuestion {
                        title: survey.survey.title,
                        question: QuestionType::LongText(Some(question_title)),
                    },
                )
                .await;
            }
        }
    }

    pub async fn remove_question(&mut self, index: usize) {
        tracing::info!("remove question button clicked");

        let mut keys = (self.deleted_key_list)();
        keys.push(index);

        self.deleted_key_list.set(keys);
    }

    pub fn clicked_update_button(&mut self, index: usize) {
        self.update_key.set(index);
        self.update_button_clicked.set(true);
        let questions = self.get_survey().questions;
        let question = questions.get(index);

        let question_title = question.unwrap().question.clone();
        let options = question.unwrap().options.clone();

        let gsi2: Vec<&str> = question.unwrap().gsi2.split("#").collect();
        let survey_type = gsi2[3];

        if survey_type == "single-choice" {
            self.question_title.set(question_title.clone());

            let mut os: Vec<ObjectiveQuestionOption> = vec![];

            for (i, d) in options.unwrap().iter().enumerate() {
                os.push(ObjectiveQuestionOption {
                    hint: format!("옵션 {}", i + 1),
                    text_value: d.to_string(),
                });
            }

            self.objective_questions.set(os);
            self.selected_question_types.set(0);
        } else if survey_type == "text" {
            self.question_title.set(question_title.clone());
            self.selected_question_types.set(1);
        } else {
            self.question_title.set(question_title.clone());
            self.selected_question_types.set(2);
        }
    }

    pub async fn clicked_temporary_save(&mut self) {
        let keys = (self.deleted_key_list)();
        let questions = self.get_survey().questions;

        let email: String = use_login_service().get_email().clone();
        let survey = self.get_survey();

        for i in keys {
            let question = questions.get(i);

            let _ = upsert_survey(
                email.clone(),
                survey.survey.id.clone(),
                StatusType::TemporarySave,
                SurveyUpdateItem::RemoveQuestion(question.unwrap().id.clone()),
            )
            .await;
        }
    }

    pub async fn update_question(&mut self, _index: usize, _title: String) {
        tracing::info!("update question button clicked");
        // let survey = self.get_survey();
        // let _ = upsert_survey(
        //     survey.survey.id,
        //     SurveyUpdateItem::UpdateQuestion {
        //         id: index.to_string(),
        //         title: Some(title),
        //         question: Some(survey.questions.get(index).unwrap().question.clone()),
        //     },
        // )
        // .await;
    }
}
