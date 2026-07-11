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
            let query = "SELECT id, tenant_id, customer_id, channel, status, CAST(created_at AS text) as created_at FROM unified_threads WHERE tenant_id = $1 ORDER BY created_at DESC";
            match sqlx::query(query).bind(&tenant_id).fetch_all(&state.db.pool).await {
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
            let query = "SELECT id, tenant_id, customer_id, channel, status, CAST(created_at AS text) as created_at FROM unified_threads WHERE tenant_id = ? ORDER BY created_at DESC";
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
            let query = "SELECT id, tenant_id, source, original_content, translated_content, draft_reply, status, sender_id, CAST(created_at AS text) as created_at FROM omni_inbox_messages WHERE tenant_id = $1 AND customer_id = (SELECT customer_id FROM unified_threads WHERE id = $2) ORDER BY created_at ASC";
            match sqlx::query(query).bind(&tenant_id).bind(&conversation_id).fetch_all(&state.db.pool).await {
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
            let query = "SELECT id, tenant_id, source, original_content, translated_content, draft_reply, status, sender_id, CAST(created_at AS text) as created_at FROM omni_inbox_messages WHERE tenant_id = ? AND customer_id = (SELECT customer_id FROM unified_threads WHERE id = ?) ORDER BY created_at ASC";
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
    Json(payload): Json<OmnichannelPayload>,
) -> impl IntoResponse {
    let customer_id = resolve_identity(&state.db, &payload.tenant_id, &payload.source, &payload.sender_id).await;

    let id = Uuid::new_v4().to_string();
    let _target_language = payload.target_language.unwrap_or_else(|| "English".to_string());

    let pool = &state.db.pool;

    let insert_result = match &state.db.store {
        crate::db::DbStore::Postgres => {
            sqlx::query(
                r#"
                INSERT INTO omni_inbox_messages (id, tenant_id, source, original_content, translated_content, target_language, draft_reply, status, sender_id, customer_id, created_at)
                VALUES ($1, $2, $3, $4, $5, 'English', '', 'unread', $6, $7, NOW())
                "#
            )
            .bind(&id)
            .bind(&payload.tenant_id)
            .bind(&payload.source)
            .bind(&payload.message)
            .bind(&payload.message)
            .bind(&payload.sender_id)
            .bind(&customer_id)
            .execute(pool)
            .await.map(|_| ())
        },
        crate::db::DbStore::Sqlite(sqlite_pool) => {
            sqlx::query(
                r#"
                INSERT INTO omni_inbox_messages (id, tenant_id, source, original_content, translated_content, target_language, draft_reply, status, sender_id, customer_id, created_at)
                VALUES (?, ?, ?, ?, ?, 'English', '', 'unread', ?, ?, CURRENT_TIMESTAMP)
                "#
            )
            .bind(&id)
            .bind(&payload.tenant_id)
            .bind(&payload.source)
            .bind(&payload.message)
            .bind(&payload.message)
            .bind(&payload.sender_id)
            .bind(&customer_id)
            .execute(sqlite_pool)
            .await.map(|_| ())
        }
    };

    let payload_json = serde_json::json!({
        "message_id": id,
        "inbox_message_id": id,
        "source": payload.source,
        "content": payload.message,
        "sender_id": payload.sender_id
    });
    let job_id = Uuid::new_v4().to_string();

    if let Err(e) = insert_result {
        tracing::error!("Failed to insert into inbox_messages: {}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(WebhookResponse { success: false, message_id: None })).into_response();
    }

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

    let event = DepartmentEvent {
        id: Uuid::new_v4().to_string(),
        tenant_id: payload.tenant_id.clone(),
        event_type: "tenant.omnichannel.message.received".to_string(),
        payload: payload_json.clone(),
    };

    match state.orchestrator.dispatch_event(event).await {
        Ok(_) => (StatusCode::OK, Json(WebhookResponse { success: true, message_id: Some(id) })).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(WebhookResponse { success: false, message_id: None })).into_response()
    }
}
