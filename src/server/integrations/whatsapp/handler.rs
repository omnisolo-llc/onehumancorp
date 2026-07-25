use axum::{routing::{get, post}, Router};
use super::webhook::{verify_webhook, handle_webhook};

pub fn whatsapp_routes() -> Router {
    whatsapp_routes_with_redis(None)
}

pub fn whatsapp_routes_with_redis(redis_client: Option<redis::Client>) -> Router {
    Router::new()
        .route("/webhook", get(verify_webhook))
        .route("/webhook", post(handle_webhook))
        .with_state(redis_client)
}