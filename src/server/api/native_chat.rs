use crate::db::DB;
use crate::services::chat::service::ChatService;
use axum::{
    extract::{
        ws::{Message as WsMessage, WebSocket, WebSocketUpgrade},
        Json, Query, State,
    },
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use futures::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DB>,
    pub redis: redis::Client,
}

#[derive(Debug, Deserialize)]
pub struct ChatWebhookPayload {
    pub tenant_id: String,
    pub inbox_id: String,
    pub channel_type: String, // e.g. "whatsapp", "web_widget"
    pub sender_id: String, // could be phone number or some internal contact identity
    pub sender_name: Option<String>,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct WebhookResponse {
    pub success: bool,
    pub message_id: Option<Uuid>,
}

#[derive(Deserialize)]
pub struct WsQuery {
    pub tenant_id: String,
}

pub fn router(db: Arc<DB>, redis: redis::Client) -> Router {
    let state = AppState { db, redis };
    Router::new()
        .route("/webhook", post(handle_webhook))
        .route("/ws", get(ws_handler))
        .with_state(state)
}

pub async fn handle_webhook(
    State(state): State<AppState>,
    Json(payload): Json<ChatWebhookPayload>,
) -> impl IntoResponse {
    let tenant_id = match Uuid::parse_str(&payload.tenant_id) {
        Ok(id) => id,
        Err(_) => return (StatusCode::BAD_REQUEST, Json(WebhookResponse { success: false, message_id: None })).into_response(),
    };

    let inbox_id = match Uuid::parse_str(&payload.inbox_id) {
        Ok(id) => id,
        Err(_) => return (StatusCode::BAD_REQUEST, Json(WebhookResponse { success: false, message_id: None })).into_response(),
    };

    let chat_service = ChatService::new(state.db.pool.clone());

    let contact = match chat_service.create_contact(tenant_id, payload.sender_name, None, Some(payload.sender_id.clone())).await {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("Failed to create contact: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(WebhookResponse { success: false, message_id: None })).into_response();
        }
    };

    let conversation = match chat_service.start_conversation(tenant_id, inbox_id, contact.id, None).await {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("Failed to start conversation: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(WebhookResponse { success: false, message_id: None })).into_response();
        }
    };

    let message = match chat_service.send_message(tenant_id, conversation.id, "contact".to_string(), Some(contact.id), payload.message.clone()).await {
        Ok(m) => m,
        Err(e) => {
            tracing::error!("Failed to send message: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(WebhookResponse { success: false, message_id: None })).into_response();
        }
    };

    let channel_name = format!("chat:{}", tenant_id);
    let mut conn = match state.redis.get_multiplexed_async_connection().await {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("Failed to get redis conn: {}", e);
            return (StatusCode::OK, Json(WebhookResponse { success: true, message_id: Some(message.id) })).into_response();
        }
    };

    let msg_json = serde_json::to_string(&message).unwrap_or_default();
    let _: redis::RedisResult<()> = redis::cmd("PUBLISH").arg(&channel_name).arg(&msg_json).query_async(&mut conn).await;

    (StatusCode::OK, Json(WebhookResponse { success: true, message_id: Some(message.id) })).into_response()
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Query(query): Query<WsQuery>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let tenant_id = query.tenant_id;
    ws.on_upgrade(move |socket| handle_socket(socket, tenant_id, state.redis))
}

async fn handle_socket(socket: WebSocket, tenant_id: String, redis_client: redis::Client) {
    let (mut sender, mut _receiver) = socket.split();

    let channel_name = format!("chat:{}", tenant_id);
    let mut pubsub_conn = match redis_client.get_async_pubsub().await {
        Ok(conn) => conn,
        Err(e) => {
            tracing::error!("Failed to get redis pubsub conn: {}", e);
            return;
        }
    };

    tokio::spawn(async move {
        if let Err(e) = pubsub_conn.subscribe(&channel_name).await {
            tracing::error!("Failed to subscribe to chat channel: {}", e);
            return;
        }

        let mut pubsub_stream = pubsub_conn.on_message();
        while let Some(msg) = pubsub_stream.next().await {
            if let Ok(payload) = msg.get_payload::<String>() {
                if sender.send(WsMessage::Text(payload.into())).await.is_err() {
                    break;
                }
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_webhook_payload_deserialization() {
        let json = r#"{
            "tenant_id": "00000000-0000-0000-0000-000000000001",
            "inbox_id": "00000000-0000-0000-0000-000000000002",
            "channel_type": "whatsapp",
            "sender_id": "+1234567890",
            "sender_name": "John Doe",
            "message": "Hello world"
        }"#;

        let payload: ChatWebhookPayload = serde_json::from_str(json).unwrap();
        assert_eq!(payload.tenant_id, "00000000-0000-0000-0000-000000000001");
        assert_eq!(payload.inbox_id, "00000000-0000-0000-0000-000000000002");
        assert_eq!(payload.channel_type, "whatsapp");
        assert_eq!(payload.sender_id, "+1234567890");
        assert_eq!(payload.sender_name.unwrap(), "John Doe");
        assert_eq!(payload.message, "Hello world");
    }
}
