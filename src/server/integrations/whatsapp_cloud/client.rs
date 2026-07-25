use async_trait::async_trait;
use serde_json::json;
use std::sync::OnceLock;

#[async_trait]
pub trait WhatsAppCloudClientWrapper: Send + Sync {
    async fn send_message(&self, to: &str, body: &str) -> Result<(), String>;
    async fn send_template(&self, to: &str, template_name: &str, language_code: &str, components: serde_json::Value) -> Result<(), String>;
    async fn send_interactive_message(&self, to: &str, interactive_payload: serde_json::Value) -> Result<(), String>;
    async fn send_media_message(&self, to: &str, media_type: &str, media_id_or_link: &str, caption: Option<&str>, filename: Option<&str>) -> Result<(), String>;
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

    async fn post_payload(&self, url: &str, payload: &serde_json::Value) -> Result<(), String> {
        let client = HTTP_CLIENT.get_or_init(reqwest::Client::new);
        let res = client
            .post(url)
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
        let url = format!(
            "https://graph.facebook.com/v19.0/{}/messages",
            self.phone_number_id
        );

        let payload = json!({
            "messaging_product": "whatsapp",
            "to": to,
            "type": "text",
            "text": {
                "body": body
            }
        });

        self.post_payload(&url, &payload).await
    }

    async fn send_template(&self, to: &str, template_name: &str, language_code: &str, components: serde_json::Value) -> Result<(), String> {
        let url = format!(
            "https://graph.facebook.com/v19.0/{}/messages",
            self.phone_number_id
        );

        let mut template = json!({
            "name": template_name,
            "language": {
                "code": language_code,
                "policy": "deterministic"
            }
        });

        if !components.is_null() {
            template.as_object_mut().unwrap().insert("components".to_string(), components);
        }

        let payload = json!({
            "messaging_product": "whatsapp",
            "recipient_type": "individual",
            "to": to,
            "type": "template",
            "template": template
        });

        self.post_payload(&url, &payload).await
    }

    async fn send_interactive_message(&self, to: &str, interactive_payload: serde_json::Value) -> Result<(), String> {
        let url = format!(
            "https://graph.facebook.com/v19.0/{}/messages",
            self.phone_number_id
        );

        let payload = json!({
            "messaging_product": "whatsapp",
            "to": to,
            "type": "interactive",
            "interactive": interactive_payload
        });

        self.post_payload(&url, &payload).await
    }

    async fn send_media_message(&self, to: &str, media_type: &str, media_id_or_link: &str, caption: Option<&str>, filename: Option<&str>) -> Result<(), String> {
        let url = format!(
            "https://graph.facebook.com/v19.0/{}/messages",
            self.phone_number_id
        );

        let mut media_payload = json!({});
        if media_id_or_link.starts_with("http") {
            media_payload.as_object_mut().unwrap().insert("link".to_string(), json!(media_id_or_link));
        } else {
            media_payload.as_object_mut().unwrap().insert("id".to_string(), json!(media_id_or_link));
        }

        if let Some(c) = caption {
            media_payload.as_object_mut().unwrap().insert("caption".to_string(), json!(c));
        }

        if media_type == "document" {
            if let Some(f) = filename {
                media_payload.as_object_mut().unwrap().insert("filename".to_string(), json!(f));
            }
        }

        let payload = json!({
            "messaging_product": "whatsapp",
            "to": to,
            "type": media_type,
            media_type: media_payload
        });

        self.post_payload(&url, &payload).await
    }
}
