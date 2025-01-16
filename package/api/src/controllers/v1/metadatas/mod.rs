use by_axum::{
    axum::{
        extract::{Path, Query, State},
        middleware,
        routing::{get, post},
        Json, Router,
    },
    log::root,
};
use slog::o;

use crate::{
    common::CommonQueryResponse, middleware::auth::authorization_middleware, utils::error::ApiError,
};

use models::prelude::*;

#[derive(Clone, Debug)]
pub struct MetadataControllerV1 {
    log: slog::Logger,
}

impl MetadataControllerV1 {
    pub fn router() -> Router {
        let log = root().new(o!("api-controller" => "MetadataControllerV1"));
        let ctrl = MetadataControllerV1 { log };

        Router::new()
            .route(
                "/organizations/:organization_id",
                post(Self::upsert_metadata).get(Self::list_metadatas),
            )
            .route(
                "/organizations/:organization_id/metadata/:metadata_id",
                post(Self::act_metadata).get(Self::get_metadata),
            )
            .route(
                "/organizations/:organization_id/metadata",
                get(Self::search_metadata),
            )
            .route("/upload", post(Self::upload_metadata))
            .with_state(ctrl)
            .layer(middleware::from_fn(authorization_middleware))
    }

    pub async fn search_metadata(
        State(ctrl): State<MetadataControllerV1>,
        Path(organization_id): Path<String>,
        Query(params): Query<SearchParams>,
    ) -> Result<Json<CommonQueryResponse<MetadataSummary>>, ApiError> {
        let log = ctrl.log.new(o!("api" => "search_metadata"));
        slog::debug!(log, "search_metadata {:?} {:?}", organization_id, params);
        Ok(Json(CommonQueryResponse {
            items: vec![
                MetadataSummary {
                    id: "1".to_string(),
                    name: "공론자료제목명".to_string(),
                    urls: vec![
                    "https://metadata.dagit.club/images/666e4e5b-fd92-40fb-b60e-111c82c6f914.png"
                        .to_string(),
                ],
                    metadata_type: Some(MetadataType::Report),
                    metadata_field: Some(Field::Economy),
                    metadata_purpose: Some(MetadataPurpose::PublicDiscussion),
                    metadata_source: Some(MetadataSource::Internal),
                    metadata_authority: Some(MetadataAuthority::Public),
                    public_opinion_projects: None,
                    public_survey_projects: None,
                    updated_at: 1759276800,
                },
                MetadataSummary {
                    id: "2".to_string(),
                    name: "공론자료제목명".to_string(),
                    urls: vec![
                    "https://metadata.dagit.club/images/666e4e5b-fd92-40fb-b60e-111c82c6f914.png"
                        .to_string(),
                ],
                    metadata_type: Some(MetadataType::Statistics),
                    metadata_field: Some(Field::Society),
                    metadata_purpose: Some(MetadataPurpose::AcademicResearch),
                    metadata_source: Some(MetadataSource::External),
                    metadata_authority: Some(MetadataAuthority::Restricted),
                    public_opinion_projects: None,
                    public_survey_projects: None,
                    updated_at: 1759276800,
                },
                MetadataSummary {
                    id: "3".to_string(),
                    name: "공론자료제목명".to_string(),
                    urls: vec![
                    "https://metadata.dagit.club/images/666e4e5b-fd92-40fb-b60e-111c82c6f914.png"
                        .to_string(),
                ],
                    metadata_type: Some(MetadataType::Statistics),
                    metadata_field: Some(Field::Environment),
                    metadata_purpose: Some(MetadataPurpose::DevelopmentPolicy),
                    metadata_source: Some(MetadataSource::Goverment),
                    metadata_authority: Some(MetadataAuthority::Private),
                    public_opinion_projects: None,
                    public_survey_projects: None,
                    updated_at: 1759276800,
                },
            ],
            bookmark: None,
        }))
    }

    pub async fn upload_metadata(
        State(ctrl): State<MetadataControllerV1>,
        Json(body): Json<GetPutObjectUriRequest>,
    ) -> Result<Json<GetPutObjectUriResponse>, ApiError> {
        let log = ctrl.log.new(o!("api" => "upload_metadata"));
        slog::debug!(log, "upload_metadata: {:?}", body);
        Ok(Json(GetPutObjectUriResponse {
            presigned_uris: vec![],
            uris: vec![],
        }))
    }

    pub async fn act_metadata(
        State(ctrl): State<MetadataControllerV1>,
        Path((organization_id, metadata_id)): Path<(String, String)>,
        Json(body): Json<MetadataActionRequest>,
    ) -> Result<(), ApiError> {
        let log = ctrl.log.new(o!("api" => "act_metadata"));
        slog::debug!(log, "act_metadata: {:?} {:?}", organization_id, metadata_id);

        match body {
            MetadataActionRequest::Delete => {
                ctrl.remove_metadata(&organization_id, &metadata_id).await?;
            }
        }

        Ok(())
    }

    pub async fn get_metadata(
        State(ctrl): State<MetadataControllerV1>,
        Path((organization_id, metadata_id)): Path<(String, String)>,
    ) -> Result<Json<MetadataSummary>, ApiError> {
        let log = ctrl.log.new(o!("api" => "get_metadata"));
        slog::debug!(log, "get_metadata: {:?} {:?}", organization_id, metadata_id);

        Ok(Json(MetadataSummary {
            id: "1".to_string(),
            name: "공론자료제목명".to_string(),
            urls: vec![
                "https://metadata.dagit.club/images/666e4e5b-fd92-40fb-b60e-111c82c6f914.png"
                    .to_string(),
            ],
            metadata_type: Some(MetadataType::Report),
            metadata_field: Some(Field::Economy),
            metadata_purpose: Some(MetadataPurpose::PublicDiscussion),
            metadata_source: Some(MetadataSource::Internal),
            metadata_authority: Some(MetadataAuthority::Public),
            public_opinion_projects: None,
            public_survey_projects: None,
            updated_at: 1759276800,
        }))
    }

    pub async fn upsert_metadata(
        State(ctrl): State<MetadataControllerV1>,
        Path(organization_id): Path<String>,
        Json(body): Json<UpsertMetadataRequest>,
    ) -> Result<Json<UpsertMetadataRequest>, ApiError> {
        let log = ctrl.log.new(o!("api" => "upsert_metadata"));
        slog::debug!(log, "upsert_metadata {:?} {:?}", organization_id, body);
        Ok(Json(UpsertMetadataRequest::default()))
    }

    pub async fn list_metadatas(
        Path(organization_id): Path<String>,
        State(ctrl): State<MetadataControllerV1>,
        Query(pagination): Query<Pagination>,
    ) -> Result<Json<CommonQueryResponse<MetadataSummary>>, ApiError> {
        let log = ctrl.log.new(o!("api" => "list_metadatas"));
        slog::debug!(log, "list_metadatas {:?} {:?}", organization_id, pagination);

        Ok(Json(CommonQueryResponse {
            items: vec![
                MetadataSummary {
                    id: "1".to_string(),
                    name: "공론자료제목명".to_string(),
                    urls: vec![
                    "https://metadata.dagit.club/images/666e4e5b-fd92-40fb-b60e-111c82c6f914.png"
                        .to_string(),
                ],
                    metadata_type: Some(MetadataType::Report),
                    metadata_field: Some(Field::Economy),
                    metadata_purpose: Some(MetadataPurpose::PublicDiscussion),
                    metadata_source: Some(MetadataSource::Internal),
                    metadata_authority: Some(MetadataAuthority::Public),
                    public_opinion_projects: None,
                    public_survey_projects: None,
                    updated_at: 1759276800,
                },
                MetadataSummary {
                    id: "2".to_string(),
                    name: "공론자료제목명".to_string(),
                    urls: vec![
                    "https://metadata.dagit.club/images/666e4e5b-fd92-40fb-b60e-111c82c6f914.png"
                        .to_string(),
                ],
                    metadata_type: Some(MetadataType::Statistics),
                    metadata_field: Some(Field::Society),
                    metadata_purpose: Some(MetadataPurpose::AcademicResearch),
                    metadata_source: Some(MetadataSource::External),
                    metadata_authority: Some(MetadataAuthority::Restricted),
                    public_opinion_projects: None,
                    public_survey_projects: None,
                    updated_at: 1759276800,
                },
                MetadataSummary {
                    id: "3".to_string(),
                    name: "공론자료제목명".to_string(),
                    urls: vec![
                    "https://metadata.dagit.club/images/666e4e5b-fd92-40fb-b60e-111c82c6f914.png"
                        .to_string(),
                ],
                    metadata_type: Some(MetadataType::Statistics),
                    metadata_field: Some(Field::Environment),
                    metadata_purpose: Some(MetadataPurpose::DevelopmentPolicy),
                    metadata_source: Some(MetadataSource::Goverment),
                    metadata_authority: Some(MetadataAuthority::Private),
                    public_opinion_projects: None,
                    public_survey_projects: None,
                    updated_at: 1759276800,
                },
                MetadataSummary {
                    id: "4".to_string(),
                    name: "공론자료제목명".to_string(),
                    urls: vec![
                    "https://metadata.dagit.club/images/666e4e5b-fd92-40fb-b60e-111c82c6f914.png"
                        .to_string(),
                ],
                    metadata_type: Some(MetadataType::Thesis),
                    metadata_field: Some(Field::Education),
                    metadata_purpose: Some(MetadataPurpose::Education),
                    metadata_source: Some(MetadataSource::Company),
                    metadata_authority: Some(MetadataAuthority::Public),
                    public_opinion_projects: None,
                    public_survey_projects: None,
                    updated_at: 1759276800,
                },
                MetadataSummary {
                    id: "5".to_string(),
                    name: "공론자료제목명".to_string(),
                    urls: vec![
                    "https://metadata.dagit.club/images/666e4e5b-fd92-40fb-b60e-111c82c6f914.png"
                        .to_string(),
                ],
                    metadata_type: Some(MetadataType::Presentation),
                    metadata_field: Some(Field::Technology),
                    metadata_purpose: Some(MetadataPurpose::PublicDiscussion),
                    metadata_source: Some(MetadataSource::Internal),
                    metadata_authority: Some(MetadataAuthority::Public),
                    public_opinion_projects: None,
                    public_survey_projects: None,
                    updated_at: 1759276800,
                },
                MetadataSummary {
                    id: "6".to_string(),
                    name: "공론자료제목명".to_string(),
                    urls: vec![
                    "https://metadata.dagit.club/images/666e4e5b-fd92-40fb-b60e-111c82c6f914.png"
                        .to_string(),
                ],
                    metadata_type: Some(MetadataType::Media),
                    metadata_field: Some(Field::Health),
                    metadata_purpose: Some(MetadataPurpose::PublicDiscussion),
                    metadata_source: Some(MetadataSource::Internal),
                    metadata_authority: Some(MetadataAuthority::Public),
                    public_opinion_projects: None,
                    public_survey_projects: None,
                    updated_at: 1759276800,
                },
            ],
            bookmark: None,
        }))
    }
}

impl MetadataControllerV1 {
    pub async fn remove_metadata(
        &self,
        organization_id: &str,
        metadata_id: &str,
    ) -> Result<(), ApiError> {
        let log = self.log.new(o!("api" => "remove_metadata"));
        slog::debug!(
            log,
            "remove_metadata {:?} {:?}",
            organization_id,
            metadata_id
        );
        Ok(())
    }
}
