use axum::{routing::get, Router};
use crate::services::chat_engine::handler::ws_handler;

pub fn router() -> Router {
    Router::new()
        .route("/ws", get(ws_handler))
}
