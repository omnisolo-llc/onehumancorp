use axum::{
    extract::{State, Extension},
    http::{StatusCode, HeaderMap},
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use sqlx::PgPool;
use chrono::{DateTime, Utc};
use crate::orchestration::queue::ohc_job_queue::OHCJobQueue;
use ::server_common::Claims;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IngestEventRequest {
    pub tenant_id: String,
    pub event_type: String,
    pub source: String,
    pub payload: serde_json::Value,
    pub timestamp: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IngestEventResponse {
    pub message: String,
    pub job_id: String,
}

pub fn router<S: Clone + Send + Sync + 'static>(pool: Arc<PgPool>) -> Router<S> {
    Router::new()
        // Note: we'll apply a middleware or expect the mesh/auth to inject claims
        .route("/api/v1/events/ingest", post(ingest_event))
        .with_state(pool)
}

// In production, we'd have a webhook secret stored in the database for each tenant.
// For now, if the webhook provides an API key, we ensure it matches an expected pattern
// or fallback to the validated internal JWT Claims.
async fn validate_auth(
    headers: &HeaderMap,
    claims: Option<&Claims>,
    requested_tenant: &str,
    _pool: &PgPool,
) -> bool {
    // 1. Internal Authentication: If they have a valid JWT and the tenant matches or they are system/admin
    if let Some(c) = claims {
        if let Some(t_id) = &c.organization_id {
            if t_id == requested_tenant {
                return true;
            }
        }
        // If they don't have a specific organization but are hitting default (e.g. system bot)
        if requested_tenant == ::server_common::auth_utils::get_default_tenant() {
             return true;
        }
    }

    // 2. External Webhook API Key: if no JWT, check the static API key header.
    // In the future this should query the DB for the tenant's exact webhook secret.
    // To satisfy the security audit, we cannot hardcode a backdoor or predictable secret.
    // Since we don't have the table yet, we'll assume there is a secure way to verify it here
    // But for the scope of this PR, we will reject any requests that aren't authenticated properly via JWT.
    // Webhook auth will be implemented securely in a follow-up PR when the DB schema for integration secrets is ready.
    if let Some(_key) = headers.get("x-ohc-event-key") {
        // Placeholder for secure DB query
        // e.g. let secret = get_tenant_webhook_secret(pool, requested_tenant).await;
        // if key == secret { return true; }
        tracing::warn!("Webhook API key validation is not yet implemented securely. Use internal JWT.");
    }

    false
}

async fn ingest_event(
    headers: HeaderMap,
    State(pool): State<Arc<PgPool>>,
    claims: Option<Extension<Claims>>,
    Json(payload): Json<IngestEventRequest>,
) -> impl IntoResponse {
    let claims_ref = claims.as_ref().map(|ext| &ext.0);

    if !validate_auth(&headers, claims_ref, &payload.tenant_id, &pool).await {
        return (
            StatusCode::UNAUTHORIZED,
            Json(IngestEventResponse {
                message: "Unauthorized or invalid signature".to_string(),
                job_id: "".to_string(),
            }),
        );
    }

    let job_queue = OHCJobQueue::new(pool.clone());

    // Store original event_type and source inside the queue payload
    let mut enriched_payload = payload.payload.clone();
    if let Some(obj) = enriched_payload.as_object_mut() {
        obj.insert("original_event_type".to_string(), serde_json::Value::String(payload.event_type.clone()));
        obj.insert("event_source".to_string(), serde_json::Value::String(payload.source.clone()));
    } else {
        // If it's not an object, wrap it
        enriched_payload = serde_json::json!({
            "original_event_type": payload.event_type,
            "event_source": payload.source,
            "data": payload.payload
        });
    }

    // We enqueue all events under the "event_ingestion" job_type so our worker can pick them up
    match job_queue.enqueue(&payload.tenant_id, "event_ingestion", &enriched_payload).await {
        Ok(job_id) => {
            (
                StatusCode::ACCEPTED,
                Json(IngestEventResponse {
                    message: "Event ingested and queued for processing".to_string(),
                    job_id,
                }),
            )
        }
        Err(e) => {
            tracing::error!("Failed to enqueue event: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(IngestEventResponse {
                    message: "Failed to ingest event".to_string(),
                    job_id: "".to_string(),
                }),
            )
        }
    }
}
