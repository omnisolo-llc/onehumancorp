use serde::{Deserialize, Serialize};
use reqwest::Client;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmailPayload {
    pub to: String,
    pub subject: String,
    pub html: String,
}

pub struct ResendClient {
    pub api_key: String,
    pub http_client: Client,
}

impl ResendClient {
    pub fn new(api_key: String) -> Self {
        ResendClient {
            api_key,
            http_client: Client::new(),
        }
    }

    pub async fn send_email(&self, payload: &EmailPayload, tenant_id: &str) -> Result<String, String> {
        let _ = ::server_telemetry::record_api_call_cost(
            &crate::db::get_pool(),
            tenant_id,
            "resend_send_email",
            0.02
        ).await;

        let req_body = serde_json::json!({
            "from": "marketing@onehumancorp.com",
            "to": [payload.to],
            "subject": payload.subject,
            "html": payload.html,
        });

        let res = self.http_client.post("https://api.resend.com/emails")
            .bearer_auth(&self.api_key)
            .json(&req_body)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    Ok("Email sent successfully".to_string())
                } else {
                    Err(format!("Resend API Error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network Error: {}", e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_resend_client_creation() {
        let client = ResendClient::new("token".to_string());
        assert_eq!(client.api_key, "token");
    }

    #[tokio::test]
    async fn test_resend_send_error() {
        let client = ResendClient::new("token".to_string());
        let payload = EmailPayload {
            to: "test@test.com".to_string(),
            subject: "subject".to_string(),
            html: "<html></html>".to_string(),
        };
        let _ = client.send_email(&payload, "tenant1").await;
    }
}
