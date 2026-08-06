use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, ActiveModelTrait, Set};
use serde::Deserialize;
use uuid::Uuid;

use crate::models::{inbox, conversation, message};

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

#[derive(Deserialize)]
pub struct CreateMessageReq {
    pub conversation_id: Uuid,
    pub sender_id: Option<Uuid>,
    pub sender_type: String,
    pub content: String,
    pub message_type: String,
}

pub async fn list_inboxes(
    State(state): State<AppState>,
    Path(tenant_id): Path<Uuid>,
) -> Result<Json<Vec<inbox::Model>>, StatusCode> {
    let inboxes = inbox::Entity::find()
        .filter(inbox::Column::TenantId.eq(tenant_id))
        .all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(inboxes))
}

pub async fn list_conversations(
    State(state): State<AppState>,
    Path(inbox_id): Path<Uuid>,
) -> Result<Json<Vec<conversation::Model>>, StatusCode> {
    let conversations = conversation::Entity::find()
        .filter(conversation::Column::InboxId.eq(inbox_id))
        .all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(conversations))
}

pub async fn get_messages(
    State(state): State<AppState>,
    Path(conversation_id): Path<Uuid>,
) -> Result<Json<Vec<message::Model>>, StatusCode> {
    let messages = message::Entity::find()
        .filter(message::Column::ConversationId.eq(conversation_id))
        .all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(messages))
}

pub async fn create_message(
    State(state): State<AppState>,
    Path(tenant_id): Path<Uuid>,
    Json(payload): Json<CreateMessageReq>,
) -> Result<Json<message::Model>, StatusCode> {
    let new_message = message::ActiveModel {
        message_id: Set(Uuid::new_v4()),
        tenant_id: Set(tenant_id),
        conversation_id: Set(payload.conversation_id),
        sender_id: Set(payload.sender_id),
        sender_type: Set(payload.sender_type),
        content: Set(payload.content),
        message_type: Set(payload.message_type),
        created_at: Set(chrono::Utc::now()),
    };

    let inserted = new_message
        .insert(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(inserted))
}
