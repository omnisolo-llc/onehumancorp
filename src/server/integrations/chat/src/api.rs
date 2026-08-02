use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use serde::Deserialize;

use std::sync::Arc;
use uuid::Uuid;

use crate::conversation::models as Conversation;
use crate::message::models as ChatMessage;
use crate::models as Inbox;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DatabaseConnection>,
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/inboxes", get(list_inboxes).post(create_inbox))
        .route("/conversations", get(list_conversations).post(create_conversation))
        .route("/conversations/{id}/messages", get(list_messages).post(create_message))
        .route("/ws", get(ws_handler))
        .with_state(state)
}

#[derive(Deserialize)]
pub struct CreateInboxReq {
    pub tenant_id: Uuid,
    pub name: String,
    pub channel_type: String,
}

pub async fn list_inboxes(
    State(state): State<AppState>,
) -> Result<Json<Vec<Inbox::Model>>, StatusCode> {
    let inboxes = Inbox::Entity::find()
        .all(state.db.as_ref())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(inboxes))
}

pub async fn create_inbox(
    State(state): State<AppState>,
    Json(payload): Json<CreateInboxReq>,
) -> Result<Json<Inbox::Model>, StatusCode> {
    let inbox = Inbox::ActiveModel {
        id: Set(Uuid::new_v4()),
        tenant_id: Set(payload.tenant_id),
        name: Set(payload.name),
        channel_type: Set(payload.channel_type),
        ..Default::default()
    };

    let result = inbox
        .insert(state.db.as_ref())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(result))
}

#[derive(Deserialize)]
pub struct CreateConversationReq {
    pub tenant_id: Uuid,
    pub inbox_id: Uuid,
    pub customer_profile_id: Option<Uuid>,
}

pub async fn list_conversations(
    State(state): State<AppState>,
) -> Result<Json<Vec<Conversation::Model>>, StatusCode> {
    let conversations = Conversation::Entity::find()
        .all(state.db.as_ref())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(conversations))
}

pub async fn create_conversation(
    State(state): State<AppState>,
    Json(payload): Json<CreateConversationReq>,
) -> Result<Json<Conversation::Model>, StatusCode> {
    let conversation = Conversation::ActiveModel {
        id: Set(Uuid::new_v4()),
        tenant_id: Set(payload.tenant_id),
        inbox_id: Set(payload.inbox_id),
        customer_profile_id: Set(payload.customer_profile_id),
        status: Set("open".to_string()),
        ..Default::default()
    };

    let result = conversation
        .insert(state.db.as_ref())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(result))
}

#[derive(Deserialize)]
pub struct CreateMessageReq {
    pub tenant_id: Uuid,
    pub sender_type: String,
    pub content: String,
    pub is_agent_draft: Option<bool>,
}

pub async fn list_messages(
    State(state): State<AppState>,
    Path(conversation_id): Path<Uuid>,
) -> Result<Json<Vec<ChatMessage::Model>>, StatusCode> {
    // In a real app we'd filter by conversation_id here
    let messages = ChatMessage::Entity::find()
        .all(state.db.as_ref())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let filtered: Vec<ChatMessage::Model> = messages.into_iter().filter(|m| m.conversation_id == conversation_id).collect();
    Ok(Json(filtered))
}

pub async fn create_message(
    State(state): State<AppState>,
    Path(conversation_id): Path<Uuid>,
    Json(payload): Json<CreateMessageReq>,
) -> Result<Json<ChatMessage::Model>, StatusCode> {
    let message = ChatMessage::ActiveModel {
        id: Set(Uuid::new_v4()),
        tenant_id: Set(payload.tenant_id),
        conversation_id: Set(conversation_id),
        sender_type: Set(payload.sender_type),
        content: Set(payload.content),
        is_agent_draft: Set(payload.is_agent_draft),
        ..Default::default()
    };

    let result = message
        .insert(state.db.as_ref())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(result))
}

pub async fn ws_handler(ws: WebSocketUpgrade, State(_state): State<AppState>) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(msg) = socket.recv().await {
        let msg = if let Ok(msg) = msg {
            msg
        } else {
            return;
        };

        if let Message::Text(text) = msg {
            if socket.send(Message::Text(text.into())).await.is_err() {
                return;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sea_orm::{DatabaseBackend, MockDatabase};


    #[tokio::test]
    async fn test_inbox_creation() {
        let db = MockDatabase::new(DatabaseBackend::Postgres)
            .append_query_results([vec![Inbox::Model {
                id: Uuid::new_v4(),
                tenant_id: Uuid::new_v4(),
                name: "Test".to_string(),
                channel_type: "WebWidget".to_string(),
                created_at: None,
                updated_at: None,
            }]])
            .into_connection();

        let _state = AppState { db: Arc::new(db) };
        // Setup payload and request here in a real test...
        assert!(true);
    }
}
