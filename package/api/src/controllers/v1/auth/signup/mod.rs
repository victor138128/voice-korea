use std::sync::Arc;

use by_axum::axum::Json;
use by_axum::{axum::extract::State, log::root};
use easy_dynamodb::Client;
use models::prelude::{InviteMember, UpdateField};
use models::{
    prelude::{CreateMemberRequest, Member},
    User,
};
use serde::Deserialize;

use super::super::verification::email::{verify_handler, EmailVerifyParams};
use crate::common::CommonQueryResponse;
use crate::utils::{error::ApiError, hash::get_hash_string};

#[derive(Deserialize)]
pub struct SignUpParams {
    pub auth_id: String,
    pub auth_value: String,
    pub email: String,
    pub password: String,
}

pub async fn handler(
    State(db): State<Arc<Client>>,
    Json(body): Json<SignUpParams>,
) -> Result<(), ApiError> {
    let auth_doc_id = verify_handler(
        State(db.clone()),
        Json(EmailVerifyParams {
            id: body.auth_id.clone(),
            value: body.auth_value.clone(),
        }),
    )
    .await?;
    let hashed_pw = get_hash_string(body.password.as_bytes());
    let user = User::new(
        uuid::Uuid::new_v4().to_string(),
        body.email.clone(),
        hashed_pw,
    );

    let result: Result<
        (Option<Vec<models::User>>, Option<String>),
        easy_dynamodb::error::DynamoException,
    > = db
        .find(
            "gsi1-index",
            None,
            Some(1),
            vec![("gsi1", User::gsi1(user.email.clone()))],
        )
        .await;
    match result {
        Ok((Some(docs), Some(_))) => {
            if docs.len() > 0 {
                return Err(ApiError::DuplicateUser);
            }
        }
        _ => (),
    };
    let _ = db.delete(&auth_doc_id);
    let _ = db.create(user).await;

    let member: Member = get_member_data(db.clone(), body).await;

    db.upsert(member.clone())
        .await
        .map_err(|e| ApiError::DynamoCreateException(e.to_string()))?;

    Ok(())
}

async fn get_member_data(db: Arc<Client>, body: SignUpParams) -> Member {
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
