use axum::{
    extract::{Extension, Path, State, WebSocketUpgrade, ws::{Message, WebSocket}},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use serde_json::Value;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChatInbox {
    pub id: String,
    pub tenant_id: String,
    pub name: String,
    pub channel_type: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChatContact {
    pub id: String,
    pub tenant_id: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChatConversation {
    pub id: String,
    pub tenant_id: String,
    pub inbox_id: String,
    pub contact_id: String,
    pub status: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, sqlx::FromRow)]
pub struct ChatMessage {
    pub id: String,
    pub tenant_id: String,
    pub conversation_id: String,
    pub sender_type: String,
    pub content: String,
    pub is_draft: Option<bool>,
    // Store dates as String or omit them for simplified json
}

pub async fn list_conversations(
    Extension(db): Extension<Arc<crate::db::DB>>,
    Extension(claims): Extension<::server_common::Claims>,
) -> impl IntoResponse {
    let tenant_id = claims.tenant_id.unwrap_or_default();

    // In a real app we'd fetch conversations, here we just return a stub for testing the UI initially
    Json(serde_json::json!({
        "conversations": [
            { "id": "conv_1", "name": "Support Inquiry" }
        ]
    }))
}

pub async fn list_messages(
    Extension(db): Extension<Arc<crate::db::DB>>,
    Extension(claims): Extension<::server_common::Claims>,
    Path(conversation_id): Path<String>,
) -> impl IntoResponse {
    let tenant_id = claims.tenant_id.unwrap_or_default();

    // Attempt to fetch from DB
    let result = sqlx::query_as::<_, ChatMessage>("SELECT id, tenant_id, conversation_id, sender_type, content, is_draft FROM chat_messages WHERE tenant_id = $1 AND conversation_id = $2 ORDER BY created_at ASC")
        .bind(&tenant_id)
        .bind(&conversation_id)
        .fetch_all(&db.pool)
        .await;

    match result {
        Ok(msgs) => Json(serde_json::json!({ "messages": msgs })),
        Err(e) => {
            tracing::error!("Error fetching messages: {}", e);
            Json(serde_json::json!({ "messages": [] }))
        }
    }
}

pub async fn create_message(
    Extension(db): Extension<Arc<crate::db::DB>>,
    Extension(claims): Extension<::server_common::Claims>,
    Json(payload): Json<Value>,
) -> impl IntoResponse {
    let tenant_id = claims.tenant_id.unwrap_or_default();
    let id = uuid::Uuid::new_v4().to_string();
    let content = payload.get("content").and_then(|c| c.as_str()).unwrap_or("");
    let conversation_id = payload.get("conversation_id").and_then(|c| c.as_str()).unwrap_or("");
    let sender_type = payload.get("sender_type").and_then(|c| c.as_str()).unwrap_or("customer");
    let is_draft = payload.get("is_draft").and_then(|c| c.as_bool()).unwrap_or(false);

    let result = sqlx::query(
        "INSERT INTO chat_messages (id, tenant_id, conversation_id, sender_type, content, is_draft) VALUES ($1, $2, $3, $4, $5, $6)"
    )
    .bind(&id)
    .bind(&tenant_id)
    .bind(&conversation_id)
    .bind(&sender_type)
    .bind(&content)
    .bind(&is_draft)
    .execute(&db.pool)
    .await;

    if result.is_err() {
        tracing::error!("Failed to insert message");
    }

    let msg = serde_json::json!({
        "id": id,
        "tenant_id": tenant_id,
        "conversation_id": conversation_id,
        "content": content,
        "status": "sent",
        "is_draft": is_draft,
        "sender_type": sender_type
    });

    // Simulate agent drafting a reply when a customer sends a message
    if sender_type == "customer" {
        // Here we would use the NATS Event Mesh client to publish an event:
        // nats_client.publish("ohc.events.chat.new_message", msg_bytes).await;
        // The Ambassador would listen and then create a new drafted message.
    }

    Json(msg)
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Extension(_claims): Extension<::server_common::Claims>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket))
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            // Echo or broadcast logic using PubSubManager goes here
        } else {
            break;
        }
    }
}

pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/conversations", get(list_conversations))
        .route("/conversations/:id/messages", get(list_messages))
        .route("/messages", post(create_message))
        .route("/ws", get(ws_handler))
}
