use crate::orchestration::departments::orchestrator::DepartmentOrchestrator;
use crate::orchestration::departments::types::DepartmentEvent;
use axum::{
    Router,
    extract::{Extension, Json, State},
    http::StatusCode,
    response::IntoResponse,
    routing::post,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
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
    claims: Option<Extension<::server_common::Claims>>,
    Json(payload): Json<BookingRequestPayload>,
) -> impl IntoResponse {
    let tenant_id = match claims
        .as_ref()
        .and_then(|Extension(claims)| ::server_common::auth_utils::signed_tenant_id(claims))
    {
        Some(tenant_id) => tenant_id,
        _ => {
            return (
                axum::http::StatusCode::UNAUTHORIZED,
                axum::Json(serde_json::json!({"error": "unauthorized"})),
            )
                .into_response();
        }
    };
    if payload.description.trim().is_empty()
        || payload.description.chars().count() > 10_000
        || payload.timestamp.chars().count() > 128
        || payload
            .file_name
            .as_ref()
            .is_some_and(|name| name.chars().count() > 255)
    {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "invalid booking request"})),
        )
            .into_response();
    }

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
        Ok(_) => {}
        Err(e) => {
            tracing::error!("Failed to dispatch booking request event: {}", e);
        }
    }

    // 2. Also inject directly to agent feed to ensure owner sees it immediately.
    let feed_id = uuid::Uuid::new_v4().to_string();
    let mut tx = match pool.begin().await {
        Ok(tx) => tx,
        Err(error) => {
            tracing::error!("failed to begin booking request transaction: {error}");
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "internal error"})),
            )
                .into_response();
        }
    };
    if let Err(error) =
        ::server_common::auth_utils::set_org_context(&mut *tx, &tenant_id_clone).await
    {
        tracing::error!("failed to bind booking request tenant context: {error}"); // pii-safe
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": "internal error"})),
        )
            .into_response();
    }
    let feed_result = sqlx::query(
        r#"
        INSERT INTO agent_feed (id, tenant_id, event_source, lifecycle_state, context_payload)
        VALUES ($1, $2, 'booking_request', 'new', $3)
        "#,
    )
    .bind(&feed_id)
    .bind(&tenant_id_clone)
    .bind(serde_json::json!({
        "message": payload.description,
        "source": "booking_form"
    }))
    .execute(&mut *tx)
    .await;
    if let Err(error) = feed_result {
        tracing::error!("failed to persist booking request: {error}");
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": "internal error"})),
        )
            .into_response();
    }
    if let Err(error) = tx.commit().await {
        tracing::error!("failed to commit booking request: {error}");
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": "internal error"})),
        )
            .into_response();
    }

    (
        StatusCode::OK,
        Json(BookingRequestResponse {
            success: true,
            request_id: Some(feed_id),
        }),
    )
        .into_response()
}
