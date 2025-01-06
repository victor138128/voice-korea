use by_axum::{
    axum::{
        extract::{Path, Query, State},
        middleware,
        routing::post,
        Extension, Json, Router,
    },
    log::root,
};
use slog::o;

use crate::{
    common::CommonQueryResponse,
    middleware::auth::authorization_middleware,
    utils::{error::ApiError, jwt::Claims},
};

use models::prelude::*;

#[derive(Clone, Debug)]
pub struct GroupControllerV1 {
    log: slog::Logger,
}

#[derive(Debug, serde::Deserialize)]
pub struct Pagination {
    pub size: Option<usize>,
    pub bookmark: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionRequest {
    UpdateName(String),
    Delete,
}

impl GroupControllerV1 {
    pub fn router(_db: std::sync::Arc<easy_dynamodb::Client>) -> Router {
        let log = root().new(o!("api-controller" => "GroupControllerV1"));
        let ctrl = GroupControllerV1 { log };

        Router::new()
            .route("/", post(Self::create_group).get(Self::list_groups))
            .route("/:group_id", post(Self::act_group).get(Self::get_group))
            .layer(middleware::from_fn(authorization_middleware)) //FIXME: fix management authorization
            .with_state(ctrl.clone())
    }

    pub async fn act_group(
        Extension(claims): Extension<Claims>,
        State(ctrl): State<GroupControllerV1>,
        Path(group_id): Path<String>,
        Json(body): Json<ActionRequest>,
    ) -> Result<(), ApiError> {
        let log = ctrl.log.new(o!("api" => "act_group"));
        slog::debug!(log, "act_group: {:?}", group_id);

        match body {
            ActionRequest::UpdateName(group_name) => {
                ctrl.update_group_name(ctrl.clone(), &group_id, group_name)
                    .await?;
            }
            ActionRequest::Delete => {
                ctrl.remove_group(ctrl.clone(), &claims.id, &group_id)
                    .await?;
            }
        }

        Ok(())
    }

    pub async fn create_group(
        Extension(claims): Extension<Claims>,
        State(ctrl): State<GroupControllerV1>,
        Json(body): Json<CreateGroupRequest>,
    ) -> Result<Json<Group>, ApiError> {
        let log = ctrl.log.new(o!("api" => "create_group"));
        slog::debug!(log, "create_group {:?}", body);

        let cli = easy_dynamodb::get_client(log.clone());
        let id = uuid::Uuid::new_v4().to_string();
        let group: Group = (body, id, claims.id).into();

        match cli.create(group.clone()).await {
            Ok(()) => Ok(Json(group)),
            Err(e) => {
                slog::error!(log, "Create Group Failed {e:?}");
                Err(ApiError::DynamoCreateException(e.to_string()))
            }
        }
    }

    pub async fn list_groups(
        State(ctrl): State<GroupControllerV1>,
        Query(pagination): Query<Pagination>,
    ) -> Result<Json<CommonQueryResponse<Group>>, ApiError> {
        let log = ctrl.log.new(o!("api" => "list_groups"));
        slog::debug!(log, "list_groups {:?}", pagination);

        let size = if let Some(size) = pagination.size {
            if size > 100 {
                Some(100)
            } else {
                Some(size as i32)
            }
        } else {
            Some(100)
        };

        let bookmark = pagination.bookmark;

        let res: CommonQueryResponse<Group> =
            CommonQueryResponse::query(&log, "type-index", bookmark, size, vec![("type", "group")])
                .await?;
        Ok(Json(res))
    }

    pub async fn get_group(
        State(ctrl): State<GroupControllerV1>,
        Path(group_id): Path<String>,
    ) -> Result<Json<Group>, ApiError> {
        let log = ctrl.log.new(o!("api" => "get_group"));
        slog::debug!(log, "get_group {:?}", group_id);
        let cli = easy_dynamodb::get_client(log.clone());

        let res = cli.get::<Group>(&group_id).await;

        match res {
            Ok(v) => match v {
                Some(v) => {
                    if !v.r#type.contains("deleted") {
                        Ok(Json(v))
                    } else {
                        Err(ApiError::NotFound)
                    }
                }
                None => Err(ApiError::NotFound),
            },
            Err(e) => {
                slog::error!(log, "Group Query Failed {e:?}");
                Err(ApiError::DynamoQueryException(e.to_string()))
            }
        }
    }
}

impl GroupControllerV1 {
    pub async fn update_group_name(
        &self,
        ctrl: GroupControllerV1,
        group_id: &str,
        group_name: String,
    ) -> Result<(), ApiError> {
        let log = ctrl.log.new(o!("api" => "update_group"));
        slog::debug!(log, "update_group_name {:?} {:?}", group_id, group_name);
        let cli = easy_dynamodb::get_client(log.clone());

        let now = chrono::Utc::now().timestamp_millis();

        let res = cli
            .update(
                group_id,
                vec![
                    ("updated_at", UpdateField::I64(now)),
                    ("name", UpdateField::String(group_name)),
                ],
            )
            .await;

        match res {
            Ok(()) => Ok(()),
            Err(e) => {
                slog::error!(log, "Group name Update Failed {e:?}");
                Err(ApiError::DynamoUpdateException(e.to_string()))
            }
        }
    }

    pub async fn remove_group(
        &self,
        ctrl: GroupControllerV1,
        user_id: &str,
        group_id: &str,
    ) -> Result<(), ApiError> {
        let log = ctrl.log.new(o!("api" => "remove group"));
        slog::debug!(log, "remove group {:?}", group_id);
        let cli = easy_dynamodb::get_client(log.clone());
        let now = chrono::Utc::now().timestamp_millis();

        let res = cli
            .update(
                group_id,
                vec![
                    ("deleted_at", UpdateField::I64(now)),
                    ("type", UpdateField::String(Group::get_deleted_type())),
                    ("gsi1", UpdateField::String(Group::get_gsi_deleted(user_id))),
                ],
            )
            .await;

        match res {
            Ok(()) => Ok(()),
            Err(e) => {
                slog::error!(log, "Remove Group Failed {e:?}");
                Err(ApiError::DynamoUpdateException(e.to_string()))
            }
        }
    }
}
