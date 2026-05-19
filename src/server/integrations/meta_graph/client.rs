use async_trait::async_trait;
use reqwest::Client;

#[async_trait]
pub trait MetaGraphClientWrapper: Send + Sync {
    async fn send_message(&self, recipient_id: &str, message: &str) -> Result<(), String>;
}

pub struct RealMetaGraphClient {
    pub access_token: String,
    http_client: Client,
}

impl RealMetaGraphClient {
    pub fn new(access_token: String) -> Self {
        Self { access_token, http_client: Client::new() }
    }
}

#[async_trait]
impl MetaGraphClientWrapper for RealMetaGraphClient {
    async fn send_message(&self, recipient_id: &str, message: &str) -> Result<(), String> {
        let url = "https://graph.facebook.com/v19.0/me/messages";
        let res = self.http_client.post(url)
            .bearer_auth(&self.access_token)
            .json(&serde_json::json!({
                "recipient": {
                    "id": recipient_id
                },
                "message": {
                    "text": message
                }
            }))
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let _ = ::server_telemetry::record_api_call_cost(
                        &crate::db::get_pool(),
                        "unknown",
                        "meta_graph_send_message",
                        0.05
                    ).await;
                    Ok(())
                } else {
                    Err(format!("Meta Graph API error: {}", resp.status()))
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
    fn test_client_creation() {
        let client = RealMetaGraphClient::new("token".to_string());
        assert_eq!(client.access_token, "token");
    }

    #[tokio::test]
    async fn test_send_message_error_handling() {
        let client = RealMetaGraphClient::new("token".to_string());
        let _ = client.send_message("123", "test").await;
    }
}
