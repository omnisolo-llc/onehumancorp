use crate::omnichannel::models::{Tenant, Contact, Inbox, Conversation, Message};
use uuid::Uuid;
use chrono::Utc;

#[test]
fn test_models_creation() {
    let tenant_id = Uuid::new_v4();
    let tenant = Tenant {
        id: tenant_id,
        name: "Maya's Bakery".to_string(),
    };

    assert_eq!(tenant.name, "Maya's Bakery");

    let contact = Contact {
        id: Uuid::new_v4(),
        tenant_id,
        name: Some("Customer Sarah".to_string()),
        email: Some("sarah@example.com".to_string()),
        phone_number: None,
        custom_attributes: None,
    };

    assert_eq!(contact.tenant_id, tenant_id);

    let inbox = Inbox {
        id: Uuid::new_v4(),
        tenant_id,
        name: "Instagram DM".to_string(),
        channel_type: "instagram".to_string(),
        channel_credentials: None,
    };

    let conversation = Conversation {
        id: Uuid::new_v4(),
        tenant_id,
        inbox_id: inbox.id,
        contact_id: contact.id,
        status: "open".to_string(),
        last_activity_at: Utc::now(),
    };

    let message = Message {
        id: Uuid::new_v4(),
        tenant_id,
        conversation_id: conversation.id,
        content: "Do you make vegan cakes?".to_string(),
        message_type: "incoming".to_string(),
        created_at: Utc::now(),
    };

    assert_eq!(message.tenant_id, tenant_id);
    assert_eq!(message.message_type, "incoming");
}
