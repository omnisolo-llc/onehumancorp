use super::models::{ChatInbox, ChatConversation, ChatMessage};
use uuid::Uuid;
use chrono::Utc;

#[test]
fn test_models_creation() {
    let tenant_id = Uuid::new_v4();
    let inbox_id = Uuid::new_v4();
    let contact_id = Uuid::new_v4();
    let conversation_id = Uuid::new_v4();

    let inbox = ChatInbox {
        id: inbox_id,
        tenant_id,
        name: "Main Inbox".to_string(),
        created_at: Some(Utc::now()),
        updated_at: Some(Utc::now()),
    };
    assert_eq!(inbox.name, "Main Inbox");

    let conversation = ChatConversation {
        id: conversation_id,
        tenant_id,
        inbox_id,
        contact_id,
        assignee_id: None,
        status: "open".to_string(),
        created_at: Some(Utc::now()),
        updated_at: Some(Utc::now()),
    };
    assert_eq!(conversation.status, "open");

    let message = ChatMessage {
        id: Uuid::new_v4(),
        tenant_id,
        conversation_id,
        sender_type: "contact".to_string(),
        sender_id: Some(contact_id),
        content: "Hello!".to_string(),
        created_at: Some(Utc::now()),
        updated_at: Some(Utc::now()),
    };
    assert_eq!(message.content, "Hello!");
}
