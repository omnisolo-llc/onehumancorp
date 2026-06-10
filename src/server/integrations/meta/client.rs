use async_trait::async_trait;
use reqwest::Client;

#[async_trait]
pub trait MetaClientWrapper: Send + Sync {
    async fn send_message(&self, platform: &str, to: &str, body: &str) -> Result<(), String>;
}

pub struct RealMetaClient {
    access_token: String,
    phone_number_id: String,
    http_client: Client,
}

impl RealMetaClient {
    pub fn new(access_token: String, phone_number_id: String) -> Self {
        Self {
            access_token,
            phone_number_id,
            http_client: Client::new(),
        }
    }
}

#[async_trait]
impl MetaClientWrapper for RealMetaClient {
    async fn send_message(&self, platform: &str, to: &str, body: &str) -> Result<(), String> {
        let url = match platform {
            "whatsapp" => {
                if self.phone_number_id.is_empty() {
                    "https://graph.facebook.com/v19.0/me/messages".to_string()
                } else {
                    format!("https://graph.facebook.com/v19.0/{}/messages", self.phone_number_id)
                }
            },
            _ => "https://graph.facebook.com/v19.0/me/messages".to_string(), // Simplified URL mapping
        };

        let payload = serde_json::json!({
            "recipient": {
                "id": to
            },
            "message": {
                "text": body
            },
            "messaging_type": "RESPONSE"
        });

        let res = self.http_client.post(&url)
            .bearer_auth(&self.access_token)
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    Ok(())
                } else {
                    Err(format!("Meta API error: {}", resp.status()))
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
        let client = RealMetaClient::new("token".to_string(), "phone".to_string());
        assert_eq!(client.access_token, "token");
        assert_eq!(client.phone_number_id, "phone");
    }

    // Because send_message issues a real network request using reqwest,
    // we omit a full unit test calling it here to prevent external dependencies and network flakes in the test suite.
    // Provider tests cover the mock path.
}
