use async_trait::async_trait;
use serde_json::Value;

use super::ChannelAdapter;

pub struct WhatsAppAdapter {
    pub access_token: String,
    pub phone_number_id: String,
}

#[async_trait]
impl ChannelAdapter for WhatsAppAdapter {
    async fn handle_webhook(&self, _payload: Value) -> Result<(), String> {
        // Parse WhatsApp webhook payload
        // Create DB records (Contact, Conversation, Message)
        // Trigger AI Agent (via job queue or direct call)
        Ok(())
    }

    async fn send_message(&self, to: &str, content: &str) -> Result<(), String> {
        let client = reqwest::Client::new();
        let url = format!("https://graph.facebook.com/v17.0/{}/messages", self.phone_number_id);

        let payload = serde_json::json!({
            "messaging_product": "whatsapp",
            "to": to,
            "type": "text",
            "text": {
                "body": content
            }
        });

        let res = client.post(&url)
            .bearer_auth(&self.access_token)
            .json(&payload)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if res.status().is_success() {
            Ok(())
        } else {
            let error_text = res.text().await.unwrap_or_default();
            Err(format!("WhatsApp API error: {}", error_text))
        }
    }
}
