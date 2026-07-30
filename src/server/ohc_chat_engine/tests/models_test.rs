use ohc_chat_engine::models::{Inbox, Contact, Conversation, Message};
use chrono::Utc;
use uuid::Uuid;

#[test]
fn test_models_creation() {
    let inbox = Inbox {
        id: Uuid::new_v4(),
        tenant_id: "t1".to_string(),
        name: "test".to_string(),
        channel_type: "web".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    assert_eq!(inbox.name, "test");
    assert_eq!(inbox.channel_type, "web");

    let contact = Contact {
        id: Uuid::new_v4(),
        tenant_id: "t1".to_string(),
        name: "Bob".to_string(),
        email: Some("bob@example.com".to_string()),
        phone_number: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    assert_eq!(contact.name, "Bob");

    let conv = Conversation {
        id: Uuid::new_v4(),
        tenant_id: "t1".to_string(),
        inbox_id: inbox.id,
        contact_id: contact.id,
        status: "open".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    assert_eq!(conv.status, "open");

    let msg = Message {
        id: Uuid::new_v4(),
        tenant_id: "t1".to_string(),
        conversation_id: conv.id,
        content: "hey".to_string(),
        message_type: "incoming".to_string(),
        content_attributes: None,
        external_source_ids: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    assert_eq!(msg.content, "hey");
}
