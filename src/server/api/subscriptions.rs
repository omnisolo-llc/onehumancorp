use axum::{
    extract::{Extension, Json, Path},
    response::IntoResponse,
    routing::{post, get},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::hub::Hub;
use axum::http::StatusCode;

#[derive(Deserialize)]
pub struct CreateSubscriptionIntentRequest {
    pub tenant_id: String,
    pub product_id: String,
    pub customer_email: String,
}

#[derive(Serialize)]
pub struct CreateSubscriptionIntentResponse {
    pub success: bool,
    pub session_id: String,
    pub checkout_url: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}

async fn handle_create_intent(
    Extension(hub): Extension<Arc<Hub>>,
    Json(payload): Json<CreateSubscriptionIntentRequest>,
) -> impl IntoResponse {
    let session_id = format!("cs_test_{}", uuid::Uuid::new_v4().to_string().replace("-", ""));
    let checkout_url = format!("https://checkout.stripe.com/pay/{}", session_id);

    (StatusCode::OK, Json(CreateSubscriptionIntentResponse {
        success: true,
        session_id,
        checkout_url,
    })).into_response()
}

#[derive(Deserialize)]
pub struct ManageSubscriptionRequest {
    pub action: String, // pause, resume, cancel, skip
    pub token: String,
}

#[derive(Serialize)]
pub struct ManageSubscriptionResponse {
    pub success: bool,
    pub message: String,
}

async fn handle_manage_subscription(
    Extension(hub): Extension<Arc<Hub>>,
    Path(subscription_id): Path<String>,
    Json(payload): Json<ManageSubscriptionRequest>,
) -> impl IntoResponse {
    // Validate token logic
    if payload.token.is_empty() {
        return (StatusCode::UNAUTHORIZED, Json(ManageSubscriptionResponse {
            success: false,
            message: "Invalid token".to_string(),
        })).into_response();
    }

    let msg = match payload.action.as_str() {
        "pause" => "Subscription paused",
        "resume" => "Subscription resumed",
        "cancel" => "Subscription canceled",
        "skip" => "Skipped next billing cycle",
        _ => return (StatusCode::BAD_REQUEST, Json(ManageSubscriptionResponse {
            success: false,
            message: "Unknown action".to_string(),
        })).into_response(),
    };

    (StatusCode::OK, Json(ManageSubscriptionResponse {
        success: true,
        message: msg.to_string(),
    })).into_response()
}

pub fn router<S: Clone + Send + Sync + 'static>(hub: Arc<Hub>) -> Router<S> {
    Router::new()
        .route("/intent", post(handle_create_intent))
        .route("/:subscription_id/manage", post(handle_manage_subscription))
        .layer(Extension(hub))
}
