use super::omnichannel::*;
use chrono::Utc;
use uuid::Uuid;

#[test]
fn test_omnichannel_models_tenant_isolation() {
    let tenant1 = TenantId(Uuid::new_v4());
    let tenant2 = TenantId(Uuid::new_v4());

    let inbox1 = Inbox {
        id: Uuid::new_v4(),
        tenant_id: tenant1.clone(),
        name: "IG DMs".to_string(),
        channel: Channel::Instagram,
    };

    let inbox2 = Inbox {
        id: Uuid::new_v4(),
        tenant_id: tenant2.clone(),
        name: "SMS Inbox".to_string(),
        channel: Channel::TwilioSms,
    };

    assert_eq!(inbox1.tenant_id, tenant1);
    assert_eq!(inbox2.tenant_id, tenant2);
    assert_ne!(inbox1.tenant_id, inbox2.tenant_id);
}

#[test]
fn test_conversation_and_message_linking() {
    let tenant = TenantId(Uuid::new_v4());

    let contact = Contact {
        id: Uuid::new_v4(),
        tenant_id: tenant.clone(),
        name: "Maya".to_string(),
        email: None,
        phone_number: Some("+123456789".to_string()),
    };

    let conversation = Conversation {
        id: Uuid::new_v4(),
        tenant_id: tenant.clone(),
        inbox_id: Uuid::new_v4(),
        contact_id: contact.id.clone(),
        assignee_id: None,
        status: ConversationStatus::Open,
        created_at: Utc::now(),
    };

    let message = Message {
        id: Uuid::new_v4(),
        tenant_id: tenant.clone(),
        conversation_id: conversation.id.clone(),
        sender_type: SenderType::Contact,
        sender_id: contact.id.clone(),
        content: "Hello!".to_string(),
        status: MessageStatus::Sent,
        created_at: Utc::now(),
    };

    assert_eq!(message.conversation_id, conversation.id);
    assert_eq!(conversation.contact_id, contact.id);
    assert_eq!(message.sender_id, contact.id);
}
