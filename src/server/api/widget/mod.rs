pub mod chat;

use axum::{routing::{get, post}, Router};
use std::sync::Arc;
use crate::db::DB;

pub fn router(db: Arc<DB>) -> Router {
    let state = Arc::new(chat::WidgetChatState::new(db));

    Router::new()
        .route("/conversations", post(chat::create_conversation))
        .route("/messages", post(chat::create_message))
        .route("/:tenant_id/conversations/:conversation_id/messages", get(chat::get_messages))
        .with_state(state)
}
