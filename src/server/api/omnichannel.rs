use axum::{
    extract::{State, Json},
    response::IntoResponse,
    http::StatusCode,
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::db::{DB, DbStore};
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;

/// Attempts to resolve the incoming sender identity to an existing customer ID.
pub async fn resolve_identity(
    db: &Arc<DB>,
    tenant_id: &str,
    source: &str,
    sender_id: &str,
) -> Option<String> {
    if sender_id.is_empty() || sender_id == "unknown" {
        return None;
    }

    let query_res: Result<Option<String>, _> = match &db.store {
        DbStore::Postgres => {
            let q = if source == "email" {
                "SELECT id FROM customers WHERE tenant_id = $1 AND email = $2 LIMIT 1"
            } else if source == "whatsapp" {
                "SELECT id FROM customers WHERE tenant_id = $1 AND phone = $2 LIMIT 1"
            } else if source == "instagram" {
                "SELECT id FROM customers WHERE tenant_id = $1 AND name = $2 LIMIT 1"
            } else {
                return None;
            };

            sqlx::query_scalar(q)
                .bind(tenant_id)
                .bind(sender_id)
                .fetch_optional(&db.pool)
                .await
        }
        DbStore::Sqlite(pool) => {
            let q = if source == "email" {
                "SELECT id FROM customers WHERE tenant_id = ? AND email = ? LIMIT 1"
            } else if source == "whatsapp" {
                "SELECT id FROM customers WHERE tenant_id = ? AND phone = ? LIMIT 1"
            } else if source == "instagram" {
                "SELECT id FROM customers WHERE tenant_id = ? AND name = ? LIMIT 1"
            } else {
                return None;
            };

            sqlx::query_scalar(q)
                .bind(tenant_id)
                .bind(sender_id)
                .fetch_optional(pool)
                .await
        }
    };

    match query_res {
        Ok(opt) => opt,
        Err(e) => {
            tracing::error!("Failed to resolve identity for {}: {}", sender_id, e);
            None
        }
    }
}

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
    pub target_language: Option<String>,
}

#[derive(Serialize)]
pub struct WebhookResponse {
    pub success: bool,
    pub request_id: Option<String>,
}

pub async fn omnichannel_webhook_handler(
    State(state): State<OmnichannelWebhookState>,
    Json(payload): Json<OmnichannelPayload>,
) -> impl IntoResponse {
    if payload.message.is_empty() || payload.source.is_empty() || payload.sender_id.is_empty() {
        return (StatusCode::BAD_REQUEST, Json(WebhookResponse { success: false, request_id: None })).into_response();
    }

    let customer_id = resolve_identity(&state.db, &payload.tenant_id, &payload.source, &payload.sender_id).await;

    let target_language = payload.target_language.unwrap_or_else(|| "English".to_string());
    let inbox_id = Uuid::new_v4().to_string();

    let insert_result = match &state.db.store {
        DbStore::Postgres => {
            sqlx::query(
                "INSERT INTO omni_inbox_messages (id, tenant_id, source, original_content, translated_content, source_language, target_language, status, sender_id, customer_id, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, 'unread', $8, $9, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)"
            )
            .bind(&inbox_id)
            .bind(&payload.tenant_id)
            .bind(&payload.source)
            .bind(&payload.message)
            .bind(&payload.message) // Initially untranslated
            .bind("Unknown")
            .bind(&target_language)
            .bind(&payload.sender_id)
            .bind(&customer_id)
            .execute(&state.db.pool)
            .await.map(|_| ())
        },
        DbStore::Sqlite(sqlite_pool) => {
            sqlx::query(
                "INSERT INTO omni_inbox_messages (id, tenant_id, source, original_content, translated_content, source_language, target_language, status, sender_id, customer_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, 'unread', ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)"
            )
            .bind(&inbox_id)
            .bind(&payload.tenant_id)
            .bind(&payload.source)
            .bind(&payload.message)
            .bind(&payload.message)
            .bind("Unknown")
            .bind(&target_language)
            .bind(&payload.sender_id)
            .bind(&customer_id)
            .execute(sqlite_pool)
            .await.map(|_| ())
        }
    };

    if let Err(e) = insert_result {
        tracing::error!("Failed to insert omni_inbox_message: {}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(WebhookResponse { success: false, request_id: None })).into_response();
    }

    let event = crate::orchestration::departments::types::DepartmentEvent {
        id: Uuid::new_v4().to_string(),
        tenant_id: payload.tenant_id.clone(),
        event_type: "tenant.omnichannel.message.received".to_string(),
        payload: serde_json::json!({
            "source": payload.source,
            "original_message": payload.message,
            "target_language": target_language,
            "inbox_message_id": inbox_id,
            "sender_id": payload.sender_id,
            "customer_id": customer_id,
        }),
    };

    match state.orchestrator.dispatch_event(event).await {
        Ok(_) => (StatusCode::OK, Json(WebhookResponse { success: true, request_id: Some(inbox_id) })).into_response(),
        Err(e) => {
            if e.contains("AI Budget exhausted") {
                (StatusCode::TOO_MANY_REQUESTS, Json(WebhookResponse { success: false, request_id: None })).into_response()
            } else {
                tracing::error!("Orchestrator dispatch failed: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, Json(WebhookResponse { success: false, request_id: None })).into_response()
            }
        }
    }
}
