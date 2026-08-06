use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, put},
    Json, Router,
};
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;

use crate::auth::AuthenticatedUser;
use super::models::{CreateDraftRequest, CreateMessageRequest};
use super::service::ChatService;

pub fn chat_routes(chat_service: ChatService) -> Router {
    Router::new()
        .route("/inboxes", get(get_inboxes))
        .route("/inboxes/:inbox_id/conversations", get(get_conversations))
        .route("/conversations/:conversation_id/messages", get(get_messages))
        .route("/conversations/:conversation_id/messages", post(send_message))
        .route("/conversations/:conversation_id/drafts", post(draft_ai_message))
        .route("/messages/:message_id/approve", put(approve_draft))
        .with_state(chat_service)
}

async fn get_inboxes(
    State(service): State<ChatService>,
    user: AuthenticatedUser,
) -> impl IntoResponse {
    match service.get_inboxes(&user.tenant_id).await {
        Ok(inboxes) => (StatusCode::OK, Json(inboxes)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

async fn get_conversations(
    State(service): State<ChatService>,
    Path(inbox_id): Path<Uuid>,
    user: AuthenticatedUser,
) -> impl IntoResponse {
    match service.get_conversations(&user.tenant_id, inbox_id).await {
        Ok(conversations) => (StatusCode::OK, Json(conversations)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

async fn get_messages(
    State(service): State<ChatService>,
    Path(conversation_id): Path<Uuid>,
    user: AuthenticatedUser,
) -> impl IntoResponse {
    match service.get_messages(&user.tenant_id, conversation_id).await {
        Ok(messages) => (StatusCode::OK, Json(messages)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

async fn send_message(
    State(service): State<ChatService>,
    Path(conversation_id): Path<Uuid>,
    user: AuthenticatedUser,
    Json(payload): Json<CreateMessageRequest>,
) -> impl IntoResponse {
    match service
        .send_message(&user.tenant_id, conversation_id, user.id, &payload.content)
        .await
    {
        Ok(message) => (StatusCode::CREATED, Json(message)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

async fn draft_ai_message(
    State(service): State<ChatService>,
    Path(conversation_id): Path<Uuid>,
    user: AuthenticatedUser,
    Json(payload): Json<CreateDraftRequest>,
) -> impl IntoResponse {
    match service
        .draft_ai_message(&user.tenant_id, conversation_id, &payload.content)
        .await
    {
        Ok(message) => (StatusCode::CREATED, Json(message)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

async fn approve_draft(
    State(service): State<ChatService>,
    Path(message_id): Path<Uuid>,
    user: AuthenticatedUser,
) -> impl IntoResponse {
    match service.approve_draft(&user.tenant_id, message_id).await {
        Ok(message) => (StatusCode::OK, Json(message)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}
// E2E helper endpoint
pub async fn create_test_conversation(
    State(service): State<ChatService>,
    user: AuthenticatedUser,
) -> impl IntoResponse {
    match service.create_test_conversation(&user.tenant_id).await {
        Ok(conversation) => (StatusCode::CREATED, Json(conversation)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": e.to_string() })),
        )
            .into_response(),
    }
}

pub fn dev_chat_routes(chat_service: ChatService) -> Router {
    Router::new()
        .route("/dev/test-conversation", post(create_test_conversation))
        .with_state(chat_service)
}
