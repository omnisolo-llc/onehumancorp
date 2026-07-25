use super::chat::*;
use super::repository::*;
use chrono::Utc;
use uuid::Uuid;

#[tokio::test]
async fn test_in_memory_chat_repository_inbox() {
    let repo = InMemoryChatRepository::new();
    let tenant_id = Uuid::new_v4();
    let inbox_id = Uuid::new_v4();

    let inbox = Inbox {
        id: inbox_id,
        tenant_id,
        name: "Test WhatsApp".to_string(),
        channel_adapter: ChannelAdapter::WhatsApp,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    assert!(repo.save_inbox(inbox.clone()).await.is_ok());

    let fetched = repo.get_inbox(tenant_id, inbox_id).await.unwrap();
    assert_eq!(fetched.name, "Test WhatsApp");
    assert_eq!(fetched.channel_adapter, ChannelAdapter::WhatsApp);
}

#[tokio::test]
async fn test_in_memory_chat_repository_contact() {
    let repo = InMemoryChatRepository::new();
    let tenant_id = Uuid::new_v4();
    let contact_id = Uuid::new_v4();

    let contact = Contact {
        id: contact_id,
        tenant_id,
        name: "Maya".to_string(),
        email: Some("maya@example.com".to_string()),
        phone_number: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    assert!(repo.save_contact(contact.clone()).await.is_ok());

    let fetched = repo.get_contact(tenant_id, contact_id).await.unwrap();
    assert_eq!(fetched.name, "Maya");
    assert_eq!(fetched.email, Some("maya@example.com".to_string()));
}

#[tokio::test]
async fn test_in_memory_chat_repository_conversation_and_message() {
    let repo = InMemoryChatRepository::new();
    let tenant_id = Uuid::new_v4();
    let contact_inbox_id = Uuid::new_v4();
    let conversation_id = Uuid::new_v4();
    let message_id = Uuid::new_v4();

    let conversation = Conversation {
        id: conversation_id,
        tenant_id,
        contact_inbox_id,
        status: ConversationStatus::Open,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    assert!(repo.save_conversation(conversation.clone()).await.is_ok());

    let message = Message {
        id: message_id,
        tenant_id,
        conversation_id,
        content: "Hello!".to_string(),
        message_type: MessageType::Incoming,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    assert!(repo.save_message(message.clone()).await.is_ok());

    let fetched_conv = repo
        .get_conversation(tenant_id, conversation_id)
        .await
        .unwrap();
    assert_eq!(fetched_conv.status, ConversationStatus::Open);

    let fetched_msg = repo.get_message(tenant_id, message_id).await.unwrap();
    assert_eq!(fetched_msg.content, "Hello!");
    assert_eq!(fetched_msg.message_type, MessageType::Incoming);
}
