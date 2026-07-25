use axum::{
    extract::{Path, State, WebSocketUpgrade, ws::{WebSocket, Message as WsMessage}},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use sqlx::PgPool;
use uuid::Uuid;
use crate::services::chat_engine::{ChatEngineService, Inbox, Conversation, Message};
use futures_util::{sink::SinkExt, stream::StreamExt};

#[derive(Clone)]
pub struct AppState {
    pub chat_service: Arc<ChatEngineService>,
    pub pool: PgPool,
}

#[derive(Deserialize)]
pub struct CreateInboxRequest {
    pub name: String,
    pub channel_type: String,
    pub channel_config: Option<serde_json::Value>,
}

#[derive(Deserialize)]
pub struct CreateConversationRequest {
    pub inbox_id: Uuid,
    pub contact_id: Uuid,
    pub assignee_id: Option<Uuid>,
}

#[derive(Deserialize)]
pub struct WebhookPayload {
    pub source: String,
    pub sender_id: String,
    pub message: String,
    // Add additional fields as needed
}

#[derive(Serialize)]
pub struct WebhookResponse {
    pub success: bool,
    pub message_id: Option<String>,
}

pub fn router<S>(pool: PgPool) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    let state = AppState {
        chat_service: Arc::new(ChatEngineService::new(pool.clone())),
        pool,
    };

    Router::new()
        .route("/api/v1/chat/:tenant_id/inboxes", post(create_inbox).get(get_inboxes))
        .route("/api/v1/chat/:tenant_id/conversations", post(create_conversation).get(get_conversations))
        .route("/api/v1/chat/:tenant_id/webhook", post(handle_webhook))
        .route("/api/v1/chat/ws/:tenant_id", get(ws_handler))
        .with_state(state)
}

async fn create_inbox(
    State(state): State<AppState>,
    Path(tenant_id_str): Path<String>,
    Json(payload): Json<CreateInboxRequest>,
) -> Result<Json<Inbox>, axum::http::StatusCode> {
    let tenant_id = Uuid::parse_str(&tenant_id_str).map_err(|_| axum::http::StatusCode::BAD_REQUEST)?;
    match state.chat_service.create_inbox(tenant_id, payload.name, payload.channel_type, payload.channel_config).await {
        Ok(inbox) => Ok(Json(inbox)),
        Err(e) => {
            tracing::error!("Failed to create inbox: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn get_inboxes(
    State(state): State<AppState>,
    Path(tenant_id_str): Path<String>,
) -> Result<Json<Vec<Inbox>>, axum::http::StatusCode> {
    let tenant_id = Uuid::parse_str(&tenant_id_str).map_err(|_| axum::http::StatusCode::BAD_REQUEST)?;
    match state.chat_service.get_inboxes(tenant_id).await {
        Ok(inboxes) => Ok(Json(inboxes)),
        Err(e) => {
            tracing::error!("Failed to fetch inboxes: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn create_conversation(
    State(state): State<AppState>,
    Path(tenant_id_str): Path<String>,
    Json(payload): Json<CreateConversationRequest>,
) -> Result<Json<Conversation>, axum::http::StatusCode> {
    let tenant_id = Uuid::parse_str(&tenant_id_str).map_err(|_| axum::http::StatusCode::BAD_REQUEST)?;
    match state.chat_service.create_conversation(tenant_id, payload.inbox_id, payload.contact_id, payload.assignee_id).await {
        Ok(conversation) => Ok(Json(conversation)),
        Err(e) => {
            tracing::error!("Failed to create conversation: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn get_conversations(
    State(state): State<AppState>,
    Path(tenant_id_str): Path<String>,
) -> Result<Json<Vec<Conversation>>, axum::http::StatusCode> {
    let tenant_id = Uuid::parse_str(&tenant_id_str).map_err(|_| axum::http::StatusCode::BAD_REQUEST)?;
    match state.chat_service.get_conversations(tenant_id).await {
        Ok(conversations) => Ok(Json(conversations)),
        Err(e) => {
            tracing::error!("Failed to fetch conversations: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn handle_webhook(
    State(state): State<AppState>,
    Path(tenant_id_str): Path<String>,
    Json(payload): Json<WebhookPayload>,
) -> Result<Json<WebhookResponse>, axum::http::StatusCode> {
    let tenant_id = Uuid::parse_str(&tenant_id_str).map_err(|_| axum::http::StatusCode::BAD_REQUEST)?;

    // In a real implementation, we would look up the conversation based on sender_id and source.
    // For this implementation, we will stub out the conversation creation flow since it's required by FK.
    let inbox = match state.chat_service.create_inbox(tenant_id, "Webhook Inbox".to_string(), "api".to_string(), None).await {
        Ok(i) => i,
        Err(e) => {
            tracing::error!("Failed to auto-create inbox for webhook: {}", e);
            return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
        }
    };
    let contact_id = Uuid::new_v4();
    let conversation = match state.chat_service.create_conversation(tenant_id, inbox.id, contact_id, None).await {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("Failed to auto-create conversation for webhook: {}", e);
            return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let conversation_id = conversation.id;

    let message_res = state.chat_service.create_message(
        tenant_id,
        conversation_id,
        None, // Assuming contact
        "contact".to_string(),
        payload.message.clone(),
        "incoming".to_string(),
        Some(serde_json::json!({"source": payload.source, "sender_id": payload.sender_id})),
    ).await;

    match message_res {
        Ok(msg) => {
            // Publish to Redis for WebSockets
            let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());
            if let Ok(client) = redis::Client::open(redis_url) {
                if let Ok(mut con) = client.get_multiplexed_async_connection().await {
                    let channel_name = format!("chat_engine:{}", tenant_id_str);
                    let msg_json = serde_json::to_string(&msg).unwrap_or_default();
                    let _: Result<(), _> = redis::cmd("PUBLISH").arg(channel_name).arg(msg_json).query_async(&mut con).await;
                }
            }
            Ok(Json(WebhookResponse { success: true, message_id: Some(msg.id.to_string()) }))
        },
        Err(e) => {
            tracing::error!("Failed to create message from webhook: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    Path(tenant_id): Path<String>,
) -> axum::response::Response {
    ws.on_upgrade(move |socket| handle_socket(socket, tenant_id))
}

async fn handle_socket(socket: WebSocket, tenant_id: String) {
    let (mut sender, mut receiver) = socket.split();

    let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());
    if let Ok(client) = redis::Client::open(redis_url) {
        if let Ok(con) = client.get_multiplexed_async_connection().await {
            let channel_name = format!("chat_engine:{}", tenant_id);
            if let Ok(mut pubsub) = client.get_async_pubsub().await {
                if pubsub.subscribe(&channel_name).await.is_ok() {
                    let mut pubsub_stream = pubsub.into_on_message();

                    let mut send_task = tokio::spawn(async move {
                        while let Some(msg) = pubsub_stream.next().await {
                            if let Ok(payload) = msg.get_payload::<String>() {
                                if sender.send(WsMessage::Text(payload.into())).await.is_err() {
                                    break;
                                }
                            }
                        }
                    });

                    let mut recv_task = tokio::spawn(async move {
                        while let Some(Ok(_)) = receiver.next().await {
                            // Ignore incoming messages for now, this is a read-only stream from server to client
                        }
                    });

                    tokio::select! {
                        _ = (&mut send_task) => recv_task.abort(),
                        _ = (&mut recv_task) => send_task.abort(),
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::extract::State;
    use axum::extract::Path;
    use axum::Json;

    #[tokio::test]
    async fn test_create_inbox_api() {
        let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());
        if let Ok(pool) = PgPool::connect(&database_url).await {
            let state = AppState {
                chat_service: Arc::new(ChatEngineService::new(pool.clone())),
                pool: pool.clone(),
            };

            let _ = sqlx::query("CREATE TABLE IF NOT EXISTS inboxes (id UUID PRIMARY KEY, tenant_id UUID NOT NULL, name TEXT NOT NULL, channel_type TEXT NOT NULL, channel_config JSONB, is_active BOOLEAN NOT NULL DEFAULT true, created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP, updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP);")
                .execute(&state.pool)
                .await;

            let tenant_id = Uuid::new_v4().to_string();
            let req = CreateInboxRequest {
                name: "Test API Inbox".to_string(),
                channel_type: "email".to_string(),
                channel_config: None,
            };

            let res = create_inbox(State(state), Path(tenant_id), Json(req)).await;
            assert!(res.is_ok());
        }
    }

    #[tokio::test]
    async fn test_handle_webhook_api() {
        let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());
        if let Ok(pool) = PgPool::connect(&database_url).await {
            let state = AppState {
                chat_service: Arc::new(ChatEngineService::new(pool.clone())),
                pool: pool.clone(),
            };

            let _ = sqlx::query("CREATE TABLE IF NOT EXISTS inboxes (id UUID PRIMARY KEY, tenant_id UUID NOT NULL, name TEXT NOT NULL, channel_type TEXT NOT NULL, channel_config JSONB, is_active BOOLEAN NOT NULL DEFAULT true, created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP, updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP);")
                .execute(&state.pool)
                .await;
            let _ = sqlx::query("CREATE TABLE IF NOT EXISTS conversations (id UUID PRIMARY KEY, tenant_id UUID NOT NULL, inbox_id UUID NOT NULL REFERENCES inboxes(id) ON DELETE CASCADE, contact_id UUID NOT NULL, assignee_id UUID, status TEXT NOT NULL DEFAULT 'open', last_activity_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP, created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP, updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP);")
                .execute(&state.pool)
                .await;
            let _ = sqlx::query("CREATE TABLE IF NOT EXISTS messages (id UUID PRIMARY KEY, tenant_id UUID NOT NULL, conversation_id UUID NOT NULL REFERENCES conversations(id) ON DELETE CASCADE, sender_id UUID, sender_type TEXT NOT NULL, content TEXT NOT NULL, message_type TEXT NOT NULL, external_source_ids JSONB, created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP);")
                .execute(&state.pool)
                .await;

            let tenant_id = Uuid::new_v4().to_string();
            let req = WebhookPayload {
                source: "whatsapp".to_string(),
                sender_id: "+123456789".to_string(),
                message: "Hello from webhook API test".to_string(),
            };

            let res = handle_webhook(State(state), Path(tenant_id), Json(req)).await;
            assert!(res.is_ok());
            let json_res = res.unwrap().0;
            assert!(json_res.success);
            assert!(json_res.message_id.is_some());
        }
    }
}
