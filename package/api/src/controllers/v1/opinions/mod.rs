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
pub struct PublicOpinionControllerV1 {
    log: slog::Logger,
}

#[derive(Debug, serde::Deserialize)]
pub struct Pagination {
    pub _size: Option<i32>,
    pub _bookmark: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct SearchParams {
    pub _keyword: String,
}

impl PublicOpinionControllerV1 {
    pub fn router() -> Router {
        let log = root().new(o!("api-controller" => "PublicOpinionControllerV1"));
        let ctrl = PublicOpinionControllerV1 { log };

        Router::new()
            .route(
                "/organizations/:organization_id",
                post(Self::create_opinion).get(Self::list_opinions),
            )
            .route(
                "/organizations/:organization_id/search/opinions",
                get(Self::search_opinion),
            )
            // .route(
            //     "/organizations/:organization_id/opinions/:project_id",
            //     post(Self::act_opinion),
            // )
            .with_state(ctrl)
            .layer(middleware::from_fn(authorization_middleware))
    }

    pub async fn create_opinion(
        State(ctrl): State<PublicOpinionControllerV1>,
        Path(organization_id): Path<String>,
        Json(body): Json<CreateOpinionRequest>,
    ) -> Result<Json<OpinionResponse>, ApiError> {
        let log = ctrl.log.new(o!("api" => "create_opinion"));
        slog::debug!(log, "create_opinion {:?} {:?}", organization_id, body);
        Ok(Json(OpinionResponse {
            project_id: "project id 1".to_string(),
            opinion_type: OpinionType::Economy,
            project_name: "공론주제".to_string(),
            total_response_count: 60,
            response_count: 40,
            panels: vec![
                PanelInfo {
                    id: "1".to_string(),
                    name: "패널1".to_string(),
                },
                PanelInfo {
                    id: "2".to_string(),
                    name: "패널2".to_string(),
                },
                PanelInfo {
                    id: "3".to_string(),
                    name: "패널3".to_string(),
                },
            ],
            start_date: 1759244400,
            end_date: 1764601200,
            status: ProjectStatus::Finish,
        }))
    }

    pub async fn search_opinion(
        State(ctrl): State<PublicOpinionControllerV1>,
        Path(organization_id): Path<String>,
        Query(params): Query<SearchParams>,
    ) -> Result<Json<CommonQueryResponse<OpinionResponse>>, ApiError> {
        let log = ctrl.log.new(o!("api" => "search_opinion"));
        slog::debug!(log, "search_opinion {:?} {:?}", organization_id, params);
        Ok(Json(CommonQueryResponse {
            items: vec![
                OpinionResponse {
                    project_id: "project id 1".to_string(),
                    opinion_type: OpinionType::Economy,
                    project_name: "공론주제".to_string(),
                    total_response_count: 60,
                    response_count: 40,
                    panels: vec![
                        PanelInfo {
                            id: "1".to_string(),
                            name: "패널1".to_string(),
                        },
                        PanelInfo {
                            id: "2".to_string(),
                            name: "패널2".to_string(),
                        },
                        PanelInfo {
                            id: "3".to_string(),
                            name: "패널3".to_string(),
                        },
                    ],
                    start_date: 1759244400,
                    end_date: 1764601200,
                    status: ProjectStatus::Finish,
                },
                OpinionResponse {
                    project_id: "project id 6".to_string(),
                    opinion_type: OpinionType::Economy,
                    project_name: "공론주제".to_string(),
                    total_response_count: 60,
                    response_count: 40,
                    panels: vec![
                        PanelInfo {
                            id: "1".to_string(),
                            name: "패널1".to_string(),
                        },
                        PanelInfo {
                            id: "2".to_string(),
                            name: "패널2".to_string(),
                        },
                        PanelInfo {
                            id: "3".to_string(),
                            name: "패널3".to_string(),
                        },
                    ],
                    start_date: 1759244400,
                    end_date: 1764601200,
                    status: ProjectStatus::InProgress,
                },
            ],
            bookmark: None,
        }))
    }

    pub async fn list_opinions(
        Path(organization_id): Path<String>,
        State(ctrl): State<PublicOpinionControllerV1>,
        Query(pagination): Query<Pagination>,
    ) -> Result<Json<CommonQueryResponse<OpinionResponse>>, ApiError> {
        let log = ctrl.log.new(o!("api" => "list_opinions"));
        slog::debug!(log, "list_opinions {:?} {:?}", organization_id, pagination);
        Ok(Json(CommonQueryResponse {
            items: vec![
                OpinionResponse {
                    project_id: "project id 1".to_string(),
                    opinion_type: OpinionType::Economy,
                    project_name: "공론주제".to_string(),
                    total_response_count: 60,
                    response_count: 40,
                    panels: vec![
                        PanelInfo {
                            id: "1".to_string(),
                            name: "패널1".to_string(),
                        },
                        PanelInfo {
                            id: "2".to_string(),
                            name: "패널2".to_string(),
                        },
                        PanelInfo {
                            id: "3".to_string(),
                            name: "패널3".to_string(),
                        },
                    ],
                    start_date: 1759244400,
                    end_date: 1764601200,
                    status: ProjectStatus::Finish,
                },
                OpinionResponse {
                    project_id: "project id 2".to_string(),
                    opinion_type: OpinionType::Economy,
                    project_name: "공론주제".to_string(),
                    total_response_count: 60,
                    response_count: 40,
                    panels: vec![
                        PanelInfo {
                            id: "1".to_string(),
                            name: "패널1".to_string(),
                        },
                        PanelInfo {
                            id: "2".to_string(),
                            name: "패널2".to_string(),
                        },
                        PanelInfo {
                            id: "3".to_string(),
                            name: "패널3".to_string(),
                        },
                    ],
                    start_date: 1759244400,
                    end_date: 1764601200,
                    status: ProjectStatus::Finish,
                },
                OpinionResponse {
                    project_id: "project id 3".to_string(),
                    opinion_type: OpinionType::Economy,
                    project_name: "공론주제".to_string(),
                    total_response_count: 60,
                    response_count: 40,
                    panels: vec![
                        PanelInfo {
                            id: "1".to_string(),
                            name: "패널1".to_string(),
                        },
                        PanelInfo {
                            id: "2".to_string(),
                            name: "패널2".to_string(),
                        },
                        PanelInfo {
                            id: "3".to_string(),
                            name: "패널3".to_string(),
                        },
                    ],
                    start_date: 1759244400,
                    end_date: 1764601200,
                    status: ProjectStatus::Ready,
                },
                OpinionResponse {
                    project_id: "project id 4".to_string(),
                    opinion_type: OpinionType::Economy,
                    project_name: "공론주제".to_string(),
                    total_response_count: 60,
                    response_count: 40,
                    panels: vec![
                        PanelInfo {
                            id: "1".to_string(),
                            name: "패널1".to_string(),
                        },
                        PanelInfo {
                            id: "2".to_string(),
                            name: "패널2".to_string(),
                        },
                        PanelInfo {
                            id: "3".to_string(),
                            name: "패널3".to_string(),
                        },
                    ],
                    start_date: 1759244400,
                    end_date: 1764601200,
                    status: ProjectStatus::Ready,
                },
                OpinionResponse {
                    project_id: "project id 5".to_string(),
                    opinion_type: OpinionType::Economy,
                    project_name: "공론주제".to_string(),
                    total_response_count: 60,
                    response_count: 40,
                    panels: vec![
                        PanelInfo {
                            id: "1".to_string(),
                            name: "패널1".to_string(),
                        },
                        PanelInfo {
                            id: "2".to_string(),
                            name: "패널2".to_string(),
                        },
                        PanelInfo {
                            id: "3".to_string(),
                            name: "패널3".to_string(),
                        },
                    ],
                    start_date: 1759244400,
                    end_date: 1764601200,
                    status: ProjectStatus::InProgress,
                },
                OpinionResponse {
                    project_id: "project id 6".to_string(),
                    opinion_type: OpinionType::Economy,
                    project_name: "공론주제".to_string(),
                    total_response_count: 60,
                    response_count: 40,
                    panels: vec![
                        PanelInfo {
                            id: "1".to_string(),
                            name: "패널1".to_string(),
                        },
                        PanelInfo {
                            id: "2".to_string(),
                            name: "패널2".to_string(),
                        },
                        PanelInfo {
                            id: "3".to_string(),
                            name: "패널3".to_string(),
                        },
                    ],
                    start_date: 1759244400,
                    end_date: 1764601200,
                    status: ProjectStatus::InProgress,
                },
            ],
            bookmark: None,
        }))
    }
}
