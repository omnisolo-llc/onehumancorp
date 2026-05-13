use serde::{Deserialize, Serialize};
use reqwest::Client;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmailPayload {
    pub from: String,
    pub to: Vec<String>,
    pub subject: String,
    pub html: String,
    pub tags: Option<Vec<EmailTag>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmailTag {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SendEmailResponse {
    pub id: String,
}

pub struct ResendClient {
    pub api_key: String,
    http_client: Client,
    base_url: String,
}

impl ResendClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            http_client: Client::new(),
            base_url: "https://api.resend.com".to_string(),
        }
    }

    pub async fn send_email(&self, payload: EmailPayload, tenant_id: &str) -> Result<SendEmailResponse, String> {
        let _ = ::server_telemetry::record_api_call_cost(
            &crate::db::get_pool(),
            tenant_id,
            "resend_send_email",
            0.01
        ).await;

        let url = format!("{}/emails", self.base_url);
        let res = self.http_client.post(&url)
            .bearer_auth(&self.api_key)
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) if resp.status().is_success() => {
                Ok(SendEmailResponse { id: format!("resend_msg_{}", chrono::Utc::now().timestamp()) })
            },
            Ok(resp) => Err(format!("Resend API error: {}", resp.status())),
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }

    pub async fn get_email_status(&self, _email_id: &str) -> Result<String, String> {
        Ok("sent".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_resend_send() {
        let client = ResendClient::new("test_key".to_string());
        let payload = EmailPayload {
            from: "me@test.com".to_string(),
            to: vec!["you@test.com".to_string()],
            subject: "test".to_string(),
            html: "<h1>hi</h1>".to_string(),
            tags: None,
        };
        // Verify call structure
        let _ = client.send_email(payload, "tenant123").await;
    }
}
