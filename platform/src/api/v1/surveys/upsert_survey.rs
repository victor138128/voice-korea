#![allow(unused_imports)]
use dioxus::prelude::{
    server_fn::codec::{GetUrl, Json, PostUrl},
    *,
};
use dioxus_logger::tracing;
use serde::{Deserialize, Serialize};
use server_fn::ServerFn;

use crate::{
    api::common::TypeField,
    models::{
        question::{Question, QuestionType},
        survey::{
            self, QuestionSequence, Quota, StatusType, SurveySequenceModel, SurveyStatus,
            SurveySummary,
        },
    },
    service::login_service::use_login_service,
};

use super::{GetSurveyResponse, Status};

#[server(endpoint = "/v1/surveys", input = Json, output = Json)]
pub async fn upsert_survey(
    email: String,
    survey_id: String,
    status: StatusType,
    item: SurveyUpdateItem,
) -> Result<(), ServerFnError> {
    let log = crate::utils::logger::new_api("POST", &format!("/v1/empty/surveys"));
    let cli = crate::utils::db::get(&log);
    slog::debug!(log, "/v1/surveys: {:?} {:?}", survey_id, item);

    match item {
        SurveyUpdateItem::RemoveQuestion(id) => {
            if status == StatusType::Save {
                match cli
                    .update::<TypeField>(
                        &survey_id,
                        vec![(
                            "gsi2",
                            TypeField::S(QuestionSequence::SelectResponse.to_string()),
                        )],
                    )
                    .await
                {
                    Ok(()) => {
                        tracing::info!("db select response update success");
                    }
                    Err(e) => {
                        tracing::info!("db select response update failed: {survey_id} {e}");
                        return Err(ServerFnError::ServerError(format!("DB update failed")));
                    }
                }
            } else if status == StatusType::Back {
                match cli
                    .update::<TypeField>(
                        &survey_id,
                        vec![("gsi2", TypeField::S(QuestionSequence::Title.to_string()))],
                    )
                    .await
                {
                    Ok(()) => {
                        tracing::info!("db title update success");
                    }
                    Err(e) => {
                        tracing::info!("db title update failed: {survey_id} {e}");
                        return Err(ServerFnError::ServerError(format!("DB update failed")));
                    }
                }
            } else {
                let _ = remove_question(id).await;
            }
        }
        SurveyUpdateItem::Title(title) => {
            let _ = write_title(email, survey_id, status, title).await;
        }
        SurveyUpdateItem::AddQuestion { title, question } => {
            let _ = write_question(email, survey_id, status, title, question).await;
        }
        SurveyUpdateItem::UpdateQuestion {
            id,
            title,
            question,
        } => {
            let _ = update_question_survey(id, title.unwrap(), question.unwrap()).await;
        }
        SurveyUpdateItem::AddResponder(_quota) => {
            //TODO: save, temporary save, back 상태일 때 구현
            //save: 저장 후 QuestionSequence를 Summary 상태로 이동
            //temporary save: 저장만 구현
            //back: QuestionSequence를 AddQuestion 상태로 이동
        }
        SurveyUpdateItem::RemoveResponder(_quota_id) => {
            //FIXME: quota_id는 어떻게 산정할지?
        }
        SurveyUpdateItem::SetPeriod(start_date, end_date) => {
            tracing::debug!("start: {} end: {}", start_date, end_date);
        }
    }

    Ok(())
}

#[server(endpoint = "/v1/empty/surveys", input = Json, output = Json)]
pub async fn create_empty_survey(email: String) -> Result<String, ServerFnError> {
    let log = crate::utils::logger::new_api("POST", &format!("/v1/empty/surveys"));
    let cli = crate::utils::db::get(&log);

    let timestamp = (chrono::Utc::now().timestamp_millis() / 1000) as u64;

    let id = format!("{email}#survey#{:?}", timestamp).clone();

    match cli
        .create(SurveySummary {
            id: id.clone(),
            title: "".to_string(),
            status: SurveyStatus::Draft,
            updated_at: timestamp,
            created_at: timestamp,
            responses: None,
            expected_responses: None,
            questions: 0,
            quotas: None,
            r#type: "survey".to_string(),
            gsi1: email,
            gsi2: QuestionSequence::Title.to_string(),
        })
        .await
    {
        Ok(()) => {
            tracing::debug!("survey create success",);
        }
        Err(e) => {
            return Err(ServerFnError::ServerError(format!("DB create failed {e}")));
        }
    }

    Ok(format!("{}", timestamp))
}

#[server]
async fn remove_question(id: String) -> Result<(), ServerFnError> {
    let log = crate::utils::logger::new_api("POST", &format!("/v1/surveys"));
    let cli: easy_dynamodb::Client = crate::utils::db::get(&log);

    match cli.delete(id.as_str()).await {
        Ok(_) => {
            tracing::info!("deleted success");
        }
        Err(e) => {
            tracing::info!("deleted failed: {e}");
            return Err(ServerFnError::ServerError(format!("delete failed: {id}")));
        }
    }

    Ok(())
}

#[server]
async fn write_question(
    email: String,
    survey_id: String,
    status: StatusType,
    title: String,
    question: QuestionType,
) -> Result<(), ServerFnError> {
    let id: Vec<&str> = survey_id.split("#").collect();
    let survey_id = id[2].to_string();
    let log = crate::utils::logger::new_api("POST", &format!("/v1/surveys"));
    let cli: easy_dynamodb::Client = crate::utils::db::get(&log);
    let survey_summary: Result<Option<SurveySummary>, easy_dynamodb::error::DynamoException> = cli
        .get(format!("{}#survey#{}", email, survey_id).as_str())
        .await;

    let res: Result<
        (Option<Vec<Question>>, Option<String>),
        easy_dynamodb::error::DynamoException,
    > = cli
        .find(
            "gsi1-index",
            None,
            Some(100),
            vec![("gsi1", format!("{}#survey-question#{}", email, survey_id))],
        )
        .await;

    match res {
        Ok(v) => {
            let questions_vec = &v.0.unwrap();

            if questions_vec.len() != 0 {
                let len = questions_vec.len();
                match status {
                    StatusType::TemporarySave => {
                        let mut survey: SurveySummary = match survey_summary {
                            Ok(s) => s.unwrap(),
                            Err(e) => {
                                return Err(ServerFnError::ServerError(format!(
                                    "Get Summary Failed: {e}"
                                )))
                            }
                        };

                        survey.questions += 1;

                        match cli
                            .update::<TypeField>(
                                format!("{}#survey#{}", email, survey_id).as_str(),
                                vec![("questions", TypeField::N(survey.questions as i64))],
                            )
                            .await
                        {
                            Ok(()) => {}
                            Err(_e) => {
                                return Err(ServerFnError::ServerError(format!(
                                    "survey update failed"
                                )));
                            }
                        }

                        let _ = create_question_survey(
                            question,
                            email,
                            survey_id,
                            title.clone(),
                            len as u64,
                        )
                        .await;
                    }
                    StatusType::Save => {
                        let mut survey = survey_summary.unwrap().unwrap();
                        survey.questions += 1;
                        survey.gsi2 = QuestionSequence::SelectResponse.to_string();

                        match cli
                            .update::<TypeField>(
                                format!("{}#survey#{}", email, survey_id).as_str(),
                                vec![
                                    ("questions", TypeField::N(survey.clone().questions as i64)),
                                    ("gsi2", TypeField::S(survey.clone().gsi2)),
                                ],
                            )
                            .await
                        {
                            Ok(()) => {}
                            Err(_e) => {
                                return Err(ServerFnError::ServerError(format!(
                                    "survey update failed"
                                )));
                            }
                        }

                        let _ = create_question_survey(
                            question,
                            email,
                            survey_id,
                            title.clone(),
                            len as u64,
                        )
                        .await;
                    }
                    _ => {}
                }
            } else {
                match status {
                    StatusType::TemporarySave => {
                        let mut survey: SurveySummary = match survey_summary {
                            Ok(s) => s.unwrap(),
                            Err(e) => {
                                return Err(ServerFnError::ServerError(format!(
                                    "Get Summary Failed: {e}"
                                )))
                            }
                        };

                        survey.questions += 1;

                        match cli
                            .update::<TypeField>(
                                format!("{}#survey#{}", email, survey_id).as_str(),
                                vec![("questions", TypeField::N(survey.questions as i64))],
                            )
                            .await
                        {
                            Ok(()) => {}
                            Err(_e) => {
                                return Err(ServerFnError::ServerError(format!(
                                    "survey update failed"
                                )));
                            }
                        }

                        let _ =
                            create_question_survey(question, email, survey_id, title.clone(), 0)
                                .await;
                    }
                    StatusType::Save => {
                        let mut survey = survey_summary.unwrap().unwrap();
                        survey.questions += 1;
                        survey.gsi2 = QuestionSequence::SelectResponse.to_string();

                        match cli
                            .update::<TypeField>(
                                format!("{}#survey#{}", email, survey_id).as_str(),
                                vec![
                                    ("questions", TypeField::N(survey.clone().questions as i64)),
                                    ("gsi2", TypeField::S(survey.clone().gsi2)),
                                ],
                            )
                            .await
                        {
                            Ok(()) => {}
                            Err(_e) => {
                                return Err(ServerFnError::ServerError(format!(
                                    "survey update failed"
                                )));
                            }
                        }

                        let _ =
                            create_question_survey(question, email, survey_id, title.clone(), 0)
                                .await;
                    }
                    _ => {}
                }
            }
        }
        Err(e) => return Err(ServerFnError::ServerError(format!("DB query failed {e}"))),
    }

    Ok(())
}

#[server]
async fn update_question_survey(
    id: String,
    title: String,
    question: QuestionType,
) -> Result<(), ServerFnError> {
    let log = crate::utils::logger::new_api("POST", &format!("/v1/surveys"));
    let cli: easy_dynamodb::Client = crate::utils::db::get(&log);

    let id_vec: Vec<&str> = id.split("#").collect();
    let id_str = format!("{}#{}#{}", id_vec[0], id_vec[1], id_vec[2]);

    match question {
        QuestionType::SingleChoice { question, options } => {
            match cli
                .update(
                    id.as_str(),
                    vec![
                        ("title", TypeField::S(title)),
                        ("question", TypeField::S(question.unwrap())),
                        ("options", TypeField::V(Some(options))),
                        (
                            "gsi2",
                            TypeField::S(format!("{}#{}", id_str, "single-choice")),
                        ),
                    ],
                )
                .await
            {
                Ok(()) => {
                    tracing::info!("update success");
                }
                Err(e) => {
                    tracing::info!("update failed: {e}");
                    return Err(ServerFnError::ServerError(format!("DB update failed: {e}")));
                }
            }
        }
        QuestionType::LongText(text) => {
            match cli
                .update(
                    id.as_str(),
                    vec![
                        ("title", TypeField::S(title)),
                        ("question", TypeField::S(text.unwrap())),
                        ("options", TypeField::V(None)),
                        ("gsi2", TypeField::S(format!("{}#{}", id_str, "long-text"))),
                    ],
                )
                .await
            {
                Ok(()) => {
                    tracing::info!("update success");
                }
                Err(e) => {
                    tracing::info!("update failed: {e}");
                    return Err(ServerFnError::ServerError(format!("DB update failed: {e}")));
                }
            }
        }
        QuestionType::Text(text) => {
            match cli
                .update(
                    id.as_str(),
                    vec![
                        ("title", TypeField::S(title)),
                        ("question", TypeField::S(text.unwrap())),
                        ("options", TypeField::V(None)),
                        ("gsi2", TypeField::S(format!("{}#{}", id_str, "text"))),
                    ],
                )
                .await
            {
                Ok(()) => {
                    tracing::info!("update success");
                }
                Err(e) => {
                    tracing::info!("update failed: {e}");
                    return Err(ServerFnError::ServerError(format!("DB update failed: {e}")));
                }
            }
        }
    }

    Ok(())
}

#[server]
async fn create_question_survey(
    question: QuestionType,
    email: String,
    survey_id: String,
    title: String,
    question_count: u64,
) -> Result<(), ServerFnError> {
    let log = crate::utils::logger::new_api("POST", &format!("/v1/surveys"));
    let cli: easy_dynamodb::Client = crate::utils::db::get(&log);
    let timestamp = (chrono::Utc::now().timestamp_millis() / 1000) as u64;
    match question {
        QuestionType::SingleChoice { question, options } => {
            match cli
                .create(Question {
                    id: format!("{}#survey-question#{}#{}", email, survey_id, question_count),
                    survey_id: survey_id.clone(),
                    title: title.clone(),
                    question: question.unwrap(),
                    options: Some(options),
                    created_at: timestamp,
                    updated_at: timestamp,
                    gsi1: format!("{}#survey-question#{}", email, survey_id),
                    gsi2: format!("{}#survey-question#{}#single-choice", email, survey_id),
                })
                .await
            {
                Ok(()) => {
                    tracing::info!("create success");
                }
                Err(e) => {
                    tracing::info!("create failed: {e}");
                    return Err(ServerFnError::ServerError(format!("DB create failed: {e}")));
                }
            }
        }
        QuestionType::LongText(text) => {
            match cli
                .create(Question {
                    id: format!("{}#survey-detail#{}#{}", email, survey_id, question_count),
                    survey_id: survey_id.clone(),
                    title: title.clone(),
                    question: text.unwrap(),
                    options: None,
                    created_at: timestamp,
                    updated_at: timestamp,
                    gsi1: format!("{}#survey-question#{}", email, survey_id),
                    gsi2: format!("{}#survey-question#{}#long-text", email, survey_id),
                })
                .await
            {
                Ok(()) => {}
                Err(e) => {
                    return Err(ServerFnError::ServerError(format!("DB create failed: {e}")));
                }
            }
        }
        QuestionType::Text(text) => {
            match cli
                .create(Question {
                    id: format!("{}#survey-detail#{}#{}", email, survey_id, question_count),
                    survey_id: survey_id.clone(),
                    title: title.clone(),
                    question: text.unwrap(),
                    options: None,
                    created_at: timestamp,
                    updated_at: timestamp,
                    gsi1: format!("{}#survey-question#{}", email, survey_id),
                    gsi2: format!("{}#survey-question#{}#text", email, survey_id),
                })
                .await
            {
                Ok(()) => {}
                Err(e) => {
                    return Err(ServerFnError::ServerError(format!("DB create failed: {e}")));
                }
            }
        }
    }

    Ok(())
}

#[server]
async fn write_title(
    email: String,
    survey_id: String,
    status: StatusType,
    title: String,
) -> Result<(), ServerFnError> {
    let log = crate::utils::logger::new_api("POST", &format!("/v1/surveys"));
    let cli: easy_dynamodb::Client = crate::utils::db::get(&log);
    let key = format!("{email}#survey#{}", survey_id).clone();

    match status {
        StatusType::TemporarySave => {
            match cli
                .update::<TypeField>(&key, vec![("title", TypeField::S(title))])
                .await
            {
                Ok(()) => {}
                Err(_e) => {
                    return Err(ServerFnError::ServerError(format!("DB update failed")));
                }
            }
        }
        _ => {
            match cli
                .update::<TypeField>(
                    &key,
                    vec![
                        ("title", TypeField::S(title)),
                        (
                            "gsi2",
                            TypeField::S(QuestionSequence::AddQuestion.to_string()),
                        ),
                    ],
                )
                .await
            {
                Ok(()) => {}
                Err(_e) => {
                    return Err(ServerFnError::ServerError(format!("DB update failed")));
                }
            }
        }
    }

    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurveyUpdateItem {
    Title(String),
    AddQuestion {
        title: String,
        question: QuestionType,
    },
    UpdateQuestion {
        id: String,
        title: Option<String>,
        question: Option<QuestionType>,
    },
    RemoveQuestion(String),
    AddResponder(Quota),
    // RemoveResponder requires quota id to be removed.
    RemoveResponder(String),

    // SetPeriod requires epoch timestamp in seconds for start and end time.
    SetPeriod(u64, u64),
}
