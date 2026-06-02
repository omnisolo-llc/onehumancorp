use axum::{
    response::IntoResponse,
    http::StatusCode,
    routing::post,
    Router,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub struct ConnectGoogleBusinessRequest {
    pub auth_code: String,
}

#[derive(Serialize, Deserialize)]
pub struct WebhookResponse {
    pub success: bool,
    pub message: String,
}

pub async fn connect_google_business(
    Json(_payload): Json<ConnectGoogleBusinessRequest>,
) -> impl IntoResponse {
    // In a real implementation we would exchange the auth_code for an access token
    // and save it to the tenant's integrations config in the DB.
    (
        StatusCode::OK,
        Json(WebhookResponse {
            success: true,
            message: "Successfully connected to Google Business Profile".to_string(),
        }),
    )
}

pub async fn handle_review_webhook(
    Json(_payload): Json<Value>,
) -> impl IntoResponse {
    // In a real system, we'd use the orchestrator here.
    // We'll mock the success for now as the orchestrator handles the AI response internally
    // when properly wired via events.
    (
        StatusCode::OK,
        Json(WebhookResponse {
            success: true,
            message: "Review webhook processed".to_string(),
        }),
    )
}

pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/connect", post(connect_google_business))
        .route("/reviews/webhook", post(handle_review_webhook))
}
