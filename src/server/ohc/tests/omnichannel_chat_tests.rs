use server_ohc::domain::omnichannel_chat::models::{Inbox, Channel, Conversation, Message, Contact, ContactInbox};
use uuid::Uuid;

#[test]
fn test_omnichannel_models_compilation() {
    let tenant_id = Uuid::new_v4();
    let inbox = Inbox {
        id: Uuid::new_v4(),
        tenant_id,
        name: "Test Inbox".to_string(),
    };
    assert_eq!(inbox.name, "Test Inbox");

    let channel = Channel {
        id: Uuid::new_v4(),
        tenant_id,
        inbox_id: inbox.id,
        channel_type: "api".to_string(),
        webhook_url: None,
    };
    assert_eq!(channel.channel_type, "api");

    let contact = Contact {
        id: Uuid::new_v4(),
        tenant_id,
        email: Some("test@example.com".to_string()),
        phone_number: None,
    };

    let _contact_inbox = ContactInbox {
        id: Uuid::new_v4(),
        tenant_id,
        contact_id: contact.id,
        inbox_id: inbox.id,
        source_id: "ext_123".to_string(),
    };

    let conversation = Conversation {
        id: Uuid::new_v4(),
        tenant_id,
        inbox_id: inbox.id,
        contact_id: contact.id,
        status: "open".to_string(),
    };

    let message = Message {
        id: Uuid::new_v4(),
        tenant_id,
        conversation_id: conversation.id,
        content: "Hello World".to_string(),
        message_type: "text".to_string(),
    };
    assert_eq!(message.content, "Hello World");
}
