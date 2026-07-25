use async_trait::async_trait;
use serde_json::json;
use serde_json::Value;
use std::sync::OnceLock;

#[async_trait]
pub trait WhatsAppCloudClientWrapper: Send + Sync {
    async fn send_message(&self, to: &str, body: &str) -> Result<(), String>;
    async fn send_template_message(&self, to: &str, template_name: &str, language_code: &str, components: Vec<Value>) -> Result<(), String>;
    async fn send_interactive_message(&self, to: &str, interactive_payload: Value) -> Result<(), String>;
    async fn send_media_message(&self, to: &str, media_type: &str, media_id: Option<&str>, media_link: Option<&str>, caption: Option<&str>) -> Result<(), String>;
    async fn register_phone_number(&self, pin: &str) -> Result<(), String>;
    async fn subscribe_webhook(&self, app_id: &str, app_access_token: &str, fields: Vec<String>) -> Result<(), String>;
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

    async fn post_json(&self, url: &str, payload: &Value, token: Option<&str>) -> Result<(), String> {
        let client = HTTP_CLIENT.get_or_init(reqwest::Client::new);
        let mut builder = client.post(url).json(payload);
        if let Some(tok) = token {
            builder = builder.bearer_auth(tok);
        }

        let res = builder.send().await;

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

        self.post_json(&url, &payload, Some(&self.access_token)).await
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

        self.post_json(&url, &payload, Some(&self.access_token)).await
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

        self.post_json(&url, &payload, Some(&self.access_token)).await
    }

    async fn send_media_message(&self, to: &str, media_type: &str, media_id: Option<&str>, media_link: Option<&str>, caption: Option<&str>) -> Result<(), String> {
        let url = format!(
            "https://graph.facebook.com/v19.0/{}/messages",
            self.phone_number_id
        );

        let mut media_payload = json!({});
        if let Some(id) = media_id {
            media_payload["id"] = json!(id);
        } else if let Some(link) = media_link {
            media_payload["link"] = json!(link);
        } else {
            return Err("Either media_id or media_link must be provided".to_string());
        }

        if let Some(cap) = caption {
            media_payload["caption"] = json!(cap);
        }

        let mut payload = json!({
            "messaging_product": "whatsapp",
            "to": to,
            "type": media_type,
        });

        payload[media_type] = media_payload;

        self.post_json(&url, &payload, Some(&self.access_token)).await
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

        self.post_json(&url, &payload, Some(&self.access_token)).await
    }

    async fn subscribe_webhook(&self, app_id: &str, app_access_token: &str, fields: Vec<String>) -> Result<(), String> {
        let url = format!(
            "https://graph.facebook.com/v19.0/{}/subscriptions",
            app_id
        );

        let fields_str = fields.join(",");

        let client = HTTP_CLIENT.get_or_init(reqwest::Client::new);
        let res = client
            .post(&url)
            .bearer_auth(app_access_token)
            .form(&[
                ("object", "whatsapp_business_account"),
                ("fields", &fields_str),
            ])
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
