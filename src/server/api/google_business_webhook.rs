use axum::{
    response::IntoResponse,
    http::StatusCode,
    routing::post,
    Router,
    Json,
    extract::State,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::hub::Hub;
use ::server_ohc::orchestration::Message;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GoogleReviewPayload {
    pub location_id: String,
    pub review_id: String,
    pub reviewer_name: String,
    pub star_rating: String,
    pub comment: String,
}

pub async fn handle_google_review_webhook(
    State(hub): State<Arc<Hub>>,
    Json(payload): Json<GoogleReviewPayload>,
) -> impl IntoResponse {
    let tenant_id = "default_tenant_id".to_string();

    let event_payload = serde_json::json!({
        "review_id": payload.review_id,
        "reviewer_name": payload.reviewer_name,
        "star_rating": payload.star_rating,
        "comment": payload.comment,
        "location_id": payload.location_id,
        "tenant_id": tenant_id,
    });

    let msg = Message {
        id: uuid::Uuid::new_v4().to_string(),
        from_agent: "google_business_webhook".to_string(),
        to_agent: "event_bus".to_string(),
        r#type: "NewReview".to_string(),
        content: event_payload.to_string(),
        occurred_at_unix: chrono::Utc::now().timestamp(),
        meeting_id: "".to_string(),
    };

    if let Err(e) = hub.clone().publish(msg) {
        tracing::error!("Failed to publish NewReview event: {}", e);
        return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to process review").into_response();
    }

    (StatusCode::OK, "Review received").into_response()
}

pub fn router(hub: Arc<Hub>) -> Router {
    Router::new()
        .route("/webhooks/google_business", post(handle_google_review_webhook))
        .with_state(hub)
}
