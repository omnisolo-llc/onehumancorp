pub mod widget;
pub mod ws;

#[cfg(test)]
mod widget_test;

use axum::{routing::{get, post}, Router};
use std::sync::Arc;
use crate::db::DB;

pub fn router() -> Router<Arc<DB>> {
    Router::new()
        .route("/widget/config", get(widget::get_widget_config_handler))
        .route("/widget/session", post(widget::create_session_handler))
        .route("/widget/messages", get(widget::get_messages_handler))
        .route("/ws", get(ws::ws_handler))
}
