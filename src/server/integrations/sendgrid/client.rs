use async_trait::async_trait;
use reqwest::Client;

#[async_trait]
pub trait SendGridClientWrapper: Send + Sync {
    async fn send_email(&self, to: &str, subject: &str, body: &str) -> Result<(), String>;
}

pub struct RealSendGridClient {
    pub api_key: String,
    http_client: Client,
}

impl RealSendGridClient {
    pub fn new(api_key: String) -> Self {
        Self { api_key, http_client: Client::new() }
    }
}

#[async_trait]
impl SendGridClientWrapper for RealSendGridClient {
    async fn send_email(&self, to: &str, subject: &str, body: &str) -> Result<(), String> {
        let url = "https://api.sendgrid.com/v3/mail/send";
        let res = self.http_client.post(url)
            .bearer_auth(&self.api_key)
            .json(&serde_json::json!({
                "personalizations": [{
                    "to": [{"email": to}]
                }],
                "from": {"email": "no-reply@example.com"},
                "subject": subject,
                "content": [{"type": "text/plain", "value": body}]
            }))
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let _ = ::server_telemetry::record_api_call_cost(
                        &crate::db::get_pool(),
                        "unknown",
                        "sendgrid_send_email",
                        0.05
                    ).await;
                    Ok(())
                } else {
                    Err(format!("SendGrid API error: {}", resp.status()))
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
    fn test_client_creation() {
        let client = RealSendGridClient::new("key".to_string());
        assert_eq!(client.api_key, "key");
    }

    #[tokio::test]
    async fn test_send_email_error_handling() {
        let client = RealSendGridClient::new("key".to_string());
        let _ = client.send_email("test@example.com", "Subject", "Body").await;
    }
}
