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

pub fn router() -> Router {
    Router::new()
        .route("/api/v1/syndication/toggle", post(toggle_syndication))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::Request;
    use axum::body::Body;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_toggle_syndication() {
        let app = router();

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
