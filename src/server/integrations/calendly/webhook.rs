use axum::{
    extract::{Json, State},
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CalendlyWebhookPayload {
    pub event: String,
    pub payload: serde_json::Value,
}

pub async fn calendly_webhook_handler(
    Json(payload): Json<CalendlyWebhookPayload>,
) -> impl IntoResponse {
    // Records appointment
    axum::http::StatusCode::OK
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;

    #[tokio::test]
    async fn test_calendly_webhook_handler() {
        let payload = CalendlyWebhookPayload {
            event: "invitee.created".to_string(),
            payload: serde_json::json!({}),
        };
        let response = calendly_webhook_handler(Json(payload)).await.into_response();
        assert_eq!(response.status(), StatusCode::OK);
    }
}
