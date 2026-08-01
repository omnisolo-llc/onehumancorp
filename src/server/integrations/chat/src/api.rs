use axum::{
    routing::{get, post},
    Router,
};

pub fn router() -> Router {
    Router::new()
        .route("/ws", get(super::ws::ws_handler))
}
