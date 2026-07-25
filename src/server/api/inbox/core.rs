use axum::{
    extract::{State, Json, Path},
    response::IntoResponse,
    http::StatusCode,
    routing::{post, get},
    Router,
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::Row;
use crate::db::DB;

#[derive(Clone)]
pub struct InboxCoreState {
    pub db: Arc<DB>,
}

#[derive(sqlx::FromRow)]
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Inbox {
    pub id: String,
    pub tenant_id: String,
    pub name: String,
    pub channel_type: String,
    pub channel_config: Option<serde_json::Value>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(sqlx::FromRow)]
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Contact {
    pub id: String,
    pub tenant_id: String,
    pub name: Option<String>,
    pub identifier: String,
    pub attributes: Option<serde_json::Value>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(sqlx::FromRow)]
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Conversation {
    pub id: String,
    pub tenant_id: String,
    pub inbox_id: String,
    pub contact_id: String,
    pub status: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(sqlx::FromRow)]
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Message {
    pub id: String,
    pub tenant_id: String,
    pub conversation_id: String,
    pub sender_id: String,
    pub sender_type: String,
    pub content: String,
    pub message_type: String,
    pub additional_attributes: Option<serde_json::Value>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

// Payloads
#[derive(Deserialize)]
pub struct CreateInboxReq {
    pub name: String,
    pub channel_type: String,
    pub channel_config: Option<serde_json::Value>,
}

#[derive(Deserialize)]
pub struct CreateContactReq {
    pub name: Option<String>,
    pub identifier: String,
    pub attributes: Option<serde_json::Value>,
}

#[derive(Deserialize)]
pub struct StartConversationReq {
    pub inbox_id: String,
    pub contact_id: String,
    pub status: Option<String>,
}

#[derive(Deserialize)]
pub struct SendMessageReq {
    pub sender_id: String,
    pub sender_type: String,
    pub content: String,
    pub message_type: String,
    pub additional_attributes: Option<serde_json::Value>,
}

// Handlers
pub async fn create_inbox(
    State(state): State<InboxCoreState>,
    Path(tenant_id): Path<String>,
    Json(payload): Json<CreateInboxReq>,
) -> impl IntoResponse {
    let id = Uuid::new_v4().to_string();

    match sqlx::query_as::<_, Inbox>(
        r#"
        INSERT INTO inboxes (id, tenant_id, name, channel_type, channel_config, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, NOW(), NOW())
        RETURNING id, tenant_id, name, channel_type, channel_config, created_at, updated_at
        "#
    )
    .bind(id)
    .bind(tenant_id)
    .bind(payload.name)
    .bind(payload.channel_type)
    .bind(payload.channel_config)
    .fetch_one(&state.db.pool)
    .await
    {
        Ok(inbox) => (StatusCode::CREATED, Json(inbox)).into_response(),
        Err(e) => {
            tracing::error!("Failed to create inbox: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
        }
    }
}

pub async fn create_contact(
    State(state): State<InboxCoreState>,
    Path(tenant_id): Path<String>,
    Json(payload): Json<CreateContactReq>,
) -> impl IntoResponse {
    let id = Uuid::new_v4().to_string();

    match sqlx::query_as::<_, Contact>(
        r#"
        INSERT INTO contacts (id, tenant_id, name, identifier, attributes, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, NOW(), NOW())
        RETURNING id, tenant_id, name, identifier, attributes, created_at, updated_at
        "#
    )
    .bind(id)
    .bind(tenant_id)
    .bind(payload.name)
    .bind(payload.identifier)
    .bind(payload.attributes)
    .fetch_one(&state.db.pool)
    .await
    {
        Ok(contact) => (StatusCode::CREATED, Json(contact)).into_response(),
        Err(e) => {
            tracing::error!("Failed to create contact: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
        }
    }
}

pub async fn start_conversation(
    State(state): State<InboxCoreState>,
    Path(tenant_id): Path<String>,
    Json(payload): Json<StartConversationReq>,
) -> impl IntoResponse {
    let id = Uuid::new_v4().to_string();
    let status = payload.status.unwrap_or_else(|| "open".to_string());

    match sqlx::query_as::<_, Conversation>(
        r#"
        INSERT INTO conversations (id, tenant_id, inbox_id, contact_id, status, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, NOW(), NOW())
        RETURNING id, tenant_id, inbox_id, contact_id, status, created_at, updated_at
        "#
    )
    .bind(id)
    .bind(tenant_id)
    .bind(payload.inbox_id)
    .bind(payload.contact_id)
    .bind(status)
    .fetch_one(&state.db.pool)
    .await
    {
        Ok(conversation) => (StatusCode::CREATED, Json(conversation)).into_response(),
        Err(e) => {
            tracing::error!("Failed to start conversation: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
        }
    }
}

pub async fn send_message(
    State(state): State<InboxCoreState>,
    Path((tenant_id, conversation_id)): Path<(String, String)>,
    Json(payload): Json<SendMessageReq>,
) -> impl IntoResponse {
    let id = Uuid::new_v4().to_string();

    match sqlx::query_as::<_, Message>(
        r#"
        INSERT INTO messages (id, tenant_id, conversation_id, sender_id, sender_type, content, message_type, additional_attributes, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, NOW(), NOW())
        RETURNING id, tenant_id, conversation_id, sender_id, sender_type, content, message_type, additional_attributes, created_at, updated_at
        "#
    )
    .bind(id)
    .bind(tenant_id)
    .bind(conversation_id)
    .bind(payload.sender_id)
    .bind(payload.sender_type)
    .bind(payload.content)
    .bind(payload.message_type)
    .bind(payload.additional_attributes)
    .fetch_one(&state.db.pool)
    .await
    {
        Ok(message) => (StatusCode::CREATED, Json(message)).into_response(),
        Err(e) => {
            tracing::error!("Failed to send message: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
        }
    }
}

pub async fn get_conversations(
    State(state): State<InboxCoreState>,
    Path((tenant_id, inbox_id)): Path<(String, String)>,
) -> impl IntoResponse {
    match sqlx::query_as::<_, Conversation>(
        r#"
        SELECT id, tenant_id, inbox_id, contact_id, status, created_at, updated_at
        FROM conversations
        WHERE tenant_id = $1 AND inbox_id = $2
        ORDER BY updated_at DESC
        "#
    )
    .bind(tenant_id)
    .bind(inbox_id)
    .fetch_all(&state.db.pool)
    .await
    {
        Ok(conversations) => (StatusCode::OK, Json(conversations)).into_response(),
        Err(e) => {
            tracing::error!("Failed to fetch conversations: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
        }
    }
}

pub async fn get_messages(
    State(state): State<InboxCoreState>,
    Path((tenant_id, conversation_id)): Path<(String, String)>,
) -> impl IntoResponse {
    match sqlx::query_as::<_, Message>(
        r#"
        SELECT id, tenant_id, conversation_id, sender_id, sender_type, content, message_type, additional_attributes, created_at, updated_at
        FROM messages
        WHERE tenant_id = $1 AND conversation_id = $2
        ORDER BY created_at ASC
        "#
    )
    .bind(tenant_id)
    .bind(conversation_id)
    .fetch_all(&state.db.pool)
    .await
    {
        Ok(messages) => (StatusCode::OK, Json(messages)).into_response(),
        Err(e) => {
            tracing::error!("Failed to fetch messages: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
        }
    }
}

pub fn router<S>(state: InboxCoreState) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/inbox/{tenant_id}", post(create_inbox))
        .route("/contact/{tenant_id}", post(create_contact))
        .route("/conversation/{tenant_id}", post(start_conversation))
        .route("/conversation/{tenant_id}/{inbox_id}", get(get_conversations))
        .route("/message/{tenant_id}/{conversation_id}", post(send_message).get(get_messages))
        .with_state(state)
}
