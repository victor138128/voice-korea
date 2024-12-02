use by_axum::axum::Router;

mod auth;
mod search;
mod verification;

pub fn router(db: std::sync::Arc<easy_dynamodb::Client>) -> Router {
    Router::new()
        .nest("/search", search::router())
