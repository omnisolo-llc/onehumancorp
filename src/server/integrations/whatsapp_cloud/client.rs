use async_trait::async_trait;
use serde_json::json;
use std::sync::OnceLock;

#[async_trait]
pub trait WhatsAppCloudClientWrapper: Send + Sync {
    async fn send_message(&self, to: &str, body: &str) -> Result<(), String>;
    async fn send_template_message(&self, to: &str, template: serde_json::Value) -> Result<(), String>;
    async fn send_interactive_message(&self, to: &str, interactive: serde_json::Value) -> Result<(), String>;
    async fn send_media_message(&self, to: &str, media_type: &str, media: serde_json::Value) -> Result<(), String>;
}

pub struct RealWhatsAppCloudClient {
    phone_number_id: String,
    access_token: String,
}

impl RealWhatsAppCloudClient {
    pub fn new(phone_number_id: String, access_token: String) -> Self {
        Self {
            phone_number_id,
            access_token,
        }
    }

    async fn send_payload(&self, payload: &serde_json::Value) -> Result<(), String> {
        let url = format!(
            "https://graph.facebook.com/v19.0/{}/messages",
            self.phone_number_id
        );

        let client = HTTP_CLIENT.get_or_init(reqwest::Client::new);
        let res = client
            .post(&url)
            .bearer_auth(&self.access_token)
            .json(payload)
            .send()
            .await;

        match res {
            Ok(response) => {
                if response.status().is_success() {
                    Ok(())
                } else {
                    let err_text = response.text().await.unwrap_or_default();
                    Err(format!("WhatsApp API error: {}", err_text))
                }
            }
            Err(e) => Err(format!("Reqwest error: {}", e)),
        }
    }
}

static HTTP_CLIENT: OnceLock<reqwest::Client> = OnceLock::new();

#[async_trait]
impl WhatsAppCloudClientWrapper for RealWhatsAppCloudClient {
    async fn send_message(&self, to: &str, body: &str) -> Result<(), String> {
        let payload = json!({
            "messaging_product": "whatsapp",
            "to": to,
            "type": "text",
            "text": {
                "body": body
            }
        });
        self.send_payload(&payload).await
    }

    async fn send_template_message(&self, to: &str, template: serde_json::Value) -> Result<(), String> {
        let payload = json!({
            "messaging_product": "whatsapp",
            "to": to,
            "type": "template",
            "template": template
        });
        self.send_payload(&payload).await
    }

    async fn send_interactive_message(&self, to: &str, interactive: serde_json::Value) -> Result<(), String> {
        let payload = json!({
            "messaging_product": "whatsapp",
            "to": to,
            "type": "interactive",
            "interactive": interactive
        });
        self.send_payload(&payload).await
    }

    async fn send_media_message(&self, to: &str, media_type: &str, media: serde_json::Value) -> Result<(), String> {
        if !["image", "document", "video"].contains(&media_type) {
            return Err("Unsupported media type".to_string());
        }
        let mut payload = json!({
            "messaging_product": "whatsapp",
            "to": to,
            "type": media_type,
        });

        // Ensure it's an object before mutating
        if let Some(obj) = payload.as_object_mut() {
            obj.insert(media_type.to_string(), media);
        } else {
             return Err("Invalid payload structure".to_string());
        }

        self.send_payload(&payload).await
    }
}
