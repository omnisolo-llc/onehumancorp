use async_trait::async_trait;
use reqwest::Client;

#[async_trait]
pub trait ManychatClientWrapper: Send + Sync {
    async fn send_message(&self, platform: &str, to: &str, body: &str) -> Result<(), String>;
}

pub struct RealManychatClient {
    access_token: String,
    http_client: Client,
}

impl RealManychatClient {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            http_client: Client::new(),
        }
    }
}

#[async_trait]
impl ManychatClientWrapper for RealManychatClient {
    async fn send_message(&self, platform: &str, to: &str, body: &str) -> Result<(), String> {
        let url = "https://api.manychat.com/fb/sending/sendContent".to_string(); // Simplified URL

        let payload = serde_json::json!({
            "subscriber_id": to,
            "data": {
                "version": "v2",
                "content": {
                    "messages": [
                        {
                            "type": "text",
                            "text": body
                        }
                    ]
                }
            },
            "message_tag": "ACCOUNT_UPDATE"
        });

        let res = self.http_client.post(&url)
            .bearer_auth(&self.access_token)
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let _ = ::server_telemetry::record_api_call_cost(
                        &crate::db::get_pool(),
                        "unknown", // tenant context
                        &format!("{}_send_message", platform),
                        0.01 // nominal manychat cost
                    ).await;
                    Ok(())
                } else {
                    Err(format!("Manychat API error: {}", resp.status()))
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
        let client = RealManychatClient::new("token".to_string());
        assert_eq!(client.access_token, "token");
    }
}
