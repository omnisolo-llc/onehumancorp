use crate::integrations::chat::domain::{Inbox, Contact, Conversation, Message};
use crate::integrations::chat::repository::ChatRepository;
use uuid::Uuid;
use chrono::Utc;
use sqlx::PgPool;

#[sqlx::test]
async fn test_inbox_crud_with_rls(pool: PgPool) {
    let repo = ChatRepository::new(pool);
    let tenant_id = Uuid::new_v4();
    let other_tenant_id = Uuid::new_v4();

    let inbox = Inbox {
        id: Uuid::new_v4(),
        tenant_id,
        name: "Test Inbox".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let created_inbox = repo.create_inbox(tenant_id, &inbox).await.expect("Failed to create inbox");
    assert_eq!(created_inbox.name, "Test Inbox");
    assert_eq!(created_inbox.tenant_id, tenant_id);

    let retrieved_inbox = repo.get_inbox(tenant_id, inbox.id).await.expect("Failed to get inbox");
    assert!(retrieved_inbox.is_some());
    assert_eq!(retrieved_inbox.unwrap().id, inbox.id);

    // Cross-tenant access should return nothing due to RLS
    let unauthorized_retrieval = repo.get_inbox(other_tenant_id, inbox.id).await.expect("Failed cross-tenant check");
    assert!(unauthorized_retrieval.is_none());
}

#[sqlx::test]
async fn test_conversation_and_messages(pool: PgPool) {
    let repo = ChatRepository::new(pool);
    let tenant_id = Uuid::new_v4();

    let inbox = Inbox {
        id: Uuid::new_v4(),
        tenant_id,
        name: "Test Inbox".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    repo.create_inbox(tenant_id, &inbox).await.expect("Failed to create inbox");

    let contact = Contact {
        id: Uuid::new_v4(),
        tenant_id,
        name: "Test Contact".to_string(),
        phone_number: Some("+1234567890".to_string()),
        email: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    repo.create_contact(tenant_id, &contact).await.expect("Failed to create contact");

    let conversation = Conversation {
        id: Uuid::new_v4(),
        tenant_id,
        inbox_id: inbox.id,
        contact_id: contact.id,
        status: "open".to_string(),
        assignee_id: None,
        snoozed_until: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    let created_conv = repo.create_conversation(tenant_id, &conversation).await.expect("Failed to create conversation");
    assert_eq!(created_conv.status, "open");

    let message = Message {
        id: Uuid::new_v4(),
        tenant_id,
        conversation_id: conversation.id,
        channel_id: None,
        content: "Hello from native rust chat!".to_string(),
        message_type: "incoming".to_string(),
        status: "sent".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    let created_msg = repo.create_message(tenant_id, &message).await.expect("Failed to create message");
    assert_eq!(created_msg.content, "Hello from native rust chat!");
}
