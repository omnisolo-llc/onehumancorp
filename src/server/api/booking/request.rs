use axum::{
    extract::{State, Json},
    response::IntoResponse,
    http::StatusCode,
    routing::post,
    Router,
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use crate::orchestration::departments::types::DepartmentEvent;

#[derive(Deserialize)]
pub struct BookingRequestPayload {
    pub description: String,
    #[serde(default)]
    pub timestamp: String,
    #[serde(rename = "fileName")]
    pub file_name: Option<String>,
}

#[derive(Serialize)]
pub struct BookingRequestResponse {
    pub success: bool,
    pub request_id: Option<String>,
}

pub fn router<S>(orchestrator: Arc<DepartmentOrchestrator>, pool: sqlx::PgPool) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/", post(handle_booking_request))
        .with_state((orchestrator, pool))
}

async fn handle_booking_request(
    State((orchestrator, pool)): State<(Arc<DepartmentOrchestrator>, sqlx::PgPool)>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<BookingRequestPayload>,
) -> impl IntoResponse {
    let tenant_id = match headers.get("x-tenant-id").and_then(|h| h.to_str().ok()) {
        Some(t) if !t.trim().is_empty() => t.to_string(),
        _ => return (axum::http::StatusCode::UNAUTHORIZED, axum::Json(serde_json::json!({"error": "unauthorized"}))).into_response(),
    };

    let tenant_id_clone = tenant_id.clone();
    let event = DepartmentEvent {
        id: uuid::Uuid::new_v4().to_string(),
        tenant_id,
        event_type: "tenant.omnichannel.message.received".to_string(),
        payload: serde_json::json!({
            "source": "booking_form",
            "message": payload.description,
            "timestamp": payload.timestamp,
        }),
    };


    // 1. Dispatch event to orchestrator
    match orchestrator.dispatch_event(event).await {
        Ok(_) => {},
        Err(e) => {
            tracing::error!("Failed to dispatch booking request event: {}", e);
        }
    }

    // 2. Also inject directly to agent feed to ensure owner sees it immediately.
    let feed_id = uuid::Uuid::new_v4().to_string();
    let _ = sqlx::query(
        r#"
        INSERT INTO agent_feed (id, tenant_id, event_source, lifecycle_state, context_payload)
        VALUES ($1, $2, 'booking_request', 'new', $3)
        "#
    )
    .bind(&feed_id)
    .bind(&tenant_id_clone)
    .bind(serde_json::json!({
        "message": payload.description,
        "source": "booking_form"
    }))
    .execute(&pool)
    .await;

    (
        StatusCode::OK,
        Json(BookingRequestResponse {
            success: true,
            request_id: Some(feed_id),
        }),
    ).into_response()

}
