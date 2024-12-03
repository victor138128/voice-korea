use by_axum::axum::{routing::post, Router};

pub mod email;

pub fn router(db: std::sync::Arc<easy_dynamodb::Client>) -> Router {
    Router::new()
        .route("/email/send", post(email::send_handler))
        .route("/email/verify", post(email::verify_handler))
        .with_state(db)
}
