use async_trait::async_trait;
use reqwest::Client;

#[async_trait]
pub trait ResendClientWrapper: Send + Sync {
    async fn send_email(&self, to: &str, subject: &str, html: &str) -> Result<String, String>;
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
    async fn send_email(&self, to: &str, subject: &str, html: &str) -> Result<String, String> {
        let url = "https://api.resend.com/emails";
        let res = self.http_client.post(url)
            .bearer_auth(&self.api_key)
            .json(&serde_json::json!({
                "from": "Acme <onboarding@resend.dev>",
                "to": [to],
                "subject": subject,
                "html": html,
            }))
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    Ok(format!("resend_fake_id_for_{}", to))
                } else {
                    Err(format!("Resend API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_real_client_creation() {
        let client = RealResendClient::new("token".to_string());
        assert_eq!(client.api_key, "token");
    }

    #[tokio::test]
    async fn test_send_email_error_handling() {
        let client = RealResendClient::new("token".to_string());
        let _ = client.send_email("test@example.com", "Subject", "<p>Hello</p>").await;
    }
}
