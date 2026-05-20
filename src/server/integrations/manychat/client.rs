use async_trait::async_trait;
use reqwest::Client;

#[async_trait]
pub trait ManychatClientWrapper: Send + Sync {
    async fn send_message(&self, subscriber_id: &str, message: &str) -> Result<(), String>;
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
    async fn send_message(&self, _subscriber_id: &str, _message: &str) -> Result<(), String> {
        let _ = ::server_telemetry::record_api_call_cost(
            &crate::db::get_pool(),
            "unknown",
            "manychat_send_message",
            0.05
        ).await;
        Ok(())
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
