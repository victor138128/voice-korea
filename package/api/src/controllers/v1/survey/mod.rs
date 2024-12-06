use std::sync::Arc;

use crate::utils::{error::ApiError, nonce_lab};
use by_axum::{
    axum::{
        extract::{Path, Query, State},
        routing::get,
        Json, Router,
    },
    log::root,
};
use easy_dynamodb::Client;
use models::prelude::{
    CreateSurveyDto, CreateSurveyRequest, CreateSurveyResponse, ListSurveyDraftRequest,
    ListSurveyDraftResponse, Survey, SurveyDocument, SurveyDraftStatus, SurveyStatus,
    SurveySummary, UpsertSurveyDraftRequest,
};
use serde::Deserialize;

pub fn router(db: Arc<Client>) -> Router {
    Router::new()
        .route("/", get(list_survey).post(create_survey))
        .route("/:id", get(get_survey))
        .route("/draft", get(list_survey_draft).patch(upsert_survey_draft))
        .route("/draft/:id", get(get_survey_draft))
        .with_state(db)
}

#[derive(Deserialize)]
pub struct GetParams {
    pub page: Option<u32>,
    pub size: Option<u32>,
    pub status: Option<SurveyStatus>,
    // pub order: Option<Order>,
    // pub sort: Option<Sort>,
}

#[derive(Deserialize, Debug, Default)]
pub enum Sort {
    #[default]
    #[serde(rename = "desc")]
    Desc,
    #[serde(rename = "asc")]
    Asc,
}

#[derive(Deserialize, Debug, Default)]
pub enum Order {
    #[default]
    #[serde(rename = "END_DATE")]
    EndDate,
    #[serde(rename = "POINT")]
    Point,
}

async fn list_survey(
    Query(params): Query<GetParams>,
) -> Result<Json<Vec<SurveySummary>>, ApiError> {
    let nonce_lab_client = nonce_lab::NonceLabClient::new()
        .map_err(|e| ApiError::ReqwestClientFailed(e.to_string()))?;
    let mut query = vec![];
    if let Some(page) = params.page {
        query.push(("cursor", page.to_string()));
    }
    if let Some(size) = params.size {
        query.push(("take", size.to_string()));
    }
    if let Some(status) = params.status {
        query.push((
            "status",
            serde_json::to_string(&status).unwrap_or("ALL".to_string()),
        ))
    }
    let res = nonce_lab_client
        .get("/v1/vendor/survey")
        .query(&query)
        .send()
        .await;

    let res: Result<Vec<SurveySummary>, reqwest::Error> = match res {
        Ok(v) => v.json().await,
        Err(e) => return Err(ApiError::ReqwestFailed(e.to_string())),
    };

    //FIXME: remove code after nonce_lab add updated_at to their response.
    let survey = match res {
        Ok(v) => v,
        Err(e) => {
            slog::error!(root(), "{:?}", e);
            return Err(ApiError::JSONSerdeError(e.to_string()));
        }
    };
    let survey = survey
        .into_iter()
        .map(|mut v| {
            v.updated_at = chrono::Utc::now().timestamp_millis();
            v
        })
        .collect();

    Ok(Json(survey))
}

async fn get_survey(Path(id): Path<String>) -> Result<Json<Survey>, ApiError> {
    let nonce_lab_client = nonce_lab::NonceLabClient::new()
        .map_err(|e| ApiError::ReqwestClientFailed(e.to_string()))?;
    match nonce_lab_client
        .get(&format!("/v1/vendor/survey/{}", id))
        .send()
        .await
    {
        Ok(v) => match v.json().await {
            Ok(v) => Ok(Json(v)),
            Err(e) => {
                slog::error!(root(), "{e:?}");
                Err(ApiError::JSONSerdeError(e.to_string()))
            }
        },
        Err(e) => Err(ApiError::ReqwestFailed(e.to_string())),
    }
}

async fn get_survey_draft(
    State(db): State<Arc<Client>>,
    Path(id): Path<String>,
) -> Result<Json<Survey>, ApiError> {
    let result: Result<Option<SurveyDocument>, easy_dynamodb::error::DynamoException> =
        db.get(&id).await;
    let survey_draft = match result {
        Ok(Some(v)) => v,
        Ok(None) => return Err(ApiError::SurveyNotFound(id)),
        Err(e) => return Err(ApiError::DynamoQueryException(e.to_string())),
    };
    let mut survey = Survey::default();
    survey.title = survey_draft.title;
    survey.quotas = survey_draft.quotas;
    survey.question_count = survey_draft.questions.len() as u32;
    survey.questions = survey_draft.questions;
    survey.draft_id = Some(survey_draft.id);
    survey.draft_status = Some(survey_draft.status);

    Ok(Json(survey))
}

async fn list_survey_draft(
    State(db): State<Arc<Client>>,
    Query(params): Query<ListSurveyDraftRequest>,
) -> Result<Json<ListSurveyDraftResponse>, ApiError> {
    let result: Result<
        (Option<Vec<SurveyDocument>>, Option<String>),
        easy_dynamodb::error::DynamoException,
    > = db
        .find(
            "type-index",
            params.bookmark,
            Some(params.size.unwrap_or(10)),
            vec![("type", SurveyDocument::get_type())],
        )
        .await;

    let (docs, bookmark) = match result {
        Ok((Some(docs), bookmark)) => (docs, bookmark),
        _ => return Err(ApiError::NotFound),
    };
    let survey_drafts = docs
        .into_iter()
        .map(|survey_draft| {
            let mut survey = SurveySummary::default();
            survey.draft_id = Some(survey_draft.id);
            survey.draft_status = Some(survey_draft.status);
            survey.updated_at = survey_draft.updated_at;
            survey.title = survey_draft.title;
            survey
        })
        .collect();

    Ok(Json(ListSurveyDraftResponse {
        bookmark,
        survey: survey_drafts,
    }))
}

async fn upsert_survey_draft(
    State(db): State<Arc<Client>>,
    Json(req): Json<UpsertSurveyDraftRequest>,
) -> Result<Json<String>, ApiError> {
    let mut survey_draft = if let Some(draft_id) = req.id {
        let result: Result<Option<SurveyDocument>, easy_dynamodb::error::DynamoException> =
            db.get(&draft_id).await;
        match result {
            Ok(Some(v)) => v,
            Ok(None) => return Err(ApiError::SurveyNotFound(draft_id)),
            Err(e) => return Err(ApiError::DynamoQueryException(e.to_string())),
        }
    } else {
        SurveyDocument::new(uuid::Uuid::new_v4().to_string())
    };
    let draft_id = survey_draft.id.clone();
    if let Some(title) = req.title {
        survey_draft.title = title;
    }
    if let Some(questions) = req.questions {
        survey_draft.questions = questions;
    }
    if let Some(quotas) = req.quotas {
        survey_draft.quotas = quotas;
    }

    if let Some(status) = req.status {
        survey_draft.status = status;
    }
    survey_draft.updated_at = chrono::Utc::now().timestamp_millis();
    let _ = db.upsert(survey_draft).await;
    Ok(Json(draft_id))
}

//Survey Draft to Survey(nonce_lab)
async fn create_survey(
    State(db): State<Arc<Client>>,
    Json(req): Json<CreateSurveyRequest>,
) -> Result<Json<CreateSurveyResponse>, ApiError> {
    let draft_id = req.draft_id;
    let result: Result<Option<SurveyDocument>, easy_dynamodb::error::DynamoException> =
        db.get(&draft_id).await;
    let survey_draft = match result {
        Ok(Some(v)) => v,
        Ok(None) => return Err(ApiError::SurveyNotFound(draft_id)),
        Err(e) => return Err(ApiError::DynamoQueryException(e.to_string())),
    };
    let nonce_lab_client = nonce_lab::NonceLabClient::new()
        .map_err(|e| ApiError::ReqwestClientFailed(e.to_string()))?;

    let survey_dto = CreateSurveyDto {
        custom_id: survey_draft.id,
        status: SurveyStatus::InProgress,
        started_at: req.started_at,
        ended_at: req.ended_at,
        title: survey_draft.title,
        quotas: survey_draft.quotas,
        questions: survey_draft.questions,
        description: None,
    };
    let survey: Survey = match nonce_lab_client
        .post("/v1/vendor/survey")
        .json(&survey_dto)
        .send()
        .await
    {
        Ok(v) => match v.json().await {
            Ok(v) => v,
            Err(e) => {
                slog::error!(root(), "{e:?}");
                return Err(ApiError::JSONSerdeError(e.to_string()));
            }
        },
        Err(e) => return Err(ApiError::ReqwestFailed(e.to_string())),
    };

    let _ = db.delete(&draft_id).await;

    Ok(Json(CreateSurveyResponse {
        id: uuid::Uuid::new_v4().to_string(),
        survey_id: survey.id,
    }))
}
