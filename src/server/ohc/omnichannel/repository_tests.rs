use super::models::{ChannelConnection, Contact, Conversation, Inbox, Message, MessageType, ReceiptStatus};
use super::repository::{InMemoryOmnichannelRepository, OmnichannelRepository};

#[test]
fn test_tenant_isolation_inbox() {
    let mut repo = InMemoryOmnichannelRepository::new();
    let inbox = Inbox {
        tenant_id: "tenant-a".to_string(),
        id: "inbox-1".to_string(),
        name: "Support".to_string(),
    };
    repo.create_inbox(inbox).unwrap();

    // Cross-tenant access should fail
    assert!(repo.get_inbox("tenant-b", "inbox-1").is_none());
    assert!(repo.update_inbox("tenant-b", "inbox-1", "Sales".to_string()).is_err());
    assert!(repo.delete_inbox("tenant-b", "inbox-1").is_err());

    // Same-tenant access should succeed
    assert!(repo.get_inbox("tenant-a", "inbox-1").is_some());
    assert!(repo.update_inbox("tenant-a", "inbox-1", "Sales".to_string()).is_ok());
    assert_eq!(repo.get_inbox("tenant-a", "inbox-1").unwrap().name, "Sales");
    assert!(repo.delete_inbox("tenant-a", "inbox-1").is_ok());
    assert!(repo.get_inbox("tenant-a", "inbox-1").is_none());
}

#[test]
fn test_tenant_isolation_channel() {
    let mut repo = InMemoryOmnichannelRepository::new();
    let channel = ChannelConnection {
        tenant_id: "tenant-a".to_string(),
        id: "channel-1".to_string(),
        inbox_id: "inbox-1".to_string(),
        provider: "whatsapp".to_string(),
        capabilities: vec![],
    };
    repo.create_channel(channel).unwrap();

    assert!(repo.get_channel("tenant-b", "channel-1").is_none());
    assert!(repo.update_channel("tenant-b", "channel-1", vec!["test".to_string()]).is_err());
    assert!(repo.delete_channel("tenant-b", "channel-1").is_err());

    assert!(repo.get_channel("tenant-a", "channel-1").is_some());
    assert!(repo.update_channel("tenant-a", "channel-1", vec!["test".to_string()]).is_ok());
    assert_eq!(repo.get_channel("tenant-a", "channel-1").unwrap().capabilities.len(), 1);
    assert!(repo.delete_channel("tenant-a", "channel-1").is_ok());
    assert!(repo.get_channel("tenant-a", "channel-1").is_none());
}

#[test]
fn test_tenant_isolation_contact() {
    let mut repo = InMemoryOmnichannelRepository::new();
    let contact = Contact {
        tenant_id: "tenant-a".to_string(),
        id: "contact-1".to_string(),
        name: "Maya".to_string(),
        identity: "maya@example.com".to_string(),
    };
    repo.create_contact(contact).unwrap();

    assert!(repo.get_contact("tenant-b", "contact-1").is_none());
    assert!(repo.update_contact("tenant-b", "contact-1", "Maya Baker".to_string()).is_err());
    assert!(repo.delete_contact("tenant-b", "contact-1").is_err());

    assert!(repo.get_contact("tenant-a", "contact-1").is_some());
    assert!(repo.update_contact("tenant-a", "contact-1", "Maya Baker".to_string()).is_ok());
    assert_eq!(repo.get_contact("tenant-a", "contact-1").unwrap().name, "Maya Baker");
    assert!(repo.delete_contact("tenant-a", "contact-1").is_ok());
    assert!(repo.get_contact("tenant-a", "contact-1").is_none());
}

#[test]
fn test_tenant_isolation_conversation() {
    let mut repo = InMemoryOmnichannelRepository::new();
    let conv = Conversation {
        tenant_id: "tenant-a".to_string(),
        id: "conv-1".to_string(),
        channel_id: "channel-1".to_string(),
        contact_id: "contact-1".to_string(),
        status: "open".to_string(),
    };
    repo.create_conversation(conv).unwrap();

    assert!(repo.get_conversation("tenant-b", "conv-1").is_none());
    assert!(repo.update_conversation("tenant-b", "conv-1", "closed".to_string()).is_err());
    assert!(repo.delete_conversation("tenant-b", "conv-1").is_err());

    assert!(repo.get_conversation("tenant-a", "conv-1").is_some());
    assert!(repo.update_conversation("tenant-a", "conv-1", "closed".to_string()).is_ok());
    assert_eq!(repo.get_conversation("tenant-a", "conv-1").unwrap().status, "closed");
    assert!(repo.delete_conversation("tenant-a", "conv-1").is_ok());
    assert!(repo.get_conversation("tenant-a", "conv-1").is_none());
}

#[test]
fn test_tenant_isolation_message() {
    let mut repo = InMemoryOmnichannelRepository::new();
    let msg = Message {
        tenant_id: "tenant-a".to_string(),
        id: "msg-1".to_string(),
        conversation_id: "conv-1".to_string(),
        content: "Hello".to_string(),
        message_type: MessageType::Inbound,
        receipt_status: ReceiptStatus::Delivered,
    };
    repo.create_message(msg).unwrap();

    assert!(repo.get_message("tenant-b", "msg-1").is_none());
    assert!(repo.update_message_receipt("tenant-b", "msg-1", ReceiptStatus::Read).is_err());
    assert!(repo.delete_message("tenant-b", "msg-1").is_err());

    assert!(repo.get_message("tenant-a", "msg-1").is_some());
    assert!(repo.update_message_receipt("tenant-a", "msg-1", ReceiptStatus::Read).is_ok());
    assert_eq!(repo.get_message("tenant-a", "msg-1").unwrap().receipt_status, ReceiptStatus::Read);
    assert!(repo.delete_message("tenant-a", "msg-1").is_ok());
    assert!(repo.get_message("tenant-a", "msg-1").is_none());
}
