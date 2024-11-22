use by_axum::axum::Router;

mod search;

pub fn router() -> Router {
    Router::new().merge(search::router())
}
