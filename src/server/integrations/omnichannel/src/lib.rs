pub mod entities;
pub mod adapter;

#[cfg(test)]
mod tests {
    use super::*;
    use entities::{inbox, conversation, message, channel, contact};
    use sea_orm::entity::prelude::*;
    use adapter::ChannelAdapter;
    use async_trait::async_trait;
    use serde_json::Value;

    struct MockAdapter;

    #[async_trait]
    impl ChannelAdapter for MockAdapter {
        async fn send_message(&self, _recipient_id: &str, _content: &str) -> Result<(), String> {
            Ok(())
        }

        async fn ingest_webhook(&self, _payload: Value) -> Result<(), String> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_adapter() {
        let adapter = MockAdapter;
        assert!(adapter.send_message("test", "test").await.is_ok());
        assert!(adapter.ingest_webhook(serde_json::json!({})).await.is_ok());
    }

    #[test]
    fn test_entities_exist() {
        let _inbox = inbox::Model {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            name: "test".to_string(),
            created_at: None,
            updated_at: None,
        };
        let _conversation = conversation::Model {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            inbox_id: Uuid::new_v4(),
            contact_id: Uuid::new_v4(),
            assignee_id: None,
            status: "open".to_string(),
            created_at: None,
            updated_at: None,
        };
        let _message = message::Model {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            conversation_id: Uuid::new_v4(),
            sender_type: "agent".to_string(),
            sender_id: None,
            content: "test".to_string(),
            content_type: "text".to_string(),
            created_at: None,
            updated_at: None,
        };
        let _channel = channel::Model {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            inbox_id: Uuid::new_v4(),
            channel_type: "test".to_string(),
            config: None,
            created_at: None,
            updated_at: None,
        };
        let _contact = contact::Model {
            id: Uuid::new_v4(),
            tenant_id: Uuid::new_v4(),
            name: Some("test".to_string()),
            email: None,
            phone: None,
            created_at: None,
            updated_at: None,
        };
    }
}
