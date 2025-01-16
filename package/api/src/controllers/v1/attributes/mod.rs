use by_axum::{
    axum::{
        extract::{Path, Query, State},
        middleware,
        routing::{get, post},
        Extension, Json, Router,
    },
    log::root,
};
use slog::o;

use crate::{
    common::CommonQueryResponse, middleware::auth::authorization_middleware, utils::error::ApiError,
};

use models::prelude::*;

#[derive(Clone, Debug)]
pub struct AttributeControllerV1 {
    log: slog::Logger,
}

impl AttributeControllerV1 {
    pub fn router() -> Router {
        let log = root().new(o!("api-controller" => "AttributeControllerV1"));
        let ctrl = AttributeControllerV1 { log };

        Router::new()
            .route("/", post(Self::upsert_attribute).get(Self::list_attributes))
            .route(
                "/attributes/:attribute_id",
                post(Self::act_attribute).get(Self::get_attribute),
            )
            .route("/attributes", get(Self::search_attribute))
            .with_state(ctrl)
            .layer(middleware::from_fn(authorization_middleware))
    }

    pub async fn search_attribute(
        Extension(organizations): Extension<OrganizationMiddlewareParams>,
        State(ctrl): State<AttributeControllerV1>,
        Query(params): Query<SearchParams>,
    ) -> Result<Json<CommonQueryResponse<AttributeSummary>>, ApiError> {
        let organization_id = organizations.id;
        let log = ctrl.log.new(o!("api" => "search_attribute"));
        slog::debug!(log, "search_attribute {:?} {:?}", organization_id, params);
        Ok(Json(CommonQueryResponse {
            items: vec![
                AttributeSummary {
                    id: "1".to_string(),
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
                        PanelAttributeDetailInfo {
                            id: Some("3".to_string()),
                            name: "속성3".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("4".to_string()),
                            name: "속성4".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("5".to_string()),
                            name: "속성5".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("6".to_string()),
                            name: "속성6".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("7".to_string()),
                            name: "속성7".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("8".to_string()),
                            name: "속성8".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("9".to_string()),
                            name: "속성9".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("10".to_string()),
                            name: "속성10".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("11".to_string()),
                            name: "속성11".to_string(),
                        },
                    ],
                },
                AttributeSummary {
                    id: "2".to_string(),
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
                        PanelAttributeDetailInfo {
                            id: Some("3".to_string()),
                            name: "속성3".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("4".to_string()),
                            name: "속성4".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("5".to_string()),
                            name: "속성5".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("6".to_string()),
                            name: "속성6".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("7".to_string()),
                            name: "속성7".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("8".to_string()),
                            name: "속성8".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("9".to_string()),
                            name: "속성9".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("10".to_string()),
                            name: "속성10".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("11".to_string()),
                            name: "속성11".to_string(),
                        },
                    ],
                },
                AttributeSummary {
                    id: "3".to_string(),
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
                        PanelAttributeDetailInfo {
                            id: Some("3".to_string()),
                            name: "속성3".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("4".to_string()),
                            name: "속성4".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("5".to_string()),
                            name: "속성5".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("6".to_string()),
                            name: "속성6".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("7".to_string()),
                            name: "속성7".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("8".to_string()),
                            name: "속성8".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("9".to_string()),
                            name: "속성9".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("10".to_string()),
                            name: "속성10".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("11".to_string()),
                            name: "속성11".to_string(),
                        },
                    ],
                },
            ],
            bookmark: None,
        }))
    }

    pub async fn act_attribute(
        Extension(organizations): Extension<OrganizationMiddlewareParams>,
        State(ctrl): State<AttributeControllerV1>,
        Path(attribute_id): Path<String>,
        Json(body): Json<AttributeActionRequest>,
    ) -> Result<(), ApiError> {
        let organization_id = organizations.id;
        let log = ctrl.log.new(o!("api" => "act_attribute"));
        slog::debug!(
            log,
            "act_attribute: {:?} {:?}",
            organization_id,
            attribute_id
        );

        match body {
            AttributeActionRequest::Delete => {
                ctrl.remove_attribute(&organization_id, &attribute_id)
                    .await?;
            }
        }

        Ok(())
    }

    pub async fn get_attribute(
        Extension(organizations): Extension<OrganizationMiddlewareParams>,
        State(ctrl): State<AttributeControllerV1>,
        Path(attribute_id): Path<String>,
    ) -> Result<Json<AttributeSummary>, ApiError> {
        let organization_id = organizations.id;
        let log = ctrl.log.new(o!("api" => "get_attribute"));
        slog::debug!(
            log,
            "get_attribute: {:?} {:?}",
            organization_id,
            attribute_id
        );

        Ok(Json(AttributeSummary {
            id: "1".to_string(),
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
                PanelAttributeDetailInfo {
                    id: Some("3".to_string()),
                    name: "속성3".to_string(),
                },
                PanelAttributeDetailInfo {
                    id: Some("4".to_string()),
                    name: "속성4".to_string(),
                },
                PanelAttributeDetailInfo {
                    id: Some("5".to_string()),
                    name: "속성5".to_string(),
                },
                PanelAttributeDetailInfo {
                    id: Some("6".to_string()),
                    name: "속성6".to_string(),
                },
                PanelAttributeDetailInfo {
                    id: Some("7".to_string()),
                    name: "속성7".to_string(),
                },
                PanelAttributeDetailInfo {
                    id: Some("8".to_string()),
                    name: "속성8".to_string(),
                },
                PanelAttributeDetailInfo {
                    id: Some("9".to_string()),
                    name: "속성9".to_string(),
                },
                PanelAttributeDetailInfo {
                    id: Some("10".to_string()),
                    name: "속성10".to_string(),
                },
                PanelAttributeDetailInfo {
                    id: Some("11".to_string()),
                    name: "속성11".to_string(),
                },
            ],
        }))
    }

    pub async fn upsert_attribute(
        Extension(organizations): Extension<OrganizationMiddlewareParams>,
        State(ctrl): State<AttributeControllerV1>,
        Json(body): Json<UpsertAttributeRequest>,
    ) -> Result<Json<UpsertAttributeRequest>, ApiError> {
        let organization_id = organizations.id;
        let log = ctrl.log.new(o!("api" => "upsert_attribute"));
        slog::debug!(log, "upsert_attribute {:?} {:?}", organization_id, body);
        Ok(Json(UpsertAttributeRequest::default()))
    }

    pub async fn list_attributes(
        Extension(organizations): Extension<OrganizationMiddlewareParams>,
        State(ctrl): State<AttributeControllerV1>,
        Query(pagination): Query<Pagination>,
    ) -> Result<Json<CommonQueryResponse<AttributeSummary>>, ApiError> {
        let organization_id = organizations.id;
        let log = ctrl.log.new(o!("api" => "list_attributes"));
        slog::debug!(
            log,
            "list_attributes {:?} {:?}",
            organization_id,
            pagination
        );

        Ok(Json(CommonQueryResponse {
            items: vec![
                AttributeSummary {
                    id: "1".to_string(),
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
                        PanelAttributeDetailInfo {
                            id: Some("3".to_string()),
                            name: "속성3".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("4".to_string()),
                            name: "속성4".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("5".to_string()),
                            name: "속성5".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("6".to_string()),
                            name: "속성6".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("7".to_string()),
                            name: "속성7".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("8".to_string()),
                            name: "속성8".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("9".to_string()),
                            name: "속성9".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("10".to_string()),
                            name: "속성10".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("11".to_string()),
                            name: "속성11".to_string(),
                        },
                    ],
                },
                AttributeSummary {
                    id: "2".to_string(),
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
                        PanelAttributeDetailInfo {
                            id: Some("3".to_string()),
                            name: "속성3".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("4".to_string()),
                            name: "속성4".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("5".to_string()),
                            name: "속성5".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("6".to_string()),
                            name: "속성6".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("7".to_string()),
                            name: "속성7".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("8".to_string()),
                            name: "속성8".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("9".to_string()),
                            name: "속성9".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("10".to_string()),
                            name: "속성10".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("11".to_string()),
                            name: "속성11".to_string(),
                        },
                    ],
                },
                AttributeSummary {
                    id: "3".to_string(),
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
                        PanelAttributeDetailInfo {
                            id: Some("3".to_string()),
                            name: "속성3".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("4".to_string()),
                            name: "속성4".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("5".to_string()),
                            name: "속성5".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("6".to_string()),
                            name: "속성6".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("7".to_string()),
                            name: "속성7".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("8".to_string()),
                            name: "속성8".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("9".to_string()),
                            name: "속성9".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("10".to_string()),
                            name: "속성10".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("11".to_string()),
                            name: "속성11".to_string(),
                        },
                    ],
                },
                AttributeSummary {
                    id: "4".to_string(),
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
                        PanelAttributeDetailInfo {
                            id: Some("3".to_string()),
                            name: "속성3".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("4".to_string()),
                            name: "속성4".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("5".to_string()),
                            name: "속성5".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("6".to_string()),
                            name: "속성6".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("7".to_string()),
                            name: "속성7".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("8".to_string()),
                            name: "속성8".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("9".to_string()),
                            name: "속성9".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("10".to_string()),
                            name: "속성10".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("11".to_string()),
                            name: "속성11".to_string(),
                        },
                    ],
                },
                AttributeSummary {
                    id: "5".to_string(),
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
                        PanelAttributeDetailInfo {
                            id: Some("3".to_string()),
                            name: "속성3".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("4".to_string()),
                            name: "속성4".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("5".to_string()),
                            name: "속성5".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("6".to_string()),
                            name: "속성6".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("7".to_string()),
                            name: "속성7".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("8".to_string()),
                            name: "속성8".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("9".to_string()),
                            name: "속성9".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("10".to_string()),
                            name: "속성10".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("11".to_string()),
                            name: "속성11".to_string(),
                        },
                    ],
                },
                AttributeSummary {
                    id: "6".to_string(),
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
                        PanelAttributeDetailInfo {
                            id: Some("3".to_string()),
                            name: "속성3".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("4".to_string()),
                            name: "속성4".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("5".to_string()),
                            name: "속성5".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("6".to_string()),
                            name: "속성6".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("7".to_string()),
                            name: "속성7".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("8".to_string()),
                            name: "속성8".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("9".to_string()),
                            name: "속성9".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("10".to_string()),
                            name: "속성10".to_string(),
                        },
                        PanelAttributeDetailInfo {
                            id: Some("11".to_string()),
                            name: "속성11".to_string(),
                        },
                    ],
                },
            ],
            bookmark: None,
        }))
    }
}

impl AttributeControllerV1 {
    pub async fn remove_attribute(
        &self,
        organization_id: &str,
        attribute_id: &str,
    ) -> Result<(), ApiError> {
        let log = self.log.new(o!("api" => "remove_attribute"));
        slog::debug!(
            log,
            "remove_attribute {:?} {:?}",
            organization_id,
            attribute_id
        );
        Ok(())
    }
}
