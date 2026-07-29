use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::db::DB;
use crate::ohc::domain::omnichannel::{Channel, Contact, Conversation, Inbox, Message};
use crate::utils::auth::extract_tenant_id;

pub fn router(db: Arc<DB>) -> Router {
    Router::new()
        .route(
            "/api/v1/omnichannel/webhook/:channel_id",
            post(handle_webhook),
        )
        .route(
            "/api/v1/omnichannel/inboxes",
            get(list_inboxes).post(create_inbox),
        )
        .route(
            "/api/v1/omnichannel/channels",
            get(list_channels).post(create_channel),
        )
        .route(
            "/api/v1/omnichannel/conversations",
            get(list_conversations).post(create_conversation),
        )
        .route(
            "/api/v1/omnichannel/messages",
            get(list_messages).post(create_message),
        )
        .with_state(db)
}

#[derive(Deserialize)]
pub struct WebhookPayload {
    pub message: String,
    pub sender: String,
}

pub async fn handle_webhook(
    State(db): State<Arc<DB>>,
    Path(channel_id): Path<Uuid>,
    Json(payload): Json<WebhookPayload>,
) -> Result<StatusCode, (StatusCode, String)> {
    // For webhooks, we need a way to determine tenant context.
    // Assuming channel_id allows us to find the channel and its tenant.
    let channel_record = sqlx::query!(
        r#"SELECT tenant_id, inbox_id FROM channels WHERE id = $1"#,
        channel_id
    )
    .fetch_optional(&db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if let Some(record) = channel_record {
        let tenant_id = record.tenant_id;

        // 1. Find or create contact
        // 2. Find or create conversation
        // 3. Create message
        // 4. Trigger Job Queue / WebSocket broadcast
        // Simple implementation for test purposes

        let conversation_id = Uuid::new_v4();
        sqlx::query!(
            r#"INSERT INTO conversations (id, tenant_id, inbox_id, status) VALUES ($1, $2, $3, 'open')"#,
            conversation_id, tenant_id, record.inbox_id
        ).execute(&db.pool).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        let message_id = Uuid::new_v4();
        sqlx::query!(
            r#"INSERT INTO messages (id, tenant_id, conversation_id, content, sender_type) VALUES ($1, $2, $3, $4, 'contact')"#,
            message_id, tenant_id, conversation_id, payload.message
        ).execute(&db.pool).await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        // TODO: Enqueue AI job
        // TODO: Broadcast via WS

        Ok(StatusCode::OK)
    } else {
        Err((StatusCode::NOT_FOUND, "Channel not found".to_string()))
    }
}

// Stubs for other endpoints
pub async fn list_inboxes() -> Result<Json<Vec<Inbox>>, (StatusCode, String)> { Ok(Json(vec![])) }
pub async fn create_inbox() -> Result<StatusCode, (StatusCode, String)> { Ok(StatusCode::CREATED) }
pub async fn list_channels() -> Result<Json<Vec<Channel>>, (StatusCode, String)> { Ok(Json(vec![])) }
pub async fn create_channel() -> Result<StatusCode, (StatusCode, String)> { Ok(StatusCode::CREATED) }
pub async fn list_conversations() -> Result<Json<Vec<Conversation>>, (StatusCode, String)> { Ok(Json(vec![])) }
pub async fn create_conversation() -> Result<StatusCode, (StatusCode, String)> { Ok(StatusCode::CREATED) }
pub async fn list_messages() -> Result<Json<Vec<Message>>, (StatusCode, String)> { Ok(Json(vec![])) }
pub async fn create_message() -> Result<StatusCode, (StatusCode, String)> { Ok(StatusCode::CREATED) }
