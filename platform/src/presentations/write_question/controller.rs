#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus_logger::tracing;

use crate::{
    api::v1::surveys::{
        // upsert_survey::{upsert_survey, SurveyUpdateItem},
        GetSurveyResponse,
    },
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
    survey: Signal<GetSurveyResponse>,
    step: Signal<QuestionStep>,
    question_types: Signal<Vec<QuestionOption>>,
    selected_question_types: Signal<u64>,
    objective_questions: Signal<Vec<ObjectiveQuestionOption>>,
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

        let mut ctrl = Self {
            survey: use_signal(|| GetSurveyResponse::default()),
            step: use_signal(|| QuestionStep::List),
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
        };

        let _ = use_effect(move || {
            let id_value = id.clone();
            let email_value = email.clone();
            spawn(async move {
                let res = async move {
                    crate::utils::api::get::<GetSurveyResponse>(&format!(
                        "/v1/email/{}/surveys/{}",
                        email_value, id_value
                    ))
                    .await
                }
                .await;

                ctrl.survey.set(res)
            });
        });

        ctrl
    }

    pub fn get_question_types(&mut self) -> Vec<QuestionOption> {
        (self.question_types)()
    }

    pub fn get_selected_question(&mut self) -> u64 {
        (self.selected_question_types)()
    }

    pub fn get_survey(&mut self) -> GetSurveyResponse {
        (self.survey)()
    }

    pub fn get_step(&mut self) -> QuestionStep {
        (self.step)()
    }

    pub fn get_objective_questions(&mut self) -> Vec<ObjectiveQuestionOption> {
        (self.objective_questions)()
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

    // pub async fn write_question(&mut self) {
    //     let selected_question_type = self.get_selected_question();
    //     let survey = self.get_survey();

    //     if selected_question_type == 0 { //객관식
    //         let _ = upsert_survey(survey.survey.id, SurveyUpdateItem::AddQuestion { title: , question: () })
    //     }
    // }

    pub async fn remove_question(&mut self, _index: usize) {
        tracing::info!("remove question button clicked");
        // let survey = self.get_survey();
        // let _ = upsert_survey(
        //     survey.survey.id,
        //     SurveyUpdateItem::RemoveQuestion(survey.questions.get(index).unwrap().title.clone()),
        // )
        // .await;
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
