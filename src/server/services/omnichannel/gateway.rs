use axum::{
    extract::{State},
    http::StatusCode,
    response::IntoResponse,
    routing::{post},
    Json, Router,
};
use serde::{Deserialize};
use sqlx::PgPool;
use tracing::{error, info};
use std::sync::Arc;
use crate::omnichannel::identity::IdentityResolutionEngine;
use uuid::Uuid;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub identity_engine: Arc<IdentityResolutionEngine>,
}

#[derive(Debug, Deserialize)]
pub struct WebhookPayload {
    pub tenant_id: String,
    pub source: String,
    pub identifier: String,
    pub message: String,
}

pub fn router(pool: PgPool) -> Router {
    let state = AppState {
        pool: pool.clone(),
        identity_engine: Arc::new(IdentityResolutionEngine::new(pool.clone())),
    };

    Router::new()
        .route("/webhooks/omnichannel", post(handle_webhook))
        .with_state(state)
}

pub async fn handle_webhook(
    State(state): State<AppState>,
    Json(payload): Json<WebhookPayload>,
) -> impl IntoResponse {
    info!("Received omnichannel webhook from {} for tenant {}", payload.source, payload.tenant_id);

    // 1. Identity Resolution
    let customer = match state.identity_engine.resolve_customer(&payload.tenant_id, &payload.identifier, &payload.source).await {
        Ok(c) => c,
        Err(e) => {
            error!("Failed to resolve identity: {:?}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    // 2. Queue job for Message Triage Worker to handle drafting, context synthesis, and agent feed routing.
    let message_id = Uuid::new_v4().to_string();

    // Attempt to persist the incoming message immediately for history
    if let Err(e) = sqlx::query(
        "INSERT INTO omni_inbox_messages (id, tenant_id, customer_id, source, content, status, direction, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, 'received', 'inbound', NOW(), NOW())"
    )
    .bind(&message_id)
    .bind(&payload.tenant_id)
    .bind(&customer.id)
    .bind(&payload.source)
    .bind(&payload.message)
    .execute(&state.pool).await {
        tracing::error!("Failed to persist incoming message: {:?}", e);
        // Fallback to SQLite query if using SQLite memory backend
        let _ = sqlx::query(
            "INSERT INTO omni_inbox_messages (id, tenant_id, customer_id, source, content, status, direction, created_at, updated_at) VALUES (?, ?, ?, ?, ?, 'received', 'inbound', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)"
        )
        .bind(&message_id)
        .bind(&payload.tenant_id)
        .bind(&customer.id)
        .bind(&payload.source)
        .bind(&payload.message)
        .execute(&state.pool).await;
    }

    let job_payload = serde_json::json!({
        "message_id": message_id,
        "source": payload.source,
        "content": payload.message,
        "customer_id": customer.id
    });

    let job_id = Uuid::new_v4().to_string();

    // Insert into ohc_job_queue
    if let Err(e) = sqlx::query(
        "INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status, created_at, updated_at, next_retry_at) VALUES ($1, $2, 'message_triage', $3, 'PENDING', NOW(), NOW(), NOW())"
    )
    .bind(&job_id)
    .bind(&payload.tenant_id)
    .bind(&job_payload)
    .execute(&state.pool).await {
        tracing::error!("Failed to enqueue job: {:?}", e);
        let _ = sqlx::query(
            "INSERT INTO ohc_job_queue (id, tenant_id, job_type, payload, status, created_at, updated_at, next_retry_at) VALUES (?, ?, 'message_triage', ?, 'PENDING', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)"
        )
        .bind(&job_id)
        .bind(&payload.tenant_id)
        .bind(job_payload.to_string())
        .execute(&state.pool).await;
    }

    (StatusCode::OK, "Webhook processed and draft queued").into_response()
}
