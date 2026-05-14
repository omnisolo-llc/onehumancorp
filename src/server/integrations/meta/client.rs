use serde::{Deserialize, Serialize};
use reqwest::Client;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MetaMessage {
    pub id: String,
    pub text: String,
}

pub struct MetaClient {
    pub access_token: String,
    pub http_client: Client,
}

impl MetaClient {
    pub fn new(access_token: String) -> Self {
        MetaClient {
            access_token,
            http_client: Client::new(),
        }
    }

    pub async fn send_message(&self, recipient_id: &str, text: &str, tenant_id: &str) -> Result<String, String> {
        let _ = ::server_telemetry::record_api_call_cost(
            &crate::db::get_pool(),
            tenant_id,
            "meta_send_message",
            0.01
        ).await;

        let url = format!("https://graph.facebook.com/v19.0/me/messages?access_token={}", self.access_token);
        let payload = serde_json::json!({
            "recipient": { "id": recipient_id },
            "message": { "text": text }
        });

        let res = self.http_client.post(&url)
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    Ok(format!("Message sent to {}", recipient_id))
                } else {
                    Err(format!("Meta API Error: {}", resp.status()))
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
    async fn test_meta_client_creation() {
        let client = MetaClient::new("token".to_string());
        assert_eq!(client.access_token, "token");
    }

    #[tokio::test]
    async fn test_meta_client_error() {
        let client = MetaClient::new("token".to_string());
        // Should return error for invalid URL or auth
        let _ = client.send_message("123", "test", "tenant1").await;
    }
}
