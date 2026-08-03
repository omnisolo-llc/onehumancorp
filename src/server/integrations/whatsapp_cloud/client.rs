use async_trait::async_trait;
use once_cell::sync::Lazy;
use regex::Regex;
use serde_json::json;
use std::sync::OnceLock;

static BSUID_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[A-Z]{2}\.(?:ENT\.)?[A-Za-z0-9]{1,128}$").unwrap()
});

#[async_trait]
pub trait WhatsAppCloudClientWrapper: Send + Sync {
    async fn send_message(&self, to: &str, body: &str) -> Result<(), String>;
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

    pub fn generate_payload(to: &str, body: &str) -> serde_json::Value {
        if BSUID_REGEX.is_match(to) {
            json!({
                "messaging_product": "whatsapp",
                "recipient_type": "individual",
                "recipient": to,
                "type": "text",
                "text": {
                    "body": body
                }
            })
        } else {
            json!({
                "messaging_product": "whatsapp",
                "to": to,
                "type": "text",
                "text": {
                    "body": body
                }
            })
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

        let payload = Self::generate_payload(to, body);

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
