use async_trait::async_trait;
use reqwest::Client;

#[async_trait]
pub trait SendgridClientWrapper: Send + Sync {
    async fn send_email(&self, to: &str, from: &str, subject: &str, body: &str) -> Result<(), String>;
}

pub struct RealSendgridClient {
    api_key: String,
    http_client: Client,
}

impl RealSendgridClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            http_client: Client::new(),
        }
    }
}

#[async_trait]
impl SendgridClientWrapper for RealSendgridClient {
    async fn send_email(&self, to: &str, from: &str, subject: &str, body: &str) -> Result<(), String> {
        let url = "https://api.sendgrid.com/v3/mail/send";
        let payload = serde_json::json!({
            "personalizations": [{
                "to": [{"email": to}]
            }],
            "from": {"email": from},
            "subject": subject,
            "content": [{"type": "text/html", "value": body}]
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
    fn test_real_client_creation() {
        let client = RealSendgridClient::new("token".to_string());
        assert_eq!(client.api_key, "token");
    }

    #[tokio::test]
    async fn test_send_email_error_handling() {
        let client = RealSendgridClient::new("token".to_string());
        let _ = client.send_email("to@test.com", "from@test.com", "Subject", "Body").await;
    }
}
