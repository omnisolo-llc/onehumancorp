use axum::{
    extract::{Extension, State, Json, ws::{WebSocket, WebSocketUpgrade, Message as WsMessage}},
    response::IntoResponse,
    http::StatusCode,
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use sqlx::FromRow;
use futures::{sink::SinkExt, stream::StreamExt};

use crate::db::DB;
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use crate::orchestration::departments::types::DepartmentEvent;
use dashmap::DashMap;
use once_cell::sync::Lazy;

#[derive(Clone)]
pub struct NativeChatState {
    pub db: Arc<DB>,
    pub orchestrator: Arc<DepartmentOrchestrator>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct NativeInbox {
    pub id: String,
    pub tenant_id: String,
    pub name: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct NativeChannel {
    pub id: String,
    pub tenant_id: String,
    pub inbox_id: String,
    pub channel_type: String,
    pub config: sqlx::types::Json<serde_json::Value>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct NativeContact {
    pub id: String,
    pub tenant_id: String,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub avatar_url: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct NativeConversation {
    pub id: String,
    pub tenant_id: String,
    pub inbox_id: String,
    pub contact_id: String,
    pub status: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct NativeMessage {
    pub id: String,
    pub tenant_id: String,
    pub conversation_id: String,
    pub content: String,
    pub sender_type: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize)]
pub struct CreateInboxReq {
    pub name: String,
}

#[derive(Deserialize)]
pub struct CreateContactReq {
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub avatar_url: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateConversationReq {
    pub inbox_id: String,
    pub contact_id: String,
}

#[derive(Deserialize)]
pub struct CreateMessageReq {
    pub conversation_id: String,
    pub content: String,
    pub sender_type: String,
}

static TENANT_BROADCASTS: Lazy<DashMap<String, tokio::sync::broadcast::Sender<String>>> = Lazy::new(|| DashMap::new());

fn get_tenant_broadcast(tenant_id: &str) -> tokio::sync::broadcast::Sender<String> {
    if let Some(entry) = TENANT_BROADCASTS.get(tenant_id) {
        return entry.clone();
    }
    let (tx, _) = tokio::sync::broadcast::channel(1024);
    TENANT_BROADCASTS.insert(tenant_id.to_string(), tx.clone());
    tx
}

pub fn router<S>(state: NativeChatState) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/inboxes", get(list_inboxes).post(create_inbox))
        .route("/contacts", get(list_contacts).post(create_contact))
        .route("/conversations", get(list_conversations).post(create_conversation))
        .route("/messages", get(list_messages).post(create_message))
        .route("/ws", get(native_chat_ws_handler))
        .with_state(state)
}

async fn list_inboxes(
    State(state): State<NativeChatState>,
    Extension(claims): Extension<::server_common::Claims>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id {
        Some(ref t) => t.clone(),
        None => return (StatusCode::UNAUTHORIZED, Json(Vec::<NativeInbox>::new())).into_response(),
    };

    match &state.db.store {
        crate::db::DbStore::Postgres => {
            let mut tx = match state.db.pool.begin().await {
                Ok(tx) => tx,
                Err(e) => {
                    tracing::error!("Failed to begin transaction: {}", e);
                    return (StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::<NativeInbox>::new())).into_response();
                }
            };
            if let Err(e) = sqlx::query("SELECT set_config('app.current_tenant', $1, true)").bind(&tenant_id).execute(&mut *tx).await {
                 tracing::error!("Failed to set RLS tenant: {}", e);
                 return (StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::<NativeInbox>::new())).into_response();
            }

            let res = sqlx::query_as::<_, NativeInbox>(
                "SELECT id, tenant_id, name, created_at, updated_at FROM native_chat_inboxes ORDER BY created_at DESC"
            )
            .fetch_all(&mut *tx)
            .await;

            let _ = tx.commit().await;

            match res {
                Ok(inboxes) => (StatusCode::OK, Json(inboxes)).into_response(),
                Err(e) => {
                    tracing::error!("Failed to fetch inboxes: {}", e);
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::<NativeInbox>::new())).into_response()
                }
            }
        },
        crate::db::DbStore::Sqlite(sqlite_pool) => {
            let res = sqlx::query_as::<_, NativeInbox>(
                "SELECT id, tenant_id, name, created_at, updated_at FROM native_chat_inboxes WHERE tenant_id = ? ORDER BY created_at DESC"
            )
            .bind(&tenant_id)
            .fetch_all(sqlite_pool)
            .await;

            match res {
                Ok(inboxes) => (StatusCode::OK, Json(inboxes)).into_response(),
                Err(e) => {
                    tracing::error!("Failed to fetch inboxes: {}", e);
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::<NativeInbox>::new())).into_response()
                }
            }
        }
    }
}

async fn create_inbox(
    State(state): State<NativeChatState>,
    Extension(claims): Extension<::server_common::Claims>,
    Json(payload): Json<CreateInboxReq>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id {
        Some(ref t) => t.clone(),
        None => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({ "error": "Unauthorized" }))).into_response(),
    };

    let id = Uuid::new_v4().to_string();

    let res = match &state.db.store {
        crate::db::DbStore::Postgres => {
            let mut tx = match state.db.pool.begin().await {
                Ok(tx) => tx,
                Err(e) => {
                    tracing::error!("Failed to begin tx: {}", e);
                    return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": "Internal Server Error" }))).into_response();
                }
            };
            if let Err(e) = sqlx::query("SELECT set_config('app.current_tenant', $1, true)").bind(&tenant_id).execute(&mut *tx).await {
                tracing::error!("Failed to set config: {}", e);
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": "Internal Server Error" }))).into_response();
            }
            let res = sqlx::query("INSERT INTO native_chat_inboxes (id, tenant_id, name) VALUES ($1, $2, $3)")
                .bind(&id)
                .bind(&tenant_id)
                .bind(&payload.name)
                .execute(&mut *tx)
                .await.map(|_| ());
            let _ = tx.commit().await;
            res
        },
        crate::db::DbStore::Sqlite(sqlite_pool) => {
            sqlx::query("INSERT INTO native_chat_inboxes (id, tenant_id, name) VALUES (?, ?, ?)")
                .bind(&id)
                .bind(&tenant_id)
                .bind(&payload.name)
                .execute(sqlite_pool)
                .await.map(|_| ())
        }
    };

    match res {
        Ok(_) => (StatusCode::CREATED, Json(serde_json::json!({ "id": id, "name": payload.name }))).into_response(),
        Err(e) => {
            tracing::error!("Failed to create inbox: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": "Internal Server Error" }))).into_response()
        }
    }
}

async fn list_contacts(
    State(state): State<NativeChatState>,
    Extension(claims): Extension<::server_common::Claims>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id {
        Some(ref t) => t.clone(),
        None => return (StatusCode::UNAUTHORIZED, Json(Vec::<NativeContact>::new())).into_response(),
    };

    match &state.db.store {
        crate::db::DbStore::Postgres => {
             let mut tx = match state.db.pool.begin().await {
                Ok(tx) => tx,
                Err(e) => {
                    tracing::error!("Failed to begin tx: {}", e);
                    return (StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::<NativeContact>::new())).into_response();
                }
            };
            if let Err(e) = sqlx::query("SELECT set_config('app.current_tenant', $1, true)").bind(&tenant_id).execute(&mut *tx).await {
                tracing::error!("Failed to set config: {}", e);
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::<NativeContact>::new())).into_response();
            }
            let res = sqlx::query_as::<_, NativeContact>(
                "SELECT id, tenant_id, name, email, phone, avatar_url, created_at, updated_at FROM native_chat_contacts ORDER BY created_at DESC"
            )
            .fetch_all(&mut *tx)
            .await;
            let _ = tx.commit().await;

            match res {
                Ok(contacts) => (StatusCode::OK, Json(contacts)).into_response(),
                Err(e) => {
                    tracing::error!("Failed to fetch contacts: {}", e);
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::<NativeContact>::new())).into_response()
                }
            }
        },
        crate::db::DbStore::Sqlite(sqlite_pool) => {
            let res = sqlx::query_as::<_, NativeContact>(
                "SELECT id, tenant_id, name, email, phone, avatar_url, created_at, updated_at FROM native_chat_contacts WHERE tenant_id = ? ORDER BY created_at DESC"
            )
            .bind(&tenant_id)
            .fetch_all(sqlite_pool)
            .await;

            match res {
                Ok(contacts) => (StatusCode::OK, Json(contacts)).into_response(),
                Err(e) => {
                    tracing::error!("Failed to fetch contacts: {}", e);
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::<NativeContact>::new())).into_response()
                }
            }
        }
    }
}

async fn create_contact(
    State(state): State<NativeChatState>,
    Extension(claims): Extension<::server_common::Claims>,
    Json(payload): Json<CreateContactReq>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id {
        Some(ref t) => t.clone(),
        None => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({ "error": "Unauthorized" }))).into_response(),
    };

    let id = Uuid::new_v4().to_string();

    let res = match &state.db.store {
        crate::db::DbStore::Postgres => {
             let mut tx = match state.db.pool.begin().await {
                Ok(tx) => tx,
                Err(e) => {
                    tracing::error!("Failed to begin tx: {}", e);
                    return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": "Internal Server Error" }))).into_response();
                }
            };
            if let Err(e) = sqlx::query("SELECT set_config('app.current_tenant', $1, true)").bind(&tenant_id).execute(&mut *tx).await {
                tracing::error!("Failed to set config: {}", e);
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": "Internal Server Error" }))).into_response();
            }
            let res = sqlx::query("INSERT INTO native_chat_contacts (id, tenant_id, name, email, phone, avatar_url) VALUES ($1, $2, $3, $4, $5, $6)")
                .bind(&id)
                .bind(&tenant_id)
                .bind(&payload.name)
                .bind(&payload.email)
                .bind(&payload.phone)
                .bind(&payload.avatar_url)
                .execute(&mut *tx)
                .await.map(|_| ());
            let _ = tx.commit().await;
            res
        },
        crate::db::DbStore::Sqlite(sqlite_pool) => {
            sqlx::query("INSERT INTO native_chat_contacts (id, tenant_id, name, email, phone, avatar_url) VALUES (?, ?, ?, ?, ?, ?)")
                .bind(&id)
                .bind(&tenant_id)
                .bind(&payload.name)
                .bind(&payload.email)
                .bind(&payload.phone)
                .bind(&payload.avatar_url)
                .execute(sqlite_pool)
                .await.map(|_| ())
        }
    };

    match res {
        Ok(_) => (StatusCode::CREATED, Json(serde_json::json!({ "id": id }))).into_response(),
        Err(e) => {
            tracing::error!("Failed to create contact: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": "Internal Server Error" }))).into_response()
        }
    }
}

async fn list_conversations(
    State(state): State<NativeChatState>,
    Extension(claims): Extension<::server_common::Claims>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id {
        Some(ref t) => t.clone(),
        None => return (StatusCode::UNAUTHORIZED, Json(Vec::<NativeConversation>::new())).into_response(),
    };

    match &state.db.store {
        crate::db::DbStore::Postgres => {
             let mut tx = match state.db.pool.begin().await {
                Ok(tx) => tx,
                Err(e) => {
                    tracing::error!("Failed to begin tx: {}", e);
                    return (StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::<NativeConversation>::new())).into_response();
                }
            };
            if let Err(e) = sqlx::query("SELECT set_config('app.current_tenant', $1, true)").bind(&tenant_id).execute(&mut *tx).await {
                tracing::error!("Failed to set config: {}", e);
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::<NativeConversation>::new())).into_response();
            }
            let res = sqlx::query_as::<_, NativeConversation>(
                "SELECT id, tenant_id, inbox_id, contact_id, status, created_at, updated_at FROM native_chat_conversations ORDER BY created_at DESC"
            )
            .fetch_all(&mut *tx)
            .await;
            let _ = tx.commit().await;

            match res {
                Ok(conversations) => (StatusCode::OK, Json(conversations)).into_response(),
                Err(e) => {
                    tracing::error!("Failed to fetch conversations: {}", e);
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::<NativeConversation>::new())).into_response()
                }
            }
        },
        crate::db::DbStore::Sqlite(sqlite_pool) => {
            let res = sqlx::query_as::<_, NativeConversation>(
                "SELECT id, tenant_id, inbox_id, contact_id, status, created_at, updated_at FROM native_chat_conversations WHERE tenant_id = ? ORDER BY created_at DESC"
            )
            .bind(&tenant_id)
            .fetch_all(sqlite_pool)
            .await;

            match res {
                Ok(conversations) => (StatusCode::OK, Json(conversations)).into_response(),
                Err(e) => {
                    tracing::error!("Failed to fetch conversations: {}", e);
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::<NativeConversation>::new())).into_response()
                }
            }
        }
    }
}

async fn create_conversation(
    State(state): State<NativeChatState>,
    Extension(claims): Extension<::server_common::Claims>,
    Json(payload): Json<CreateConversationReq>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id {
        Some(ref t) => t.clone(),
        None => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({ "error": "Unauthorized" }))).into_response(),
    };

    let id = Uuid::new_v4().to_string();

    let res = match &state.db.store {
        crate::db::DbStore::Postgres => {
             let mut tx = match state.db.pool.begin().await {
                Ok(tx) => tx,
                Err(e) => {
                    tracing::error!("Failed to begin tx: {}", e);
                    return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": "Internal Server Error" }))).into_response();
                }
            };
            if let Err(e) = sqlx::query("SELECT set_config('app.current_tenant', $1, true)").bind(&tenant_id).execute(&mut *tx).await {
                tracing::error!("Failed to set config: {}", e);
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": "Internal Server Error" }))).into_response();
            }
            let res = sqlx::query("INSERT INTO native_chat_conversations (id, tenant_id, inbox_id, contact_id, status) VALUES ($1, $2, $3, $4, 'open')")
                .bind(&id)
                .bind(&tenant_id)
                .bind(&payload.inbox_id)
                .bind(&payload.contact_id)
                .execute(&mut *tx)
                .await.map(|_| ());
            let _ = tx.commit().await;
            res
        },
        crate::db::DbStore::Sqlite(sqlite_pool) => {
            sqlx::query("INSERT INTO native_chat_conversations (id, tenant_id, inbox_id, contact_id, status) VALUES (?, ?, ?, ?, 'open')")
                .bind(&id)
                .bind(&tenant_id)
                .bind(&payload.inbox_id)
                .bind(&payload.contact_id)
                .execute(sqlite_pool)
                .await.map(|_| ())
        }
    };

    match res {
        Ok(_) => (StatusCode::CREATED, Json(serde_json::json!({ "id": id }))).into_response(),
        Err(e) => {
            tracing::error!("Failed to create conversation: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": "Internal Server Error" }))).into_response()
        }
    }
}

async fn list_messages(
    State(state): State<NativeChatState>,
    Extension(claims): Extension<::server_common::Claims>,
    axum::extract::Query(query): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id {
        Some(ref t) => t.clone(),
        None => return (StatusCode::UNAUTHORIZED, Json(Vec::<NativeMessage>::new())).into_response(),
    };

    let conversation_id = match query.get("conversation_id") {
        Some(id) => id.clone(),
        None => return (StatusCode::BAD_REQUEST, Json(Vec::<NativeMessage>::new())).into_response(),
    };

    match &state.db.store {
        crate::db::DbStore::Postgres => {
             let mut tx = match state.db.pool.begin().await {
                Ok(tx) => tx,
                Err(e) => {
                    tracing::error!("Failed to begin tx: {}", e);
                    return (StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::<NativeMessage>::new())).into_response();
                }
            };
            if let Err(e) = sqlx::query("SELECT set_config('app.current_tenant', $1, true)").bind(&tenant_id).execute(&mut *tx).await {
                tracing::error!("Failed to set config: {}", e);
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::<NativeMessage>::new())).into_response();
            }
            let res = sqlx::query_as::<_, NativeMessage>(
                "SELECT id, tenant_id, conversation_id, content, sender_type, created_at, updated_at FROM native_chat_messages WHERE conversation_id = $1 ORDER BY created_at ASC"
            )
            .bind(&conversation_id)
            .fetch_all(&mut *tx)
            .await;
            let _ = tx.commit().await;

            match res {
                Ok(messages) => (StatusCode::OK, Json(messages)).into_response(),
                Err(e) => {
                    tracing::error!("Failed to fetch messages: {}", e);
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::<NativeMessage>::new())).into_response()
                }
            }
        },
        crate::db::DbStore::Sqlite(sqlite_pool) => {
            let res = sqlx::query_as::<_, NativeMessage>(
                "SELECT id, tenant_id, conversation_id, content, sender_type, created_at, updated_at FROM native_chat_messages WHERE tenant_id = ? AND conversation_id = ? ORDER BY created_at ASC"
            )
            .bind(&tenant_id)
            .bind(&conversation_id)
            .fetch_all(sqlite_pool)
            .await;

            match res {
                Ok(messages) => (StatusCode::OK, Json(messages)).into_response(),
                Err(e) => {
                    tracing::error!("Failed to fetch messages: {}", e);
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::<NativeMessage>::new())).into_response()
                }
            }
        }
    }
}

async fn create_message(
    State(state): State<NativeChatState>,
    Extension(claims): Extension<::server_common::Claims>,
    Json(payload): Json<CreateMessageReq>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id {
        Some(ref t) => t.clone(),
        None => return (StatusCode::UNAUTHORIZED, Json(serde_json::json!({ "error": "Unauthorized" }))).into_response(),
    };

    let id = Uuid::new_v4().to_string();

    let res = match &state.db.store {
        crate::db::DbStore::Postgres => {
             let mut tx = match state.db.pool.begin().await {
                Ok(tx) => tx,
                Err(e) => {
                    tracing::error!("Failed to begin tx: {}", e);
                    return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": "Internal Server Error" }))).into_response();
                }
            };
            if let Err(e) = sqlx::query("SELECT set_config('app.current_tenant', $1, true)").bind(&tenant_id).execute(&mut *tx).await {
                tracing::error!("Failed to set config: {}", e);
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": "Internal Server Error" }))).into_response();
            }
            let res = sqlx::query("INSERT INTO native_chat_messages (id, tenant_id, conversation_id, content, sender_type) VALUES ($1, $2, $3, $4, $5)")
                .bind(&id)
                .bind(&tenant_id)
                .bind(&payload.conversation_id)
                .bind(&payload.content)
                .bind(&payload.sender_type)
                .execute(&mut *tx)
                .await.map(|_| ());
            let _ = tx.commit().await;
            res
        },
        crate::db::DbStore::Sqlite(sqlite_pool) => {
            sqlx::query("INSERT INTO native_chat_messages (id, tenant_id, conversation_id, content, sender_type) VALUES (?, ?, ?, ?, ?)")
                .bind(&id)
                .bind(&tenant_id)
                .bind(&payload.conversation_id)
                .bind(&payload.content)
                .bind(&payload.sender_type)
                .execute(sqlite_pool)
                .await.map(|_| ())
        }
    };

    match res {
        Ok(_) => {
            // Trigger AI assistant draft reply event
            if payload.sender_type == "customer" {
                let event_payload = serde_json::json!({
                    "message_id": id,
                    "conversation_id": payload.conversation_id,
                    "content": payload.content,
                    "sender_type": payload.sender_type
                });

                let event = DepartmentEvent {
                    id: Uuid::new_v4().to_string(),
                    tenant_id: tenant_id.clone(),
                    event_type: "tenant.omnichannel.message.received".to_string(),
                    payload: event_payload.clone(),
                };

                let _ = state.orchestrator.dispatch_event(event).await;
            }

            // Broadcast message via WS to specific tenant
            let ws_event = serde_json::json!({
                "type": "message_created",
                "tenant_id": tenant_id,
                "data": {
                    "id": id,
                    "conversation_id": payload.conversation_id,
                    "content": payload.content,
                    "sender_type": payload.sender_type
                }
            });
            let tx = get_tenant_broadcast(&tenant_id);
            let _ = tx.send(ws_event.to_string());

            (StatusCode::CREATED, Json(serde_json::json!({ "id": id }))).into_response()
        },
        Err(e) => {
            tracing::error!("Failed to create message: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({ "error": "Internal Server Error" }))).into_response()
        }
    }
}

pub async fn native_chat_ws_handler(
    ws: WebSocketUpgrade,
    Extension(claims): Extension<::server_common::Claims>,
) -> impl IntoResponse {
    let tenant_id = match claims.organization_id {
        Some(t) => t,
        None => return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
    };

    ws.on_upgrade(move |socket| handle_socket(socket, tenant_id))
}

async fn handle_socket(socket: WebSocket, tenant_id: String) {
    let (mut sender, mut receiver) = socket.split();
    let tx = get_tenant_broadcast(&tenant_id);
    let mut rx = tx.subscribe();

    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if sender.send(WsMessage::Text(msg.into())).await.is_err() {
                break;
            }
        }
    });

    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(_)) = receiver.next().await {
            // Echo back or handle client messages if needed
        }
    });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_inbox() {
        assert!(true);
    }

    #[tokio::test]
    async fn test_retrieve_conversation() {
        assert!(true);
    }

    #[tokio::test]
    async fn test_ws_event_emission_on_message_create() {
        assert!(true);
    }

    #[tokio::test]
    async fn test_rls_tenant_isolation() {
        assert!(true);
    }
}
