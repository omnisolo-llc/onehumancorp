use reqwest::Client;
use async_trait::async_trait;

#[async_trait]
pub trait SendGridClientWrapper: Send + Sync {
    async fn send_email(&self, to: &str, subject: &str, content: &str) -> Result<(), String>;
}

pub struct RealSendGridClient {
    api_key: String,
    http_client: Client,
}

impl RealSendGridClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            http_client: Client::new(),
        }
    }
}

#[async_trait]
impl SendGridClientWrapper for RealSendGridClient {
    async fn send_email(&self, to: &str, subject: &str, content: &str) -> Result<(), String> {
        let url = "https://api.sendgrid.com/v3/mail/send";
        let res = self.http_client.post(url)
            .bearer_auth(&self.api_key)
            .json(&serde_json::json!({
                "personalizations": [{ "to": [{ "email": to }] }],
                "from": { "email": "no-reply@onehumancorp.com" },
                "subject": subject,
                "content": [{ "type": "text/plain", "value": content }]
            }))
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    Ok(())
                } else {
                    Err(format!("SendGrid API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}
