use async_trait::async_trait;
use serde_json::json;
use std::sync::OnceLock;
use regex::Regex;

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
}

static HTTP_CLIENT: OnceLock<reqwest::Client> = OnceLock::new();
static BSUID_REGEX: OnceLock<Regex> = OnceLock::new();

pub fn build_message_payload(to: &str, body: &str) -> serde_json::Value {
    let re = BSUID_REGEX.get_or_init(|| Regex::new(r"^[A-Z]{2}\.(?:ENT\.)?[A-Za-z0-9]{1,128}$").unwrap());
    if re.is_match(to) {
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

#[async_trait]
impl WhatsAppCloudClientWrapper for RealWhatsAppCloudClient {
    async fn send_message(&self, to: &str, body: &str) -> Result<(), String> {
        let url = format!(
            "https://graph.facebook.com/v19.0/{}/messages",
            self.phone_number_id
        );

        let payload = build_message_payload(to, body);

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
