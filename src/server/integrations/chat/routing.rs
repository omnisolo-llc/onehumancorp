use axum::{
    extract::{Path, State, ws::{WebSocketUpgrade}},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
// Assuming DB is available in server_lib
// For independent compilation testing we will use a generic pool or DB trait, or just mock it.
// To keep it simple, we will use a dummy state for cargo check and rely on the real `server_lib` state via Bazel later if needed, but since we are compiling this as a standalone crate for `cargo check`, we should define a struct.

#[derive(Clone)]
pub struct ChatState {
    // In a real app this would hold a reference to `sqlx::PgPool` or similar.
    // For now we will just use a generic DB.
}

#[derive(Serialize)]
pub struct ChatInboxResponse {
    pub id: Uuid,
    pub name: String,
    pub channel_type: String,
}

#[derive(Serialize)]
pub struct ChatConversationResponse {
    pub id: Uuid,
    pub inbox_id: Uuid,
    pub contact_id: Uuid,
    pub status: String,
}

#[derive(Serialize)]
pub struct ChatMessageResponse {
    pub id: Uuid,
    pub conversation_id: Uuid,
    pub content: String,
    pub message_type: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Deserialize)]
pub struct StartConversationRequest {
    pub contact_id: Uuid,
    pub inbox_id: Uuid,
    pub initial_message: String,
}

#[derive(Deserialize)]
pub struct SendMessageRequest {
    pub conversation_id: Uuid,
    pub content: String,
}

// These endpoints need a db connection. We will just return unimplemented for now to satisfy cargo check of the routing layer.

async fn list_inboxes(
    State(_state): State<ChatState>,
    Path(_tenant_id): Path<Uuid>,
) -> Result<Json<Vec<ChatInboxResponse>>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}

async fn list_conversations(
    State(_state): State<ChatState>,
    Path(_tenant_id): Path<Uuid>,
) -> Result<Json<Vec<ChatConversationResponse>>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}

async fn get_messages(
    State(_state): State<ChatState>,
    Path((_tenant_id, _conversation_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<Vec<ChatMessageResponse>>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}

async fn start_conversation(
    State(_state): State<ChatState>,
    Path(_tenant_id): Path<Uuid>,
    Json(_payload): Json<StartConversationRequest>,
) -> Result<Json<ChatConversationResponse>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}

async fn send_message(
    State(_state): State<ChatState>,
    Path(_tenant_id): Path<Uuid>,
    Json(_payload): Json<SendMessageRequest>,
) -> Result<Json<ChatMessageResponse>, StatusCode> {
    Err(StatusCode::NOT_IMPLEMENTED)
}

async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(|mut socket| async move {
        // Echo loop
        while let Some(msg) = tokio_stream::StreamExt::next(&mut socket).await {
            if let Ok(msg) = msg {
                let _ = axum::extract::ws::WebSocket::send(&mut socket, msg).await;
            } else {
                break;
            }
        }
    })
}

pub fn router<S: Clone + Send + Sync + 'static>(state: ChatState) -> Router<S> {
    Router::new()
        .route("/inboxes/{tenant_id}", get(list_inboxes))
        .route("/conversations/{tenant_id}", get(list_conversations).post(start_conversation))
        .route("/messages/{tenant_id}/{conversation_id}", get(get_messages))
        .route("/messages/{tenant_id}", post(send_message))
        .route("/ws", get(ws_handler))
        .with_state(state)
}
