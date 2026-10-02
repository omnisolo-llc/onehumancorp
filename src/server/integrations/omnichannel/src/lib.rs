pub mod models;
pub mod traits;

#[cfg(test)]
mod tests {
    use super::models::*;
    use super::traits::*;
    use async_trait::async_trait;
    use sea_orm::entity::prelude::*;
    use serde_json::Value;

    struct MockChannelAdapter;

    #[async_trait]
    impl ChannelAdapter for MockChannelAdapter {
        async fn send_message(&self, _recipient_id: &str, _content: &str) -> Result<(), String> {
            Ok(())
        }
        async fn receive_webhook(&self, _payload: Value) -> Result<(), String> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_mock_channel_adapter() {
        let adapter = MockChannelAdapter;
        assert!(adapter.send_message("user_1", "hello").await.is_ok());
        assert!(adapter.receive_webhook(serde_json::json!({})).await.is_ok());
    }

    #[test]
    fn test_inbox_model_creation() {
        let id = Uuid::new_v4();
        let tenant_id = Uuid::new_v4();
        let now = chrono::Utc::now().into();
        let model = inbox::Model {
            id,
            tenant_id,
            name: "WhatsApp".to_string(),
            created_at: now,
            updated_at: now,
        };
        assert_eq!(model.id, id);
        assert_eq!(model.tenant_id, tenant_id);
        assert_eq!(model.name, "WhatsApp");
    }

    #[test]
    fn test_conversation_model_creation() {
        let id = Uuid::new_v4();
        let tenant_id = Uuid::new_v4();
        let inbox_id = Uuid::new_v4();
        let contact_id = Uuid::new_v4();
        let now = chrono::Utc::now().into();
        let model = conversation::Model {
            id,
            tenant_id,
            inbox_id,
            contact_id,
            assignee_id: None,
            status: "open".to_string(),
            created_at: now,
            updated_at: now,
        };
        assert_eq!(model.id, id);
        assert_eq!(model.tenant_id, tenant_id);
        assert_eq!(model.inbox_id, inbox_id);
        assert_eq!(model.status, "open");
    }

    #[test]
    fn test_message_model_creation() {
        let id = Uuid::new_v4();
        let tenant_id = Uuid::new_v4();
        let conversation_id = Uuid::new_v4();
        let now = chrono::Utc::now().into();
        let model = message::Model {
            id,
            tenant_id,
            conversation_id,
            sender_type: "contact".to_string(),
            sender_id: None,
            content_type: "text".to_string(),
            content: "Hello there".to_string(),
            created_at: now,
            updated_at: now,
        };
        assert_eq!(model.id, id);
        assert_eq!(model.tenant_id, tenant_id);
        assert_eq!(model.conversation_id, conversation_id);
        assert_eq!(model.content, "Hello there");
        assert_eq!(model.content_type, "text");
    }
}
