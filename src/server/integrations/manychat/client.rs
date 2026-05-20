use async_trait::async_trait;
use reqwest::Client;

#[async_trait]
pub trait ManychatClientWrapper: Send + Sync {
    async fn send_message(&self, to: &str, body: &str) -> Result<(), String>;
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
    async fn send_message(&self, to: &str, body: &str) -> Result<(), String> {
        let url = "https://api.manychat.com/fb/sending/sendContent".to_string();

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
            "message_tag": "NON_PROMOTIONAL_SUBSCRIPTION"
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
                        "manychat_send_message",
                        0.01
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

    #[tokio::test]
    async fn test_send_message_error_handling() {
        let client = RealManychatClient::new("token".to_string());
        let _ = client.send_message("123", "test").await;
    }
}
