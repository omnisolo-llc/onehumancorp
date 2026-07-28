use async_trait::async_trait;
use serde_json::json;
use std::sync::OnceLock;

#[async_trait]
pub trait WhatsAppCloudClientWrapper: Send + Sync {
    async fn send_message(&self, to: &str, body: &str) -> Result<(), String>;
    async fn send_template(&self, to: &str, template_name: &str, language_code: &str) -> Result<(), String>;
    async fn send_media(&self, to: &str, media_type: &str, media_id_or_url: &str, caption: Option<&str>) -> Result<(), String>;
    async fn send_interactive_buttons(&self, to: &str, body_text: &str, buttons: Vec<(&str, &str)>) -> Result<(), String>;
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

    async fn send_payload(&self, payload: serde_json::Value) -> Result<(), String> {
        let url = format!(
            "https://graph.facebook.com/v19.0/{}/messages",
            self.phone_number_id
        );

        let client = HTTP_CLIENT.get_or_init(reqwest::Client::new);
        let res = client
            .post(&url)
            .bearer_auth(&self.access_token)
            .json(&payload)
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
        self.send_payload(payload).await
    }

    async fn send_template(&self, to: &str, template_name: &str, language_code: &str) -> Result<(), String> {
        let payload = json!({
            "messaging_product": "whatsapp",
            "to": to,
            "type": "template",
            "template": {
                "name": template_name,
                "language": {
                    "code": language_code
                }
            }
        });
        self.send_payload(payload).await
    }

    async fn send_media(&self, to: &str, media_type: &str, media_id_or_url: &str, caption: Option<&str>) -> Result<(), String> {
        let is_url = media_id_or_url.starts_with("http");
        let media_obj = if is_url {
            if let Some(c) = caption {
                json!({ "link": media_id_or_url, "caption": c })
            } else {
                json!({ "link": media_id_or_url })
            }
        } else {
            if let Some(c) = caption {
                json!({ "id": media_id_or_url, "caption": c })
            } else {
                json!({ "id": media_id_or_url })
            }
        };

        let mut payload = json!({
            "messaging_product": "whatsapp",
            "to": to,
            "type": media_type,
        });
        payload[media_type] = media_obj;

        self.send_payload(payload).await
    }

    async fn send_interactive_buttons(&self, to: &str, body_text: &str, buttons: Vec<(&str, &str)>) -> Result<(), String> {
        let buttons_json: Vec<_> = buttons.into_iter().map(|(id, title)| {
            json!({
                "type": "reply",
                "reply": {
                    "id": id,
                    "title": title
                }
            })
        }).collect();

        let payload = json!({
            "messaging_product": "whatsapp",
            "to": to,
            "type": "interactive",
            "interactive": {
                "type": "button",
                "body": {
                    "text": body_text
                },
                "action": {
                    "buttons": buttons_json
                }
            }
        });
        self.send_payload(payload).await
    }
}
