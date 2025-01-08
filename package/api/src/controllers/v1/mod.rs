use by_axum::axum::Router;
use groups::GroupControllerV1;
use members::MemberControllerV1;

mod auth;
mod groups;
mod members;
mod search;
mod survey;
mod verification;

pub fn router(db: std::sync::Arc<easy_dynamodb::Client>) -> Router {
    Router::new()
        .nest("/search", search::router())
        .nest("/auth", auth::router(db.clone()))
        .nest("/verification", verification::router(db.clone()))
        .nest("/survey", survey::router(db.clone()))
        .nest("/groups", GroupControllerV1::router(db.clone()))
        .nest("/members", MemberControllerV1::router(db.clone()))
}
