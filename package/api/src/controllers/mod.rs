use std::sync::Arc;

pub mod v1;

pub fn router(db: Arc<easy_dynamodb::Client>) -> by_axum::axum::Router {
    by_axum::axum::Router::new().nest("/v1", v1::router(db))
}
