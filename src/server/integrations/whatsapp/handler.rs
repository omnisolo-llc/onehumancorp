use axum::{routing::{get, post}, Router};
use ::server_lib::api::meta_webhook::MetaWebhookState;
use super::webhook::{verify_webhook, handle_webhook};

pub fn whatsapp_routes(state: MetaWebhookState) -> Router {
    Router::new()
        .route("/webhook", get(verify_webhook))
        .route("/webhook", post(handle_webhook))
        .with_state(state)
}
