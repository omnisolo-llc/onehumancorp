pub struct MetaGraphClient {
    pub access_token: String,
}

impl MetaGraphClient {
    pub fn new(access_token: String) -> Self {
        Self { access_token }
    }

    pub async fn send_message(&self, to: &str, text: &str) -> Result<(), String> {
        let _ = crate::telemetry::record_api_call_cost(&crate::db::get_pool(), "unknown", "meta_graph_send", 0.01).await;
        // Simulating writing to inbox queue
        tracing::info!("Sending Meta message to {} with text {}", to, text);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = MetaGraphClient::new("token".to_string());
        assert_eq!(client.access_token, "token");
    }

    #[tokio::test]
    async fn test_send_message() {
        let client = MetaGraphClient::new("token".to_string());
        let res = client.send_message("user_123", "Hello").await;
        assert!(res.is_ok());
    }
}
