use crate::{
    middleware::auth::authorization_middleware,
    utils::{error::ApiError, jwt::Claims, time::convert_rfc3339_to_timestamp_milllis},
};

mod nonce_lab;
use by_axum::axum::{
    extract::{Path, Query, State},
    middleware,
    routing::{get, post},
    Extension, Json, Router,
};
use nonce_lab::NonceLabClient;
mod local_models;

use easy_dynamodb::Client;
use local_models::{NonceLabCreateSurveyRequest, NonceLabQuota, NonceLabSurveyQuestion};

use models::prelude::{
    CompleteSurveyResponse, ListSurveyResponse, ProgressSurveyResponse, QuestionId, Quota, QuotaId,
    Survey, SurveyDraftStatus, SurveyQuestion, SurveyResultAnswer, SurveyResultAnswerType,
    SurveyResultDocument, SurveyStatus, UpsertSurveyDraftRequest,
};
use serde::Deserialize;
use std::{collections::HashMap, sync::Arc};

#[derive(Clone)]

pub struct AxumState {
    db: Arc<Client>,
    nonce_lab_client: Arc<NonceLabClient>,
}
pub fn router(db: Arc<Client>) -> Router {
    let nonce_lab_client = Arc::new(NonceLabClient::new());

    Router::new()
        .route("/", get(list_survey).patch(upsert_survey_draft))
        .route("/:id", get(get_survey).post(progress_survey))
        .route("/:id/result", get(get_survey_result))
        .route("/:id/complete", post(complete_survey))
        .with_state(AxumState {
            db,
            nonce_lab_client,
        })
        .layer(middleware::from_fn(authorization_middleware))
}

const UPDATE_DELAY_MS: i64 = 300_000; // 300 seconds (5 minutes)

#[derive(Deserialize)]
pub struct GetParams {
    pub size: Option<i32>,
    pub bookmark: Option<String>,
}
async fn list_survey(
    Extension(claims): Extension<Claims>,
    State(state): State<AxumState>,
    Query(params): Query<GetParams>,
) -> Result<Json<ListSurveyResponse>, ApiError> {
    let result: Result<
        (Option<Vec<Survey>>, Option<String>),
        easy_dynamodb::error::DynamoException,
    > = state
        .db
        .find(
            "gsi1-index",
            params.bookmark,
            Some(params.size.unwrap_or(10)),
            vec![("gsi1", Survey::get_gsi1(&claims.id))],
        )
        .await;
    let (mut surveys, bookmark) = match result {
        Ok((Some(docs), bookmark)) => (docs, bookmark),
        _ => return Err(ApiError::NotFound),
    };
    let now = chrono::Utc::now().timestamp_millis();
    for survey in surveys.iter_mut() {
        if let Some(nonce_lab_id) = survey.nonce_lab_id {
            if survey.status == SurveyStatus::InProgress
                && survey.updated_at + UPDATE_DELAY_MS <= now
            {
                let current_responses = state
                    .nonce_lab_client
                    .get_survey_responses(nonce_lab_id)
                    .await?;
                let _ = state
                    .db
                    .update(
                        &survey.id,
                        vec![
                            ("updated_at", UpdateField::I64(now)),
                            ("response_count", UpdateField::N(current_responses)),
                        ],
                    )
                    .await;
                survey.response_count = current_responses;
            }
        }
    }

    Ok(Json(ListSurveyResponse {
        bookmark,
        survey: surveys,
    }))
}

async fn get_survey(
    Extension(claims): Extension<Claims>,
    State(state): State<AxumState>,
    Path(id): Path<String>,
) -> Result<Json<Survey>, ApiError> {
    let now = chrono::Utc::now().timestamp_millis();
    let result: Result<Option<Survey>, easy_dynamodb::error::DynamoException> =
        state.db.get(&id).await;
    let mut survey = match result {
        Ok(Some(v)) => {
            if v.creator != claims.id {
                return Err(ApiError::InvalidCredentials("Not Creator".to_string()));
            } else {
                v
            }
        }
        Ok(None) => return Err(ApiError::SurveyNotFound(id)),
        Err(e) => return Err(ApiError::DynamoQueryException(e.to_string())),
    };
    if survey.status == SurveyStatus::InProgress
        && survey.nonce_lab_id.is_some()
        && survey.updated_at + UPDATE_DELAY_MS <= now
    {
        let current_responses = state
            .nonce_lab_client
            .get_survey_responses(survey.nonce_lab_id.unwrap())
            .await?;
        let _ = state
            .db
            .update(
                &survey.id,
                vec![
                    ("updated_at", UpdateField::I64(now)),
                    ("response_count", UpdateField::N(current_responses)),
                ],
            )
            .await;
        survey.response_count = current_responses;
    }

    Ok(Json(survey))
}

async fn upsert_survey_draft(
    Extension(claims): Extension<Claims>,
    State(state): State<AxumState>,
    Json(req): Json<UpsertSurveyDraftRequest>,
) -> Result<Json<String>, ApiError> {
    let mut survey = if let Some(id) = req.id {
        let result: Result<Option<Survey>, easy_dynamodb::error::DynamoException> =
            state.db.get(&id).await;
        match result {
            Ok(Some(v)) => {
                if v.status != SurveyStatus::Draft {
                    return Err(ApiError::NotDraftSurvey);
                }
                if v.creator != claims.id {
                    return Err(ApiError::InvalidCredentials("Not Creator".to_string()));
                }
                v
            }
            Ok(None) => return Err(ApiError::SurveyNotFound(id)),
            Err(e) => return Err(ApiError::DynamoQueryException(e.to_string())),
        }
    } else {
        Survey::new(uuid::Uuid::new_v4().to_string(), claims.id)
    };

    let draft_id = survey.id.clone();
    if let Some(title) = req.title {
        survey.title = title;
    }
    if let Some(questions) = req.questions {
        survey.questions = questions;
    }
    if let Some(quotas) = req.quotas {
        survey.quotas = quotas;
    }
    if let Some(started_at) = req.started_at {
        survey.started_at = Some(started_at);
    }
    if let Some(ended_at) = req.ended_at {
        survey.ended_at = Some(ended_at);
    }
    if let Some(status) = req.status {
        survey.draft_status = Some(status);
    }

    survey.updated_at = chrono::Utc::now().timestamp_millis();
    let _ = state.db.upsert(survey).await;
    Ok(Json(draft_id))
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
enum UpdateField {
    #[serde(untagged)]
    Id(u32),
    #[serde(untagged)]
    S(SurveyStatus),
    #[serde(untagged)]
    Null(Option<bool>),
    #[serde(untagged)]
    N(Option<u64>),
    #[serde(untagged)]
    I64(i64),
    #[serde(untagged)]
    Str(String),
}

//Update Survey Status from Draft to in_progress
async fn progress_survey(
    Extension(claims): Extension<Claims>,
    State(state): State<AxumState>,
    Path(id): Path<String>,
) -> Result<Json<ProgressSurveyResponse>, ApiError> {
    let result: Result<Option<Survey>, easy_dynamodb::error::DynamoException> =
        state.db.get(&id).await;
    let survey = match result {
        Ok(Some(v)) => {
            if v.creator != claims.id {
                return Err(ApiError::InvalidCredentials("Not Ownder".to_string()));
            }
            if v.status != SurveyStatus::Draft {
                return Err(ApiError::NotDraftSurvey);
            }
            if !matches!(v.draft_status, Some(SurveyDraftStatus::Complete))
                || v.title == String::default()
                || v.questions.is_empty()
                || v.quotas.is_empty()
                || v.started_at.is_none()
                || v.ended_at.is_none()
            {
                return Err(ApiError::InCompleteDraft);
            }
            v
        }
        Ok(None) => return Err(ApiError::SurveyNotFound(id)),
        Err(e) => return Err(ApiError::DynamoQueryException(e.to_string())),
    };
    let expected_responses: u64 = survey.quotas.iter().map(|v| v.quota).sum();
    let quotas: Vec<NonceLabQuota> = survey.quotas.into_iter().map(|v| v.into()).collect();
    let questions: Vec<NonceLabSurveyQuestion> =
        survey.questions.into_iter().map(|v| v.into()).collect();
    let survey_dto = NonceLabCreateSurveyRequest {
        custom_id: survey.id.clone(),
        status: SurveyStatus::InProgress,
        started_at: survey.started_at.unwrap() / 100,
        ended_at: survey.ended_at.unwrap() / 100,
        title: survey.title,
        quotas,
        questions,
        description: None,
        expected_responses,
    };
    let now = chrono::Utc::now().timestamp_millis();

    let nonce_lab_id = state.nonce_lab_client.create_survey(survey_dto).await?;
    match state
        .db
        .update(
            &survey.id,
            vec![
                ("nonce_lab_id", UpdateField::Id(nonce_lab_id)),
                ("status", UpdateField::S(SurveyStatus::InProgress)),
                ("draft_status", UpdateField::Null(None)),
                ("updated_at", UpdateField::I64(now)),
                ("response_count", UpdateField::N(Some(0))),
                (
                    "total_response_count",
                    UpdateField::N(Some(expected_responses)),
                ),
            ],
        )
        .await
    {
        Ok(_) => Ok(Json(ProgressSurveyResponse {
            id: uuid::Uuid::new_v4().to_string(),
            nonce_lab_id,
        })),
        Err(e) => Err(ApiError::DynamoUpdateException(e.to_string())),
    }
}

async fn complete_survey(
    Extension(claims): Extension<Claims>,
    State(state): State<AxumState>,
    Path(id): Path<String>,
) -> Result<Json<CompleteSurveyResponse>, ApiError> {
    // Get survey from DB
    let result: Result<Option<Survey>, easy_dynamodb::error::DynamoException> =
        state.db.get(&id).await;
    let survey = match result {
        Ok(Some(v)) => {
            if v.creator != claims.id {
                return Err(ApiError::InvalidCredentials("Not Ownder".to_string()));
            }
            if v.status != SurveyStatus::InProgress {
                return Err(ApiError::NotInProgressSurvey);
            }
            v
        }
        Ok(None) => return Err(ApiError::SurveyNotFound(id)),
        Err(e) => return Err(ApiError::DynamoQueryException(e.to_string())),
    };
    let now = chrono::Utc::now().timestamp_millis();
    // Create Survey Result Docuemnt
    let nonce_lab_id = survey.nonce_lab_id.unwrap();
    let response_count_map = state
        .nonce_lab_client
        .get_survey(nonce_lab_id)
        .await?
        .response_count_map
        .unwrap_or_default();
    let survey_answers = state
        .nonce_lab_client
        .get_survey_result(nonce_lab_id)
        .await?;
    let mut answers: Vec<SurveyResultAnswer> = vec![];
    let mut quotas: HashMap<QuotaId, Quota> = HashMap::new();
    let mut questions: HashMap<QuestionId, SurveyQuestion> = HashMap::new();

    let mut survey_responses_by_question: HashMap<QuestionId, HashMap<String, u64>> =
        HashMap::new();
    for (i, question) in survey.questions.into_iter().enumerate() {
        questions.insert(i as QuestionId, question);
    }
    for quota in survey_answers.quotas.into_iter() {
        let quota_id = quota.id.unwrap_or_default();
        quotas.insert(quota_id as QuotaId, quota.into());
    }
    for item in survey_answers.response_array.into_iter() {
        let quota_id = item.quota_id;
        let responded_at =
            convert_rfc3339_to_timestamp_milllis(&item.responded_at).unwrap_or_default();
        for (question_id, answer) in item.answers.into_iter().enumerate() {
            let answer_type = if let Some(v) = answer.text_answer {
                SurveyResultAnswerType::Text(v)
            } else if let Some(v) = answer.choice_answer {
                SurveyResultAnswerType::Select(v)
            } else {
                SurveyResultAnswerType::NotResponsed
            };
            let text_list = match answer_type.clone() {
                SurveyResultAnswerType::Text(v) => vec![v],
                SurveyResultAnswerType::Select(v) => v,
                _ => vec![],
            };
            for text in text_list {
                survey_responses_by_question
                    .entry(question_id as QuestionId)
                    .or_insert_with(HashMap::new)
                    .entry(text)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }
            let result_answer = SurveyResultAnswer {
                responded_at,
                quota_id,
                question_id: question_id as QuestionId,
                answer_type,
            };

            answers.push(result_answer);
        }
    }
    let doc_id = uuid::Uuid::new_v4().to_string();
    let _ = state
        .db
        .create(SurveyResultDocument {
            gsi1: SurveyResultDocument::get_gsi1(&claims.id, &survey.id),
            id: doc_id.clone(),
            user_id: claims.id.clone(),
            survey_id: survey.id.clone(),
            r#type: SurveyResultDocument::get_type(),
            created_at: chrono::Utc::now().timestamp_millis(),
            expected_responses: survey.total_response_count.unwrap_or_default(),
            acutal_responses: survey.response_count.unwrap_or_default(),
            answers,
            quotas,
            questions,
            survey_responses_by_quota: response_count_map,
            survey_responses_by_question,
        })
        .await;

    // Update Survey State
    match state
        .db
        .update(
            &survey.id,
            vec![
                ("status", UpdateField::S(SurveyStatus::Finished)),
                ("updated_at", UpdateField::I64(now)),
                ("result_id", UpdateField::Str(doc_id)),
            ],
        )
        .await
    {
        Ok(_) => Ok(Json(CompleteSurveyResponse {
            id: uuid::Uuid::new_v4().to_string(),
        })),
        Err(e) => Err(ApiError::DynamoUpdateException(e.to_string())),
    }
}

async fn get_survey_result(
    Extension(claims): Extension<Claims>,
    State(state): State<AxumState>,
    Path(id): Path<String>,
) -> Result<Json<SurveyResultDocument>, ApiError> {
    let query_result = state
        .db
        .find(
            "gsi1-index",
            None,
            Some(1),
            vec![("gsi1", SurveyResultDocument::get_gsi1(&claims.id, &id))],
        )
        .await;

    let docs = match query_result {
        Ok((Some(docs), _)) if !docs.is_empty() => docs,
        _ => return Err(ApiError::NotFound),
    };

    Ok(Json(docs.into_iter().next().unwrap()))
}
