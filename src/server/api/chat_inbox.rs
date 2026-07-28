use axum::{
    extract::{Extension, Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use crate::db::DB;
use crate::services::chat::service::ChatService;

#[derive(Clone)]
pub struct ChatInboxState {
    pub db: Arc<DB>,
}

pub fn router(db: Arc<DB>, auth_store: Arc<::server_auth::Store>) -> Router {
    let state = ChatInboxState { db };
    Router::new()
        .route("/conversations", get(list_conversations))
        .route("/conversations/:conversation_id/messages", get(get_messages))
        .route("/conversations/:conversation_id/messages", post(send_message))
        .layer(axum::middleware::from_fn_with_state(
            auth_store,
            ::server_auth::strict_bearer_auth_middleware,
        ))
        .with_state(state)
}

fn chat_tenant(claims: &::server_common::Claims) -> Result<Uuid, StatusCode> {
    let tenant_id_str = claims
        .organization_id
        .as_deref()
        .map(str::trim)
        .filter(|id| !id.is_empty() && !id.eq_ignore_ascii_case("system"))
        .ok_or(StatusCode::UNAUTHORIZED)?;
    Uuid::parse_str(tenant_id_str).map_err(|_| StatusCode::BAD_REQUEST)
}

pub async fn list_conversations(
    State(state): State<ChatInboxState>,
    Extension(claims): Extension<::server_common::Claims>,
) -> impl IntoResponse {
    let tenant_id = match chat_tenant(&claims) {
        Ok(id) => id,
        Err(status) => return status.into_response(),
    };

    // We only support postgres for chat right now because ChatService uses PgPool
    let pool = match &state.db.store {
        crate::db::DbStore::Postgres => state.db.pool.clone(),
        _ => return (StatusCode::NOT_IMPLEMENTED, Json("Only Postgres is supported")).into_response(),
    };

    let chat_service = ChatService::new(pool);
    match chat_service.list_conversations(tenant_id).await {
        Ok(conversations) => (StatusCode::OK, Json(conversations)).into_response(),
        Err(e) => {
            tracing::error!("Failed to list conversations: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())).into_response()
        }
    }
}

pub async fn get_messages(
    State(state): State<ChatInboxState>,
    Extension(claims): Extension<::server_common::Claims>,
    Path(conversation_id): Path<String>,
) -> impl IntoResponse {
    let tenant_id = match chat_tenant(&claims) {
        Ok(id) => id,
        Err(status) => return status.into_response(),
    };

    let conversation_uuid = match Uuid::parse_str(&conversation_id) {
        Ok(id) => id,
        Err(_) => return StatusCode::BAD_REQUEST.into_response(),
    };

    let pool = match &state.db.store {
        crate::db::DbStore::Postgres => state.db.pool.clone(),
        _ => return (StatusCode::NOT_IMPLEMENTED, Json("Only Postgres is supported")).into_response(),
    };

    let chat_service = ChatService::new(pool);
    match chat_service.get_messages_for_conversation(tenant_id, conversation_uuid).await {
        Ok(messages) => (StatusCode::OK, Json(messages)).into_response(),
        Err(e) => {
            tracing::error!("Failed to get messages: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())).into_response()
        }
    }
}

#[derive(Deserialize)]
pub struct SendMessageRequest {
    pub content: String,
    pub sender_type: String, // 'contact', 'agent', 'bot'
}

pub async fn send_message(
    State(state): State<ChatInboxState>,
    Extension(claims): Extension<::server_common::Claims>,
    Path(conversation_id): Path<String>,
    Json(payload): Json<SendMessageRequest>,
) -> impl IntoResponse {
    let tenant_id = match chat_tenant(&claims) {
        Ok(id) => id,
        Err(status) => return status.into_response(),
    };

    let conversation_uuid = match Uuid::parse_str(&conversation_id) {
        Ok(id) => id,
        Err(_) => return StatusCode::BAD_REQUEST.into_response(),
    };

    let user_id_str = claims.sub.clone();
    let sender_id = Uuid::parse_str(&user_id_str).ok();

    let pool = match &state.db.store {
        crate::db::DbStore::Postgres => state.db.pool.clone(),
        _ => return (StatusCode::NOT_IMPLEMENTED, Json("Only Postgres is supported")).into_response(),
    };

    let chat_service = ChatService::new(pool);
    match chat_service.send_message(
        tenant_id,
        conversation_uuid,
        payload.sender_type,
        sender_id,
        payload.content
    ).await {
        Ok(message) => {
            // Trigger a real-time event via Redis Pub/Sub
            if let Some(mut redis_client) = crate::redis_pool::get_redis_client() {
                let channel = format!("unified:chat_inbox:{}", tenant_id);
                let envelope = serde_json::json!({
                    "channel": "chat_inbox",
                    "topic": format!("conversation:{}", conversation_id),
                    "data": {
                        "action": "new_message",
                        "message": message
                    },
                    "seq": 0,
                    "ts": chrono::Utc::now().timestamp_millis()
                });

                let _: Result<(), redis::RedisError> = redis::cmd("PUBLISH")
                    .arg(&channel)
                    .arg(envelope.to_string())
                    .query(&mut redis_client);
            }

            (StatusCode::CREATED, Json(message)).into_response()
        },
        Err(e) => {
            tracing::error!("Failed to send message: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())).into_response()
        }
    }
}
