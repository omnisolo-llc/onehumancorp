pub mod google_calendar_webhook;

use axum::Router;
use axum::routing::post;
use google_calendar_webhook::google_calendar_webhook_handler;

pub fn routes<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new().route(
        "/webhooks/google-calendar",
        post(google_calendar_webhook_handler),
    )
}
