use axum::{
    routing::{get, post},
    Router,
    extract::{State, Path},
    Json,
};
use crate::db::ChatDb;
use crate::models::{Inbox, Conversation, Message};
use crate::websocket::ws_handler;

#[derive(Clone)]
pub struct AppState {
    pub db: ChatDb,
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/api/v1/inboxes", post(create_inbox).get(list_inboxes))
        .route("/api/v1/conversations", post(create_conversation).get(list_conversations))
        .route("/api/v1/conversations/:id/messages", post(create_message).get(list_messages))
        .route("/ws", get(ws_handler))
        .with_state(state)
}

// Dummy implementations for now to ensure compilation
async fn create_inbox(State(_state): State<AppState>, Json(payload): Json<Inbox>) -> Json<Inbox> {
    Json(payload)
}

async fn list_inboxes(State(_state): State<AppState>) -> Json<Vec<Inbox>> {
    Json(vec![])
}

async fn create_conversation(State(_state): State<AppState>, Json(payload): Json<Conversation>) -> Json<Conversation> {
    Json(payload)
}

async fn list_conversations(State(_state): State<AppState>) -> Json<Vec<Conversation>> {
    Json(vec![])
}

async fn create_message(State(_state): State<AppState>, Path(_id): Path<uuid::Uuid>, Json(payload): Json<Message>) -> Json<Message> {
    Json(payload)
}

async fn list_messages(State(_state): State<AppState>, Path(_id): Path<uuid::Uuid>) -> Json<Vec<Message>> {
    Json(vec![])
}
