use axum::{routing::{get, post}, Router};
use super::webhook::{verify_webhook, handle_webhook};

use std::sync::Arc;

#[derive(Clone)]
pub struct WhatsAppState {
    pub redis_client: redis::Client,
    pub access_token: String,
    pub phone_number_id: String,
}

pub fn whatsapp_routes(state: Arc<WhatsAppState>) -> Router {
    Router::new()
        .route("/webhook", get(verify_webhook))
        .route("/webhook", post(handle_webhook))
        .with_state(state)
}
