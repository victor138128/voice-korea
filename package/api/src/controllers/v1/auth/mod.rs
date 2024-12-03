use by_axum::axum::{
    routing::{post, put},
    Router,
};

mod login;
mod reset;
mod signup;

pub fn router(db: std::sync::Arc<easy_dynamodb::Client>) -> Router {
    Router::new()
        .route("/login", post(login::handler))
        .route("/signup", post(signup::handler))
        .route("/reset", put(reset::handler))
        .with_state(db)
}
