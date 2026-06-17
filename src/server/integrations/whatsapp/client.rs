use async_trait::async_trait;
use reqwest::Client;

#[async_trait]
pub trait WhatsAppClientWrapper: Send + Sync {
    async fn send_message(&self, phone_number_id: &str, to: &str, body: &str) -> Result<(), String>;
}

pub struct RealWhatsAppClient {
    access_token: String,
    http_client: Client,
}

impl RealWhatsAppClient {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            http_client: Client::new(),
        }
    }
}

#[async_trait]
impl WhatsAppClientWrapper for RealWhatsAppClient {
    async fn send_message(&self, phone_number_id: &str, to: &str, body: &str) -> Result<(), String> {
        let url = format!("https://graph.facebook.com/v19.0/{}/messages", phone_number_id);

        let payload = serde_json::json!({
            "messaging_product": "whatsapp",
            "recipient_type": "individual",
            "to": to,
            "type": "text",
            "text": {
                "preview_url": false,
                "body": body
            }
        });

        let res = self.http_client.post(&url)
            .bearer_auth(&self.access_token)
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    Ok(())
                } else {
                    let status = resp.status();
                    let err_text = resp.text().await.unwrap_or_default();
                    Err(format!("WhatsApp API error: {} - {}", status, err_text))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}
