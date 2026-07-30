use axum::{
    extract::{State, Query},
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use std::sync::Arc;
use super::routes::AppState;

pub fn create_webhook_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/webhooks/meta", get(verify_webhook).post(handle_webhook))
        .with_state(state)
}

#[derive(Deserialize)]
pub struct VerifyWebhookQuery {
    #[serde(rename = "hub.mode")]
    pub mode: Option<String>,
    #[serde(rename = "hub.verify_token")]
    pub verify_token: Option<String>,
    #[serde(rename = "hub.challenge")]
    pub challenge: Option<String>,
}

async fn verify_webhook(
    Query(query): Query<VerifyWebhookQuery>,
) -> String {
    if let (Some(mode), Some(token), Some(challenge)) = (query.mode, query.verify_token, query.challenge) {
        if mode == "subscribe" && token == "ohc_meta_webhook_secret" {
            return challenge;
        }
    }
    "Invalid Request".to_string()
}

#[derive(Deserialize, Debug)]
pub struct MetaWebhookPayload {
    pub object: String,
    pub entry: Vec<serde_json::Value>,
}

async fn handle_webhook(
    State(_state): State<Arc<AppState>>,
    Json(payload): Json<MetaWebhookPayload>,
) -> &'static str {
    println!("Received Meta webhook: {:?}", payload);
    "EVENT_RECEIVED"
}
