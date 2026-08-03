use axum::{
    extract::Extension,
    extract::{State, Json},
    response::IntoResponse,
    http::StatusCode,
    routing::post,
    Router,
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use crate::orchestration::departments::types::DepartmentEvent;
use crate::db::DB;
use crate::services::chat::service::ChatService;
use super::identity::resolve_identity;

#[derive(Clone)]
pub struct OmnichannelWebhookState {
    pub db: Arc<DB>,
    pub orchestrator: Arc<DepartmentOrchestrator>,
}

#[derive(Deserialize)]
pub struct OmnichannelPayload {
    pub tenant_id: String,
    #[serde(alias = "channel")]
    pub source: String,
    pub sender_id: String,
    pub message: String,
    #[serde(default)]
    pub target_language: Option<String>,
}

#[derive(Serialize)]
pub struct WebhookResponse {
    pub success: bool,
    pub message_id: Option<String>,
}

#[derive(Serialize)]
pub struct ConversationResponse {
    pub id: String,
    pub tenant_id: String,
    pub customer_id: Option<String>,
    pub channel: String,
    pub status: String,
    pub created_at: String,
}

#[derive(Serialize)]
pub struct MessageResponse {
    pub id: String,
    pub tenant_id: String,
    pub source: String,
    pub original_content: String,
    pub translated_content: String,
    pub draft_reply: String,
    pub status: String,
    pub sender_id: String,
    pub created_at: String,
}

pub fn router<S>(state: OmnichannelWebhookState) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/webhook", post(handle_omnichannel_webhook))
        .route("/conversations/{tenant_id}", axum::routing::get(get_conversations))
        .route("/messages/{tenant_id}/{conversation_id}", axum::routing::get(get_messages))
        .with_state(state)
}

pub async fn get_conversations(
    State(state): State<OmnichannelWebhookState>,
    Extension(claims): Extension<::server_common::Claims>,
    axum::extract::Path(tenant_id): axum::extract::Path<String>,
) -> impl IntoResponse {
    if claims.organization_id.as_deref() != Some(&tenant_id) {
        return (StatusCode::UNAUTHORIZED, Json(Vec::<ConversationResponse>::new())).into_response();
    }
    match &state.db.store {
        crate::db::DbStore::Postgres => {
            let query = "SELECT c.id::text, c.tenant_id::text, c.contact_id::text as customer_id, i.name as channel, c.status, CAST(c.created_at AS text) as created_at FROM chat_conversations c JOIN chat_inboxes i ON c.inbox_id = i.id WHERE c.tenant_id = $1 ORDER BY c.created_at DESC";
            match sqlx::query(query).bind(Uuid::parse_str(&tenant_id).unwrap_or_default()).fetch_all(&state.db.pool).await {
                Ok(rows) => {
                    let conversations: Vec<ConversationResponse> = rows.into_iter().map(|row| ConversationResponse {
                        id: sqlx::Row::get(&row, "id"),
                        tenant_id: sqlx::Row::get(&row, "tenant_id"),
                        customer_id: sqlx::Row::try_get(&row, "customer_id").ok(),
                        channel: sqlx::Row::get(&row, "channel"),
                        status: sqlx::Row::get(&row, "status"),
                        created_at: sqlx::Row::try_get(&row, "created_at").unwrap_or_default(),
                    }).collect();
                    (StatusCode::OK, Json(conversations)).into_response()
                },
                Err(e) => {
                    tracing::error!("Failed to fetch conversations: {}", e);
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::<ConversationResponse>::new())).into_response()
                }
            }
        },
        crate::db::DbStore::Sqlite(sqlite_pool) => {
            let query = "SELECT c.id, c.tenant_id, c.contact_id as customer_id, i.name as channel, c.status, CAST(c.created_at AS text) as created_at FROM chat_conversations c JOIN chat_inboxes i ON c.inbox_id = i.id WHERE c.tenant_id = ? ORDER BY c.created_at DESC";
            match sqlx::query(query).bind(&tenant_id).fetch_all(sqlite_pool).await {
                Ok(rows) => {
                    let conversations: Vec<ConversationResponse> = rows.into_iter().map(|row| ConversationResponse {
                        id: sqlx::Row::get(&row, "id"),
                        tenant_id: sqlx::Row::get(&row, "tenant_id"),
                        customer_id: sqlx::Row::try_get(&row, "customer_id").ok(),
                        channel: sqlx::Row::get(&row, "channel"),
                        status: sqlx::Row::get(&row, "status"),
                        created_at: sqlx::Row::try_get(&row, "created_at").unwrap_or_default(),
                    }).collect();
                    (StatusCode::OK, Json(conversations)).into_response()
                },
                Err(e) => {
                    tracing::error!("Failed to fetch conversations: {}", e);
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::<ConversationResponse>::new())).into_response()
                }
            }
        }
    }
}

pub async fn get_messages(
    State(state): State<OmnichannelWebhookState>,
    Extension(claims): Extension<::server_common::Claims>,
    axum::extract::Path((tenant_id, conversation_id)): axum::extract::Path<(String, String)>,
) -> impl IntoResponse {
    if claims.organization_id.as_deref() != Some(&tenant_id) {
        return (StatusCode::UNAUTHORIZED, Json(Vec::<MessageResponse>::new())).into_response();
    }
    match &state.db.store {
        crate::db::DbStore::Postgres => {
            let query = "SELECT m.id::text, m.tenant_id::text, i.name as source, m.content as original_content, m.content as translated_content, '' as draft_reply, 'unread' as status, COALESCE(m.sender_id::text, '') as sender_id, CAST(m.created_at AS text) as created_at FROM chat_messages m JOIN chat_conversations c ON m.conversation_id = c.id JOIN chat_inboxes i ON c.inbox_id = i.id WHERE m.tenant_id = $1 AND m.conversation_id = $2 ORDER BY m.created_at ASC";
            match sqlx::query(query).bind(Uuid::parse_str(&tenant_id).unwrap_or_default()).bind(Uuid::parse_str(&conversation_id).unwrap_or_default()).fetch_all(&state.db.pool).await {
                Ok(rows) => {
                    let messages: Vec<MessageResponse> = rows.into_iter().map(|row| MessageResponse {
                        id: sqlx::Row::get(&row, "id"),
                        tenant_id: sqlx::Row::get(&row, "tenant_id"),
                        source: sqlx::Row::get(&row, "source"),
                        original_content: sqlx::Row::get(&row, "original_content"),
                        translated_content: sqlx::Row::get(&row, "translated_content"),
                        draft_reply: sqlx::Row::try_get(&row, "draft_reply").unwrap_or_default(),
                        status: sqlx::Row::get(&row, "status"),
                        sender_id: sqlx::Row::try_get(&row, "sender_id").unwrap_or_default(),
                        created_at: sqlx::Row::try_get(&row, "created_at").unwrap_or_default(),
                    }).collect();
                    (StatusCode::OK, Json(messages)).into_response()
                },
                Err(e) => {
                    tracing::error!("Failed to fetch messages: {}", e);
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::<MessageResponse>::new())).into_response()
                }
            }
        },
        crate::db::DbStore::Sqlite(sqlite_pool) => {
            let query = "SELECT m.id, m.tenant_id, i.name as source, m.content as original_content, m.content as translated_content, '' as draft_reply, 'unread' as status, COALESCE(m.sender_id, '') as sender_id, CAST(m.created_at AS text) as created_at FROM chat_messages m JOIN chat_conversations c ON m.conversation_id = c.id JOIN chat_inboxes i ON c.inbox_id = i.id WHERE m.tenant_id = ? AND m.conversation_id = ? ORDER BY m.created_at ASC";
            match sqlx::query(query).bind(&tenant_id).bind(&conversation_id).fetch_all(sqlite_pool).await {
                Ok(rows) => {
                    let messages: Vec<MessageResponse> = rows.into_iter().map(|row| MessageResponse {
                        id: sqlx::Row::get(&row, "id"),
                        tenant_id: sqlx::Row::get(&row, "tenant_id"),
                        source: sqlx::Row::get(&row, "source"),
                        original_content: sqlx::Row::get(&row, "original_content"),
                        translated_content: sqlx::Row::get(&row, "translated_content"),
                        draft_reply: sqlx::Row::try_get(&row, "draft_reply").unwrap_or_default(),
                        status: sqlx::Row::get(&row, "status"),
                        sender_id: sqlx::Row::try_get(&row, "sender_id").unwrap_or_default(),
                        created_at: sqlx::Row::try_get(&row, "created_at").unwrap_or_default(),
                    }).collect();
                    (StatusCode::OK, Json(messages)).into_response()
                },
                Err(e) => {
                    tracing::error!("Failed to fetch messages: {}", e);
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(Vec::<MessageResponse>::new())).into_response()
                }
            }
        }
    }
}

pub async fn handle_omnichannel_webhook(
    State(state): State<OmnichannelWebhookState>,
    Extension(claims): Extension<::server_common::Claims>,
    Json(payload): Json<OmnichannelPayload>,
) -> impl IntoResponse {
    if claims.organization_id.as_deref() != Some(payload.tenant_id.as_str()) {
        return (
            StatusCode::FORBIDDEN,
            Json(WebhookResponse { success: false, message_id: None }),
        )
            .into_response();
    }
    let pool = state.db.pool.clone();
    let chat_service = ChatService::new(pool.clone());
    let tenant_uuid = Uuid::parse_str(&payload.tenant_id).unwrap_or_default();

    // In a real app we'd resolve or create inbox based on source
    let inbox_name = payload.source.clone();
    let inbox = match sqlx::query_as::<_, crate::services::chat::models::ChatInbox>(
        "SELECT id, tenant_id, name, created_at, updated_at FROM chat_inboxes WHERE tenant_id = $1 AND name = $2"
    )
    .bind(tenant_uuid)
    .bind(&inbox_name)
    .fetch_optional(&pool).await {
        Ok(Some(i)) => i,
        _ => chat_service.create_inbox(tenant_uuid, inbox_name).await.unwrap()
    };

    // Create or fetch contact
    let contact = match sqlx::query_as::<_, crate::services::chat::models::ChatContact>(
        "SELECT id, tenant_id, name, email, phone, created_at, updated_at FROM chat_contacts WHERE tenant_id = $1 AND phone = $2"
    )
    .bind(tenant_uuid)
    .bind(&payload.sender_id)
    .fetch_optional(&pool).await {
        Ok(Some(c)) => c,
        _ => chat_service.create_contact(tenant_uuid, None, None, Some(payload.sender_id.clone())).await.unwrap()
    };

    // Create or fetch conversation
    let conversation = match sqlx::query_as::<_, crate::services::chat::models::ChatConversation>(
        "SELECT id, tenant_id, inbox_id, contact_id, assignee_id, status, created_at, updated_at FROM chat_conversations WHERE tenant_id = $1 AND inbox_id = $2 AND contact_id = $3"
    )
    .bind(tenant_uuid)
    .bind(inbox.id)
    .bind(contact.id)
    .fetch_optional(&pool).await {
        Ok(Some(cv)) => cv,
        _ => chat_service.start_conversation(tenant_uuid, inbox.id, contact.id, None).await.unwrap()
    };

    let message = match chat_service.send_message(
        tenant_uuid,
        conversation.id,
        "contact".to_string(),
        Some(contact.id),
        payload.message.clone()
    ).await {
        Ok(m) => m,
        Err(e) => {
            tracing::error!("Failed to insert chat_message: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(WebhookResponse { success: false, message_id: None })).into_response();
        }
    };

    let payload_json = serde_json::json!({
        "message_id": message.id.to_string(),
        "inbox_message_id": message.id.to_string(),
        "source": payload.source,
        "content": payload.message,
        "sender_id": payload.sender_id,
        "conversation_id": conversation.id.to_string()
    });
    let job_id = Uuid::new_v4().to_string();

    let enqueue_result = match &state.db.store {
        crate::db::DbStore::Postgres => {
            sqlx::query("INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status) VALUES ($1, $2, 'message_triage', $3, 'PENDING')")
                .bind(&job_id)
                .bind(&payload.tenant_id)
                .bind(payload_json.to_string())
                .execute(&state.db.pool)
                .await
                .map(|_| ())
        },
        crate::db::DbStore::Sqlite(sqlite_pool) => {
            sqlx::query("INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status) VALUES (?, ?, 'message_triage', ?, 'PENDING')")
                .bind(&job_id)
                .bind(&payload.tenant_id)
                .bind(payload_json.to_string())
                .execute(sqlite_pool)
                .await
                .map(|_| ())
        }
    };

    if let Err(e) = enqueue_result {
        tracing::error!("Failed to enqueue message_triage job: {}", e);
    }

    // Broadcast to websocket clients
    let ws_msg = crate::api::realtime::RealtimeServerMessage::NewMessage {
        conversation_id: conversation.id.to_string(),
        message: payload_json.clone(),
    };
    if let Ok(serialized) = serde_json::to_string(&ws_msg) {
        let tx = crate::api::realtime::get_realtime_broadcast_tx();
        let _ = tx.send(serialized);
    }

    let event = DepartmentEvent {
        id: Uuid::new_v4().to_string(),
        tenant_id: payload.tenant_id.clone(),
        event_type: "tenant.omnichannel.message.received".to_string(),
        payload: payload_json.clone(),
    };

    match state.orchestrator.dispatch_event(event).await {
        Ok(_) => (StatusCode::OK, Json(WebhookResponse { success: true, message_id: Some(message.id.to_string()) })).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(WebhookResponse { success: false, message_id: None })).into_response()
    }
}
