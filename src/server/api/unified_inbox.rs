use axum::{
    extract::{State, Json},
    response::IntoResponse,
    http::StatusCode,
    routing::{post, get},
    Router,
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::db::DB;
use crate::{ui_tenant_id, UiTenantQuery};
use axum::extract::Query;

#[derive(Clone)]
pub struct UnifiedInboxState {
    pub db: Arc<DB>,
}

#[derive(Deserialize)]
pub struct UnifiedInboxPayload {
    pub tenant_id: String,
    pub channel_type: String,
    pub sender_id: String,
    pub content: String,
}

#[derive(Serialize)]
pub struct WebhookResponse {
    pub success: bool,
    pub message_id: Option<String>,
}

#[derive(Serialize)]
pub struct TriageFeedResponse {
    pub messages: Vec<serde_json::Value>,
}

pub fn router<S>(state: UnifiedInboxState) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/webhook", post(ingest_message_handler))
        .route("/triage-feed", get(get_triage_feed_handler))
        .with_state(state)
}

pub async fn ingest_message_handler(
    State(state): State<UnifiedInboxState>,
    Json(payload): Json<UnifiedInboxPayload>,
) -> impl IntoResponse {
    if payload.content.is_empty() {
        return (StatusCode::BAD_REQUEST, Json(WebhookResponse { success: false, message_id: None })).into_response();
    }

    let tenant_id = payload.tenant_id;
    let channel_type = payload.channel_type.to_lowercase();
    let sender_id = payload.sender_id;
    let content = payload.content;

    let message_id = Uuid::new_v4().to_string();

    let insert_result = match &state.db.store {
        crate::db::DbStore::Postgres => {
            sqlx::query(
                "INSERT INTO unified_messages (id, tenant_id, channel_type, sender_id, content, status, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, 'unread', NOW(), NOW())"
            )
            .bind(&message_id)
            .bind(&tenant_id)
            .bind(&channel_type)
            .bind(&sender_id)
            .bind(&content)
            .execute(&state.db.pool)
            .await.map(|_| ())
        },
        crate::db::DbStore::Sqlite(sqlite_pool) => {
            sqlx::query(
                "INSERT INTO unified_messages (id, tenant_id, channel_type, sender_id, content, status, created_at, updated_at) VALUES (?, ?, ?, ?, ?, 'unread', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)"
            )
            .bind(&message_id)
            .bind(&tenant_id)
            .bind(&channel_type)
            .bind(&sender_id)
            .bind(&content)
            .execute(sqlite_pool)
            .await.map(|_| ())
        }
    };

    if let Err(e) = insert_result {
        tracing::error!("Failed to insert unified_message: {}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(WebhookResponse { success: false, message_id: None })).into_response();
    }

    // Enqueue job for AI Work Triage Agent
    let job_id = Uuid::new_v4().to_string();
    let payload_json = serde_json::json!({
        "message_id": message_id,
        "channel_type": channel_type,
        "content": content,
        "sender_id": sender_id
    });

    let enqueue_result = match &state.db.store {
        crate::db::DbStore::Postgres => {
            sqlx::query("INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status) VALUES ($1, $2, 'ai_work_triage_agent', $3, 'PENDING')")
                .bind(&job_id)
                .bind(&tenant_id)
                .bind(payload_json.to_string())
                .execute(&state.db.pool)
                .await
                .map(|_| ())
        },
        crate::db::DbStore::Sqlite(sqlite_pool) => {
            sqlx::query("INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status) VALUES (?, ?, 'ai_work_triage_agent', ?, 'PENDING')")
                .bind(&job_id)
                .bind(&tenant_id)
                .bind(payload_json.to_string())
                .execute(sqlite_pool)
                .await
                .map(|_| ())
        }
    };

    if let Err(e) = enqueue_result {
        tracing::error!("Failed to enqueue ai_work_triage_agent job: {}", e);
    }

    (StatusCode::OK, Json(WebhookResponse { success: true, message_id: Some(message_id) })).into_response()
}

pub async fn get_triage_feed_handler(
    State(state): State<UnifiedInboxState>,
    Query(query): Query<UiTenantQuery>,
) -> impl IntoResponse {
    let tenant_id = ui_tenant_id(&query);

    let messages_result = match &state.db.store {
        crate::db::DbStore::Postgres => {
            sqlx::query(
                r#"
                SELECT m.id, m.channel_type, m.sender_id, m.content, m.status as message_status, CAST(m.created_at AS text) as created_at,
                       a.id as action_id, a.action_type, a.payload as action_payload, a.status as action_status
                FROM unified_messages m
                LEFT JOIN pending_work_actions a ON m.id = a.message_id
                WHERE m.tenant_id = $1
                ORDER BY m.created_at DESC
                LIMIT 50
                "#
            )
            .bind(&tenant_id)
            .fetch_all(&state.db.pool)
            .await.map(|rows| {
                use sqlx::Row;
                rows.into_iter().map(|r| {
                    let action_id: Option<String> = r.try_get("action_id").ok().flatten();
                    let action = if let Some(aid) = action_id {
                        Some(serde_json::json!({
                            "id": aid,
                            "action_type": r.get::<String, _>("action_type"),
                            "payload": r.get::<String, _>("action_payload"),
                            "status": r.get::<String, _>("action_status")
                        }))
                    } else {
                        None
                    };

                    serde_json::json!({
                        "id": r.get::<String, _>("id"),
                        "channel_type": r.get::<String, _>("channel_type"),
                        "sender_id": r.get::<String, _>("sender_id"),
                        "content": r.get::<String, _>("content"),
                        "status": r.get::<String, _>("message_status"),
                        "created_at": r.get::<String, _>("created_at"),
                        "action": action
                    })
                }).collect::<Vec<_>>()
            })
        },
        crate::db::DbStore::Sqlite(sqlite_pool) => {
            sqlx::query(
                r#"
                SELECT m.id, m.channel_type, m.sender_id, m.content, m.status as message_status, CAST(m.created_at AS TEXT) as created_at,
                       a.id as action_id, a.action_type, a.payload as action_payload, a.status as action_status
                FROM unified_messages m
                LEFT JOIN pending_work_actions a ON m.id = a.message_id
                WHERE m.tenant_id = ?
                ORDER BY m.created_at DESC
                LIMIT 50
                "#
            )
            .bind(&tenant_id)
            .fetch_all(sqlite_pool)
            .await.map(|rows| {
                use sqlx::Row;
                rows.into_iter().map(|r| {
                    let action_id: Option<String> = r.try_get("action_id").ok().flatten();
                    let action = if let Some(aid) = action_id {
                        Some(serde_json::json!({
                            "id": aid,
                            "action_type": r.get::<String, _>("action_type"),
                            "payload": r.get::<String, _>("action_payload"),
                            "status": r.get::<String, _>("action_status")
                        }))
                    } else {
                        None
                    };

                    serde_json::json!({
                        "id": r.get::<String, _>("id"),
                        "channel_type": r.get::<String, _>("channel_type"),
                        "sender_id": r.get::<String, _>("sender_id"),
                        "content": r.get::<String, _>("content"),
                        "status": r.get::<String, _>("message_status"),
                        "created_at": r.get::<String, _>("created_at"),
                        "action": action
                    })
                }).collect::<Vec<_>>()
            })
        }
    };

    match messages_result {
        Ok(messages) => (StatusCode::OK, Json(TriageFeedResponse { messages })).into_response(),
        Err(e) => {
            tracing::error!("Failed to fetch unified messages: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(TriageFeedResponse { messages: vec![] })).into_response()
        }
    }
}
