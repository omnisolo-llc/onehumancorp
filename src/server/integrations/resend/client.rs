use async_trait::async_trait;
use reqwest::Client;

#[async_trait]
pub trait ResendClientWrapper: Send + Sync {
    async fn send_email(&self, to: &str, from: &str, subject: &str, html_body: &str) -> Result<(), String>;
}

pub struct RealResendClient {
    api_key: String,
    http_client: Client,
}

impl RealResendClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            http_client: Client::new(),
        }
    }
}

#[async_trait]
impl ResendClientWrapper for RealResendClient {
    async fn send_email(&self, to: &str, from: &str, subject: &str, html_body: &str) -> Result<(), String> {
        let url = "https://api.resend.com/emails";

        let payload = serde_json::json!({
            "from": from,
            "to": [to],
            "subject": subject,
            "html": html_body
        });

        let res = self.http_client.post(url)
            .bearer_auth(&self.api_key)
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    Ok(())
                } else {
                    Err(format!("Resend API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}
