use axum::{
    routing::post,
    Router,
    extract::{State, Json},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ConnectWhatsAppCloudRequest {
    pub integration_id: String,
}

#[derive(Deserialize)]
pub struct ConnectWhatsAppTwilioRequest {
    pub integration_id: String,
    pub bot_token: String,
    pub api_token: String,
    pub from_phone: String,
    pub base_url: String,
}

pub fn router<S: Clone + Send + Sync + 'static>() -> Router<S> {
    Router::new()
        .route("/whatsapp_cloud_api/connect", post(whatsapp_cloud_api_handler))
        .route("/whatsapp/connect", post(whatsapp_handler))
        .route("/whatsapp_cloud_api", post(whatsapp_cloud_api_handler))
        .route("/whatsapp", post(whatsapp_handler))
}

async fn whatsapp_cloud_api_handler(Json(_payload): Json<ConnectWhatsAppCloudRequest>) -> impl IntoResponse {
    (StatusCode::OK, axum::Json(serde_json::json!({"success": true})))
}

async fn whatsapp_handler(Json(_payload): Json<ConnectWhatsAppTwilioRequest>) -> impl IntoResponse {
    (StatusCode::OK, axum::Json(serde_json::json!({"success": true})))
}
