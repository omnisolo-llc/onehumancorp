use async_trait::async_trait;
use reqwest::Client;

#[async_trait]
pub trait MetaClientWrapper: Send + Sync {
    async fn send_message(&self, platform: &str, from: Option<&str>, to: &str, body: &str) -> Result<(), String>;
}

pub struct RealMetaClient {
    access_token: String,
    http_client: Client,
}

impl RealMetaClient {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            http_client: Client::new(),
        }
    }
}

#[async_trait]
impl MetaClientWrapper for RealMetaClient {
    async fn send_message(&self, platform: &str, from: Option<&str>, to: &str, body: &str) -> Result<(), String> {
        let url = match platform {
            "whatsapp" => {
                if let Some(from_id) = from {
                    format!("https://graph.facebook.com/v19.0/{}/messages", from_id)
                } else {
                    "https://graph.facebook.com/v19.0/me/messages".to_string()
                }
            },
            _ => "https://graph.facebook.com/v19.0/me/messages".to_string(), // Simplified URL mapping
        };

        let payload = if platform == "instagram" || platform == "facebook" {
            serde_json::json!({
                "recipient": {
                    "id": to
                },
                "message": {
                    "text": body
                }
            })
        } else if platform == "whatsapp" {
            serde_json::json!({
                "messaging_product": "whatsapp",
                "recipient_type": "individual",
                "to": to,
                "type": "text",
                "text": {
                    "preview_url": false,
                    "body": body
                }
            })
        } else {
            serde_json::json!({
                "recipient": {
                    "id": to
                },
                "message": {
                    "text": body
                },
                "messaging_type": "RESPONSE"
            })
        };

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
        let client = RealMetaClient::new("token".to_string());
        assert_eq!(client.access_token, "token");
    }

    // Because send_message issues a real network request using reqwest,
    // we omit a full unit test calling it here to prevent external dependencies and network flakes in the test suite.
    // Provider tests cover the mock path.
}
