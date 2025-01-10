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
        Json(body): Json<GroupActionRequest>,
    ) -> Result<(), ApiError> {
        let log = ctrl.log.new(o!("api" => "act_group"));
        slog::debug!(log, "act_group: {:?}", group_id);

        match body {
            GroupActionRequest::UpdateName(group_name) => {
                ctrl.update_group_name(ctrl.clone(), &group_id, group_name)
                    .await?;
            }
            GroupActionRequest::Delete => {
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
        let group: Group = (body.clone(), id.clone(), claims.id).into();

        match cli.create(group.clone()).await {
            Ok(()) => {
                for member in body.members.clone() {
                    let _ = ctrl
                        .clone()
                        .upsert_group_member(
                            ctrl.clone(),
                            id.clone(),
                            body.name.clone(),
                            member.user_id,
                        )
                        .await?;
                }

                Ok(Json(group))
            }
            Err(e) => {
                slog::error!(log, "Create Group Failed {e:?}");
                Err(ApiError::DynamoCreateException(e.to_string()))
            }
        }
    }

    pub async fn list_groups(
        State(ctrl): State<GroupControllerV1>,
        Query(pagination): Query<Pagination>,
    ) -> Result<Json<CommonQueryResponse<GroupResponse>>, ApiError> {
        let log = ctrl.log.new(o!("api" => "list_groups"));
        let cli = easy_dynamodb::get_client(log.clone());
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

        let mut groups: Vec<GroupResponse> = vec![];

        for group in res.items {
            //FIXME: fix to parameter
            let res: CommonQueryResponse<GroupMember> = CommonQueryResponse::query(
                &log,
                "gsi1-index",
                None,
                Some(100),
                vec![("gsi1", GroupMember::get_gsi1(&group.id))],
            )
            .await?;
            let mut members: Vec<GroupMemberResponse> = vec![];

            for item in res.items {
                let member = cli
                    .get::<Member>(&item.user_id)
                    .await
                    .map_err(|e| ApiError::DynamoQueryException(e.to_string()));

                let mem = member.unwrap().unwrap();
                members.push(GroupMemberResponse {
                    id: item.id,
                    created_at: item.created_at,
                    updated_at: item.updated_at,
                    deleted_at: item.deleted_at,
                    group_id: item.group_id,
                    user_id: item.user_id,
                    user_name: mem.name.clone().unwrap_or_default(),
                    user_email: mem.email.clone(),
                    role_name: mem.role,
                    group_name: mem.group.unwrap_or_default(),
                });
            }

            groups.push(GroupResponse {
                id: group.id.clone(),
                creator: group.creator.clone(),
                created_at: group.created_at.clone(),
                updated_at: group.updated_at.clone(),
                deleted_at: group.deleted_at,
                name: group.name,
                members,
                // FIXME: implement projects api
                public_opinion_projects: vec![],
                investigation_projects: vec![],
            });
        }

        Ok(Json(CommonQueryResponse {
            items: groups,
            bookmark: res.bookmark,
        }))
    }

    pub async fn get_group(
        State(ctrl): State<GroupControllerV1>,
        Path(group_id): Path<String>,
    ) -> Result<Json<GroupResponse>, ApiError> {
        let log = ctrl.log.new(o!("api" => "get_group"));
        slog::debug!(log, "get_group {:?}", group_id);
        let cli = easy_dynamodb::get_client(log.clone());

        let res = cli.get::<Group>(&group_id).await;

        match res {
            Ok(v) => match v {
                Some(v) => {
                    if !v.r#type.contains("deleted") {
                        let res: CommonQueryResponse<GroupMember> = CommonQueryResponse::query(
                            &log,
                            "gsi1-index",
                            None,
                            Some(100),
                            vec![("gsi1", GroupMember::get_gsi1(&group_id))],
                        )
                        .await?;

                        let mut members: Vec<GroupMemberResponse> = vec![];

                        for item in res.items {
                            let member = cli
                                .get::<Member>(&item.user_id)
                                .await
                                .map_err(|e| ApiError::DynamoQueryException(e.to_string()));

                            let mem = member.unwrap().unwrap();
                            members.push(GroupMemberResponse {
                                id: item.id,
                                created_at: item.created_at,
                                updated_at: item.updated_at,
                                deleted_at: item.deleted_at,
                                group_id: item.group_id,
                                user_id: item.user_id,
                                user_name: mem.name.clone().unwrap_or_default(),
                                user_email: mem.email.clone(),
                                role_name: mem.role,
                                group_name: mem.group.unwrap_or_default(),
                            });
                        }
                        Ok(Json(GroupResponse {
                            id: v.id.clone(),
                            creator: v.creator.clone(),
                            created_at: v.created_at.clone(),
                            updated_at: v.updated_at.clone(),
                            deleted_at: v.deleted_at,
                            name: v.name,
                            members,
                            // FIXME: implement projects api
                            public_opinion_projects: vec![],
                            investigation_projects: vec![],
                        }))
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

// TODO: refactoring to group member model
impl GroupControllerV1 {
    pub async fn _remove_group_member(
        &self,
        ctrl: GroupControllerV1,
        member_id: String,
    ) -> Result<(), ApiError> {
        let log = ctrl.log.new(o!("api" => "update_member"));
        slog::debug!(log, "update_group_member");
        let cli = easy_dynamodb::get_client(log.clone());

        //check member
        let res = cli
            .get::<Member>(&member_id)
            .await
            .map_err(|e| ApiError::DynamoQueryException(e.to_string()))?;

        if res.is_none() {
            return Err(ApiError::NotFound);
        }

        // check member in group
        let res: CommonQueryResponse<GroupMember> = CommonQueryResponse::query(
            &log,
            "gsi2-index",
            None,
            Some(1),
            vec![("gsi2", GroupMember::get_gsi2(&member_id))],
        )
        .await?;

        if res.items.len() == 0 {
            return Ok(());
        }

        let group_member = res.items.first().unwrap();
        let now = chrono::Utc::now().timestamp_millis();

        let res = cli
            .update(
                &group_member.id,
                vec![
                    ("deleted_at", UpdateField::I64(now)),
                    ("type", UpdateField::String(GroupMember::get_deleted_type())),
                    (
                        "gsi1",
                        UpdateField::String(GroupMember::get_gsi1_deleted(&group_member.group_id)),
                    ),
                    (
                        "gsi2",
                        UpdateField::String(GroupMember::get_gsi2_deleted(&group_member.user_id)),
                    ),
                ],
            )
            .await;

        match res {
            Ok(()) => Ok(()),
            Err(e) => {
                slog::error!(log, "Remove Member Failed {e:?}");
                Err(ApiError::DynamoUpdateException(e.to_string()))
            }
        }
    }

    pub async fn upsert_group_member(
        &self,
        ctrl: GroupControllerV1,
        group_id: String,
        group_name: String,
        member_id: String,
    ) -> Result<(), ApiError> {
        let log = ctrl.log.new(o!("api" => "update_member"));
        slog::debug!(log, "update_group_member");
        let cli = easy_dynamodb::get_client(log.clone());

        //check member
        let res = cli
            .get::<Member>(&member_id)
            .await
            .map_err(|e| ApiError::DynamoQueryException(e.to_string()))?;

        if res.is_none() {
            return Err(ApiError::NotFound);
        }

        let member = res.unwrap();

        // check member in group
        let res: CommonQueryResponse<GroupMember> = CommonQueryResponse::query(
            &log,
            "gsi2-index",
            None,
            Some(1),
            vec![("gsi2", GroupMember::get_gsi2(&member_id))],
        )
        .await?;
        let now = chrono::Utc::now().timestamp_millis();

        if res.items.len() == 0 {
            //group member not exists
            let id = uuid::Uuid::new_v4().to_string();
            let group_member = GroupMember::new(id, group_id, member.id.clone());

            match cli.upsert(group_member.clone()).await {
                Ok(()) => {
                    let _ = cli
                        .update(
                            &member.id,
                            vec![
                                ("group", UpdateField::String(group_name)),
                                ("updated_at", UpdateField::I64(now)),
                            ],
                        )
                        .await
                        .map_err(|e| ApiError::DynamoUpdateException(e.to_string()));
                    return Ok(());
                }
                Err(e) => {
                    slog::error!(log, "Create Group Member Failed {e:?}");
                    return Err(ApiError::DynamoCreateException(e.to_string()));
                }
            }
        } else {
            //group member exists
            let item = res.items.first().unwrap();

            if item.deleted_at.is_some() {
                let group_member = GroupMember::new(item.id.clone(), group_id, member.id.clone());

                match cli.upsert(group_member.clone()).await {
                    Ok(()) => {
                        let _ = cli
                            .update(
                                &member.id,
                                vec![
                                    ("group", UpdateField::String(group_name)),
                                    ("updated_at", UpdateField::I64(now)),
                                ],
                            )
                            .await
                            .map_err(|e| ApiError::DynamoUpdateException(e.to_string()));
                        return Ok(());
                    }
                    Err(e) => {
                        slog::error!(log, "Create Group Member Failed {e:?}");
                        return Err(ApiError::DynamoCreateException(e.to_string()));
                    }
                }
            } else {
                let mut update_data: Vec<(&str, UpdateField)> = vec![];
                let now = chrono::Utc::now().timestamp_millis();
                update_data.push((
                    "gsi1",
                    UpdateField::String(GroupMember::get_gsi1(&group_id)),
                ));
                update_data.push((
                    "gsi2",
                    UpdateField::String(GroupMember::get_gsi2(&member.id)),
                ));
                update_data.push(("group_id", UpdateField::String(group_id)));
                update_data.push(("user_id", UpdateField::String(member.id.clone())));
                update_data.push((
                    "user_name",
                    UpdateField::String(member.name.unwrap_or_default()),
                ));
                update_data.push(("user_email", UpdateField::String(member.email)));
                update_data.push(("updated_at", UpdateField::I64(now)));

                cli.update(&item.id, update_data)
                    .await
                    .map_err(|e| ApiError::DynamoUpdateException(e.to_string()))?;

                let _ = cli
                    .update(
                        &member.id,
                        vec![
                            ("group", UpdateField::String(group_name)),
                            ("updated_at", UpdateField::I64(now)),
                        ],
                    )
                    .await
                    .map_err(|e| ApiError::DynamoUpdateException(e.to_string()));
            }
        }
        Ok(())
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
                    ("name", UpdateField::String(group_name.clone())),
                ],
            )
            .await;

        match res {
            Ok(()) => {
                let mut bookmark = None;
                loop {
                    // remove member from group
                    let res: CommonQueryResponse<GroupMember> = CommonQueryResponse::query(
                        &log,
                        "gsi1-index",
                        bookmark,
                        Some(100),
                        vec![("gsi1", GroupMember::get_gsi1(&group_id))],
                    )
                    .await?;

                    for member in res.items {
                        let _ = cli
                            .update(
                                &member.user_id,
                                vec![
                                    ("updated_at", UpdateField::I64(now)),
                                    ("group", UpdateField::String(group_name.clone())),
                                ],
                            )
                            .await
                            .map_err(|e| ApiError::DynamoUpdateException(e.to_string()))?;
                    }

                    if res.bookmark.is_none() {
                        break;
                    }

                    bookmark = res.bookmark;
                }
                Ok(())
            }
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

        let _ = cli
            .update(
                group_id,
                vec![
                    ("deleted_at", UpdateField::I64(now)),
                    ("type", UpdateField::String(Group::get_deleted_type())),
                    ("gsi1", UpdateField::String(Group::get_gsi_deleted(user_id))),
                ],
            )
            .await
            .map_err(|e| ApiError::DynamoQueryException(e.to_string()));

        let mut bookmark = None;
        loop {
            // remove member from group
            let res: CommonQueryResponse<GroupMember> = CommonQueryResponse::query(
                &log,
                "gsi1-index",
                bookmark,
                Some(100),
                vec![("gsi1", GroupMember::get_gsi1(&group_id))],
            )
            .await?;

            for member in res.items {
                let _ = cli
                    .update(
                        &member.id,
                        vec![
                            ("deleted_at", UpdateField::I64(now)),
                            ("type", UpdateField::String(GroupMember::get_deleted_type())),
                            (
                                "gsi1",
                                UpdateField::String(GroupMember::get_gsi1_deleted(group_id)),
                            ),
                            (
                                "gsi2",
                                UpdateField::String(GroupMember::get_gsi2_deleted(&member.user_id)),
                            ),
                        ],
                    )
                    .await
                    .map_err(|e| ApiError::DynamoUpdateException(e.to_string()))?;

                let _ = cli
                    .update(
                        &member.user_id,
                        vec![("group", UpdateField::String("".to_string()))],
                    )
                    .await
                    .map_err(|e| ApiError::DynamoUpdateException(e.to_string()));
            }

            if res.bookmark.is_none() {
                break;
            }

            bookmark = res.bookmark;
        }

        Ok(())
    }
}
