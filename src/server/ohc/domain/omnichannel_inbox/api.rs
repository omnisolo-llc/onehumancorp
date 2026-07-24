use super::service::OmnichannelService;
use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use server_common::error::Result;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

pub struct AppState {
    pub omnichannel_service: Arc<OmnichannelService>,
}

#[derive(Deserialize)]
pub struct CreateInboxRequest {
    pub name: String,
}

#[derive(Deserialize)]
pub struct CreateConversationRequest {
    pub inbox_id: Uuid,
    pub contact_id: Uuid,
}

#[derive(Deserialize)]
pub struct CreateMessageRequest {
    pub content: String,
    pub sender_type: String,
}

async fn create_inbox(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateInboxRequest>,
) -> Result<Json<super::Inbox>> {
    let tenant_id = "test-tenant".to_string(); // Mock for now since TenantId not found
    let inbox = state
        .omnichannel_service
        .create_inbox(&tenant_id, &payload.name)
        .await?;
    Ok(Json(inbox))
}

async fn list_inboxes(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<super::Inbox>>> {
    let tenant_id = "test-tenant".to_string();
    let inboxes = state.omnichannel_service.list_inboxes(&tenant_id).await?;
    Ok(Json(inboxes))
}

async fn create_conversation(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateConversationRequest>,
) -> Result<Json<super::Conversation>> {
    let tenant_id = "test-tenant".to_string();
    let conversation = state
        .omnichannel_service
        .create_conversation(&tenant_id, payload.inbox_id, payload.contact_id)
        .await?;
    Ok(Json(conversation))
}

async fn create_message(
    State(state): State<Arc<AppState>>,
    Path(conversation_id): Path<Uuid>,
    Json(payload): Json<CreateMessageRequest>,
) -> Result<Json<super::Message>> {
    let tenant_id = "test-tenant".to_string();
    let message = state
        .omnichannel_service
        .create_message(
            &tenant_id,
            conversation_id,
            &payload.content,
            &payload.sender_type,
        )
        .await?;
    Ok(Json(message))
}

async fn list_messages(
    State(state): State<Arc<AppState>>,
    Path(conversation_id): Path<Uuid>,
) -> Result<Json<Vec<super::Message>>> {
    let tenant_id = "test-tenant".to_string();
    let messages = state
        .omnichannel_service
        .list_messages(&tenant_id, conversation_id)
        .await?;
    Ok(Json(messages))
}

pub fn router(pool: PgPool) -> Router {
    let state = Arc::new(AppState {
        omnichannel_service: Arc::new(OmnichannelService::new(pool)),
    });

    Router::new()
        .route("/inboxes", post(create_inbox).get(list_inboxes))
        .route("/conversations", post(create_conversation))
        .route(
            "/conversations/:conversation_id/messages",
            post(create_message).get(list_messages),
        )
        .with_state(state)
}
