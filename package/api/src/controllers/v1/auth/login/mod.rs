use std::sync::Arc;

use by_axum::axum::extract::State;
use by_axum::axum::http::header::SET_COOKIE;
use by_axum::axum::response::Response;
use by_axum::axum::Json;
use by_axum::log::root;
use easy_dynamodb::Client;
use models::prelude::{CreateMemberRequest, InviteMember, Member, UpdateField};
use serde::Deserialize;

use crate::common::CommonQueryResponse;
use crate::utils::error::ApiError;
use crate::utils::hash::get_hash_string;
use crate::utils::jwt::generate_jwt;

#[derive(Deserialize, Debug)]
pub struct LoginParams {
    email: String,
    password: String,
}

pub async fn handler(
    State(db): State<Arc<Client>>,
    Json(body): Json<LoginParams>,
) -> Result<Response<String>, ApiError> {
    let log = root();
    let cli = easy_dynamodb::get_client(log.clone());
    let email = body.email.clone();
    let users = CommonQueryResponse::<models::User>::query(
        &log,
        "gsi1-index",
        None,
        Some(1),
        vec![("gsi1", models::User::gsi1(body.email.clone()))],
    )
    .await?;

    if users.items.len() == 0 {
        return Err(ApiError::InvalidCredentials(email));
    }
    let user = users.items.first().unwrap();

    let hashed_password = get_hash_string(body.password.as_bytes());
    slog::debug!(
        log,
        "user_password: {} hashed_password: {}",
        user.password,
        hashed_password
    );

    if user.password != hashed_password {
        return Err(ApiError::InvalidCredentials(email));
    }

    let jwt = generate_jwt(&user.id, &user.email)
        .map_err(|e| ApiError::JWTGenerationFail(e.to_string()))?;

    // If the member list has not been added, add the member list
    let res: CommonQueryResponse<Member> = CommonQueryResponse::query(
        &log,
        "gsi1-index",
        None,
        Some(100),
        vec![("gsi1", Member::get_gsi1(body.email.clone()))],
    )
    .await?;

    if res.items.len() == 0 {
        let member: Member = get_member_data(db.clone(), body).await;

        cli.upsert(member.clone())
            .await
            .map_err(|e| ApiError::DynamoCreateException(e.to_string()))?;
    } else {
        let item = res.items.first().unwrap();

        if item.deleted_at.is_some() {
            let member: Member = (
                CreateMemberRequest {
                    email,
                    name: None,
                    group: None,
                    role: None,
                },
                item.id.clone(),
            )
                .into();

            cli.upsert(member.clone())
                .await
                .map_err(|e| ApiError::DynamoCreateException(e.to_string()))?;
        }
    }

    Ok(Response::builder()
        .status(200)
        .header(
            SET_COOKIE,
            format!("token={}; HttpOnly; Secure; SameSite=None; Path=/", jwt),
        )
        .body(jwt)
        .map_err(|e| ApiError::ValidationError(e.to_string()))?)
}

async fn get_member_data(db: Arc<Client>, body: LoginParams) -> Member {
    let now = chrono::Utc::now().timestamp_millis();
    let id = uuid::Uuid::new_v4().to_string();
    let log = root();
    let res: CommonQueryResponse<InviteMember> = CommonQueryResponse::query(
        &log,
        "gsi1-index",
        None,
        Some(1),
        vec![("gsi1", InviteMember::get_gsi1(body.email.clone()))],
    )
    .await
    .unwrap();

    if res.items.len() != 0 {
        let item = res.items.first().unwrap();
        let _ = db
            .update(
                &item.id,
                vec![
                    ("deleted_at", UpdateField::I64(now)),
                    (
                        "type",
                        UpdateField::String(InviteMember::get_deleted_type()),
                    ),
                    (
                        "gsi1",
                        UpdateField::String(InviteMember::get_gsi_deleted(&body.email)),
                    ),
                ],
            )
            .await;

        if item.deleted_at.is_none() {
            (
                CreateMemberRequest {
                    email: body.email.clone(),
                    name: Some(item.name.clone()),
                    group: item.group.clone(),
                    role: item.role.clone(),
                },
                id,
            )
                .into()
        } else {
            (
                CreateMemberRequest {
                    email: body.email.clone(),
                    name: None,
                    group: None,
                    role: None,
                },
                id,
            )
                .into()
        }
    } else {
        (
            CreateMemberRequest {
                email: body.email.clone(),
                name: None,
                group: None,
                role: None,
            },
            id,
        )
            .into()
    }
}
