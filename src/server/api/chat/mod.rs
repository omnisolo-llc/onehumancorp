use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use axum::response::IntoResponse;
use serde_json::json;

use crate::services::chat::service::ChatService;

pub fn router(service: ChatService) -> Router {
    Router::new()
        .route("/inboxes", get(get_inboxes).post(create_inbox))
        .route(
            "/conversations/:conversation_id/messages",
            get(get_messages).post(send_message),
        )
        .with_state(service)
}

async fn get_inboxes(State(service): State<ChatService>) -> impl IntoResponse {
    // Hardcode tenant_id for now as we don't have the auth middleware hooked up in this isolated API test
    match service.get_inboxes("org-1").await {
        Ok(inboxes) => Json(json!({ "inboxes": inboxes })).into_response(),
        Err(e) => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            e.to_string(),
        )
            .into_response(),
    }
}

async fn create_inbox(
    State(service): State<ChatService>,
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    let name = payload["name"].as_str().unwrap_or("New Inbox");
    let channel_type = payload["channel_type"].as_str().unwrap_or("web");
    let config = payload["config"].clone();

    match service
        .create_inbox("org-1", name, channel_type, config)
        .await
    {
        Ok(inbox) => Json(json!({ "inbox": inbox })).into_response(),
        Err(e) => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            e.to_string(),
        )
            .into_response(),
    }
}

async fn get_messages(
    State(service): State<ChatService>,
    Path(conversation_id): Path<String>,
) -> impl IntoResponse {
    match service.get_conversation_messages("org-1", &conversation_id).await {
        Ok(messages) => Json(json!({ "messages": messages })).into_response(),
        Err(e) => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            e.to_string(),
        )
            .into_response(),
    }
}

async fn send_message(
    State(service): State<ChatService>,
    Path(conversation_id): Path<String>,
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    let content = payload["content"].as_str().unwrap_or("");
    let sender_type = payload["sender_type"].as_str().unwrap_or("agent");
    let sender_id = payload["sender_id"].as_str();

    match service
        .send_message("org-1", &conversation_id, sender_type, sender_id, content)
        .await
    {
        Ok(message) => Json(json!({ "message": message })).into_response(),
        Err(e) => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            e.to_string(),
        )
            .into_response(),
    }
}
