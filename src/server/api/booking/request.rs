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

pub fn router<S>(orchestrator: Arc<DepartmentOrchestrator>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/", post(handle_booking_request))
        .with_state(orchestrator)
}

async fn handle_booking_request(
    State(orchestrator): State<Arc<DepartmentOrchestrator>>,
    headers: axum::http::HeaderMap,
    auth_info_opt: Option<axum::extract::Extension<::server_auth::orchestration::AuthInfo>>,
    Json(payload): Json<BookingRequestPayload>,
) -> impl IntoResponse {
    let tenant_id = if ::server_config::get().multitenant {
        let auth_info = match auth_info_opt {
            Some(axum::extract::Extension(info)) => info,
            None => return (axum::http::StatusCode::UNAUTHORIZED, axum::Json(serde_json::json!({"error": "unauthorized"}))).into_response(),
        };
        if auth_info.org_id.trim().is_empty() {
            return (axum::http::StatusCode::UNAUTHORIZED, axum::Json(serde_json::json!({"error": "unauthorized"}))).into_response();
        }
        auth_info.org_id
    } else {
        match headers.get("x-tenant-id").and_then(|h| h.to_str().ok()) {
            Some(t) if !t.trim().is_empty() => t.to_string(),
            _ => return (axum::http::StatusCode::UNAUTHORIZED, axum::Json(serde_json::json!({"error": "unauthorized"}))).into_response(),
        }
    };

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

    match orchestrator.dispatch_event(event).await {
        Ok(_) => (
            StatusCode::OK,
            Json(BookingRequestResponse {
                success: true,
                request_id: Some(uuid::Uuid::new_v4().to_string()),
            }),
        )
            .into_response(),
        Err(e) => {
            tracing::error!("Failed to dispatch booking request event: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(BookingRequestResponse {
                    success: false,
                    request_id: None,
                }),
            )
                .into_response()
        }
    }
}
