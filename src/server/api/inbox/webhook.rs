use axum::{
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

pub fn router<S>(state: OmnichannelWebhookState) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/webhook", post(handle_omnichannel_webhook))
        .with_state(state)
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

            // First ensure a conversation exists
            let conversation_id = Uuid::new_v4().to_string();
            let _ = sqlx::query(
                r#"
                INSERT INTO unified_conversations (id, tenant_id, customer_id, channel_provider, channel_identifier, status, created_at, updated_at)
                VALUES ($1, $2, $3, $4, $5, 'open', NOW(), NOW())
                ON CONFLICT DO NOTHING
                "#
            )
            .bind(&conversation_id)
            .bind(&payload.tenant_id)
            .bind(&customer_id)
            .bind(&payload.source)
            .bind(&payload.sender_id)
            .execute(pool)
            .await;

            // Get existing or new conversation ID
            let conv_id = sqlx::query_scalar::<_, String>("SELECT id FROM unified_conversations WHERE tenant_id = $1 AND channel_identifier = $2 AND channel_provider = $3 LIMIT 1")
                .bind(&payload.tenant_id)
                .bind(&payload.sender_id)
                .bind(&payload.source)
                .fetch_one(pool)
                .await.unwrap_or(conversation_id);

            sqlx::query(
                r#"
                INSERT INTO unified_messages (id, tenant_id, conversation_id, sender_type, sender_id, content, created_at)
                VALUES ($1, $2, $3, 'customer', $4, $5, NOW())
                "#
            )
            .bind(&id)
            .bind(&payload.tenant_id)
            .bind(&conv_id)
            .bind(&payload.sender_id)
            .bind(&payload.message)
            .execute(pool)
            .await.map(|_| ())

        },
        crate::db::DbStore::Sqlite(sqlite_pool) => {

            // First ensure a conversation exists
            let conversation_id = Uuid::new_v4().to_string();
            let _ = sqlx::query(
                r#"
                INSERT INTO unified_conversations (id, tenant_id, customer_id, channel_provider, channel_identifier, status, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, 'open', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
                ON CONFLICT DO NOTHING
                "#
            )
            .bind(&conversation_id)
            .bind(&payload.tenant_id)
            .bind(&customer_id)
            .bind(&payload.source)
            .bind(&payload.sender_id)
            .execute(sqlite_pool)
            .await;

            // Get existing or new conversation ID
            let conv_id = sqlx::query_scalar::<_, String>("SELECT id FROM unified_conversations WHERE tenant_id = ? AND channel_identifier = ? AND channel_provider = ? LIMIT 1")
                .bind(&payload.tenant_id)
                .bind(&payload.sender_id)
                .bind(&payload.source)
                .fetch_one(sqlite_pool)
                .await.unwrap_or(conversation_id);

            sqlx::query(
                r#"
                INSERT INTO unified_messages (id, tenant_id, conversation_id, sender_type, sender_id, content, created_at)
                VALUES (?, ?, ?, 'customer', ?, ?, CURRENT_TIMESTAMP)
                "#
            )
            .bind(&id)
            .bind(&payload.tenant_id)
            .bind(&conv_id)
            .bind(&payload.sender_id)
            .bind(&payload.message)
            .execute(sqlite_pool)
            .await.map(|_| ())

        }
    };

    let payload_json = serde_json::json!({
        "message_id": id,
        "unified_message_id": id,
        "source": payload.source,
        "content": payload.message,
        "sender_id": payload.sender_id
    });
    let job_id = Uuid::new_v4().to_string();

    if let Err(e) = insert_result {
        tracing::error!("Failed to insert into unified_messages: {}", e);
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
