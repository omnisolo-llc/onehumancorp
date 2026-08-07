use async_trait::async_trait;
use serde_json::Value;

#[async_trait]
pub trait ChannelAdapter: Send + Sync {
    /// Send a message through the channel integration
    async fn send_message(&self, recipient_id: &str, content: &str) -> Result<(), String>;

    /// Ingest a webhook payload from the channel
    async fn handle_webhook(&self, payload: Value) -> Result<(), String>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    struct TestAdapter {
        fail_send: bool,
        fail_webhook: bool,
    }

    #[async_trait]
    impl ChannelAdapter for TestAdapter {
        async fn send_message(&self, _recipient_id: &str, _content: &str) -> Result<(), String> {
            if self.fail_send {
                Err("Failed to send".to_string())
            } else {
                Ok(())
            }
        }

        async fn handle_webhook(&self, _payload: Value) -> Result<(), String> {
            if self.fail_webhook {
                Err("Webhook failed".to_string())
            } else {
                Ok(())
            }
        }
    }

    #[tokio::test]
    async fn test_adapter_success() {
        let adapter = TestAdapter { fail_send: false, fail_webhook: false };
        assert_eq!(adapter.send_message("user1", "hello").await, Ok(()));
        assert_eq!(adapter.handle_webhook(json!({"event": "msg"})).await, Ok(()));
    }

    #[tokio::test]
    async fn test_adapter_failure() {
        let adapter = TestAdapter { fail_send: true, fail_webhook: true };
        assert_eq!(adapter.send_message("user1", "hello").await, Err("Failed to send".to_string()));
        assert_eq!(adapter.handle_webhook(json!({"event": "msg"})).await, Err("Webhook failed".to_string()));
    }
}
