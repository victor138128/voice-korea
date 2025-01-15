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
pub struct PanelControllerV1 {
    log: slog::Logger,
}

#[derive(Debug, serde::Deserialize)]
pub struct Pagination {
    pub _size: Option<usize>,
    pub _bookmark: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct SearchParams {
    pub _keyword: String,
}

impl PanelControllerV1 {
    pub fn router() -> Router {
        let log = root().new(o!("api-controller" => "PanelControllerV1"));
        let ctrl = PanelControllerV1 { log };

        Router::new()
            .route(
                "/organizations/:organization_id",
                post(Self::upsert_panel).get(Self::list_panels),
            )
            .route(
                "/organizations/:organization_id/panel/:panel_id",
                post(Self::act_panel).get(Self::get_panel),
            )
            .route(
                "/organizations/:organization_id/search/panels",
                get(Self::search_panel),
            )
            .with_state(ctrl)
            .layer(middleware::from_fn(authorization_middleware))
    }

    pub async fn search_panel(
        State(ctrl): State<PanelControllerV1>,
        Path(organization_id): Path<String>,
        Query(params): Query<SearchParams>,
    ) -> Result<Json<CommonQueryResponse<PanelSummary>>, ApiError> {
        let log = ctrl.log.new(o!("api" => "search_panel"));
        slog::debug!(log, "search_panel {:?} {:?}", organization_id, params);
        Ok(Json(CommonQueryResponse {
            items: vec![
                PanelSummary {
                    id: "1".to_string(),
                    name: "패널명1".to_string(),
                    count: 50,
                    attribute: vec![
                        PanelAttributeInfo {
                            id: Some("1".to_string()),
                            name: "직업".to_string(),
                            attribute: vec![
                                PanelAttributeDetailInfo {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                PanelAttributeDetailInfo {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeInfo {
                            id: Some("2".to_string()),
                            name: "성별".to_string(),
                            attribute: vec![
                                PanelAttributeDetailInfo {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                PanelAttributeDetailInfo {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeInfo {
                            id: Some("3".to_string()),
                            name: "나이".to_string(),
                            attribute: vec![
                                PanelAttributeDetailInfo {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                PanelAttributeDetailInfo {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeInfo {
                            id: Some("4".to_string()),
                            name: "학력".to_string(),
                            attribute: vec![
                                PanelAttributeDetailInfo {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                PanelAttributeDetailInfo {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeInfo {
                            id: Some("5".to_string()),
                            name: "거주지".to_string(),
                            attribute: vec![
                                PanelAttributeDetailInfo {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                PanelAttributeDetailInfo {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeInfo {
                            id: Some("6".to_string()),
                            name: "국적".to_string(),
                            attribute: vec![
                                PanelAttributeDetailInfo {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                PanelAttributeDetailInfo {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                    ],
                },
                PanelSummary {
                    id: "2".to_string(),
                    name: "패널명2".to_string(),
                    count: 50,
                    attribute: vec![
                        PanelAttributeInfo {
                            id: Some("1".to_string()),
                            name: "직업".to_string(),
                            attribute: vec![
                                PanelAttributeDetailInfo {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                PanelAttributeDetailInfo {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeInfo {
                            id: Some("2".to_string()),
                            name: "성별".to_string(),
                            attribute: vec![
                                PanelAttributeDetailInfo {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                PanelAttributeDetailInfo {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeInfo {
                            id: Some("3".to_string()),
                            name: "나이".to_string(),
                            attribute: vec![
                                PanelAttributeDetailInfo {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                PanelAttributeDetailInfo {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeInfo {
                            id: Some("4".to_string()),
                            name: "학력".to_string(),
                            attribute: vec![
                                PanelAttributeDetailInfo {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                PanelAttributeDetailInfo {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeInfo {
                            id: Some("5".to_string()),
                            name: "거주지".to_string(),
                            attribute: vec![
                                PanelAttributeDetailInfo {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                PanelAttributeDetailInfo {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeInfo {
                            id: Some("6".to_string()),
                            name: "국적".to_string(),
                            attribute: vec![
                                PanelAttributeDetailInfo {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                PanelAttributeDetailInfo {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                    ],
                },
                PanelSummary {
                    id: "3".to_string(),
                    name: "패널명3".to_string(),
                    count: 50,
                    attribute: vec![
                        PanelAttributeInfo {
                            id: Some("1".to_string()),
                            name: "직업".to_string(),
                            attribute: vec![
                                PanelAttributeDetailInfo {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                PanelAttributeDetailInfo {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeInfo {
                            id: Some("2".to_string()),
                            name: "성별".to_string(),
                            attribute: vec![
                                PanelAttributeDetailInfo {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                PanelAttributeDetailInfo {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeInfo {
                            id: Some("3".to_string()),
                            name: "나이".to_string(),
                            attribute: vec![
                                PanelAttributeDetailInfo {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                PanelAttributeDetailInfo {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeInfo {
                            id: Some("4".to_string()),
                            name: "학력".to_string(),
                            attribute: vec![
                                PanelAttributeDetailInfo {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                PanelAttributeDetailInfo {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeInfo {
                            id: Some("5".to_string()),
                            name: "거주지".to_string(),
                            attribute: vec![
                                PanelAttributeDetailInfo {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                PanelAttributeDetailInfo {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeInfo {
                            id: Some("6".to_string()),
                            name: "국적".to_string(),
                            attribute: vec![
                                PanelAttributeDetailInfo {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                PanelAttributeDetailInfo {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                    ],
                },
            ],
            bookmark: None,
        }))
    }

    pub async fn act_panel(
        State(ctrl): State<PanelControllerV1>,
        Path((organization_id, panel_id)): Path<(String, String)>,
        Json(body): Json<PanelActionRequest>,
    ) -> Result<(), ApiError> {
        let log = ctrl.log.new(o!("api" => "act_panel"));
        slog::debug!(log, "act_panel: {:?} {:?}", organization_id, panel_id);

        match body {
            PanelActionRequest::Delete => {
                ctrl.remove_panel(&organization_id, &panel_id).await?;
            }
        }

        Ok(())
    }

    pub async fn get_panel(
        State(ctrl): State<PanelControllerV1>,
        Path((organization_id, panel_id)): Path<(String, String)>,
    ) -> Result<Json<PanelSummary>, ApiError> {
        let log = ctrl.log.new(o!("api" => "get_panel"));
        slog::debug!(log, "get_panel: {:?} {:?}", organization_id, panel_id);

        Ok(Json(PanelSummary {
            id: "1".to_string(),
            name: "패널명1".to_string(),
            count: 50,
            attribute: vec![
                PanelAttributeInfo {
                    id: Some("1".to_string()),
                    name: "직업".to_string(),
                    attribute: vec![
                        PanelAttributeDetailInfo {
                            id: Some("1".to_string()),
                            name: "속성1".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("2".to_string()),
                            name: "속성2".to_string(),
                        },
                    ],
                },
                PanelAttributeInfo {
                    id: Some("2".to_string()),
                    name: "성별".to_string(),
                    attribute: vec![
                        PanelAttributeDetailInfo {
                            id: Some("1".to_string()),
                            name: "속성1".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("2".to_string()),
                            name: "속성2".to_string(),
                        },
                    ],
                },
                PanelAttributeInfo {
                    id: Some("3".to_string()),
                    name: "나이".to_string(),
                    attribute: vec![
                        PanelAttributeDetailInfo {
                            id: Some("1".to_string()),
                            name: "속성1".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("2".to_string()),
                            name: "속성2".to_string(),
                        },
                    ],
                },
                PanelAttributeInfo {
                    id: Some("4".to_string()),
                    name: "학력".to_string(),
                    attribute: vec![
                        PanelAttributeDetailInfo {
                            id: Some("1".to_string()),
                            name: "속성1".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("2".to_string()),
                            name: "속성2".to_string(),
                        },
                    ],
                },
                PanelAttributeInfo {
                    id: Some("5".to_string()),
                    name: "거주지".to_string(),
                    attribute: vec![
                        PanelAttributeDetailInfo {
                            id: Some("1".to_string()),
                            name: "속성1".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("2".to_string()),
                            name: "속성2".to_string(),
                        },
                    ],
                },
                PanelAttributeInfo {
                    id: Some("6".to_string()),
                    name: "국적".to_string(),
                    attribute: vec![
                        PanelAttributeDetailInfo {
                            id: Some("1".to_string()),
                            name: "속성1".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("2".to_string()),
                            name: "속성2".to_string(),
                        },
                    ],
                },
            ],
        }))
    }

    pub async fn upsert_panel(
        State(ctrl): State<PanelControllerV1>,
        Path(organization_id): Path<String>,
        Json(body): Json<UpsertPanelRequest>,
    ) -> Result<Json<UpsertPanelRequest>, ApiError> {
        let log = ctrl.log.new(o!("api" => "upsert_panel"));
        slog::debug!(log, "upsert_panel {:?} {:?}", organization_id, body);
        Ok(Json(UpsertPanelRequest::default()))
    }

    pub async fn list_panels(
        Path(organization_id): Path<String>,
        State(ctrl): State<PanelControllerV1>,
        Query(pagination): Query<Pagination>,
    ) -> Result<Json<CommonQueryResponse<PanelSummary>>, ApiError> {
        let log = ctrl.log.new(o!("api" => "list_panels"));
        slog::debug!(log, "list_panels {:?} {:?}", organization_id, pagination);

        Ok(Json(CommonQueryResponse {
            items: vec![
                PanelSummary {
                    id: "1".to_string(),
                    name: "패널명1".to_string(),
                    count: 50,
                    attribute: vec![
                        PanelAttributeInfo {
                            id: Some("1".to_string()),
                            name: "직업".to_string(),
                            attribute: vec![
                                PanelAttributeDetailInfo {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                PanelAttributeDetailInfo {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeInfo {
                            id: Some("2".to_string()),
                            name: "성별".to_string(),
                            attribute: vec![
                                PanelAttributeDetailInfo {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                PanelAttributeDetailInfo {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeInfo {
                            id: Some("3".to_string()),
                            name: "나이".to_string(),
                            attribute: vec![
                                PanelAttributeDetailInfo {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                PanelAttributeDetailInfo {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeInfo {
                            id: Some("4".to_string()),
                            name: "학력".to_string(),
                            attribute: vec![
                                PanelAttributeDetailInfo {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                PanelAttributeDetailInfo {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeInfo {
                            id: Some("5".to_string()),
                            name: "거주지".to_string(),
                            attribute: vec![
                                PanelAttributeDetailInfo {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                PanelAttributeDetailInfo {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeInfo {
                            id: Some("6".to_string()),
                            name: "국적".to_string(),
                            attribute: vec![
                                PanelAttributeDetailInfo {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                PanelAttributeDetailInfo {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                    ],
                },
                PanelSummary {
                    id: "2".to_string(),
                    name: "패널명2".to_string(),
                    count: 50,
                    attribute: vec![
                        PanelAttributeInfo {
                            id: Some("1".to_string()),
                            name: "직업".to_string(),
                            attribute: vec![
                                PanelAttributeDetailInfo {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                PanelAttributeDetailInfo {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeInfo {
                            id: Some("2".to_string()),
                            name: "성별".to_string(),
                            attribute: vec![
                                PanelAttributeDetailInfo {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                PanelAttributeDetailInfo {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeInfo {
                            id: Some("3".to_string()),
                            name: "나이".to_string(),
                            attribute: vec![
                                PanelAttributeDetailInfo {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                PanelAttributeDetailInfo {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeInfo {
                            id: Some("4".to_string()),
                            name: "학력".to_string(),
                            attribute: vec![
                                PanelAttributeDetailInfo {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                PanelAttributeDetailInfo {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeInfo {
                            id: Some("5".to_string()),
                            name: "거주지".to_string(),
                            attribute: vec![
                                PanelAttributeDetailInfo {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                PanelAttributeDetailInfo {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeInfo {
                            id: Some("6".to_string()),
                            name: "국적".to_string(),
                            attribute: vec![
                                PanelAttributeDetailInfo {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                PanelAttributeDetailInfo {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                    ],
                },
                PanelSummary {
                    id: "3".to_string(),
                    name: "패널명3".to_string(),
                    count: 50,
                    attribute: vec![
                        PanelAttributeInfo {
                            id: Some("1".to_string()),
                            name: "직업".to_string(),
                            attribute: vec![
                                PanelAttributeDetailInfo {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                PanelAttributeDetailInfo {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeInfo {
                            id: Some("2".to_string()),
                            name: "성별".to_string(),
                            attribute: vec![
                                PanelAttributeDetailInfo {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                PanelAttributeDetailInfo {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeInfo {
                            id: Some("3".to_string()),
                            name: "나이".to_string(),
                            attribute: vec![
                                PanelAttributeDetailInfo {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                PanelAttributeDetailInfo {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeInfo {
                            id: Some("4".to_string()),
                            name: "학력".to_string(),
                            attribute: vec![
                                PanelAttributeDetailInfo {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                PanelAttributeDetailInfo {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeInfo {
                            id: Some("5".to_string()),
                            name: "거주지".to_string(),
                            attribute: vec![
                                PanelAttributeDetailInfo {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                PanelAttributeDetailInfo {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeInfo {
                            id: Some("6".to_string()),
                            name: "국적".to_string(),
                            attribute: vec![
                                PanelAttributeDetailInfo {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                PanelAttributeDetailInfo {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                    ],
                },
                PanelSummary {
                    id: "4".to_string(),
                    name: "패널명4".to_string(),
                    count: 50,
                    attribute: vec![
                        PanelAttributeInfo {
                            id: Some("1".to_string()),
                            name: "직업".to_string(),
                            attribute: vec![
                                PanelAttributeDetailInfo {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                PanelAttributeDetailInfo {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeInfo {
                            id: Some("2".to_string()),
                            name: "성별".to_string(),
                            attribute: vec![
                                PanelAttributeDetailInfo {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                PanelAttributeDetailInfo {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeInfo {
                            id: Some("3".to_string()),
                            name: "나이".to_string(),
                            attribute: vec![
                                PanelAttributeDetailInfo {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                PanelAttributeDetailInfo {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeInfo {
                            id: Some("4".to_string()),
                            name: "학력".to_string(),
                            attribute: vec![
                                PanelAttributeDetailInfo {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                PanelAttributeDetailInfo {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeInfo {
                            id: Some("5".to_string()),
                            name: "거주지".to_string(),
                            attribute: vec![
                                PanelAttributeDetailInfo {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                PanelAttributeDetailInfo {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeInfo {
                            id: Some("6".to_string()),
                            name: "국적".to_string(),
                            attribute: vec![
                                PanelAttributeDetailInfo {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                PanelAttributeDetailInfo {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                    ],
                },
                PanelSummary {
                    id: "5".to_string(),
                    name: "패널명5".to_string(),
                    count: 50,
                    attribute: vec![
                        PanelAttributeInfo {
                            id: Some("1".to_string()),
                            name: "직업".to_string(),
                            attribute: vec![
                                PanelAttributeDetailInfo {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                PanelAttributeDetailInfo {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeInfo {
                            id: Some("2".to_string()),
                            name: "성별".to_string(),
                            attribute: vec![
                                PanelAttributeDetailInfo {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                PanelAttributeDetailInfo {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeInfo {
                            id: Some("3".to_string()),
                            name: "나이".to_string(),
                            attribute: vec![
                                PanelAttributeDetailInfo {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                PanelAttributeDetailInfo {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeInfo {
                            id: Some("4".to_string()),
                            name: "학력".to_string(),
                            attribute: vec![
                                PanelAttributeDetailInfo {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                PanelAttributeDetailInfo {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeInfo {
                            id: Some("5".to_string()),
                            name: "거주지".to_string(),
                            attribute: vec![
                                PanelAttributeDetailInfo {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                PanelAttributeDetailInfo {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeInfo {
                            id: Some("6".to_string()),
                            name: "국적".to_string(),
                            attribute: vec![
                                PanelAttributeDetailInfo {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                PanelAttributeDetailInfo {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                    ],
                },
                PanelSummary {
                    id: "6".to_string(),
                    name: "패널명6".to_string(),
                    count: 50,
                    attribute: vec![
                        PanelAttributeInfo {
                            id: Some("1".to_string()),
                            name: "직업".to_string(),
                            attribute: vec![
                                PanelAttributeDetailInfo {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                PanelAttributeDetailInfo {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeInfo {
                            id: Some("2".to_string()),
                            name: "성별".to_string(),
                            attribute: vec![
                                PanelAttributeDetailInfo {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                PanelAttributeDetailInfo {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeInfo {
                            id: Some("3".to_string()),
                            name: "나이".to_string(),
                            attribute: vec![
                                PanelAttributeDetailInfo {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                PanelAttributeDetailInfo {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeInfo {
                            id: Some("4".to_string()),
                            name: "학력".to_string(),
                            attribute: vec![
                                PanelAttributeDetailInfo {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                PanelAttributeDetailInfo {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeInfo {
                            id: Some("5".to_string()),
                            name: "거주지".to_string(),
                            attribute: vec![
                                PanelAttributeDetailInfo {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                PanelAttributeDetailInfo {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeInfo {
                            id: Some("6".to_string()),
                            name: "국적".to_string(),
                            attribute: vec![
                                PanelAttributeDetailInfo {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                PanelAttributeDetailInfo {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                    ],
                },
                PanelSummary {
                    id: "7".to_string(),
                    name: "패널명7".to_string(),
                    count: 50,
                    attribute: vec![
                        PanelAttributeInfo {
                            id: Some("1".to_string()),
                            name: "직업".to_string(),
                            attribute: vec![
                                PanelAttributeDetailInfo {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                PanelAttributeDetailInfo {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeInfo {
                            id: Some("2".to_string()),
                            name: "성별".to_string(),
                            attribute: vec![
                                PanelAttributeDetailInfo {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                PanelAttributeDetailInfo {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeInfo {
                            id: Some("3".to_string()),
                            name: "나이".to_string(),
                            attribute: vec![
                                PanelAttributeDetailInfo {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                PanelAttributeDetailInfo {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeInfo {
                            id: Some("4".to_string()),
                            name: "학력".to_string(),
                            attribute: vec![
                                PanelAttributeDetailInfo {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                PanelAttributeDetailInfo {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeInfo {
                            id: Some("5".to_string()),
                            name: "거주지".to_string(),
                            attribute: vec![
                                PanelAttributeDetailInfo {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                PanelAttributeDetailInfo {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                        PanelAttributeInfo {
                            id: Some("6".to_string()),
                            name: "국적".to_string(),
                            attribute: vec![
                                PanelAttributeDetailInfo {
                                    id: Some("1".to_string()),
                                    name: "속성1".to_string(),
                                },
                                PanelAttributeDetailInfo {
                                    id: Some("2".to_string()),
                                    name: "속성2".to_string(),
                                },
                            ],
                        },
                    ],
                },
            ],
            bookmark: None,
        }))
    }
}

impl PanelControllerV1 {
    pub async fn remove_panel(
        &self,
        organization_id: &str,
        panel_id: &str,
    ) -> Result<(), ApiError> {
        let log = self.log.new(o!("api" => "remove_panel"));
        slog::debug!(log, "remove_panel {:?} {:?}", organization_id, panel_id);
        Ok(())
    }
}
