use ohc_chat_engine::models::{Inbox, Contact, Conversation, Message};
use chrono::Utc;
use uuid::Uuid;

#[test]
fn test_models_creation() {
    let inbox = Inbox {
        id: Uuid::new_v4(),
        tenant_id: Uuid::new_v4(),
        name: "test".to_string(),
        channel_type: "web".to_string(),
        created_at: Some(Utc::now()),
        updated_at: Some(Utc::now()),
    };
    assert_eq!(inbox.name, "test");
    assert_eq!(inbox.channel_type, "web");

    let contact = Contact {
        id: Uuid::new_v4(),
        tenant_id: Uuid::new_v4(),
        name: Some("Bob".to_string()),
        email: Some("bob@example.com".to_string()),
        phone: None,
        created_at: Some(Utc::now()),
        updated_at: Some(Utc::now()),
    };
    assert_eq!(contact.name, Some("Bob".to_string()));

    let conv = Conversation {
        id: Uuid::new_v4(),
        tenant_id: Uuid::new_v4(),
        inbox_id: inbox.id,
        contact_id: contact.id,
        status: "open".to_string(),
        created_at: Some(Utc::now()),
        updated_at: Some(Utc::now()),
    };
    assert_eq!(conv.status, "open");

    let msg = Message {
        id: Uuid::new_v4(),
        tenant_id: Uuid::new_v4(),
        conversation_id: conv.id,
        content: "hey".to_string(),
        sender_type: "contact".to_string(),
        created_at: Some(Utc::now()),
        updated_at: Some(Utc::now()),
    };
    assert_eq!(msg.content, "hey");
}
