use axum::{
    extract::{State, Json},
    response::IntoResponse,
    http::StatusCode,
};
use std::sync::Arc;
use serde::Deserialize;

use crate::db::DB;
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use crate::api::agents::identity_resolution::resolve_customer_identity;

#[derive(Clone)]
pub struct OmnichannelWebhookState {
    pub db: Arc<DB>,
    pub orchestrator: Arc<DepartmentOrchestrator>,
}

#[derive(Deserialize)]
pub struct OmnichannelWebhookPayload {
    pub tenant_id: String,
    pub source: String,
    pub sender_id: String,
    pub message: String,
    pub target_language: Option<String>,
}

pub async fn omnichannel_webhook_post_handler(
    State(state): State<OmnichannelWebhookState>,
    Json(payload): Json<OmnichannelWebhookPayload>,
) -> impl IntoResponse {
    if payload.message.trim().is_empty() {
        return StatusCode::OK.into_response();
    }

    tracing::info!("Received Omnichannel message from {}: {}", payload.sender_id, payload.message);

    // Resolve Identity
    let customer_id = match resolve_customer_identity(
        &state.db.store,
        &payload.tenant_id,
        &payload.source,
        &payload.sender_id,
    ).await {
        Ok(id) => id,
        Err(e) => {
            tracing::error!("Failed to resolve identity: {}", e);
            "unknown_customer".to_string()
        }
    };

    let target_language = payload.target_language.unwrap_or_else(|| "English".to_string());
    let inbox_id = uuid::Uuid::new_v4().to_string();

    // Store message
    let insert_result = match &state.db.store {
        crate::db::DbStore::Postgres => {
            sqlx::query(
                "INSERT INTO inbox_messages (id, tenant_id, source, original_content, content, translated_from_language, draft_reply, status, sender_id, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7, 'unread', $8, NOW())"
            )
            .bind(&inbox_id)
            .bind(&payload.tenant_id)
            .bind(&payload.source)
            .bind(&payload.message)
            .bind(&payload.message)
            .bind(None::<String>)
            .bind(None::<String>)
            .bind(&payload.sender_id)
            .execute(&state.db.pool)
            .await.map(|_| ())
        },
        crate::db::DbStore::Sqlite(sqlite_pool) => {
            sqlx::query(
                "INSERT INTO inbox_messages (id, tenant_id, source, original_content, content, translated_from_language, draft_reply, status, sender_id, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, 'unread', ?, CURRENT_TIMESTAMP)"
            )
            .bind(&inbox_id)
            .bind(&payload.tenant_id)
            .bind(&payload.source)
            .bind(&payload.message)
            .bind(&payload.message)
            .bind(None::<String>)
            .bind(None::<String>)
            .bind(&payload.sender_id)
            .execute(sqlite_pool)
            .await.map(|_| ())
        }
    };

    if let Err(e) = insert_result {
        tracing::error!("Failed to insert omnichannel inbox message: {}", e);
    }

    let event = crate::orchestration::departments::types::DepartmentEvent {
        id: uuid::Uuid::new_v4().to_string(),
        tenant_id: payload.tenant_id.clone(),
        event_type: "tenant.omnichannel.message.received".to_string(),
        payload: serde_json::json!({
            "source": payload.source,
            "original_message": payload.message,
            "target_language": target_language,
            "inbox_message_id": inbox_id,
            "customer_id": customer_id,
            "sender_id": payload.sender_id,
        }),
    };

    match state.orchestrator.dispatch_event(event).await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => {
            tracing::error!("Failed to dispatch omnichannel message event: {}", e);
            if e.contains("AI Budget exhausted") {
                StatusCode::TOO_MANY_REQUESTS.into_response()
            } else {
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
        }
    }
}
