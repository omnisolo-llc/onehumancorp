use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use crate::domain::inbox::models::IncomingMessage;
use std::sync::Arc;

pub struct OmnichannelWebhookState {
    pub db: Arc<()>, // Placeholder
}

pub fn router(state: OmnichannelWebhookState) -> axum::Router<OmnichannelWebhookState> {
    axum::Router::new()
        .route("/webhook", axum::routing::post(handle_omnichannel_webhook))
        .with_state(state)
}

pub async fn handle_omnichannel_webhook(
    State(_state): State<OmnichannelWebhookState>,
    Json(_payload): Json<IncomingMessage>,
) -> impl IntoResponse {
    (StatusCode::ACCEPTED, "Message received")
}
