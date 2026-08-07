use server_integrations_omnichannel::entities::{chat_inbox, chat_channel, chat_contact, chat_conversation, chat_message};
use server_integrations_omnichannel::traits::ChannelAdapter;
use async_trait::async_trait;
use serde_json::Value;
use uuid::Uuid;
use chrono::Utc;

struct MockChannelAdapter;

#[async_trait]
impl ChannelAdapter for MockChannelAdapter {
    async fn send_message(&self, _tenant_id: &str, _to: &str, _content: &str) -> Result<(), String> {
        Ok(())
    }
    async fn handle_webhook(&self, _payload: Value) -> Result<(), String> {
        Ok(())
    }
}

#[tokio::test]
async fn test_chat_inbox_model() {
    let tenant_id = Uuid::new_v4();
    let inbox = chat_inbox::Model {
        id: Uuid::new_v4(),
        tenant_id,
        name: "Test Inbox".to_string(),
        created_at: Utc::now().into(),
        updated_at: Utc::now().into(),
    };
    assert_eq!(inbox.tenant_id, tenant_id);
}

#[tokio::test]
async fn test_chat_channel_model() {
    let tenant_id = Uuid::new_v4();
    let channel = chat_channel::Model {
        id: Uuid::new_v4(),
        tenant_id,
        inbox_id: Uuid::new_v4(),
        channel_type: "whatsapp".to_string(),
        config: serde_json::json!({"token": "123"}),
        created_at: Utc::now().into(),
        updated_at: Utc::now().into(),
    };
    assert_eq!(channel.tenant_id, tenant_id);
}

#[tokio::test]
async fn test_chat_contact_model() {
    let tenant_id = Uuid::new_v4();
    let contact = chat_contact::Model {
        id: Uuid::new_v4(),
        tenant_id,
        name: Some("John".to_string()),
        email: None,
        phone: Some("123".to_string()),
        created_at: Utc::now().into(),
        updated_at: Utc::now().into(),
    };
    assert_eq!(contact.tenant_id, tenant_id);
}

#[tokio::test]
async fn test_chat_conversation_model() {
    let tenant_id = Uuid::new_v4();
    let conv = chat_conversation::Model {
        id: Uuid::new_v4(),
        tenant_id,
        inbox_id: Uuid::new_v4(),
        contact_id: Uuid::new_v4(),
        assignee_id: None,
        status: "open".to_string(),
        created_at: Utc::now().into(),
        updated_at: Utc::now().into(),
    };
    assert_eq!(conv.tenant_id, tenant_id);
}

#[tokio::test]
async fn test_chat_message_model() {
    let tenant_id = Uuid::new_v4();
    let msg = chat_message::Model {
        id: Uuid::new_v4(),
        tenant_id,
        conversation_id: Uuid::new_v4(),
        sender_type: "contact".to_string(),
        sender_id: None,
        content: "hello".to_string(),
        content_type: "text".to_string(),
        created_at: Utc::now().into(),
        updated_at: Utc::now().into(),
    };
    assert_eq!(msg.tenant_id, tenant_id);
    assert_eq!(msg.content_type, "text");
}

#[tokio::test]
async fn test_channel_adapter() {
    let adapter = MockChannelAdapter;
    let res = adapter.send_message("t1", "c1", "hello").await;
    assert!(res.is_ok());
}
