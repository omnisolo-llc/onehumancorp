use crate::services::chat::service::ChatService;
use crate::services::chat::models::{ChatMessage, ChatInbox};
use axum::{extract::State, Extension, Json, http::StatusCode};
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct SendMessageReq {
    pub conversation_id: Uuid,
    pub sender_type: String,
    pub sender_id: Option<Uuid>,
    pub content: String,
    pub content_type: Option<String>,
    pub additional_attributes: Option<serde_json::Value>,
}

#[derive(serde::Deserialize)]
pub struct CreateInboxReq {
    pub name: String,
    pub enable_auto_assignment: Option<bool>,
    pub greeting_message: Option<String>,
    pub working_hours_enabled: Option<bool>,
}

pub async fn create_omni_inbox_handler(
    State(pool): State<sqlx::PgPool>,
    Extension(tenant_id): Extension<Uuid>,
    Json(req): Json<CreateInboxReq>,
) -> Result<Json<ChatInbox>, (StatusCode, String)> {
    let service = ChatService::new(pool);
    match service.create_inbox_record(tenant_id, req.name, req.enable_auto_assignment, req.greeting_message, req.working_hours_enabled).await {
        Ok(inbox) => Ok(Json(inbox)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn send_omni_message_handler(
    State(pool): State<sqlx::PgPool>,
    Extension(tenant_id): Extension<Uuid>,
    Json(req): Json<SendMessageReq>,
) -> Result<Json<ChatMessage>, (StatusCode, String)> {
    let service = ChatService::new(pool);
    match service.send_chat_message(tenant_id, req.conversation_id, req.sender_type, req.sender_id, req.content, req.content_type, req.additional_attributes).await {
        Ok(msg) => Ok(Json(msg)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn get_omni_messages_handler(
    State(pool): State<sqlx::PgPool>,
    Extension(tenant_id): Extension<Uuid>,
) -> Result<Json<Vec<ChatMessage>>, (StatusCode, String)> {
    let msgs = sqlx::query_as::<_, ChatMessage>(
        r#"
        SELECT id, tenant_id, conversation_id, sender_type, sender_id, content, content_type, additional_attributes, created_at, updated_at
        FROM chat_messages
        WHERE tenant_id = $1
        ORDER BY created_at ASC
        LIMIT 50
        "#
    )
    .bind(tenant_id)
    .fetch_all(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(msgs))
}
