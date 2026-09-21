pub mod google_calendar_webhook;

use axum::Router;
use google_calendar_webhook::google_calendar_webhook_handler;
use axum::routing::post;

pub fn routes<S>() -> Router<S> where S: Clone + Send + Sync + 'static {
    Router::new()
        .route("/webhooks/google-calendar", post(google_calendar_webhook_handler))
}
