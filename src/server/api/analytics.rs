use axum::{Json, extract::Extension, http::StatusCode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::db::DB;
use server_auth::orchestration::AuthInfo;

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
    Extension(auth_info): Extension<AuthInfo>,
    Extension(nats): Extension<Arc<crate::integrations::nats::provider::NatsProvider>>,
    Json(payload): Json<EventPayload>,
) -> Result<Json<IngestResponse>, StatusCode> {

    // Authenticate producers using SPIRE (AuthInfo injected by middleware)
    if auth_info.spiffe_id.is_empty() {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let tenant_id = auth_info.org_id;

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
    Extension(auth_info): Extension<AuthInfo>,
    Extension(db): Extension<Arc<DB>>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    if auth_info.spiffe_id.is_empty() {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let tenant_id = auth_info.org_id;

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
