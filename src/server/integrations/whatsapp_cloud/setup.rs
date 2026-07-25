use axum::{
    extract::{State, Json},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use reqwest::Client;

pub struct WebhookSetupState {
    pub client: Client,
    pub access_token: String,
    pub app_id: String,
}

#[derive(Deserialize)]
pub struct WebhookSetupRequest {
    pub callback_url: String,
    pub verify_token: String,
}

pub async fn setup_webhook(
    State(state): State<Arc<WebhookSetupState>>,
    Json(payload): Json<WebhookSetupRequest>,
) -> impl IntoResponse {
    let url = format!("https://graph.facebook.com/v19.0/{}/subscriptions", state.app_id);

    let res = state.client.post(&url)
        .bearer_auth(&state.access_token)
        .form(&[
            ("object", "whatsapp_business_account"),
            ("callback_url", &payload.callback_url),
            ("verify_token", &payload.verify_token),
            ("fields", "messages,message_template_status_update"),
        ])
        .send()
        .await;

    match res {
        Ok(response) => {
            if response.status().is_success() {
                StatusCode::OK.into_response()
            } else {
                let err_text = response.text().await.unwrap_or_default();
                tracing::error!("Failed to setup webhook: {}", err_text);
                (StatusCode::INTERNAL_SERVER_ERROR, err_text).into_response()
            }
        }
        Err(e) => {
            tracing::error!("Failed to setup webhook (reqwest error): {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        }
    }
}
