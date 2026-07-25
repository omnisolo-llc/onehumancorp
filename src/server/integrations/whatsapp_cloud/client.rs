use async_trait::async_trait;
use serde_json::json;
use std::sync::OnceLock;

#[async_trait]
pub trait WhatsAppCloudClientWrapper: Send + Sync {
    async fn send_message(&self, to: &str, body: &str) -> Result<(), String>;
    async fn send_template(&self, to: &str, template_name: &str, language_code: &str) -> Result<(), String>;
    async fn send_interactive(&self, to: &str, interactive_payload: serde_json::Value) -> Result<(), String>;
    async fn send_media(&self, to: &str, media_type: &str, media_id: Option<&str>, media_link: Option<&str>, caption: Option<&str>) -> Result<(), String>;
    async fn send_location(&self, to: &str, lat: f64, long: f64, name: Option<&str>, address: Option<&str>) -> Result<(), String>;

    // Webhook and setup methods
    async fn register_phone_number(&self, pin: &str) -> Result<(), String>;
    async fn subscribe_phone_number_webhook(&self, override_callback_uri: &str, verify_token: &str) -> Result<(), String>;
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

    async fn send_template(&self, to: &str, template_name: &str, language_code: &str) -> Result<(), String> {
        let url = format!("https://graph.facebook.com/v19.0/{}/messages", self.phone_number_id);

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

    async fn send_interactive(&self, to: &str, interactive_payload: serde_json::Value) -> Result<(), String> {
        let url = format!("https://graph.facebook.com/v19.0/{}/messages", self.phone_number_id);

        let payload = json!({
            "messaging_product": "whatsapp",
            "to": to,
            "type": "interactive",
            "interactive": interactive_payload
        });

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

    async fn send_media(&self, to: &str, media_type: &str, media_id: Option<&str>, media_link: Option<&str>, caption: Option<&str>) -> Result<(), String> {
        let url = format!("https://graph.facebook.com/v19.0/{}/messages", self.phone_number_id);

        let mut media_payload = json!({});
        if let Some(id) = media_id {
            media_payload["id"] = json!(id);
        } else if let Some(link) = media_link {
            media_payload["link"] = json!(link);
        } else {
            return Err("Must provide either media_id or media_link".to_string());
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

    async fn send_location(&self, to: &str, lat: f64, long: f64, name: Option<&str>, address: Option<&str>) -> Result<(), String> {
        let url = format!("https://graph.facebook.com/v19.0/{}/messages", self.phone_number_id);

        let mut loc_payload = json!({
            "latitude": lat,
            "longitude": long,
        });

        if let Some(n) = name {
            loc_payload["name"] = json!(n);
        }
        if let Some(a) = address {
            loc_payload["address"] = json!(a);
        }

        let payload = json!({
            "messaging_product": "whatsapp",
            "to": to,
            "type": "location",
            "location": loc_payload
        });

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

    async fn register_phone_number(&self, pin: &str) -> Result<(), String> {
        let url = format!("https://graph.facebook.com/v19.0/{}/register", self.phone_number_id);

        let payload = json!({
            "messaging_product": "whatsapp",
            "pin": pin
        });

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

    async fn subscribe_phone_number_webhook(&self, override_callback_uri: &str, verify_token: &str) -> Result<(), String> {
        let url = format!("https://graph.facebook.com/v19.0/{}", self.phone_number_id);

        let payload = json!({
            "webhook_configuration": {
                "override_callback_uri": override_callback_uri,
                "verify_token": verify_token
            }
        });

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
