use axum::{
    extract::{Path, State, WebSocketUpgrade},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use axum::extract::ws::WebSocket;
use std::sync::Arc;
use uuid::Uuid;
use crate::domain::omnichannel_chat::service::ChatService;
use serde::Deserialize;

#[derive(Clone)]
pub struct AppState {
    pub chat_service: Arc<ChatService>,
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/inboxes", get(get_inboxes).post(create_inbox))
        .route("/inboxes/:inbox_id/conversations", get(get_conversations).post(create_conversation))
        .route("/conversations/:conversation_id/messages", post(add_message))
        .route("/ws", get(ws_handler))
        .with_state(state)
}

#[derive(Deserialize)]
pub struct CreateInboxRequest {
    pub tenant_id: String,
    pub name: String,
}

async fn create_inbox(
    State(state): State<AppState>,
    Json(payload): Json<CreateInboxRequest>,
) -> impl IntoResponse {
    let inbox = state.chat_service.create_inbox(&payload.tenant_id, &payload.name).await.unwrap();
    Json(inbox)
}

async fn get_inboxes(
    State(state): State<AppState>,
) -> impl IntoResponse {
    // Hardcode tenant_id for simplicity in this example
    let tenant_id = "test-tenant-id".to_string();
    let inboxes = state.chat_service.get_inboxes(&tenant_id).await.unwrap();
    Json(inboxes)
}

#[derive(Deserialize)]
pub struct CreateConversationRequest {
    pub tenant_id: String,
    pub contact_id: Uuid,
    pub status: String,
}

async fn create_conversation(
    State(state): State<AppState>,
    Path(inbox_id): Path<Uuid>,
    Json(payload): Json<CreateConversationRequest>,
) -> impl IntoResponse {
    let conv = state.chat_service.create_conversation(&payload.tenant_id, inbox_id, payload.contact_id, &payload.status).await.unwrap();
    Json(conv)
}

async fn get_conversations(
    State(state): State<AppState>,
    Path(inbox_id): Path<Uuid>,
) -> impl IntoResponse {
    let tenant_id = "test-tenant-id".to_string();
    let convs = state.chat_service.get_conversations(&tenant_id, inbox_id).await.unwrap();
    Json(convs)
}

#[derive(Deserialize)]
pub struct AddMessageRequest {
    pub tenant_id: String,
    pub content: String,
    pub sender_type: String,
    pub sender_id: Option<String>,
}

async fn add_message(
    State(state): State<AppState>,
    Path(conversation_id): Path<Uuid>,
    Json(payload): Json<AddMessageRequest>,
) -> impl IntoResponse {
    let msg = state.chat_service.add_message(&payload.tenant_id, conversation_id, &payload.content, &payload.sender_type, payload.sender_id).await.unwrap();
    Json(msg)
}

async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            if socket.send(msg).await.is_err() {
                break;
            }
        } else {
            break;
        }
    }
}
