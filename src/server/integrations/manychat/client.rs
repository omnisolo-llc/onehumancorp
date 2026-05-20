use reqwest::Client;
use serde_json::json;

pub trait ManychatClientWrapper: Send + Sync {
    // Synchronous mock wrapper for simple trait compatibility testing
    fn send_message(&self, subscriber_id: &str, message: &str) -> Result<(), String>;
}

pub struct RealManychatClient {
    access_token: String,
}

impl RealManychatClient {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
        }
    }
}

impl ManychatClientWrapper for RealManychatClient {
    fn send_message(&self, _subscriber_id: &str, _message: &str) -> Result<(), String> {
        // Implementation stub
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
}
