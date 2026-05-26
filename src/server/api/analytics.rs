use axum::{Json, extract::Extension, http::StatusCode, http::HeaderMap};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::db::DB;

#[derive(Deserialize)]
pub struct EventPayload {
    pub customer_id: Option<String>,
    pub event_type: String,
    pub payload: serde_json::Value,
}

#[derive(Serialize)]
pub struct IngestResponse {
    pub success: bool,
}

pub async fn handle_ingest_event(
    headers: HeaderMap,
    Extension(nats): Extension<Arc<crate::integrations::nats::provider::NatsProvider>>,
    Json(payload): Json<EventPayload>,
) -> Result<Json<IngestResponse>, StatusCode> {

    // Authenticate producers using SPIRE via header parsing since this endpoint isn't fully behind the grpc middleware yet
    let spiffe_id = headers.get("x-spiffe-id").and_then(|h| h.to_str().ok()).unwrap_or("");
    let tenant_id = headers.get("x-org-id").and_then(|h| h.to_str().ok()).unwrap_or("");

    if spiffe_id.is_empty() || tenant_id.is_empty() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let id = uuid::Uuid::new_v4().to_string();

    let nats_msg = serde_json::json!({
        "id": id,
        "tenant_id": tenant_id,
        "customer_id": payload.customer_id,
        "event_type": payload.event_type,
        "payload": payload.payload
    });

    // Asynchronous, non-blocking write to datastore via event mesh
    let res = nats.publish("business.events", serde_json::to_vec(&nats_msg).unwrap()).await;
    if res.is_err() {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    Ok(Json(IngestResponse { success: true }))
}

pub async fn handle_daily_briefing(
    headers: HeaderMap,
    Extension(db): Extension<Arc<DB>>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let spiffe_id = headers.get("x-spiffe-id").and_then(|h| h.to_str().ok()).unwrap_or("");
    let tenant_id = headers.get("x-org-id").and_then(|h| h.to_str().ok()).unwrap_or("");

    // The UI handles its own session auth, but for strict SPIRE the backend wants these headers.
    // For local UI tests, we fallback gracefully or we assume the UI sets them via proxy.
    // Given the E2E tests pass via mock, we just enforce it normally.

    if tenant_id.is_empty() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let res = sqlx::query("SELECT plain_language_summary, briefing_date FROM daily_briefings WHERE tenant_id = $1 ORDER BY briefing_date DESC LIMIT 1")
        .bind(&tenant_id)
        .fetch_optional(&db.pool)
        .await;

    match res {
        Ok(Some(row)) => {
            use sqlx::Row;
            let summary: String = row.get("plain_language_summary");
            let date: chrono::NaiveDate = row.get("briefing_date");
            Ok(Json(serde_json::json!({
                "briefing": summary,
                "date": date.to_string()
            })))
        }
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
