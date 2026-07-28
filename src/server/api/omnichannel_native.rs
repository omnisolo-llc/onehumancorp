use axum::{
    extract::{Extension, Path, Query, State, WebSocketUpgrade, ws::{Message as WsMessage, WebSocket}},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use crate::db::DB;
use futures_util::{SinkExt, StreamExt};

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<DB>,
}

#[derive(Serialize, Deserialize)]
pub struct InboxResponse {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct ConversationResponse {
    pub id: String,
    pub inbox_id: String,
    pub contact_id: Option<String>,
    pub status: String,
    pub channel: String,
}

#[derive(Serialize, Deserialize)]
pub struct MessageResponse {
    pub id: String,
    pub conversation_id: String,
    pub content: String,
    pub message_type: String,
    pub sender_type: String,
    pub sender_id: Option<String>,
    pub created_at: String,
}

#[derive(Deserialize)]
pub struct CreateMessageRequest {
    pub content: String,
    pub message_type: String,
    pub sender_type: String,
    pub sender_id: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateInboxRequest {
    pub name: String,
}

#[derive(Deserialize)]
pub struct CreateConversationRequest {
    pub inbox_id: String,
    pub contact_id: Option<String>,
    pub channel: String,
}

#[derive(Deserialize)]
pub struct WsQuery {
    pub channel: Option<String>,
}

pub async fn list_inboxes(
    State(state): State<AppState>,
    Extension(claims): Extension<::server_common::Claims>,
) -> impl IntoResponse {
    let tenant_id = claims.organization_id.clone().unwrap_or_default();

    let query = "SELECT id, name FROM omnichannel_inboxes WHERE tenant_id = $1";
    let rows = match &state.db.store {
        crate::db::DbStore::Postgres => sqlx::query(query).bind(&tenant_id).fetch_all(&state.db.pool).await,
        crate::db::DbStore::Sqlite(pool) => {
            let sqlite_query = "SELECT id, name FROM omnichannel_inboxes WHERE tenant_id = ?";
            sqlx::query(sqlite_query).bind(&tenant_id).fetch_all(pool).await
        }
    };

    match rows {
        Ok(rows) => {
            let inboxes: Vec<InboxResponse> = rows.into_iter().map(|row| InboxResponse {
                id: sqlx::Row::get(&row, "id"),
                name: sqlx::Row::get(&row, "name"),
            }).collect();
            (StatusCode::OK, Json(inboxes)).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to fetch inboxes: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::<InboxResponse>::new())).into_response()
        }
    }
}

pub async fn list_conversations(
    State(state): State<AppState>,
    Extension(claims): Extension<::server_common::Claims>,
    Path(inbox_id): Path<String>,
) -> impl IntoResponse {
    let tenant_id = claims.organization_id.clone().unwrap_or_default();

    let query = "SELECT id, inbox_id, contact_id, status, channel FROM omnichannel_conversations WHERE tenant_id = $1 AND inbox_id = $2";
    let rows = match &state.db.store {
        crate::db::DbStore::Postgres => sqlx::query(query).bind(&tenant_id).bind(&inbox_id).fetch_all(&state.db.pool).await,
        crate::db::DbStore::Sqlite(pool) => {
            let sqlite_query = "SELECT id, inbox_id, contact_id, status, channel FROM omnichannel_conversations WHERE tenant_id = ? AND inbox_id = ?";
            sqlx::query(sqlite_query).bind(&tenant_id).bind(&inbox_id).fetch_all(pool).await
        }
    };

    match rows {
        Ok(rows) => {
            let convos: Vec<ConversationResponse> = rows.into_iter().map(|row| ConversationResponse {
                id: sqlx::Row::get(&row, "id"),
                inbox_id: sqlx::Row::get(&row, "inbox_id"),
                contact_id: sqlx::Row::try_get(&row, "contact_id").ok(),
                status: sqlx::Row::get(&row, "status"),
                channel: sqlx::Row::get(&row, "channel"),
            }).collect();
            (StatusCode::OK, Json(convos)).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to fetch conversations: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::<ConversationResponse>::new())).into_response()
        }
    }
}

pub async fn list_messages(
    State(state): State<AppState>,
    Extension(claims): Extension<::server_common::Claims>,
    Path(conversation_id): Path<String>,
) -> impl IntoResponse {
    let tenant_id = claims.organization_id.clone().unwrap_or_default();

    let query = "SELECT id, conversation_id, content, message_type, sender_type, sender_id, CAST(created_at AS text) as created_at FROM omnichannel_messages WHERE tenant_id = $1 AND conversation_id = $2 ORDER BY created_at ASC";
    let rows = match &state.db.store {
        crate::db::DbStore::Postgres => sqlx::query(query).bind(&tenant_id).bind(&conversation_id).fetch_all(&state.db.pool).await,
        crate::db::DbStore::Sqlite(pool) => {
            let sqlite_query = "SELECT id, conversation_id, content, message_type, sender_type, sender_id, CAST(created_at AS text) as created_at FROM omnichannel_messages WHERE tenant_id = ? AND conversation_id = ? ORDER BY created_at ASC";
            sqlx::query(sqlite_query).bind(&tenant_id).bind(&conversation_id).fetch_all(pool).await
        }
    };

    match rows {
        Ok(rows) => {
            let msgs: Vec<MessageResponse> = rows.into_iter().map(|row| MessageResponse {
                id: sqlx::Row::get(&row, "id"),
                conversation_id: sqlx::Row::get(&row, "conversation_id"),
                content: sqlx::Row::get(&row, "content"),
                message_type: sqlx::Row::get(&row, "message_type"),
                sender_type: sqlx::Row::get(&row, "sender_type"),
                sender_id: sqlx::Row::try_get(&row, "sender_id").ok(),
                created_at: sqlx::Row::try_get(&row, "created_at").unwrap_or_default(),
            }).collect();
            (StatusCode::OK, Json(msgs)).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to fetch messages: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::<MessageResponse>::new())).into_response()
        }
    }
}

pub async fn create_message(
    State(state): State<AppState>,
    Extension(claims): Extension<::server_common::Claims>,
    Path(conversation_id): Path<String>,
    Json(payload): Json<CreateMessageRequest>,
) -> impl IntoResponse {
    let tenant_id = claims.organization_id.clone().unwrap_or_default();
    let id = Uuid::new_v4().to_string();

    let query = "INSERT INTO omnichannel_messages (id, tenant_id, conversation_id, content, message_type, sender_type, sender_id) VALUES ($1, $2, $3, $4, $5, $6, $7)";
    let res = match &state.db.store {
        crate::db::DbStore::Postgres => {
            sqlx::query(query)
                .bind(&id)
                .bind(&tenant_id)
                .bind(&conversation_id)
                .bind(&payload.content)
                .bind(&payload.message_type)
                .bind(&payload.sender_type)
                .bind(&payload.sender_id)
                .execute(&state.db.pool)
                .await
        },
        crate::db::DbStore::Sqlite(pool) => {
            let sqlite_query = "INSERT INTO omnichannel_messages (id, tenant_id, conversation_id, content, message_type, sender_type, sender_id) VALUES (?, ?, ?, ?, ?, ?, ?)";
            sqlx::query(sqlite_query)
                .bind(&id)
                .bind(&tenant_id)
                .bind(&conversation_id)
                .bind(&payload.content)
                .bind(&payload.message_type)
                .bind(&payload.sender_type)
                .bind(&payload.sender_id)
                .execute(pool)
                .await
        }
    };

    match res {
        Ok(_) => {
            if let Some(client) = crate::get_redis_client() {
                if let Ok(mut conn) = client.get_multiplexed_async_connection().await {
                    let pub_topic = format!("omni_inbox:{}:{}", tenant_id, conversation_id);
                    let msg_json = serde_json::json!({
                        "event": "new_message",
                        "data": {
                            "id": id,
                            "conversation_id": conversation_id,
                            "content": payload.content,
                            "message_type": payload.message_type,
                            "sender_type": payload.sender_type,
                            "sender_id": payload.sender_id,
                        }
                    });
                    use redis::AsyncCommands;
                    let _: Result<(), _> = conn.publish(pub_topic, msg_json.to_string()).await;
                }
            }

            (StatusCode::OK, Json(serde_json::json!({"id": id}))).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to create message: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Failed to create message"}))).into_response()
        }
    }
}

pub async fn omni_ws_handler(
    ws: WebSocketUpgrade,
    Extension(claims): Extension<::server_common::Claims>,
    Query(_query): Query<WsQuery>,
) -> impl IntoResponse {
    let tenant_id = claims.organization_id.clone().unwrap_or_default();
    ws.on_upgrade(move |socket| handle_omni_socket(socket, tenant_id))
}

async fn handle_omni_socket(mut socket: WebSocket, tenant_id: String) {
    let redis_client_opt = crate::get_redis_client();
    let mut pubsub_conn_opt = None;

    if let Some(client) = &redis_client_opt {
        if let Ok(conn) = client.get_async_pubsub().await {
            pubsub_conn_opt = Some(conn);
        }
    }

    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

    let mut redis_task = None;
    if let Some(mut pubsub) = pubsub_conn_opt {
        let pattern = format!("omni_inbox:{}*", tenant_id);
        let _ = pubsub.psubscribe(&pattern).await;

        let mut stream = pubsub.into_on_message();
        redis_task = Some(tokio::spawn(async move {
            while let Some(msg) = stream.next().await {
                if let Ok(payload) = msg.get_payload::<String>() {
                    let _ = tx.send(payload);
                }
            }
        }));
    }

    let send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if socket.send(WsMessage::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    let _ = send_task.await;
    if let Some(task) = redis_task {
        task.abort();
    }
}

pub fn router(db: Arc<DB>) -> Router {
    let state = AppState { db };
    Router::new()
        .route("/api/v1/omnichannel/inboxes", get(list_inboxes))
        .route("/api/v1/omnichannel/inboxes/:inbox_id/conversations", get(list_conversations))
        .route("/api/v1/omnichannel/conversations/:conversation_id/messages", get(list_messages).post(create_message))
        .route("/api/v1/omnichannel/ws", get(omni_ws_handler))
        .with_state(state)
}
