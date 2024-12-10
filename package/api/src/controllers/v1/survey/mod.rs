use crate::{
    middleware::auth::authorization_middleware,
    utils::{error::ApiError, jwt::Claims, nonce_lab},
};
use by_axum::axum::{
    extract::{Path, Query, State},
    middleware,
    routing::get,
    Extension, Json, Router,
};
mod local_models;

use easy_dynamodb::Client;
use local_models::{
    NonceLabCreateSurveyRequest, NonceLabCreateSurveyResponse, NonceLabGetSurveyDto, NonceLabQuota,
    NonceLabSurveyQuestion,
};

use models::prelude::{
    ListSurveyResponse, ProgressSurveyResponse, Survey, SurveyDraftStatus, SurveyStatus,
    UpsertSurveyDraftRequest,
};
use serde::Deserialize;
use std::sync::Arc;

pub fn router(db: Arc<Client>) -> Router {
    Router::new()
        .route("/", get(list_survey).patch(upsert_survey_draft))
        .route("/:id", get(get_survey).post(progress_survey))
        .with_state(db)
        .layer(middleware::from_fn(authorization_middleware))
}

const UPDATE_DELAY_MS: i64 = 300_000; // 300 seconds (5 minutes)

#[derive(Deserialize)]
pub struct GetParams {
    pub size: Option<i32>,
    pub bookmark: Option<String>,
}
async fn get_current_responses(nonce_lab_id: u32) -> Option<u64> {
    let nonce_lab_client = if let Ok(v) = nonce_lab::NonceLabClient::new() {
        v
    } else {
        return None;
    };
    let nonce_lab_survey: NonceLabGetSurveyDto = match nonce_lab_client
        .get(&format!("/v1/vendor/survey/{}", nonce_lab_id))
        .send()
        .await
    {
        Ok(v) => match v.json().await {
            Ok(v) => v,
            _ => return None,
        },
        _ => return None,
    };
    if let Some(v) = nonce_lab_survey.response_count_map {
        Some(v.values().sum())
    } else {
        None
    }
}
async fn list_survey(
    Extension(claims): Extension<Claims>,
    State(db): State<Arc<Client>>,
    Query(params): Query<GetParams>,
) -> Result<Json<ListSurveyResponse>, ApiError> {
    let result: Result<
        (Option<Vec<Survey>>, Option<String>),
        easy_dynamodb::error::DynamoException,
    > = db
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
                let current_responses = get_current_responses(nonce_lab_id).await;
                let _ = db
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
    State(db): State<Arc<Client>>,
    Path(id): Path<String>,
) -> Result<Json<Survey>, ApiError> {
    let result: Result<Option<Survey>, easy_dynamodb::error::DynamoException> = db.get(&id).await;
    let now = chrono::Utc::now().timestamp_millis();

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
        let current_responses = get_current_responses(survey.nonce_lab_id.unwrap()).await;
        let _ = db
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
    State(db): State<Arc<Client>>,
    Json(req): Json<UpsertSurveyDraftRequest>,
) -> Result<Json<String>, ApiError> {
    let mut survey = if let Some(id) = req.id {
        let result: Result<Option<Survey>, easy_dynamodb::error::DynamoException> =
            db.get(&id).await;
        match result {
            Ok(Some(v)) => {
                if v.status != SurveyStatus::Draft {
                    return Err(ApiError::SurveyInProgress);
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
    let _ = db.upsert(survey).await;
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
}

//Update Survey Status from Draft to in_progress
async fn progress_survey(
    Extension(claims): Extension<Claims>,
    State(db): State<Arc<Client>>,
    Path(id): Path<String>,
) -> Result<Json<ProgressSurveyResponse>, ApiError> {
    let result: Result<Option<Survey>, easy_dynamodb::error::DynamoException> = db.get(&id).await;
    let survey = match result {
        Ok(Some(v)) => {
            if v.creator != claims.id {
                return Err(ApiError::InvalidCredentials("Not Ownder".to_string()));
            }
            if v.status != SurveyStatus::Draft {
                return Err(ApiError::SurveyInProgress);
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

    let nonce_lab_client = nonce_lab::NonceLabClient::new()
        .map_err(|e| ApiError::ReqwestClientFailed(e.to_string()))?;
    let res = nonce_lab_client
        .post("/v1/vendor/survey")
        .json(&survey_dto)
        .send()
        .await
        .map_err(|v| ApiError::ReqwestFailed(v.to_string()))?;
    let res: NonceLabCreateSurveyResponse = match res.error_for_status() {
        Ok(v) => match v.json().await {
            Ok(v) => v,
            Err(e) => return Err(ApiError::JSONSerdeError(e.to_string())),
        },
        Err(e) => {
            return Err(ApiError::ReqwestFailed(e.to_string()));
        }
    };
    let nonce_lab_id = res.id;
    match db
        .update(
            &survey.id,
            vec![
                ("nonce_lab_id", UpdateField::Id(nonce_lab_id)),
                ("status", UpdateField::S(SurveyStatus::InProgress)),
                ("draft_status", UpdateField::Null(None)),
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
