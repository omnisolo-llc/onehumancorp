use crate::domain::native_chat::models::{Inbox, Contact, Conversation, Message};
use uuid::Uuid;

#[tokio::test]
async fn test_inbox_model() {
    let inbox = Inbox {
        id: Uuid::new_v4(),
        tenant_id: Uuid::new_v4(),
        name: "Test Inbox".to_string(),
        channel_type: "web_widget".to_string(),
        enable_auto_assignment: false,
        created_at: None,
        updated_at: None,
    };
    assert_eq!(inbox.name, "Test Inbox");
}

#[tokio::test]
async fn test_contact_model() {
    let contact = Contact {
        id: Uuid::new_v4(),
        tenant_id: Uuid::new_v4(),
        name: "John Doe".to_string(),
        email: Some("john@example.com".to_string()),
        phone_number: None,
        contact_type: "visitor".to_string(),
        created_at: None,
        updated_at: None,
    };
    assert_eq!(contact.name, "John Doe");
}

#[tokio::test]
async fn test_conversation_model() {
    let conversation = Conversation {
        id: Uuid::new_v4(),
        tenant_id: Uuid::new_v4(),
        inbox_id: Uuid::new_v4(),
        contact_id: Uuid::new_v4(),
        status: "open".to_string(),
        assignee_id: None,
        created_at: None,
        updated_at: None,
    };
    assert_eq!(conversation.status, "open");
}

#[tokio::test]
async fn test_message_model() {
    let message = Message {
        id: Uuid::new_v4(),
        tenant_id: Uuid::new_v4(),
        conversation_id: Uuid::new_v4(),
        content: "Hello".to_string(),
        message_type: "incoming".to_string(),
        private: false,
        created_at: None,
        updated_at: None,
    };
    assert_eq!(message.content, "Hello");
}
