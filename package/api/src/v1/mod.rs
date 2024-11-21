use axum::Router;
mod search;

pub fn router() -> Router {
    Router::new().merge(search::router())
}
