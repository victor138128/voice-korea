use by_axum::axum::Router;
use login::LoginControllerV1;
use reset::ResetControllerV1;
use signup::SignupControllerV1;

mod login;
mod reset;
mod signup;

pub fn router(db: std::sync::Arc<easy_dynamodb::Client>) -> Router {
    Router::new()
        .nest("/login", LoginControllerV1::router(db.clone()))
        .nest("/signup", SignupControllerV1::router(db.clone()))
        .nest("/reset", ResetControllerV1::router(db.clone()))
}
