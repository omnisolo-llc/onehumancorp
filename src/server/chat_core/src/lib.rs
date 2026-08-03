use axum::{
    routing::post,
    Router, Json, extract::State,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use sqlx::{PgPool, FromRow};

pub struct ChatCoreState {
    pub db: PgPool,
}

#[derive(Serialize, Deserialize)]
pub struct CreateInboxRequest {
    pub tenant_id: uuid::Uuid,
    pub name: String,
    pub channel_type: String,
    pub settings: serde_json::Value,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Inbox {
    pub id: uuid::Uuid,
    pub tenant_id: uuid::Uuid,
    pub name: String,
    pub channel_type: String,
    pub settings: serde_json::Value,
}

pub async fn create_inbox(
    State(state): State<Arc<ChatCoreState>>,
    Json(payload): Json<CreateInboxRequest>,
) -> Result<Json<Inbox>, axum::http::StatusCode> {
    let inbox = sqlx::query_as::<_, Inbox>(
        r#"
        INSERT INTO inboxes (tenant_id, name, channel_type, settings)
        VALUES ($1, $2, $3, $4)
        RETURNING id, tenant_id, name, channel_type, settings
        "#,
    )
    .bind(payload.tenant_id)
    .bind(payload.name)
    .bind(payload.channel_type)
    .bind(payload.settings)
    .fetch_one(&state.db)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(inbox))
}

#[derive(Serialize, Deserialize)]
pub struct CreateConversationRequest {
    pub tenant_id: uuid::Uuid,
    pub inbox_id: uuid::Uuid,
    pub contact_id: uuid::Uuid,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Conversation {
    pub id: uuid::Uuid,
    pub tenant_id: uuid::Uuid,
    pub inbox_id: uuid::Uuid,
    pub contact_id: uuid::Uuid,
    pub status: String,
    pub assignee_id: Option<uuid::Uuid>,
}

pub async fn start_conversation(
    State(state): State<Arc<ChatCoreState>>,
    Json(payload): Json<CreateConversationRequest>,
) -> Result<Json<Conversation>, axum::http::StatusCode> {
    let conversation = sqlx::query_as::<_, Conversation>(
        r#"
        INSERT INTO conversations (tenant_id, inbox_id, contact_id)
        VALUES ($1, $2, $3)
        RETURNING id, tenant_id, inbox_id, contact_id, status, assignee_id
        "#,
    )
    .bind(payload.tenant_id)
    .bind(payload.inbox_id)
    .bind(payload.contact_id)
    .fetch_one(&state.db)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(conversation))
}

#[derive(Serialize, Deserialize)]
pub struct SendMessageRequest {
    pub tenant_id: uuid::Uuid,
    pub conversation_id: uuid::Uuid,
    pub sender_type: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Message {
    pub id: uuid::Uuid,
    pub tenant_id: uuid::Uuid,
    pub conversation_id: uuid::Uuid,
    pub sender_type: String,
    pub content: String,
    pub status: String,
}

pub async fn send_message(
    State(state): State<Arc<ChatCoreState>>,
    Json(payload): Json<SendMessageRequest>,
) -> Result<Json<Message>, axum::http::StatusCode> {
    let mut tx = state.db.begin().await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    let message = sqlx::query_as::<_, Message>(
        r#"
        INSERT INTO messages (tenant_id, conversation_id, sender_type, content)
        VALUES ($1, $2, $3, $4)
        RETURNING id, tenant_id, conversation_id, sender_type, content, status
        "#,
    )
    .bind(payload.tenant_id)
    .bind(payload.conversation_id)
    .bind(payload.sender_type.clone())
    .bind(payload.content.clone())
    .fetch_one(&mut *tx)
    .await
    .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    if payload.sender_type == "contact" {
        // Enqueue AI job. In OHC this usually means inserting into the AI Job Queue
        // Here we simulate it by inserting a mock draft message with AI sender
        sqlx::query(
            r#"
            INSERT INTO messages (tenant_id, conversation_id, sender_type, content, status)
            VALUES ($1, $2, 'ai', 'Automated Draft Reply', 'draft')
            "#,
        )
        .bind(payload.tenant_id)
        .bind(payload.conversation_id)
        .execute(&mut *tx)
        .await
        .map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    tx.commit().await.map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(message))
}

pub fn router(state: Arc<ChatCoreState>) -> Router {
    Router::new()
        .route("/api/inboxes", post(create_inbox))
        .route("/api/conversations", post(start_conversation))
        .route("/api/messages", post(send_message))
        .with_state(state)
}
