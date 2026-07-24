use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;
use sqlx::PgPool;

use crate::domain::omnichannel::repo::OmnichannelRepo;

pub struct AppState {
    pub pool: PgPool,
}

#[derive(Deserialize)]
pub struct CreateInboxReq {
    pub name: String,
    pub channel_type: String,
    pub channel_config: Option<serde_json::Value>,
}

#[derive(Deserialize)]
pub struct CreateContactReq {
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub custom_attributes: Option<serde_json::Value>,
}

#[derive(Deserialize)]
pub struct CreateConversationReq {
    pub inbox_id: Uuid,
    pub contact_id: Uuid,
    pub status: String,
    pub assignee_id: Option<Uuid>,
}

#[derive(Deserialize)]
pub struct CreateMessageReq {
    pub sender_id: Uuid,
    pub sender_type: String,
    pub message_type: String,
    pub content: String,
    pub external_source_ids: Option<serde_json::Value>,
}

// Temporary for testing - normally extracted from auth middleware
fn get_tenant_id() -> Uuid {
    Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap_or_else(|_| Uuid::new_v4())
}

async fn create_inbox(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateInboxReq>,
) -> impl IntoResponse {
    let tenant_id = get_tenant_id();
    match OmnichannelRepo::create_inbox(
        &state.pool,
        tenant_id,
        payload.name,
        payload.channel_type,
        payload.channel_config,
    )
    .await
    {
        Ok(inbox) => (axum::http::StatusCode::CREATED, Json(inbox)).into_response(),
        Err(e) => {
            tracing::error!("Failed to create inbox: {}", e);
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        }
    }
}

async fn create_contact(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateContactReq>,
) -> impl IntoResponse {
    let tenant_id = get_tenant_id();
    match OmnichannelRepo::create_contact(
        &state.pool,
        tenant_id,
        payload.name,
        payload.email,
        payload.phone_number,
        payload.custom_attributes,
    )
    .await
    {
        Ok(contact) => (axum::http::StatusCode::CREATED, Json(contact)).into_response(),
        Err(e) => {
            tracing::error!("Failed to create contact: {}", e);
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        }
    }
}

async fn create_conversation(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateConversationReq>,
) -> impl IntoResponse {
    let tenant_id = get_tenant_id();
    match OmnichannelRepo::create_conversation(
        &state.pool,
        tenant_id,
        payload.inbox_id,
        payload.contact_id,
        payload.status,
        payload.assignee_id,
    )
    .await
    {
        Ok(conv) => (axum::http::StatusCode::CREATED, Json(conv)).into_response(),
        Err(e) => {
            tracing::error!("Failed to create conversation: {}", e);
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        }
    }
}

async fn create_message(
    State(state): State<Arc<AppState>>,
    Path(conversation_id): Path<Uuid>,
    Json(payload): Json<CreateMessageReq>,
) -> impl IntoResponse {
    let tenant_id = get_tenant_id();
    match OmnichannelRepo::create_message(
        &state.pool,
        tenant_id,
        conversation_id,
        payload.sender_id,
        payload.sender_type,
        payload.message_type,
        payload.content,
        payload.external_source_ids,
    )
    .await
    {
        Ok(msg) => (axum::http::StatusCode::CREATED, Json(msg)).into_response(),
        Err(e) => {
            tracing::error!("Failed to create message: {}", e);
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        }
    }
}

async fn list_conversations(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let tenant_id = get_tenant_id();
    match OmnichannelRepo::list_conversations(&state.pool, tenant_id).await {
        Ok(convs) => (axum::http::StatusCode::OK, Json(convs)).into_response(),
        Err(e) => {
            tracing::error!("Failed to list conversations: {}", e);
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        }
    }
}

async fn get_conversation_messages(
    State(state): State<Arc<AppState>>,
    Path(conversation_id): Path<Uuid>,
) -> impl IntoResponse {
    let tenant_id = get_tenant_id();
    match OmnichannelRepo::get_conversation_messages(&state.pool, tenant_id, conversation_id).await
    {
        Ok(msgs) => (axum::http::StatusCode::OK, Json(msgs)).into_response(),
        Err(e) => {
            tracing::error!("Failed to get messages: {}", e);
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        }
    }
}

pub fn router(pool: PgPool) -> Router {
    let state = Arc::new(AppState { pool });

    Router::new()
        .route("/api/v1/omnichannel/inboxes", post(create_inbox))
        .route("/api/v1/omnichannel/contacts", post(create_contact))
        .route("/api/v1/omnichannel/conversations", post(create_conversation).get(list_conversations))
        .route(
            "/api/v1/omnichannel/conversations/:id/messages",
            post(create_message).get(get_conversation_messages),
        )
        .with_state(state)
}
