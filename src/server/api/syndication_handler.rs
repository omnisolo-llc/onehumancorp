use axum::{
    response::IntoResponse,
    http::StatusCode,
    routing::post,
    Router,
    Json,
};
use serde::{Deserialize, Serialize};

// In a real app this would call down to a repository to toggle state.
// For now we just implement the gRPC/REST endpoint stub.

#[derive(Serialize, Deserialize, Clone)]
pub struct ToggleSyndicationRequest {
    pub channel_id: String,
    pub enabled: bool,
}

#[derive(Serialize, Deserialize)]
pub struct ToggleSyndicationResponse {
    pub success: bool,
    pub message: String,
}

pub async fn toggle_syndication(
    Json(payload): Json<ToggleSyndicationRequest>,
) -> impl IntoResponse {
    // Basic stub logic: just acknowledge the toggle request
    if payload.channel_id.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(ToggleSyndicationResponse {
                success: false,
                message: "channel_id cannot be empty".to_string(),
            }),
        );
    }

    // In real system, write this preference to the DB for the current tenant.

    (
        StatusCode::OK,
        Json(ToggleSyndicationResponse {
            success: true,
            message: format!("Channel {} toggled to {}", payload.channel_id, payload.enabled),
        }),
    )
}

pub fn router(orchestrator: std::sync::Arc<crate::orchestration::departments::orchestrator::DepartmentOrchestrator>) -> Router {
    Router::new()
        .route("/api/v1/syndication/toggle", post(toggle_syndication))
        .route("/api/v1/syndication/webhook", post(handle_webhook))
        .with_state(orchestrator)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::Request;
    use axum::body::Body;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_toggle_syndication() {
        // Using a dummy orchestrator for test
        let orchestrator = std::sync::Arc::new(crate::orchestration::departments::orchestrator::DepartmentOrchestrator::new("dummy".to_string(), std::sync::Arc::new(crate::db::DB::new_in_memory().await)));
        let app = router(orchestrator);

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/syndication/toggle")
                    .header("Content-Type", "application/json")
                    .body(Body::from(r#"{"channel_id": "google_shopping", "enabled": true}"#))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}

#[derive(Deserialize)]
pub struct WebhookPayload {
    pub tenant_id: String,
    pub event_type: String,
    pub data: serde_json::Value,
}

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

pub async fn handle_webhook(
    axum::extract::State(orchestrator): axum::extract::State<std::sync::Arc<crate::orchestration::departments::orchestrator::DepartmentOrchestrator>>,
    Json(payload): Json<WebhookPayload>,
) -> impl IntoResponse {
    if payload.event_type == "google.review.created" {
        let review_event = crate::orchestration::departments::types::DepartmentEvent {
            id: uuid::Uuid::new_v4().to_string(),
            tenant_id: payload.tenant_id.clone(),
            event_type: "tenant.review.received".to_string(),
            payload: payload.data.clone(),
        };
        let _ = orchestrator.dispatch_event(review_event).await;
    }

    (
        StatusCode::OK,
        Json(GenericResponse {
            status: "success".to_string(),
            message: "Webhook processed".to_string(),
        })
    )
}
