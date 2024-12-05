use by_axum::axum::Router;

mod auth;
mod search;
mod survey;
mod verification;

pub fn router(db: std::sync::Arc<easy_dynamodb::Client>) -> Router {
    Router::new()
        .nest("/search", search::router())
        .nest("/auth", auth::router(db.clone()))
        .nest("/verification", verification::router(db.clone()))
        .nest("/survey", survey::router(db))
}
