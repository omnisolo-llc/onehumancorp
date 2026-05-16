use async_trait::async_trait;
use reqwest::Client;

#[async_trait]
pub trait ManychatClientWrapper: Send + Sync {
    async fn send_message(&self, to: &str, body: &str) -> Result<(), String>;
}

pub struct RealManychatClient {
    api_token: String,
    http_client: Client,
}

impl RealManychatClient {
    pub fn new(api_token: String) -> Self {
        Self {
            api_token,
            http_client: Client::new(),
        }
    }
}

#[async_trait]
impl ManychatClientWrapper for RealManychatClient {
    async fn send_message(&self, to: &str, body: &str) -> Result<(), String> {
        let url = "https://api.manychat.com/fb/sending/sendContent";
        let res = self.http_client.post(url)
            .header("Authorization", format!("Bearer {}", self.api_token))
            .json(&serde_json::json!({
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
                }
            }))
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let _ = crate::telemetry::record_api_call_cost(
                        &crate::db::get_pool(),
                        "unknown",
                        "manychat_send_message",
                        0.05
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
        assert_eq!(client.api_token, "token");
    }
}
