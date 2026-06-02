use async_trait::async_trait;
use reqwest::Client;

#[async_trait]
pub trait TiktokClientWrapper: Send + Sync {
    async fn send_message(&self, to: &str, body: &str) -> Result<(), String>;
}

pub struct RealTiktokClient {
    access_token: String,
    http_client: Client,
}

impl RealTiktokClient {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            http_client: Client::new(),
        }
    }
}

#[async_trait]
impl TiktokClientWrapper for RealTiktokClient {
    async fn send_message(&self, to: &str, body: &str) -> Result<(), String> {
        let url = "https://business-api.tiktok.com/open_api/v1.3/message/send/";

        let payload = serde_json::json!({
            "recipient_id": to,
            "message_text": body
        });

        let res = self.http_client.post(url)
            .header("Access-Token", &self.access_token)
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
        let client = RealTiktokClient::new("token".to_string());
        assert_eq!(client.access_token, "token");
    }
}
