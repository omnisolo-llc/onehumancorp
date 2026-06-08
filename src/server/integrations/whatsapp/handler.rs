use axum::{routing::{get, post}, Router};
use super::webhook::{verify_webhook, handle_webhook};

pub fn whatsapp_routes() -> Router {
    Router::new()
        .route("/webhook", get(verify_webhook))
        .route("/webhook", post(handle_webhook))
}
