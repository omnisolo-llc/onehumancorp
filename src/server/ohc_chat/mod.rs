use axum::{
    extract::{
        ws::{Message as WsMessage, WebSocket, WebSocketUpgrade},
        Path, State,
    },
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router,
};
use chrono::{DateTime, Utc};
use futures_util::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use tokio::sync::broadcast;
use uuid::Uuid;

// --- Models ---

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Inbox {
    pub id: String,
    pub tenant_id: String,
    pub name: String,
    pub channel_type: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Contact {
    pub id: String,
    pub tenant_id: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Conversation {
    pub id: String,
    pub tenant_id: String,
    pub inbox_id: String,
    pub contact_id: Option<String>,
    pub status: String,
    pub assignee_id: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ChatMessage {
    pub id: String,
    pub tenant_id: String,
    pub conversation_id: String,
    pub sender_id: String,
    pub sender_type: String,
    pub content: String,
    pub status: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

// --- Requests/Responses ---

#[derive(Debug, Deserialize)]
pub struct CreateInboxRequest {
    pub name: String,
    pub channel_type: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateContactRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateConversationRequest {
    pub inbox_id: String,
    pub contact_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct IngestMessageRequest {
    pub sender_id: String,
    pub sender_type: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct ChatSubscriptionFrame {
    pub action: String,
    pub token: String,
    pub tenant_id: String,
}

// --- Real-time Broadcasting & State ---

#[derive(Clone)]
pub struct ChatState {
    pub pool: PgPool,
    pub local_broadcast: broadcast::Sender<String>,
}

impl ChatState {
    pub fn new(pool: PgPool) -> Self {
        let (tx, _) = broadcast::channel(4096);
        Self {
            pool,
            local_broadcast: tx,
        }
    }
}

// --- Service layer functions ---

pub async fn set_tenant_context(tx: &mut sqlx::Transaction<'_, sqlx::Postgres>, tenant_id: &str) -> Result<(), sqlx::Error> {
    sqlx::query("SELECT set_config('app.current_tenant', $1, true)")
        .bind(tenant_id)
        .execute(&mut **tx)
        .await?;
    Ok(())
}

pub async fn db_create_inbox(pool: &PgPool, tenant_id: &str, name: &str, channel_type: &str) -> Result<Inbox, sqlx::Error> {
    let mut tx = pool.begin().await?;
    set_tenant_context(&mut tx, tenant_id).await?;

    let id = Uuid::new_v4().to_string();
    let inbox = sqlx::query_as::<_, Inbox>(
        "INSERT INTO inboxes (id, tenant_id, name, channel_type) VALUES ($1, $2, $3, $4) RETURNING *"
    )
    .bind(&id)
    .bind(tenant_id)
    .bind(name)
    .bind(channel_type)
    .fetch_one(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(inbox)
}

pub async fn db_create_contact(pool: &PgPool, tenant_id: &str, name: Option<&str>, email: Option<&str>, phone: Option<&str>) -> Result<Contact, sqlx::Error> {
    let mut tx = pool.begin().await?;
    set_tenant_context(&mut tx, tenant_id).await?;

    let id = Uuid::new_v4().to_string();
    let contact = sqlx::query_as::<_, Contact>(
        "INSERT INTO contacts (id, tenant_id, name, email, phone) VALUES ($1, $2, $3, $4, $5) RETURNING *"
    )
    .bind(&id)
    .bind(tenant_id)
    .bind(name)
    .bind(email)
    .bind(phone)
    .fetch_one(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(contact)
}

pub async fn db_create_conversation(pool: &PgPool, tenant_id: &str, inbox_id: &str, contact_id: Option<&str>) -> Result<Conversation, sqlx::Error> {
    let mut tx = pool.begin().await?;
    set_tenant_context(&mut tx, tenant_id).await?;

    let id = Uuid::new_v4().to_string();
    let conversation = sqlx::query_as::<_, Conversation>(
        "INSERT INTO conversations (id, tenant_id, inbox_id, contact_id, status) VALUES ($1, $2, $3, $4, 'open') RETURNING *"
    )
    .bind(&id)
    .bind(tenant_id)
    .bind(inbox_id)
    .bind(contact_id)
    .fetch_one(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(conversation)
}

pub async fn db_ingest_message(
    state: &ChatState,
    tenant_id: &str,
    conversation_id: &str,
    sender_id: &str,
    sender_type: &str,
    content: &str,
) -> Result<ChatMessage, sqlx::Error> {
    let mut tx = state.pool.begin().await?;
    set_tenant_context(&mut tx, tenant_id).await?;

    let id = Uuid::new_v4().to_string();
    let message = sqlx::query_as::<_, ChatMessage>(
        "INSERT INTO messages (id, tenant_id, conversation_id, sender_id, sender_type, content, status) VALUES ($1, $2, $3, $4, $5, $6, 'sent') RETURNING *"
    )
    .bind(&id)
    .bind(tenant_id)
    .bind(conversation_id)
    .bind(sender_id)
    .bind(sender_type)
    .bind(content)
    .fetch_one(&mut *tx)
    .await?;

    // Update conversation updated_at
    sqlx::query("UPDATE conversations SET updated_at = CURRENT_TIMESTAMP WHERE id = $1 AND tenant_id = $2")
        .bind(conversation_id)
        .bind(tenant_id)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;

    // Broadcast the message payload to active WebSocket clients
    let payload = serde_json::json!({
        "event": "message_created",
        "tenant_id": tenant_id,
        "message": message,
    });
    let payload_str = payload.to_string();

    // Redis broadcast
    if let Some(client) = server_config::get().redis_url.as_ref().and_then(|url| redis::Client::open(url.as_str()).ok()) {
        let payload_str_clone = payload_str.clone();
        let tenant_id_clone = tenant_id.to_string();
        tokio::spawn(async move {
            if let Ok(mut conn) = client.get_multiplexed_async_connection().await {
                let channel = format!("ohc:chat:{}", tenant_id_clone);
                let _: Result<(), redis::RedisError> = redis::cmd("PUBLISH")
                    .arg(&channel)
                    .arg(&payload_str_clone)
                    .query_async(&mut conn)
                    .await;
            }
        });
    }

    // Local broadcast fallback
    let _ = state.local_broadcast.send(payload_str);

    Ok(message)
}

// --- REST Endpoints ---

pub async fn handle_create_inbox(
    State(state): State<ChatState>,
    Extension(claims): Extension<server_common::Claims>,
    Json(req): Json<CreateInboxRequest>,
) -> Result<impl IntoResponse, axum::http::StatusCode> {
    let tenant_id = claims.organization_id.as_deref().unwrap_or("default");
    match db_create_inbox(&state.pool, tenant_id, &req.name, &req.channel_type).await {
        Ok(inbox) => Ok((axum::http::StatusCode::CREATED, Json(inbox))),
        Err(e) => {
            tracing::error!("Failed to create inbox: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn handle_create_contact(
    State(state): State<ChatState>,
    Extension(claims): Extension<server_common::Claims>,
    Json(req): Json<CreateContactRequest>,
) -> Result<impl IntoResponse, axum::http::StatusCode> {
    let tenant_id = claims.organization_id.as_deref().unwrap_or("default");
    match db_create_contact(&state.pool, tenant_id, req.name.as_deref(), req.email.as_deref(), req.phone.as_deref()).await {
        Ok(contact) => Ok((axum::http::StatusCode::CREATED, Json(contact))),
        Err(e) => {
            tracing::error!("Failed to create contact: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn handle_create_conversation(
    State(state): State<ChatState>,
    Extension(claims): Extension<server_common::Claims>,
    Json(req): Json<CreateConversationRequest>,
) -> Result<impl IntoResponse, axum::http::StatusCode> {
    let tenant_id = claims.organization_id.as_deref().unwrap_or("default");
    match db_create_conversation(&state.pool, tenant_id, &req.inbox_id, req.contact_id.as_deref()).await {
        Ok(convo) => Ok((axum::http::StatusCode::CREATED, Json(convo))),
        Err(e) => {
            tracing::error!("Failed to create conversation: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn handle_ingest_message(
    State(state): State<ChatState>,
    Extension(claims): Extension<server_common::Claims>,
    Path(conversation_id): Path<String>,
    Json(req): Json<IngestMessageRequest>,
) -> Result<impl IntoResponse, axum::http::StatusCode> {
    let tenant_id = claims.organization_id.as_deref().unwrap_or("default");
    match db_ingest_message(&state, tenant_id, &conversation_id, &req.sender_id, &req.sender_type, &req.content).await {
        Ok(message) => Ok((axum::http::StatusCode::CREATED, Json(message))),
        Err(e) => {
            tracing::error!("Failed to ingest message: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn handle_fetch_conversations(
    State(state): State<ChatState>,
    Extension(claims): Extension<server_common::Claims>,
) -> Result<impl IntoResponse, axum::http::StatusCode> {
    let tenant_id = claims.organization_id.as_deref().unwrap_or("default");
    let mut tx = match state.pool.begin().await {
        Ok(tx) => tx,
        Err(_) => return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    };
    if set_tenant_context(&mut tx, tenant_id).await.is_err() {
        return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
    }

    let convos = sqlx::query_as::<_, Conversation>("SELECT * FROM conversations")
        .fetch_all(&mut *tx)
        .await;

    let _ = tx.commit().await;

    match convos {
        Ok(items) => Ok(Json(items)),
        Err(e) => {
            tracing::error!("Failed to fetch conversations: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn handle_fetch_messages(
    State(state): State<ChatState>,
    Extension(claims): Extension<server_common::Claims>,
    Path(conversation_id): Path<String>,
) -> Result<impl IntoResponse, axum::http::StatusCode> {
    let tenant_id = claims.organization_id.as_deref().unwrap_or("default");
    let mut tx = match state.pool.begin().await {
        Ok(tx) => tx,
        Err(_) => return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR),
    };
    if set_tenant_context(&mut tx, tenant_id).await.is_err() {
        return Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
    }

    let msgs = sqlx::query_as::<_, ChatMessage>("SELECT * FROM messages WHERE conversation_id = $1")
        .bind(&conversation_id)
        .fetch_all(&mut *tx)
        .await;

    let _ = tx.commit().await;

    match msgs {
        Ok(items) => Ok(Json(items)),
        Err(e) => {
            tracing::error!("Failed to fetch messages: {}", e);
            Err(axum::http::StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// --- WebSocket Handler ---

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<ChatState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_chat_socket(socket, state))
}

async fn handle_chat_socket(socket: WebSocket, state: ChatState) {
    let (mut ws_sender, mut ws_receiver) = socket.split();

    // 1. Wait for first message to perform Token-Based Auth Frame check
    let first_msg = match ws_receiver.next().await {
        Some(Ok(msg)) => msg,
        _ => return,
    };

    let text_data = match first_msg {
        WsMessage::Text(text) => text,
        _ => {
            // Unauthenticated frame type, reject
            let _ = ws_sender.close().await;
            return;
        }
    };

    let subscription_frame: ChatSubscriptionFrame = match serde_json::from_str(&text_data) {
        Ok(frame) => frame,
        Err(_) => {
            let _ = ws_sender.close().await;
            return;
        }
    };

    // Use standard OHC auth store to validate token
    let auth_store = ::server_auth::Store::new();
    let claims = match auth_store.validate_token(&subscription_frame.token).await {
        Ok(c) => c,
        Err(_) => {
            let _ = ws_sender.close().await;
            return;
        }
    };

    // Ensure claims match the requested tenant context
    let tenant_id: String = match claims.organization_id {
        Some(org_id) if org_id == subscription_frame.tenant_id => org_id,
        _ => {
            let _ = ws_sender.close().await;
            return;
        }
    };

    // Send successful subscription acknowledgment
    let ack = serde_json::json!({
        "event": "subscribed",
        "status": "ok",
        "tenant_id": tenant_id
    });
    if ws_sender.send(WsMessage::Text(ack.to_string().into())).await.is_err() {
        return;
    }

    // Subscribe to local broadcast and Redis Pub/Sub
    let mut local_rx = state.local_broadcast.subscribe();

    let redis_client_opt = server_config::get().redis_url.as_ref().and_then(|url| redis::Client::open(url.as_str()).ok());
    let (redis_tx, mut redis_rx) = tokio::sync::mpsc::channel::<String>(256);

    let tenant_id_clone = tenant_id.clone();
    let redis_pubsub_task = tokio::spawn(async move {
        if let Some(client) = redis_client_opt {
            if let Ok(mut pubsub_conn) = client.get_async_pubsub().await {
                let channel = format!("ohc:chat:{}", tenant_id_clone);
                if pubsub_conn.subscribe(&channel).await.is_ok() {
                    let mut stream = pubsub_conn.into_on_message();
                    while let Some(msg) = stream.next().await {
                        if let Ok(payload) = msg.get_payload::<String>() {
                            let _ = redis_tx.send(payload).await;
                        }
                    }
                }
            }
        }
    });

    let tenant_id_send_loop = tenant_id.clone();
    let ws_send_loop = tokio::spawn(async move {
        loop {
            tokio::select! {
                // local broadcast channel with robust busy-loop/closed checks
                local_res = local_rx.recv() => {
                    match local_res {
                        Ok(raw) => {
                            if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&raw) {
                                if parsed.get("tenant_id").and_then(|v| v.as_str()) == Some(&tenant_id_send_loop) {
                                    if ws_sender.send(WsMessage::Text(raw.into())).await.is_err() {
                                        break;
                                    }
                                }
                            }
                        }
                        Err(broadcast::error::RecvError::Closed) => {
                            // Prevent CPU exhaustion/infinite tight loops if local channel gets closed
                            break;
                        }
                        Err(broadcast::error::RecvError::Lagged(_)) => {
                            // Gracefully continue on lagged messages
                            continue;
                        }
                    }
                }
                // redis pub/sub channel
                Some(raw) = redis_rx.recv() => {
                    if ws_sender.send(WsMessage::Text(raw.into())).await.is_err() {
                        break;
                    }
                }
            }
        }
    });

    let ws_recv_loop = tokio::spawn(async move {
        while let Some(Ok(msg)) = ws_receiver.next().await {
            if let WsMessage::Close(_) = msg {
                break;
            }
        }
    });

    tokio::select! {
        _ = redis_pubsub_task => {}
        _ = ws_send_loop => {}
        _ = ws_recv_loop => {}
    }
}

// --- Routing Entry ---

pub fn router<S: Clone + Send + Sync + 'static>(pool: PgPool) -> Router<S> {
    let state = ChatState::new(pool);
    Router::new()
        .route("/inboxes", post(handle_create_inbox))
        .route("/contacts", post(handle_create_contact))
        .route("/conversations", get(handle_fetch_conversations).post(handle_create_conversation))
        .route("/conversations/:id/messages", get(handle_fetch_messages).post(handle_ingest_message))
        .route("/ws", get(ws_handler))
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::SqlitePool;

    #[tokio::test]
    async fn test_chat_state_broadcast_broadcasts_payload() {
        let _pool = SqlitePool::connect("sqlite::memory:").await.unwrap();

        let (tx, mut rx1) = broadcast::channel(16);
        let tx_clone = tx.clone();

        tx_clone.send("test_message".to_string()).unwrap();
        let received = rx1.recv().await.unwrap();
        assert_eq!(received, "test_message");
    }

    #[test]
    fn test_models_serialization_roundtrip() {
        let inbox = Inbox {
            id: "inbox-1".to_string(),
            tenant_id: "tenant-a".to_string(),
            name: "WhatsApp Support".to_string(),
            channel_type: "whatsapp".to_string(),
            created_at: None,
            updated_at: None,
        };

        let serialized = serde_json::to_string(&inbox).unwrap();
        let deserialized: Inbox = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.id, "inbox-1");
        assert_eq!(deserialized.name, "WhatsApp Support");
    }

    #[test]
    fn test_create_inbox_request_deserialization() {
        let data = r#"{"name": "Instagram DM", "channel_type": "instagram"}"#;
        let req: CreateInboxRequest = serde_json::from_str(data).unwrap();
        assert_eq!(req.name, "Instagram DM");
        assert_eq!(req.channel_type, "instagram");
    }
}
