use by_axum::axum::routing::put;
use by_axum::axum::Json;
use by_axum::axum::{extract::State, Router};
use by_axum::log::root;
use models::User;
use serde::Deserialize;
use slog::o;

use super::super::verification::email::{verify_handler, EmailVerifyParams};
use crate::utils::{error::ApiError, hash::get_hash_string};

#[derive(Deserialize, Debug)]
pub struct ResetParams {
    pub auth_id: String,
    pub auth_value: String,
    pub email: String,
    pub password: String,
}

#[derive(Clone, Debug)]
pub struct ResetControllerV1 {
    db: std::sync::Arc<easy_dynamodb::Client>,
    log: slog::Logger,
}

impl ResetControllerV1 {
    pub fn router(db: std::sync::Arc<easy_dynamodb::Client>) -> Router {
        let log = root().new(o!("api-controller" => "ResetControllerV1"));
        let ctrl = ResetControllerV1 { db, log };

        Router::new()
            .route("/", put(Self::reset))
            .with_state(ctrl.clone())
    }

    pub async fn reset(
        State(ctrl): State<ResetControllerV1>,
        Json(body): Json<ResetParams>,
    ) -> Result<(), ApiError> {
        let log = ctrl.log.new(o!("api" => "reset"));
        slog::debug!(log, "reset {:?}", body);
        verify_handler(
            State(ctrl.db.clone()),
            Json(EmailVerifyParams {
                id: body.auth_id,
                value: body.auth_value,
            }),
        )
        .await?;
        let email = body.email.clone();

        let result: Result<
            (Option<Vec<User>>, Option<String>),
            easy_dynamodb::error::DynamoException,
        > = ctrl
            .db
            .find(
                "gsi1-index",
                None,
                Some(1),
                vec![("gsi1", User::gsi1(body.email))],
            )
            .await;

        let (docs, _) = match result {
            Ok((Some(docs), Some(_))) => (docs, ()),
            _ => return Err(ApiError::InvalidCredentials(email)),
        };
        let user = match docs.first() {
            Some(user) => user,
            None => return Err(ApiError::InvalidCredentials(email)),
        };
        let hashed_password = get_hash_string(body.password.as_bytes());
        let _ = ctrl
            .db
            .update(&user.id, vec![("password", hashed_password)])
            .await;
        Ok(())
    }
}
