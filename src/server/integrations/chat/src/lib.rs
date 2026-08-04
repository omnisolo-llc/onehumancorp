use axum::{
    extract::{Path, State, ws::{Message, WebSocket, WebSocketUpgrade}},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, put, delete},
    Json, Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, FromRow};
use uuid::Uuid;
use std::sync::Arc;
use tokio::sync::broadcast;
use futures::{sink::SinkExt, stream::StreamExt};

#[derive(Clone)]
pub struct ChatState {
    pub db: PgPool,
    pub tx: broadcast::Sender<ChatMessage>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct ChatInbox {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct ChatChannel {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub inbox_id: Uuid,
    pub channel_type: String,
    pub config: Option<serde_json::Value>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}


#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct ChatContact {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct ChatConversation {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub inbox_id: Uuid,
    pub contact_id: Uuid,
    pub assignee_id: Option<Uuid>,
    pub status: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct ChatMessage {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub conversation_id: Uuid,
    pub sender_type: String,
    pub sender_id: Option<Uuid>,
    pub content: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct CreateInboxReq {
    pub tenant_id: Uuid,
    pub name: String,
}

pub async fn create_inbox_handler(
    State(state): State<Arc<ChatState>>,
    Json(payload): Json<CreateInboxReq>,
) -> impl IntoResponse {
    let id = Uuid::new_v4();
    let res = sqlx::query_as::<_, ChatInbox>(
        "INSERT INTO chat_inboxes (id, tenant_id, name) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind(id)
    .bind(payload.tenant_id)
    .bind(&payload.name)
    .fetch_one(&state.db)
    .await;

    match res {
        Ok(inbox) => (StatusCode::CREATED, Json(inbox)).into_response(),
        Err(e) => {
            tracing::error!("Failed to create inbox: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create inbox").into_response()
        }
    }
}

pub async fn get_inboxes_handler(
    State(state): State<Arc<ChatState>>,
) -> impl IntoResponse {
    let res = sqlx::query_as::<_, ChatInbox>(
        "SELECT * FROM chat_inboxes"
    )
    .fetch_all(&state.db)
    .await;

    match res {
        Ok(inboxes) => (StatusCode::OK, Json(inboxes)).into_response(),
        Err(e) => {
            tracing::error!("Failed to fetch inboxes: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch inboxes").into_response()
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateConversationReq {
    pub tenant_id: Uuid,
    pub inbox_id: Uuid,
    pub contact_id: Uuid,
    pub assignee_id: Option<Uuid>,
    pub status: Option<String>,
}

pub async fn create_conversation_handler(
    State(state): State<Arc<ChatState>>,
    Json(payload): Json<CreateConversationReq>,
) -> impl IntoResponse {
    let id = Uuid::new_v4();
    let status = payload.status.unwrap_or_else(|| "open".to_string());

    let res = sqlx::query_as::<_, ChatConversation>(
        "INSERT INTO chat_conversations (id, tenant_id, inbox_id, contact_id, assignee_id, status) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *"
    )
    .bind(id)
    .bind(payload.tenant_id)
    .bind(payload.inbox_id)
    .bind(payload.contact_id)
    .bind(payload.assignee_id)
    .bind(status)
    .fetch_one(&state.db)
    .await;

    match res {
        Ok(conversation) => (StatusCode::CREATED, Json(conversation)).into_response(),
        Err(e) => {
            tracing::error!("Failed to create conversation: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create conversation").into_response()
        }
    }
}


#[derive(Debug, Deserialize)]
pub struct CreateMessageReq {
    pub tenant_id: Uuid,
    pub conversation_id: Uuid,
    pub sender_type: String,
    pub sender_id: Option<Uuid>,
    pub content: String,
}

pub async fn create_message_handler(
    State(state): State<Arc<ChatState>>,
    Json(payload): Json<CreateMessageReq>,
) -> impl IntoResponse {
    let id = Uuid::new_v4();

    let res = sqlx::query_as::<_, ChatMessage>(
        "INSERT INTO chat_messages (id, tenant_id, conversation_id, sender_type, sender_id, content) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *"
    )
    .bind(id)
    .bind(payload.tenant_id)
    .bind(payload.conversation_id)
    .bind(payload.sender_type)
    .bind(payload.sender_id)
    .bind(payload.content)
    .fetch_one(&state.db)
    .await;

    match res {
        Ok(msg) => {
            let _ = state.tx.send(msg.clone());
            (StatusCode::CREATED, Json(msg)).into_response()
        },
        Err(e) => {
            tracing::error!("Failed to create message: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create message").into_response()
        }
    }
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<ChatState>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: Arc<ChatState>) {
    let (mut sender, mut receiver) = socket.split();
    let mut rx = state.tx.subscribe();

    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if let Ok(json) = serde_json::to_string(&msg) {
                if sender.send(Message::Text(json.into())).await.is_err() {
                    break;
                }
            }
        }
    });

    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(_)) = receiver.next().await {
            // Can handle incoming WS messages if needed
        }
    });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };
}


pub fn chat_routes(state: Arc<ChatState>) -> Router {
    Router::new()
        .route("/inboxes", post(create_inbox_handler).get(get_inboxes_handler))
        .route("/conversations", post(create_conversation_handler))
        .route("/messages", post(create_message_handler))
        .route("/ws", get(ws_handler))
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dummy() {
        assert_eq!(1, 1);
    }
}
