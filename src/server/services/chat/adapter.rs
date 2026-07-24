use async_trait::async_trait;
use serde_json::Value;
use super::models::NativeChatMessage;

#[async_trait]
pub trait ChannelAdapter: Send + Sync {
    async fn handle_incoming(&self, payload: Value) -> Result<NativeChatMessage, String>;
    async fn send_outgoing(&self, message: &NativeChatMessage) -> Result<(), String>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    struct DummyAdapter;

    #[async_trait]
    impl ChannelAdapter for DummyAdapter {
        async fn handle_incoming(&self, payload: Value) -> Result<NativeChatMessage, String> {
            Ok(NativeChatMessage {
                id: "msg-1".to_string(),
                tenant_id: "tenant-1".to_string(),
                conversation_id: "conv-1".to_string(),
                sender_type: "contact".to_string(),
                sender_id: None,
                content: payload["text"].as_str().unwrap_or("").to_string(),
                is_ai_draft: Some(false),
                created_at: None,
            })
        }

        async fn send_outgoing(&self, _message: &NativeChatMessage) -> Result<(), String> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_dummy_adapter() {
        let adapter = DummyAdapter;
        let payload = json!({"text": "hello"});
        let msg = adapter.handle_incoming(payload).await.unwrap();

        assert_eq!(msg.content, "hello");
        assert_eq!(msg.tenant_id, "tenant-1");

        let res = adapter.send_outgoing(&msg).await;
        assert!(res.is_ok());
    }
}
