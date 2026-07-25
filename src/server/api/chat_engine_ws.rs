use axum::{
    extract::{ws::{Message as WsMessage, WebSocket, WebSocketUpgrade}, Path, State, Extension},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use futures::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::db::DB;
use crate::common::Claims;
use crate::services::chat_engine::service::ChatEngineService;
use crate::services::chat_engine::repo::{ChatConversation, ChatMessage};
use crate::common::auth_utils::signed_tenant_id as strict_ui_claim_tenant;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DB>,
    pub chat_service: Arc<ChatEngineService>,
}

#[derive(Serialize, Deserialize)]
pub struct WsEvent {
    pub event_type: String, // "new_message", "typing", "ai_draft_ready"
    pub payload: serde_json::Value,
}

pub fn router(db: Arc<DB>) -> Router {
    let state = AppState {
        db: db.clone(),
        chat_service: Arc::new(ChatEngineService::new(db)),
    };

    Router::new()
        .route("/api/v1/chat_engine/conversations", get(list_conversations))
        .route("/api/v1/chat_engine/conversations/:id/messages", get(list_messages))
        .route("/api/v1/chat_engine/conversations/:id/messages", axum::routing::post(send_message))
        .route("/api/v1/chat_engine/messages/:id/approve", axum::routing::post(approve_message))
        .with_state(state)
}

async fn list_conversations(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<ChatConversation>>, axum::http::StatusCode> {
    let tenant_id = strict_ui_claim_tenant(&claims).ok_or(axum::http::StatusCode::UNAUTHORIZED)?;
    match state.chat_service.get_conversations(&tenant_id).await {
        Ok(convs) => Ok(Json(convs)),
        Err(e) => {
            tracing::error!("Failed to list conversations: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn list_messages(
    State(state): State<AppState>,
    Path(conversation_id): Path<String>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<Vec<ChatMessage>>, axum::http::StatusCode> {
    let tenant_id = strict_ui_claim_tenant(&claims).ok_or(axum::http::StatusCode::UNAUTHORIZED)?;
    match state.chat_service.get_messages(&tenant_id, &conversation_id).await {
        Ok(msgs) => Ok(Json(msgs)),
        Err(e) => {
            tracing::error!("Failed to list messages: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[derive(Deserialize)]
pub struct SendMessageRequest {
    pub content: String,
}

async fn send_message(
    State(state): State<AppState>,
    Path(conversation_id): Path<String>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<SendMessageRequest>,
) -> Result<Json<ChatMessage>, axum::http::StatusCode> {
    let tenant_id = strict_ui_claim_tenant(&claims).ok_or(axum::http::StatusCode::UNAUTHORIZED)?;
    match state.chat_service.ingest_message(&tenant_id, &conversation_id, "customer", &payload.content).await {
        Ok(msg) => {
            let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());
            if let Ok(client) = redis::Client::open(redis_url) {
                if let Ok(mut con) = client.get_tokio_connection().await {
                    use redis::AsyncCommands;
                    let channel = format!("ohc_chat_engine_ws:{}", tenant_id);
                    let event = WsEvent {
                        event_type: "new_message".to_string(),
                        payload: serde_json::to_value(&msg).unwrap(),
                    };
                    let _ : redis::RedisResult<()> = con.publish(&channel, serde_json::to_string(&event).unwrap()).await;
                }
            }
            Ok(Json(msg))
        },
        Err(e) => {
            tracing::error!("Failed to send message: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn approve_message(
    State(state): State<AppState>,
    Path(message_id): Path<String>,
    Extension(claims): Extension<Claims>,
) -> Result<axum::http::StatusCode, axum::http::StatusCode> {
    let tenant_id = strict_ui_claim_tenant(&claims).ok_or(axum::http::StatusCode::UNAUTHORIZED)?;

    let res = sqlx::query("SELECT conversation_id, draft_content FROM ohc_chat_messages WHERE id = $1 AND tenant_id = $2")
        .bind(&message_id)
        .bind(&tenant_id)
        .fetch_one(&state.db.pool)
        .await;

    if let Ok(row) = res {
        use sqlx::Row;
        let conv_id: String = row.get("conversation_id");
        if let Ok(draft) = row.try_get::<String, _>("draft_content") {
            let _ = state.chat_service.ingest_message(&tenant_id, &conv_id, "agent", &draft).await;
            let _ = sqlx::query("UPDATE ohc_chat_messages SET ai_draft_status = 'approved' WHERE id = $1")
                .bind(&message_id)
                .execute(&state.db.pool)
                .await;
            return Ok(axum::http::StatusCode::OK);
        }
    }

    Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Extension(claims): Extension<Claims>,
) -> impl IntoResponse {
    let tenant_id = match strict_ui_claim_tenant(&claims) {
        Some(t) => t,
        None => return (axum::http::StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    };
    ws.on_upgrade(move |socket| handle_socket(socket, tenant_id))
}

async fn handle_socket(socket: WebSocket, tenant_id: String) {
    let (mut sender, mut _receiver) = socket.split();

    let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());
    if let Ok(client) = redis::Client::open(redis_url) {
        if let Ok(con) = client.get_tokio_connection().await {
            let channel = format!("ohc_chat_engine_ws:{}", tenant_id);
            let mut pubsub = con.into_pubsub();
            if pubsub.subscribe(&channel).await.is_ok() {
                let mut stream = pubsub.into_on_message();
                while let Some(msg) = stream.next().await {
                    if let Ok(payload) = msg.get_payload::<String>() {
                        if sender.send(WsMessage::Text(payload.into())).await.is_err() {
                            break;
                        }
                    }
                }
            }
        }
    }
}
