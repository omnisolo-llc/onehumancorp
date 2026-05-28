use async_trait::async_trait;
use reqwest::Client;

#[async_trait]
pub trait TikTokClientWrapper: Send + Sync {
    async fn send_message(&self, platform: &str, to: &str, body: &str) -> Result<(), String>;
}

pub struct RealTikTokClient {
    access_token: String,
    http_client: Client,
}

impl RealTikTokClient {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            http_client: Client::new(),
        }
    }
}

#[async_trait]
impl TikTokClientWrapper for RealTikTokClient {
    async fn send_message(&self, _platform: &str, to: &str, body: &str) -> Result<(), String> {
        let url = "https://business-api.tiktok.com/open_api/v1.3/message/send/".to_string();

        let payload = serde_json::json!({
            "to_user_id": to,
            "message_type": "text",
            "text": {
                "content": body
            }
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
                        "tiktok_send_message",
                        0.01 // nominal cost
                    ).await;
                    Ok(())
                } else {
                    Err(format!("TikTok API error: {}", resp.status()))
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
        let client = RealTikTokClient::new("token".to_string());
        assert_eq!(client.access_token, "token");
    }
}
