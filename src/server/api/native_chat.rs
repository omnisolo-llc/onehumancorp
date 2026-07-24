use axum::{
    routing::get,
    Router,
};
use std::sync::Arc;
use crate::db::DB;
use crate::services::chat::ws::{chat_ws_handler, ChatWsState};

pub fn router(db: Arc<DB>) -> Router {
    Router::new()
        .route("/ws", get(chat_ws_handler))
        .with_state(ChatWsState { db })
}
