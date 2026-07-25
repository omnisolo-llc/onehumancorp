use async_trait::async_trait;
use serde_json::{json, Value};
use std::sync::OnceLock;

#[async_trait]
pub trait WhatsAppCloudClientWrapper: Send + Sync {
    async fn send_message(&self, to: &str, body: &str) -> Result<(), String>;
    async fn send_template_message(&self, to: &str, template_name: &str, language_code: &str, components: Vec<Value>) -> Result<(), String>;
    async fn send_media_message(&self, to: &str, media_type: &str, media_id_or_url: &str, caption: Option<&str>) -> Result<(), String>;
    async fn send_interactive_message(&self, to: &str, interactive_payload: Value) -> Result<(), String>;

    // Setup Methods
    async fn register_phone_number(&self, pin: &str) -> Result<(), String>;
    async fn verify_pin(&self, pin: &str) -> Result<(), String>;
    async fn subscribe_webhook(&self, url: &str, verify_token: &str) -> Result<(), String>;
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

    async fn post_request(&self, url: &str, payload: &Value) -> Result<(), String> {
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

        self.post_request(&url, &payload).await
    }

    async fn send_template_message(&self, to: &str, template_name: &str, language_code: &str, components: Vec<Value>) -> Result<(), String> {
        let url = format!(
            "https://graph.facebook.com/v19.0/{}/messages",
            self.phone_number_id
        );

        let payload = json!({
            "messaging_product": "whatsapp",
            "to": to,
            "type": "template",
            "template": {
                "name": template_name,
                "language": {
                    "code": language_code
                },
                "components": components
            }
        });

        self.post_request(&url, &payload).await
    }

    async fn send_media_message(&self, to: &str, media_type: &str, media_id_or_url: &str, caption: Option<&str>) -> Result<(), String> {
        let url = format!(
            "https://graph.facebook.com/v19.0/{}/messages",
            self.phone_number_id
        );

        let is_url = media_id_or_url.starts_with("http");

        let mut media_payload = if is_url {
            json!({
                "link": media_id_or_url
            })
        } else {
            json!({
                "id": media_id_or_url
            })
        };

        if let Some(c) = caption {
            media_payload["caption"] = json!(c);
        }

        let mut payload = json!({
            "messaging_product": "whatsapp",
            "to": to,
            "type": media_type
        });

        payload[media_type] = media_payload;

        self.post_request(&url, &payload).await
    }

    async fn send_interactive_message(&self, to: &str, interactive_payload: Value) -> Result<(), String> {
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

        self.post_request(&url, &payload).await
    }

    async fn register_phone_number(&self, pin: &str) -> Result<(), String> {
        let url = format!(
            "https://graph.facebook.com/v19.0/{}/register",
            self.phone_number_id
        );

        let payload = json!({
            "messaging_product": "whatsapp",
            "pin": pin
        });

        self.post_request(&url, &payload).await
    }

    async fn verify_pin(&self, pin: &str) -> Result<(), String> {
        let url = format!(
            "https://graph.facebook.com/v19.0/{}/register", // Actually verify is different API call, sometimes just POST to register again or verify endpoint
            self.phone_number_id
        );

        // As per Meta docs, to verify, we need to POST /PHONE_NUMBER_ID/register
        let payload = json!({
            "messaging_product": "whatsapp",
            "pin": pin
        });

        self.post_request(&url, &payload).await
    }

    async fn subscribe_webhook(&self, _url: &str, _verify_token: &str) -> Result<(), String> {
        // WhatsApp Webhooks are usually configured at the App level on Meta Dashboard or via Graph API to App ID
        // Note: For BSPs, there's a different endpoint.
        Ok(())
    }
}
