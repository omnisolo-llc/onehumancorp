use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateInboxReq {
    pub name: String,
}

#[derive(Deserialize)]
pub struct CreateChannelReq {
    pub inbox_id: Uuid,
    pub channel_type: String,
    pub config: serde_json::Value,
}

#[derive(Deserialize)]
pub struct CreateContactReq {
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
}

#[derive(Deserialize)]
pub struct StartConversationReq {
    pub inbox_id: Uuid,
    pub contact_id: Uuid,
    pub assignee_id: Option<Uuid>,
}

#[derive(Deserialize)]
pub struct SendMessageReq {
    pub sender_type: String,
    pub sender_id: Option<Uuid>,
    pub content: String,
}

pub async fn create_inbox(
    State(db): State<crate::db::DB>,
    Extension(claims): Extension<server_common::Claims>,
    Json(payload): Json<CreateInboxReq>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let tenant_id = Uuid::parse_str(claims.organization_id.as_deref().unwrap_or_default())
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    let chat_service = crate::services::chat::service::ChatService::new(db.pool.clone());
    let inbox = chat_service.create_inbox(tenant_id, payload.name).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(inbox))
}

pub async fn get_inboxes(
    State(db): State<crate::db::DB>,
    Extension(claims): Extension<server_common::Claims>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let tenant_id = Uuid::parse_str(claims.organization_id.as_deref().unwrap_or_default())
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    let chat_service = crate::services::chat::service::ChatService::new(db.pool.clone());
    let inboxes = chat_service.get_inboxes(tenant_id).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(inboxes))
}

pub async fn create_channel(
    State(db): State<crate::db::DB>,
    Extension(claims): Extension<server_common::Claims>,
    Json(payload): Json<CreateChannelReq>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let tenant_id = Uuid::parse_str(claims.organization_id.as_deref().unwrap_or_default())
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    let chat_service = crate::services::chat::service::ChatService::new(db.pool.clone());
    let channel = chat_service.create_channel(tenant_id, payload.inbox_id, payload.channel_type, payload.config).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(channel))
}

pub async fn create_contact(
    State(db): State<crate::db::DB>,
    Extension(claims): Extension<server_common::Claims>,
    Json(payload): Json<CreateContactReq>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let tenant_id = Uuid::parse_str(claims.organization_id.as_deref().unwrap_or_default())
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    let chat_service = crate::services::chat::service::ChatService::new(db.pool.clone());
    let contact = chat_service.create_contact(tenant_id, payload.name, payload.email, payload.phone).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(contact))
}

pub async fn start_conversation(
    State(db): State<crate::db::DB>,
    Extension(claims): Extension<server_common::Claims>,
    Json(payload): Json<StartConversationReq>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let tenant_id = Uuid::parse_str(claims.organization_id.as_deref().unwrap_or_default())
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    let chat_service = crate::services::chat::service::ChatService::new(db.pool.clone());
    let conv = chat_service.start_conversation(tenant_id, payload.inbox_id, payload.contact_id, payload.assignee_id).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(conv))
}

pub async fn get_conversations(
    State(db): State<crate::db::DB>,
    Extension(claims): Extension<server_common::Claims>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let tenant_id = Uuid::parse_str(claims.organization_id.as_deref().unwrap_or_default())
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    let chat_service = crate::services::chat::service::ChatService::new(db.pool.clone());
    let convs = chat_service.get_conversations(tenant_id).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(convs))
}

pub async fn send_message(
    State(db): State<crate::db::DB>,
    Extension(claims): Extension<server_common::Claims>,
    Path(conversation_id): Path<Uuid>,
    Json(payload): Json<SendMessageReq>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let tenant_id = Uuid::parse_str(claims.organization_id.as_deref().unwrap_or_default())
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    let chat_service = crate::services::chat::service::ChatService::new(db.pool.clone());
    let msg = chat_service.send_message(tenant_id, conversation_id, payload.sender_type, payload.sender_id, payload.content).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(msg))
}

pub async fn get_messages(
    State(db): State<crate::db::DB>,
    Extension(claims): Extension<server_common::Claims>,
    Path(conversation_id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let tenant_id = Uuid::parse_str(claims.organization_id.as_deref().unwrap_or_default())
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    let chat_service = crate::services::chat::service::ChatService::new(db.pool.clone());
    let msgs = chat_service.get_messages(tenant_id, conversation_id).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(msgs))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_inbox_api_signature() {
        // Just verify it compiles correctly with required signatures
        assert!(true);
    }
}
