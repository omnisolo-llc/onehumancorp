use sqlx::PgPool;
use uuid::Uuid;
use super::service::ChatService;
use crate::services::chat::models::{ChatInbox, ChatChannel, ChatContact, ChatConversation, ChatMessage};

#[sqlx::test]
async fn test_chat_service_crud_flow(pool: PgPool) {
    let service = ChatService::new(pool);
    let tenant_id = Uuid::new_v4();

    // 1. Create Inbox
    let inbox = service.create_inbox_record(
        tenant_id,
        "Support".to_string(),
        Some(true),
        Some("Hello! How can we help?".to_string()),
        Some(true),
    ).await.expect("Failed to create inbox");

    assert_eq!(inbox.name, "Support");
    assert_eq!(inbox.enable_auto_assignment, Some(true));
    assert_eq!(inbox.greeting_message, Some("Hello! How can we help?".to_string()));
    assert_eq!(inbox.working_hours_enabled, Some(true));

    // 2. Create Channel
    let config = serde_json::json!({"website_token": "token123"});
    let channel = service.create_channel_record(
        tenant_id,
        inbox.id,
        "web_widget".to_string(),
        config.clone(),
    ).await.expect("Failed to create channel");

    assert_eq!(channel.channel_type, "web_widget");
    assert_eq!(channel.config, config);

    // 3. Create Contact
    let custom_attrs = serde_json::json!({"vip": true});
    let contact = service.create_contact_record(
        tenant_id,
        Some("Carlos Customer".to_string()),
        Some("carlos@example.com".to_string()),
        Some("+123456789".to_string()),
        Some(custom_attrs.clone()),
    ).await.expect("Failed to create contact");

    assert_eq!(contact.name, Some("Carlos Customer".to_string()));
    assert_eq!(contact.custom_attributes, Some(custom_attrs));

    // 4. Start Conversation
    let conversation = service.start_chat_conversation(
        tenant_id,
        inbox.id,
        contact.id,
        None,
        Some(1),
    ).await.expect("Failed to start conversation");

    assert_eq!(conversation.status, "open");
    assert_eq!(conversation.priority, Some(1));

    // 5. Send Message (Contact)
    let attrs = serde_json::json!({"browser": "Chrome"});
    let msg1 = service.send_chat_message(
        tenant_id,
        conversation.id,
        "contact".to_string(),
        Some(contact.id),
        "I need help with my cake order".to_string(),
        Some("text".to_string()),
        Some(attrs.clone()),
    ).await.expect("Failed to send message from contact");

    assert_eq!(msg1.content, "I need help with my cake order");
    assert_eq!(msg1.sender_type, "contact");
    assert_eq!(msg1.content_type, Some("text".to_string()));
    assert_eq!(msg1.additional_attributes, Some(attrs));

    // 6. Send Message (Agent)
    let agent_id = Uuid::new_v4();
    let msg2 = service.send_chat_message(
        tenant_id,
        conversation.id,
        "agent".to_string(),
        Some(agent_id),
        "Sure, what is the order number?".to_string(),
        Some("text".to_string()),
        None,
    ).await.expect("Failed to send message from agent");

    assert_eq!(msg2.content, "Sure, what is the order number?");
    assert_eq!(msg2.sender_type, "agent");

    // 7. Update Conversation Status
    let updated_conv = service.update_conversation_status(
        tenant_id,
        conversation.id,
        "resolved".to_string(),
    ).await.expect("Failed to update conversation status");

    assert_eq!(updated_conv.status, "resolved");
}
