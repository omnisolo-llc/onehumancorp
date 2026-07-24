use axum::{
    extract::{ws::{Message as WsMessage, WebSocket, WebSocketUpgrade}, Extension, Query, Path, State},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use axum::http::StatusCode;
use std::sync::{Arc, OnceLock};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use dashmap::DashMap;
use sqlx::{Row};
use chrono::{DateTime, Utc};
use futures::{sink::SinkExt, stream::StreamExt};
use crate::db::DB;
use ::server_ohc::domain::chat::models::{Inbox, ChannelAdapter, Contact, Conversation, Message};

#[derive(Clone)]
pub struct CustomChatState {
    pub db: Arc<DB>,
}

#[derive(Deserialize)]
pub struct CreateInboxRequest {
    pub tenant_id: String,
    pub name: String,
}

#[derive(Deserialize)]
pub struct CreateChannelAdapterRequest {
    pub tenant_id: String,
    pub inbox_id: String,
    pub type_: String,
    pub credentials: String,
}

#[derive(Deserialize)]
pub struct CreateContactRequest {
    pub tenant_id: String,
    pub name: String,
    pub identifier: String,
}

#[derive(Deserialize)]
pub struct CreateConversationRequest {
    pub tenant_id: String,
    pub inbox_id: String,
    pub contact_id: String,
    pub status: String,
}

#[derive(Deserialize)]
pub struct CreateMessageRequest {
    pub tenant_id: String,
    pub conversation_id: String,
    pub content: String,
    pub sender_type: String,
}

#[derive(Deserialize)]
pub struct CustomWsQuery {
    pub conversation_id: String,
}

#[derive(Debug, Clone)]
pub enum ChatDbPool {
    Postgres(sqlx::PgPool),
    Sqlite(sqlx::SqlitePool),
}

impl ChatDbPool {
    pub async fn create_inbox(&self, tenant_id: &str, name: &str) -> Result<Inbox, sqlx::Error> {
        let id = Uuid::new_v4().to_string();
        match self {
            ChatDbPool::Postgres(pool) => {
                sqlx::query("INSERT INTO inboxes (id, tenant_id, name) VALUES ($1, $2, $3)")
                    .bind(&id)
                    .bind(tenant_id)
                    .bind(name)
                    .execute(pool)
                    .await?;
            }
            ChatDbPool::Sqlite(pool) => {
                sqlx::query("INSERT INTO inboxes (id, tenant_id, name) VALUES (?, ?, ?)")
                    .bind(&id)
                    .bind(tenant_id)
                    .bind(name)
                    .execute(pool)
                    .await?;
            }
        }
        Ok(Inbox {
            id,
            tenant_id: tenant_id.to_string(),
            name: name.to_string(),
        })
    }

    pub async fn create_channel_adapter(&self, tenant_id: &str, inbox_id: &str, type_: &str, credentials: &str) -> Result<ChannelAdapter, sqlx::Error> {
        let id = Uuid::new_v4().to_string();
        match self {
            ChatDbPool::Postgres(pool) => {
                sqlx::query("INSERT INTO channel_adapters (id, tenant_id, inbox_id, type, credentials) VALUES ($1, $2, $3, $4, $5)")
                    .bind(&id)
                    .bind(tenant_id)
                    .bind(inbox_id)
                    .bind(type_)
                    .bind(credentials)
                    .execute(pool)
                    .await?;
            }
            ChatDbPool::Sqlite(pool) => {
                sqlx::query("INSERT INTO channel_adapters (id, tenant_id, inbox_id, type, credentials) VALUES (?, ?, ?, ?, ?)")
                    .bind(&id)
                    .bind(tenant_id)
                    .bind(inbox_id)
                    .bind(type_)
                    .bind(credentials)
                    .execute(pool)
                    .await?;
            }
        }
        Ok(ChannelAdapter {
            id,
            tenant_id: tenant_id.to_string(),
            inbox_id: inbox_id.to_string(),
            type_: type_.to_string(),
            credentials: credentials.to_string(),
        })
    }

    pub async fn create_contact(&self, tenant_id: &str, name: &str, identifier: &str) -> Result<Contact, sqlx::Error> {
        let id = Uuid::new_v4().to_string();
        match self {
            ChatDbPool::Postgres(pool) => {
                sqlx::query("INSERT INTO contacts (id, tenant_id, name, identifier) VALUES ($1, $2, $3, $4)")
                    .bind(&id)
                    .bind(tenant_id)
                    .bind(name)
                    .bind(identifier)
                    .execute(pool)
                    .await?;
            }
            ChatDbPool::Sqlite(pool) => {
                sqlx::query("INSERT INTO contacts (id, tenant_id, name, identifier) VALUES (?, ?, ?, ?)")
                    .bind(&id)
                    .bind(tenant_id)
                    .bind(name)
                    .bind(identifier)
                    .execute(pool)
                    .await?;
            }
        }
        Ok(Contact {
            id,
            tenant_id: tenant_id.to_string(),
            name: name.to_string(),
            identifier: identifier.to_string(),
        })
    }

    pub async fn create_conversation(&self, tenant_id: &str, inbox_id: &str, contact_id: &str, status: &str) -> Result<Conversation, sqlx::Error> {
        let id = Uuid::new_v4().to_string();
        match self {
            ChatDbPool::Postgres(pool) => {
                sqlx::query("INSERT INTO conversations (id, tenant_id, inbox_id, contact_id, status) VALUES ($1, $2, $3, $4, $5)")
                    .bind(&id)
                    .bind(tenant_id)
                    .bind(inbox_id)
                    .bind(contact_id)
                    .bind(status)
                    .execute(pool)
                    .await?;
            }
            ChatDbPool::Sqlite(pool) => {
                sqlx::query("INSERT INTO conversations (id, tenant_id, inbox_id, contact_id, status) VALUES (?, ?, ?, ?, ?)")
                    .bind(&id)
                    .bind(tenant_id)
                    .bind(inbox_id)
                    .bind(contact_id)
                    .bind(status)
                    .execute(pool)
                    .await?;
            }
        }
        Ok(Conversation {
            id,
            tenant_id: tenant_id.to_string(),
            inbox_id: inbox_id.to_string(),
            contact_id: contact_id.to_string(),
            status: status.to_string(),
        })
    }

    pub async fn create_message(&self, tenant_id: &str, conversation_id: &str, content: &str, sender_type: &str) -> Result<Message, sqlx::Error> {
        let id = Uuid::new_v4().to_string();
        let created_at = Utc::now();
        match self {
            ChatDbPool::Postgres(pool) => {
                sqlx::query("INSERT INTO messages (id, tenant_id, conversation_id, content, sender_type, created_at) VALUES ($1, $2, $3, $4, $5, $6)")
                    .bind(&id)
                    .bind(tenant_id)
                    .bind(conversation_id)
                    .bind(content)
                    .bind(sender_type)
                    .bind(created_at)
                    .execute(pool)
                    .await?;
            }
            ChatDbPool::Sqlite(pool) => {
                sqlx::query("INSERT INTO messages (id, tenant_id, conversation_id, content, sender_type, created_at) VALUES (?, ?, ?, ?, ?, ?)")
                    .bind(&id)
                    .bind(tenant_id)
                    .bind(conversation_id)
                    .bind(content)
                    .bind(sender_type)
                    .bind(created_at)
                    .execute(pool)
                    .await?;
            }
        }
        Ok(Message {
            id,
            tenant_id: tenant_id.to_string(),
            conversation_id: conversation_id.to_string(),
            content: content.to_string(),
            sender_type: sender_type.to_string(),
            created_at,
        })
    }

    pub async fn list_conversations(&self, tenant_id: &str) -> Result<Vec<Conversation>, sqlx::Error> {
        let rows = match self {
            ChatDbPool::Postgres(pool) => {
                sqlx::query("SELECT id, tenant_id, inbox_id, contact_id, status FROM conversations WHERE tenant_id = $1 ORDER BY created_at DESC")
                    .bind(tenant_id)
                    .fetch_all(pool)
                    .await?
            }
            ChatDbPool::Sqlite(pool) => {
                sqlx::query("SELECT id, tenant_id, inbox_id, contact_id, status FROM conversations WHERE tenant_id = ? ORDER BY created_at DESC")
                    .bind(tenant_id)
                    .fetch_all(pool)
                    .await?
            }
        };

        let conversations = rows.into_iter().map(|row| {
            Conversation {
                id: row.get("id"),
                tenant_id: row.get("tenant_id"),
                inbox_id: row.get("inbox_id"),
                contact_id: row.get("contact_id"),
                status: row.get("status"),
            }
        }).collect();

        Ok(conversations)
    }

    pub async fn list_messages(&self, tenant_id: &str, conversation_id: &str) -> Result<Vec<Message>, sqlx::Error> {
        let rows = match self {
            ChatDbPool::Postgres(pool) => {
                sqlx::query("SELECT id, tenant_id, conversation_id, content, sender_type, created_at FROM messages WHERE tenant_id = $1 AND conversation_id = $2 ORDER BY created_at ASC")
                    .bind(tenant_id)
                    .bind(conversation_id)
                    .fetch_all(pool)
                    .await?
            }
            ChatDbPool::Sqlite(pool) => {
                sqlx::query("SELECT id, tenant_id, conversation_id, content, sender_type, created_at FROM messages WHERE tenant_id = ? AND conversation_id = ? ORDER BY created_at ASC")
                    .bind(tenant_id)
                    .bind(conversation_id)
                    .fetch_all(pool)
                    .await?
            }
        };

        let messages = rows.into_iter().map(|row| {
            Message {
                id: row.get("id"),
                tenant_id: row.get("tenant_id"),
                conversation_id: row.get("conversation_id"),
                content: row.get("content"),
                sender_type: row.get("sender_type"),
                created_at: row.get("created_at"),
            }
        }).collect();

        Ok(messages)
    }
}

fn get_chat_db_pool(db: &crate::db::DB) -> ChatDbPool {
    match &db.store {
        crate::db::DbStore::Postgres => ChatDbPool::Postgres(db.pool.clone()),
        crate::db::DbStore::Sqlite(sqlite_pool) => ChatDbPool::Sqlite(sqlite_pool.clone()),
    }
}

fn get_broadcast_map() -> &'static DashMap<String, tokio::sync::broadcast::Sender<String>> {
    static MAP: OnceLock<DashMap<String, tokio::sync::broadcast::Sender<String>>> = OnceLock::new();
    MAP.get_or_init(DashMap::new)
}

pub fn get_conversation_sender(conv_id: &str) -> tokio::sync::broadcast::Sender<String> {
    let map = get_broadcast_map();
    if let Some(tx) = map.get(conv_id) {
        tx.clone()
    } else {
        let (tx, _) = tokio::sync::broadcast::channel(100);
        map.insert(conv_id.to_string(), tx.clone());
        tx
    }
}

pub fn router(db: Arc<DB>) -> Router {
    let state = CustomChatState { db };
    Router::new()
        .route("/inboxes", post(create_inbox))
        .route("/channel-adapters", post(create_channel_adapter))
        .route("/contacts", post(create_contact))
        .route("/conversations", post(create_conversation))
        .route("/messages", post(create_message))
        .route("/conversations/:tenant_id", get(list_conversations))
        .route("/messages/:tenant_id/:conversation_id", get(list_messages))
        .route("/ws", get(custom_chat_ws_handler))
        .with_state(state)
}

pub async fn create_inbox(
    State(state): State<CustomChatState>,
    Extension(claims): Extension<::server_common::Claims>,
    Json(payload): Json<CreateInboxRequest>,
) -> impl IntoResponse {
    if claims.organization_id.as_deref() != Some(payload.tenant_id.as_str()) {
        return (StatusCode::FORBIDDEN, "Unauthorized tenant access").into_response();
    }

    let chat_pool = get_chat_db_pool(&state.db);
    match chat_pool.create_inbox(&payload.tenant_id, &payload.name).await {
        Ok(inbox) => (StatusCode::CREATED, Json(inbox)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn create_channel_adapter(
    State(state): State<CustomChatState>,
    Extension(claims): Extension<::server_common::Claims>,
    Json(payload): Json<CreateChannelAdapterRequest>,
) -> impl IntoResponse {
    if claims.organization_id.as_deref() != Some(payload.tenant_id.as_str()) {
        return (StatusCode::FORBIDDEN, "Unauthorized tenant access").into_response();
    }

    let chat_pool = get_chat_db_pool(&state.db);
    match chat_pool.create_channel_adapter(&payload.tenant_id, &payload.inbox_id, &payload.type_, &payload.credentials).await {
        Ok(adapter) => (StatusCode::CREATED, Json(adapter)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn create_contact(
    State(state): State<CustomChatState>,
    Extension(claims): Extension<::server_common::Claims>,
    Json(payload): Json<CreateContactRequest>,
) -> impl IntoResponse {
    if claims.organization_id.as_deref() != Some(payload.tenant_id.as_str()) {
        return (StatusCode::FORBIDDEN, "Unauthorized tenant access").into_response();
    }

    let chat_pool = get_chat_db_pool(&state.db);
    match chat_pool.create_contact(&payload.tenant_id, &payload.name, &payload.identifier).await {
        Ok(contact) => (StatusCode::CREATED, Json(contact)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn create_conversation(
    State(state): State<CustomChatState>,
    Extension(claims): Extension<::server_common::Claims>,
    Json(payload): Json<CreateConversationRequest>,
) -> impl IntoResponse {
    if claims.organization_id.as_deref() != Some(payload.tenant_id.as_str()) {
        return (StatusCode::FORBIDDEN, "Unauthorized tenant access").into_response();
    }

    let chat_pool = get_chat_db_pool(&state.db);
    match chat_pool.create_conversation(&payload.tenant_id, &payload.inbox_id, &payload.contact_id, &payload.status).await {
        Ok(conversation) => (StatusCode::CREATED, Json(conversation)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn create_message(
    State(state): State<CustomChatState>,
    Extension(claims): Extension<::server_common::Claims>,
    Json(payload): Json<CreateMessageRequest>,
) -> impl IntoResponse {
    if claims.organization_id.as_deref() != Some(payload.tenant_id.as_str()) {
        return (StatusCode::FORBIDDEN, "Unauthorized tenant access").into_response();
    }

    let chat_pool = get_chat_db_pool(&state.db);
    match chat_pool.create_message(&payload.tenant_id, &payload.conversation_id, &payload.content, &payload.sender_type).await {
        Ok(msg) => {
            // Broadcast the message content in real-time to WebSocket clients
            if let Ok(serialized) = serde_json::to_string(&msg) {
                let _ = get_conversation_sender(&payload.conversation_id).send(serialized);
            }
            (StatusCode::CREATED, Json(msg)).into_response()
        }
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn list_conversations(
    State(state): State<CustomChatState>,
    Extension(claims): Extension<::server_common::Claims>,
    Path(tenant_id): Path<String>,
) -> impl IntoResponse {
    if claims.organization_id.as_deref() != Some(tenant_id.as_str()) {
        return (StatusCode::FORBIDDEN, "Unauthorized tenant access").into_response();
    }

    let chat_pool = get_chat_db_pool(&state.db);
    match chat_pool.list_conversations(&tenant_id).await {
        Ok(conversations) => (StatusCode::OK, Json(conversations)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn list_messages(
    State(state): State<CustomChatState>,
    Extension(claims): Extension<::server_common::Claims>,
    Path((tenant_id, conversation_id)): Path<(String, String)>,
) -> impl IntoResponse {
    if claims.organization_id.as_deref() != Some(tenant_id.as_str()) {
        return (StatusCode::FORBIDDEN, "Unauthorized tenant access").into_response();
    }

    let chat_pool = get_chat_db_pool(&state.db);
    match chat_pool.list_messages(&tenant_id, &conversation_id).await {
        Ok(messages) => (StatusCode::OK, Json(messages)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn custom_chat_ws_handler(
    ws: WebSocketUpgrade,
    Extension(claims): Extension<::server_common::Claims>,
    Query(query): Query<CustomWsQuery>,
) -> impl IntoResponse {
    let _tenant_id = match claims.organization_id.as_deref() {
        Some(org_id) if !org_id.is_empty() => org_id.to_string(),
        _ => "default".to_string(),
    };
    ws.on_upgrade(move |socket| handle_custom_chat_socket(socket, query.conversation_id))
}

async fn handle_custom_chat_socket(socket: WebSocket, conversation_id: String) {
    let (mut sender, mut receiver) = socket.split();

    let tx = get_conversation_sender(&conversation_id);
    let mut rx = tx.subscribe();

    let (ws_tx, mut ws_rx) = tokio::sync::mpsc::channel::<String>(100);

    let send_task = tokio::spawn(async move {
        while let Some(msg) = ws_rx.recv().await {
            if sender.send(WsMessage::Text(msg.into())).await.is_err() {
                break;
            }
        }
    });

    let ws_tx_clone = ws_tx.clone();
    let receive_broadcast_task = tokio::spawn(async move {
        loop {
            match rx.recv().await {
                Ok(raw) => {
                    let _ = ws_tx_clone.send(raw).await;
                }
                Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => {}
                Err(tokio::sync::broadcast::error::RecvError::Closed) => break,
            }
        }
    });

    let recv_ws_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                WsMessage::Close(_) => break,
                _ => {}
            }
        }
    });

    tokio::select! {
        _ = send_task => {}
        _ = receive_broadcast_task => {}
        _ = recv_ws_task => {}
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::DbStore;

    async fn setup_test_sqlite_db() -> Arc<DB> {
        let pool = sqlx::SqlitePool::connect("sqlite::memory:").await.unwrap();

        sqlx::query("
            CREATE TABLE IF NOT EXISTS inboxes (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                name TEXT NOT NULL,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            );
            CREATE TABLE IF NOT EXISTS channel_adapters (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                inbox_id TEXT NOT NULL,
                type TEXT NOT NULL,
                credentials TEXT NOT NULL,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            );
            CREATE TABLE IF NOT EXISTS contacts (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                name TEXT NOT NULL,
                identifier TEXT NOT NULL,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            );
            CREATE TABLE IF NOT EXISTS conversations (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                inbox_id TEXT NOT NULL,
                contact_id TEXT NOT NULL,
                status TEXT NOT NULL,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            );
            CREATE TABLE IF NOT EXISTS messages (
                id TEXT PRIMARY KEY,
                tenant_id TEXT NOT NULL,
                conversation_id TEXT NOT NULL,
                content TEXT NOT NULL,
                sender_type TEXT NOT NULL,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            );
        ").execute(&pool).await.unwrap();

        Arc::new(DB {
            pool: sqlx::PgPool::connect_lazy("postgres://localhost:5432/fake").unwrap(),
            store: DbStore::Sqlite(pool),
        })
    }

    #[tokio::test]
    async fn test_get_conversation_sender() {
        let tx1 = get_conversation_sender("conv-123");
        let tx2 = get_conversation_sender("conv-123");
        assert_eq!(tx1.receiver_count(), tx2.receiver_count());
    }

    #[tokio::test]
    async fn test_full_custom_chat_flow() {
        let db = setup_test_sqlite_db().await;
        let state = CustomChatState { db: db.clone() };

        let claims = ::server_common::Claims {
            sub: "test-user".to_string(),
            organization_id: Some("tenant-1".to_string()),
            role: "ADMIN".to_string(),
            exp: 9999999999,
        };

        // 1. Create Inbox
        let inbox_req = CreateInboxRequest {
            tenant_id: "tenant-1".to_string(),
            name: "Maya's Bakery".to_string(),
        };
        let response = create_inbox(State(state.clone()), Extension(claims.clone()), Json(inbox_req)).await.into_response();
        assert_eq!(response.status(), StatusCode::CREATED);

        // 2. Create Channel Adapter
        let adapter_req = CreateChannelAdapterRequest {
            tenant_id: "tenant-1".to_string(),
            inbox_id: "fake-inbox".to_string(),
            type_: "whatsapp".to_string(),
            credentials: r#"{"token": "xyz"}"#.to_string(),
        };
        let response = create_channel_adapter(State(state.clone()), Extension(claims.clone()), Json(adapter_req)).await.into_response();
        assert_eq!(response.status(), StatusCode::CREATED);

        // 3. Create Contact
        let contact_req = CreateContactRequest {
            tenant_id: "tenant-1".to_string(),
            name: "Maya".to_string(),
            identifier: "maya@example.com".to_string(),
        };
        let response = create_contact(State(state.clone()), Extension(claims.clone()), Json(contact_req)).await.into_response();
        assert_eq!(response.status(), StatusCode::CREATED);

        // 4. Create Conversation
        let conv_req = CreateConversationRequest {
            tenant_id: "tenant-1".to_string(),
            inbox_id: "fake-inbox".to_string(),
            contact_id: "fake-contact".to_string(),
            status: "open".to_string(),
        };
        let response = create_conversation(State(state.clone()), Extension(claims.clone()), Json(conv_req)).await.into_response();
        assert_eq!(response.status(), StatusCode::CREATED);

        // 5. Create Message
        let msg_req = CreateMessageRequest {
            tenant_id: "tenant-1".to_string(),
            conversation_id: "fake-conv".to_string(),
            content: "Hello!".to_string(),
            sender_type: "customer".to_string(),
        };
        let response = create_message(State(state.clone()), Extension(claims.clone()), Json(msg_req)).await.into_response();
        assert_eq!(response.status(), StatusCode::CREATED);

        // 6. List Conversations
        let response = list_conversations(State(state.clone()), Extension(claims.clone()), Path("tenant-1".to_string())).await.into_response();
        assert_eq!(response.status(), StatusCode::OK);

        // 7. List Messages
        let response = list_messages(State(state.clone()), Extension(claims.clone()), Path(("tenant-1".to_string(), "fake-conv".to_string()))).await.into_response();
        assert_eq!(response.status(), StatusCode::OK);

        // 8. Test Unauthorized Tenant Access
        let bad_claims = ::server_common::Claims {
            sub: "bad-user".to_string(),
            organization_id: Some("tenant-2".to_string()),
            role: "ADMIN".to_string(),
            exp: 9999999999,
        };
        let inbox_req_unauth = CreateInboxRequest {
            tenant_id: "tenant-1".to_string(),
            name: "Maya's Bakery".to_string(),
        };
        let response = create_inbox(State(state.clone()), Extension(bad_claims), Json(inbox_req_unauth)).await.into_response();
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }
}
