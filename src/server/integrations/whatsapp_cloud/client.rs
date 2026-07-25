use async_trait::async_trait;
use serde_json::json;
use std::sync::OnceLock;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InteractiveButton {
    #[serde(rename = "type")]
    pub button_type: String, // typically "reply"
    pub reply: InteractiveButtonReply,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InteractiveButtonReply {
    pub id: String,
    pub title: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InteractiveAction {
    pub buttons: Vec<InteractiveButton>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InteractiveMessage {
    #[serde(rename = "type")]
    pub interactive_type: String, // typically "button"
    pub body: InteractiveBody,
    pub action: InteractiveAction,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InteractiveBody {
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MediaPayload {
    pub link: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LocationPayload {
    pub latitude: f64,
    pub longitude: f64,
    pub name: Option<String>,
    pub address: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TemplateLanguage {
    pub code: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TemplatePayload {
    pub name: String,
    pub language: TemplateLanguage,
}

#[async_trait]
pub trait WhatsAppCloudClientWrapper: Send + Sync {
    async fn send_message(&self, to: &str, body: &str) -> Result<(), String>;
    async fn send_template_message(&self, to: &str, template_name: &str, lang_code: &str) -> Result<(), String>;
    async fn send_interactive_message(&self, to: &str, interactive: &InteractiveMessage) -> Result<(), String>;
    async fn send_media_message(&self, to: &str, media_type: &str, media_link: &str) -> Result<(), String>;
    async fn send_location_message(&self, to: &str, location: &LocationPayload) -> Result<(), String>;
    async fn register_phone_number(&self, pin: &str) -> Result<(), String>;
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

    async fn register_phone_number(&self, pin: &str) -> Result<(), String> {
        let url = format!(
            "https://graph.facebook.com/v19.0/{}/register",
            self.phone_number_id
        );

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

    async fn send_template_message(&self, to: &str, template_name: &str, lang_code: &str) -> Result<(), String> {
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
                    "code": lang_code
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

    async fn send_interactive_message(&self, to: &str, interactive: &InteractiveMessage) -> Result<(), String> {
        let url = format!(
            "https://graph.facebook.com/v19.0/{}/messages",
            self.phone_number_id
        );

        let payload = json!({
            "messaging_product": "whatsapp",
            "to": to,
            "type": "interactive",
            "interactive": interactive
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

    async fn send_media_message(&self, to: &str, media_type: &str, media_link: &str) -> Result<(), String> {
        let url = format!(
            "https://graph.facebook.com/v19.0/{}/messages",
            self.phone_number_id
        );

        let mut payload = json!({
            "messaging_product": "whatsapp",
            "to": to,
            "type": media_type,
        });

        if let Some(obj) = payload.as_object_mut() {
            obj.insert(media_type.to_string(), json!({ "link": media_link }));
        }

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

    async fn send_location_message(&self, to: &str, location: &LocationPayload) -> Result<(), String> {
        let url = format!(
            "https://graph.facebook.com/v19.0/{}/messages",
            self.phone_number_id
        );

        let payload = json!({
            "messaging_product": "whatsapp",
            "to": to,
            "type": "location",
            "location": location
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
