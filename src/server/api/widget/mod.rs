pub mod chat;

use crate::db::DB;
use axum::{
    Router,
    routing::{get, post},
};
use std::sync::Arc;

pub fn router(db: Arc<DB>) -> Router {
    let state = Arc::new(chat::WidgetChatState::new(db));

    Router::new()
        .route("/conversations", post(chat::create_conversation))
        .route("/messages", post(chat::create_message))
        .route(
            "/:tenant_id/conversations/:conversation_id/messages",
            get(chat::get_messages),
        )
        .with_state(state)
}
