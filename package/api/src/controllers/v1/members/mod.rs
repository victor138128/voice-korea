use by_axum::{
    axum::{
        extract::{Path, Query, State},
        middleware,
        routing::post,
        Json, Router,
    },
    log::root,
};
use easy_dynamodb::error::DynamoException;
use slog::o;

use crate::{
    common::CommonQueryResponse, middleware::auth::authorization_middleware, utils::error::ApiError,
};

use models::prelude::*;

#[derive(Debug, serde::Deserialize)]
pub struct Pagination {
    pub size: Option<usize>,
    pub bookmark: Option<String>,
}

#[derive(Clone, Debug)]
pub struct MemberControllerV1 {
    log: slog::Logger,
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionRequest {
    Update(UpdateMemberRequest),
    Delete,
}

impl MemberControllerV1 {
    pub fn router(_db: std::sync::Arc<easy_dynamodb::Client>) -> Router {
        let log = root().new(o!("api-controller" => "MemberControllerV1"));
        let ctrl = MemberControllerV1 { log };

        //FIXME: implement projects api
        Router::new()
            .route("/", post(Self::create_member).get(Self::list_members))
            .route("/:member_id", post(Self::act_member).get(Self::get_member))
            .route("/invite", post(Self::invite_member))
            .layer(middleware::from_fn(authorization_middleware)) //FIXME: fix management authorization (오직 관리자만 해당 함수들 호출할 수 있도록 수정)
            .with_state(ctrl.clone())
    }

    pub async fn invite_member(
        State(ctrl): State<MemberControllerV1>,
        Json(body): Json<InviteMemberRequest>,
    ) -> Result<(), ApiError> {
        let log = ctrl.log.new(o!("api" => "invite_member"));
        let cli = easy_dynamodb::get_client(log.clone());
        slog::debug!(log, "invite_member: {:?}", body);

        //Error: already exists member
        let res: CommonQueryResponse<Member> = CommonQueryResponse::query(
            &log,
            "gsi1-index",
            None,
            Some(1),
            vec![("gsi1", Member::get_gsi1(body.email.clone()))],
        )
        .await?;

        if res.items.len() != 0 {
            let item = res.items.first().unwrap();

            if item.deleted_at.is_none() {
                return Err(ApiError::AlreadyExists);
            }
        }

        //Error: already invited
        let res: CommonQueryResponse<InviteMember> = CommonQueryResponse::query(
            &log,
            "gsi1-index",
            None,
            Some(1),
            vec![("gsi1", InviteMember::get_gsi1(body.email.clone()))],
        )
        .await?;

        if res.items.len() != 0 {
            let item = res.items.first().unwrap();

            if item.deleted_at.is_none() {
                return Err(ApiError::AlreadyExists);
            }
        }

        let id = uuid::Uuid::new_v4().to_string();
        let member: InviteMember = (body, id).into();

        match cli.upsert(member.clone()).await {
            Ok(()) => Ok(()),
            Err(e) => {
                slog::error!(log, "Invite Member Failed {e:?}");
                Err(ApiError::DynamoCreateException(e.to_string()))
            }
        }
    }

    pub async fn act_member(
        State(ctrl): State<MemberControllerV1>,
        Path(member_id): Path<String>,
        Json(body): Json<ActionRequest>,
    ) -> Result<(), ApiError> {
        let log = ctrl.log.new(o!("api" => "act_member"));
        slog::debug!(log, "act_member: {:?}", member_id);

        match body {
            ActionRequest::Update(req) => {
                ctrl.update_member(ctrl.clone(), &member_id, req).await?;
            }
            ActionRequest::Delete => {
                ctrl.remove_member(ctrl.clone(), &member_id).await?;
            }
        }

        Ok(())
    }

    pub async fn create_member(
        State(ctrl): State<MemberControllerV1>,
        Json(body): Json<CreateMemberRequest>,
    ) -> Result<Json<Member>, ApiError> {
        let log = ctrl.log.new(o!("api" => "create_member"));
        slog::debug!(log, "create_member {:?}", body);

        let res: CommonQueryResponse<Member> = CommonQueryResponse::query(
            &log,
            "gsi1-index",
            None,
            Some(100),
            vec![("gsi1", Member::get_gsi1(body.email.clone()))],
        )
        .await?;

        let cli = easy_dynamodb::get_client(log.clone());

        let member = if res.items.len() != 0 {
            let item = res.items.first().unwrap();

            if item.deleted_at.is_some() {
                let mem: Member = (
                    CreateMemberRequest {
                        email: body.email,
                        name: None,
                        group: None,
                        role: None,
                    },
                    item.id.clone(),
                )
                    .into();

                mem
            } else {
                return Err(ApiError::AlreadyExists);
            }
        } else {
            let id = uuid::Uuid::new_v4().to_string();
            let mem: Member = (body, id).into();

            mem
        };

        match cli.upsert(member.clone()).await {
            Ok(()) => Ok(Json(member)),
            Err(e) => {
                slog::error!(log, "Create Group Failed {e:?}");
                Err(ApiError::DynamoCreateException(e.to_string()))
            }
        }
    }

    pub async fn list_members(
        State(ctrl): State<MemberControllerV1>,
        Query(pagination): Query<Pagination>,
    ) -> Result<Json<CommonQueryResponse<Member>>, ApiError> {
        let log = ctrl.log.new(o!("api" => "list_members"));
        slog::debug!(log, "list_members {:?}", pagination);

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

        let res: CommonQueryResponse<Member> = CommonQueryResponse::query(
            &log,
            "type-index",
            bookmark,
            size,
            vec![("type", "member")],
        )
        .await?;
        Ok(Json(res))
    }

    pub async fn get_member(
        State(ctrl): State<MemberControllerV1>,
        Path(member_id): Path<String>,
    ) -> Result<Json<Member>, ApiError> {
        let log = ctrl.log.new(o!("api" => "get_member"));
        slog::debug!(log, "get_member {:?}", member_id);
        let cli = easy_dynamodb::get_client(log.clone());

        let res = cli.get::<Member>(&member_id).await;

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
                slog::error!(log, "Member Query Failed {e:?}");
                Err(ApiError::DynamoQueryException(e.to_string()))
            }
        }
    }
}

impl MemberControllerV1 {
    pub async fn update_member(
        &self,
        ctrl: MemberControllerV1,
        member_id: &str,
        req: UpdateMemberRequest,
    ) -> Result<(), ApiError> {
        let log = ctrl.log.new(o!("api" => "update_member"));
        slog::debug!(log, "update_member");
        let cli = easy_dynamodb::get_client(log.clone());

        let now = chrono::Utc::now().timestamp_millis();

        let mut update_data: Vec<(&str, UpdateField)> = vec![];

        if req.name.is_some() {
            update_data.push(("name", UpdateField::String(req.name.unwrap())));
        }
        if req.group.is_some() {
            update_data.push(("group", UpdateField::String(req.group.unwrap())));
        }
        if req.role.is_some() {
            update_data.push(("role", UpdateField::String(req.role.unwrap())));
        }

        if update_data.len() != 0 {
            update_data.push(("updated_at", UpdateField::I64(now)));
        }

        let res = cli.update(member_id, update_data).await;

        match res {
            Ok(()) => Ok(()),
            Err(e) => {
                slog::error!(log, "Member Update Failed {e:?}");
                Err(ApiError::DynamoUpdateException(e.to_string()))
            }
        }
    }

    pub async fn remove_member(
        &self,
        ctrl: MemberControllerV1,
        member_id: &str,
    ) -> Result<(), ApiError> {
        let log = ctrl.log.new(o!("api" => "remove_member"));
        slog::debug!(log, "remove member {:?}", member_id);
        let cli = easy_dynamodb::get_client(log.clone());
        let now = chrono::Utc::now().timestamp_millis();

        let d: Result<Option<Member>, DynamoException> = match cli.get(member_id).await {
            Ok(v) => Ok(v),
            Err(e) => {
                slog::error!(log, "Member Query Failed {e:?}");
                return Err(ApiError::DynamoQueryException(e.to_string()));
            }
        };

        if d.clone().unwrap().is_none() {
            return Err(ApiError::NotFound);
        }

        let res = cli
            .update(
                member_id,
                vec![
                    ("deleted_at", UpdateField::I64(now)),
                    ("type", UpdateField::String(Member::get_deleted_type())),
                    (
                        "gsi1",
                        UpdateField::String(Member::get_gsi_deleted(&d.unwrap().unwrap().email)),
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
}
