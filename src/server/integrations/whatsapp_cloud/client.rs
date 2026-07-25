use async_trait::async_trait;
use serde_json::json;
use std::sync::OnceLock;

use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct MessagePayload {
    pub messaging_product: String,
    pub to: String,
    #[serde(rename = "type")]
    pub msg_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interactive: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<serde_json::Value>,
}

#[async_trait]
pub trait WhatsAppCloudClientWrapper: Send + Sync {
    async fn send_message(&self, to: &str, body: &str) -> Result<(), String>;
    async fn send_custom_message(&self, payload: &MessagePayload) -> Result<(), String>;
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
        let payload = MessagePayload {
            messaging_product: "whatsapp".to_string(),
            to: to.to_string(),
            msg_type: "text".to_string(),
            text: Some(json!({ "body": body })),
            template: None,
            interactive: None,
            image: None,
            audio: None,
            video: None,
            document: None,
            location: None,
        };
        self.send_custom_message(&payload).await
    }

    async fn send_custom_message(&self, payload: &MessagePayload) -> Result<(), String> {
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
}
