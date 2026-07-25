use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json, Router, routing::{get, post},
};
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;
use sqlx::PgPool;
use uuid::Uuid;
use crate::services::omnichannel_native_service::OmnichannelNativeRepository;
use server_auth::orchestration::AuthInfo;
use axum::extract::ws::{WebSocketUpgrade, WebSocket};

pub fn router(pool: PgPool) -> Router {
    let repo = Arc::new(OmnichannelNativeRepository::new(pool));
    Router::new()
        .route("/conversations", get(list_conversations))
        .route("/conversations/:id/messages", get(list_messages).post(send_message))
        // Simple websocket stub endpoint
        .route("/conversations/:id/ws", get(ws_handler))
        .with_state(repo)
}

async fn list_conversations(
    State(repo): State<Arc<OmnichannelNativeRepository>>,
    axum::extract::Extension(auth_info): axum::extract::Extension<AuthInfo>,
) -> impl IntoResponse {
    let tenant_id = auth_info.org_id.trim();
    if tenant_id.is_empty() {
        return Json(json!({ "success": false, "error": "Unauthorized" })).into_response();
    }

    match repo.fetch_conversations(tenant_id).await {
        Ok(conversations) => {
            let res = conversations.iter().map(|c| {
                json!({
                    "id": c.id.to_string(),
                    "status": c.status,
                    "contact_id": c.contact_id.to_string()
                })
            }).collect::<Vec<_>>();
            Json(json!({ "success": true, "conversations": res })).into_response()
        },
        Err(e) => Json(json!({ "success": false, "error": e.to_string() })).into_response()
    }
}

async fn list_messages(
    Path(conversation_id): Path<Uuid>,
    State(repo): State<Arc<OmnichannelNativeRepository>>,
    axum::extract::Extension(auth_info): axum::extract::Extension<AuthInfo>,
) -> impl IntoResponse {
    let tenant_id = auth_info.org_id.trim();
    if tenant_id.is_empty() {
        return Json(json!({ "success": false, "error": "Unauthorized" })).into_response();
    }

    match repo.fetch_messages(conversation_id, tenant_id).await {
        Ok(messages) => {
            let res = messages.iter().map(|m| {
                json!({
                    "id": m.id.to_string(),
                    "sender_type": m.sender_type,
                    "content": m.content,
                    "status": m.status
                })
            }).collect::<Vec<_>>();
            Json(json!({ "success": true, "messages": res })).into_response()
        },
        Err(e) => Json(json!({ "success": false, "error": e.to_string() })).into_response()
    }
}

#[derive(Deserialize)]
struct SendMessagePayload {
    content: String,
    sender_type: String,
}

async fn send_message(
    Path(conversation_id): Path<Uuid>,
    State(repo): State<Arc<OmnichannelNativeRepository>>,
    axum::extract::Extension(auth_info): axum::extract::Extension<AuthInfo>,
    Json(payload): Json<SendMessagePayload>,
) -> impl IntoResponse {
    let tenant_id = auth_info.org_id.trim();
    if tenant_id.is_empty() {
        return Json(json!({ "success": false, "error": "Unauthorized" })).into_response();
    }

    if payload.content.trim().is_empty() {
        return Json(json!({ "success": false, "error": "Content cannot be empty" })).into_response();
    }

    match repo.send_message(conversation_id, &payload.sender_type, &payload.content, tenant_id).await {
        Ok(message) => {
            let res = json!({
                "id": message.id.to_string(),
                "sender_type": message.sender_type,
                "content": message.content,
                "status": message.status
            });
            Json(json!({ "success": true, "message": res })).into_response()
        },
        Err(e) => Json(json!({ "success": false, "error": e.to_string() })).into_response()
    }
}

async fn ws_handler(
    Path(conversation_id): Path<Uuid>,
    ws: WebSocketUpgrade,
    axum::extract::Extension(auth_info): axum::extract::Extension<AuthInfo>,
) -> impl IntoResponse {
    let tenant_id = auth_info.org_id.clone();
    ws.on_upgrade(move |socket| handle_socket(socket, conversation_id, tenant_id))
}

async fn handle_socket(mut _socket: WebSocket, _conversation_id: Uuid, _tenant_id: String) {
    // Basic websocket stub - in a real implementation we would subscribe to a redis pubsub here
    // for this conversation/tenant
}
