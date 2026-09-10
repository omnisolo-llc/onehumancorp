use std::sync::Arc;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use super::models::WhatsAppMessage;

pub struct WhatsAppClient {
    http_client: Client,
    access_token: String,
}

impl WhatsAppClient {
    pub fn new(access_token: String) -> Self {
        Self {
            http_client: Client::new(),
            access_token,
        }
    }

    pub async fn send_message(&self, phone_number_id: &str, to: &str, text: &str) -> Result<(), reqwest::Error> {
        let url = format!("https://graph.facebook.com/v19.0/{}/messages", phone_number_id);
        let payload = serde_json::json!({
            "messaging_product": "whatsapp",
            "to": to,
            "type": "text",
            "text": {
                "body": text
            }
        });

        self.http_client.post(&url)
            .bearer_auth(&self.access_token)
            .json(&payload)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }
}
