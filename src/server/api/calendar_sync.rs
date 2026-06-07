use axum::{routing::post, Router};
use std::sync::Arc;

pub fn router() -> Router<Arc<()>> {
    Router::new()
        .route("/bookings/webhook", post(handle_webhook))
}

async fn handle_webhook() -> &'static str {
    "Webhook received"
}
