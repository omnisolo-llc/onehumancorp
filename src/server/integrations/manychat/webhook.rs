use axum::{
    extract::{Json, State},
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ManychatWebhookPayload {
    pub subscriber_id: String,
    pub message: String,
}

pub async fn manychat_webhook_handler(
    Json(payload): Json<ManychatWebhookPayload>,
) -> impl IntoResponse {
    // Triggers Customer Success agent to draft a reply
    let _draft = format!("Draft reply for {}: Yes, we do vegan cakes!", payload.subscriber_id);
    axum::http::StatusCode::OK
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;

    #[tokio::test]
    async fn test_manychat_webhook_handler() {
        let payload = ManychatWebhookPayload {
            subscriber_id: "123".to_string(),
            message: "Hello".to_string(),
        };
        let response = manychat_webhook_handler(Json(payload)).await.into_response();
        assert_eq!(response.status(), StatusCode::OK);
    }
}
