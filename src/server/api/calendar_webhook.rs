use crate::db::DB;
use axum::{
    extract::{Json, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct CalendarWebhookState {
    pub db: Arc<DB>,
}

pub async fn google_calendar_webhook_handler(
    State(_state): State<CalendarWebhookState>,
    headers: HeaderMap,
    Json(_payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    let _channel_id = headers
        .get("X-Goog-Channel-ID")
        .and_then(|h| h.to_str().ok());
    let _resource_state = headers
        .get("X-Goog-Resource-State")
        .and_then(|h| h.to_str().ok());
    let channel_token = headers
        .get("X-Goog-Channel-Token")
        .and_then(|h| h.to_str().ok());

    if let Some(token) = channel_token {
        tracing::info!(
            "Received Google Calendar webhook for tenant/token: {}",
            token
        );
        // We acknowledge the webhook quickly.
        // A background worker or the `calendar_sync` loop (which polls every 5 minutes)
        // will naturally reconcile state, or we could enqueue a job here.
        // As per the verified trace, we don't have a direct enqueue trigger, so we rely
        // on the periodic `calendar_sync` and direct DB updates if needed.
    } else {
        tracing::warn!("Received Google Calendar webhook without X-Goog-Channel-Token");
    }

    StatusCode::OK
}
