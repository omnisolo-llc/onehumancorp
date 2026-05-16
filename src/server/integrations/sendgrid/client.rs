use async_trait::async_trait;
use reqwest::Client;

#[async_trait]
pub trait SendGridClientWrapper: Send + Sync {
    async fn send_email(&self, to: &str, from: &str, subject: &str, html_body: &str) -> Result<(), String>;
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
    async fn send_email(&self, to: &str, from: &str, subject: &str, html_body: &str) -> Result<(), String> {
        let url = "https://api.sendgrid.com/v3/mail/send";
        let payload = serde_json::json!({
            "personalizations": [{
                "to": [{"email": to}]
            }],
            "from": {"email": from},
            "subject": subject,
            "content": [{"type": "text/html", "value": html_body}]
        });

        let res = self.http_client.post(url)
            .bearer_auth(&self.api_key)
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let _ = ::server_telemetry::record_api_call_cost(
                        &crate::db::get_pool(),
                        "unknown",
                        "sendgrid_send_email",
                        0.01
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
    fn test_real_client_creation() {
        let client = RealSendGridClient::new("api_key".to_string());
        assert_eq!(client.api_key, "api_key");
    }
}
