use axum::{routing::get, Router};

pub fn create_router() -> Router {
    Router::new().route("/health", get(|| async { "OK" }))
}
