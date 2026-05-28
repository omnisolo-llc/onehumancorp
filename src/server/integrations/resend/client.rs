use async_trait::async_trait;
use reqwest::Client;

#[async_trait]
pub trait ResendClientWrapper: Send + Sync {
    async fn send_email(&self, to: &str, subject: &str, body: &str) -> Result<(), String>;
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
    async fn send_email(&self, to: &str, subject: &str, body: &str) -> Result<(), String> {
        let url = "https://api.resend.com/emails".to_string();

        let payload = serde_json::json!({
            "from": "onboarding@resend.dev",
            "to": to,
            "subject": subject,
            "html": format!("{}<br><br><a href=\"#\">Unsubscribe</a>", body)
        });

        let res = self.http_client.post(&url)
            .bearer_auth(&self.api_key)
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let _ = ::server_telemetry::record_api_call_cost(
                        &crate::db::get_pool(),
                        "unknown", // tenant context
                        "resend_send_email",
                        0.001 // nominal cost
                    ).await;
                    Ok(())
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
}
