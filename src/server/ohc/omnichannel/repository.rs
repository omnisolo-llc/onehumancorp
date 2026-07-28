use super::models::{ChannelConnection, Contact, Conversation, Inbox, Message};

pub trait OmnichannelRepository {
    // Inbox operations
    fn create_inbox(&mut self, inbox: Inbox) -> Result<(), String>;
    fn get_inbox(&self, tenant_id: &str, inbox_id: &str) -> Option<Inbox>;
    fn update_inbox(&mut self, tenant_id: &str, inbox_id: &str, name: String) -> Result<(), String>;
    fn delete_inbox(&mut self, tenant_id: &str, inbox_id: &str) -> Result<(), String>;

    // ChannelConnection operations
    fn create_channel(&mut self, channel: ChannelConnection) -> Result<(), String>;
    fn get_channel(&self, tenant_id: &str, channel_id: &str) -> Option<ChannelConnection>;
    fn update_channel(&mut self, tenant_id: &str, channel_id: &str, capabilities: Vec<String>) -> Result<(), String>;
    fn delete_channel(&mut self, tenant_id: &str, channel_id: &str) -> Result<(), String>;

    // Contact operations
    fn create_contact(&mut self, contact: Contact) -> Result<(), String>;
    fn get_contact(&self, tenant_id: &str, contact_id: &str) -> Option<Contact>;
    fn update_contact(&mut self, tenant_id: &str, contact_id: &str, name: String) -> Result<(), String>;
    fn delete_contact(&mut self, tenant_id: &str, contact_id: &str) -> Result<(), String>;

    // Conversation operations
    fn create_conversation(&mut self, conversation: Conversation) -> Result<(), String>;
    fn get_conversation(&self, tenant_id: &str, conversation_id: &str) -> Option<Conversation>;
    fn update_conversation(&mut self, tenant_id: &str, conversation_id: &str, status: String) -> Result<(), String>;
    fn delete_conversation(&mut self, tenant_id: &str, conversation_id: &str) -> Result<(), String>;

    // Message operations
    fn create_message(&mut self, message: Message) -> Result<(), String>;
    fn get_message(&self, tenant_id: &str, message_id: &str) -> Option<Message>;
    fn update_message_receipt(&mut self, tenant_id: &str, message_id: &str, status: crate::omnichannel::models::ReceiptStatus) -> Result<(), String>;
    fn delete_message(&mut self, tenant_id: &str, message_id: &str) -> Result<(), String>;
}

// In-memory implementation for testing
pub struct InMemoryOmnichannelRepository {
    inboxes: Vec<Inbox>,
    channels: Vec<ChannelConnection>,
    contacts: Vec<Contact>,
    conversations: Vec<Conversation>,
    messages: Vec<Message>,
}

impl InMemoryOmnichannelRepository {
    pub fn new() -> Self {
        Self {
            inboxes: Vec::new(),
            channels: Vec::new(),
            contacts: Vec::new(),
            conversations: Vec::new(),
            messages: Vec::new(),
        }
    }
}

impl OmnichannelRepository for InMemoryOmnichannelRepository {
    fn create_inbox(&mut self, inbox: Inbox) -> Result<(), String> {
        self.inboxes.push(inbox);
        Ok(())
    }

    fn get_inbox(&self, tenant_id: &str, inbox_id: &str) -> Option<Inbox> {
        self.inboxes
            .iter()
            .find(|i| i.tenant_id == tenant_id && i.id == inbox_id)
            .cloned()
    }

    fn update_inbox(&mut self, tenant_id: &str, inbox_id: &str, name: String) -> Result<(), String> {
        if let Some(inbox) = self.inboxes.iter_mut().find(|i| i.tenant_id == tenant_id && i.id == inbox_id) {
            inbox.name = name;
            Ok(())
        } else {
            Err("Inbox not found".to_string())
        }
    }

    fn delete_inbox(&mut self, tenant_id: &str, inbox_id: &str) -> Result<(), String> {
        let len_before = self.inboxes.len();
        self.inboxes.retain(|i| !(i.tenant_id == tenant_id && i.id == inbox_id));
        if self.inboxes.len() < len_before {
            Ok(())
        } else {
            Err("Inbox not found".to_string())
        }
    }

    fn create_channel(&mut self, channel: ChannelConnection) -> Result<(), String> {
        self.channels.push(channel);
        Ok(())
    }

    fn get_channel(&self, tenant_id: &str, channel_id: &str) -> Option<ChannelConnection> {
        self.channels
            .iter()
            .find(|c| c.tenant_id == tenant_id && c.id == channel_id)
            .cloned()
    }

    fn update_channel(&mut self, tenant_id: &str, channel_id: &str, capabilities: Vec<String>) -> Result<(), String> {
        if let Some(channel) = self.channels.iter_mut().find(|c| c.tenant_id == tenant_id && c.id == channel_id) {
            channel.capabilities = capabilities;
            Ok(())
        } else {
            Err("Channel not found".to_string())
        }
    }

    fn delete_channel(&mut self, tenant_id: &str, channel_id: &str) -> Result<(), String> {
        let len_before = self.channels.len();
        self.channels.retain(|c| !(c.tenant_id == tenant_id && c.id == channel_id));
        if self.channels.len() < len_before {
            Ok(())
        } else {
            Err("Channel not found".to_string())
        }
    }

    fn create_contact(&mut self, contact: Contact) -> Result<(), String> {
        self.contacts.push(contact);
        Ok(())
    }

    fn get_contact(&self, tenant_id: &str, contact_id: &str) -> Option<Contact> {
        self.contacts
            .iter()
            .find(|c| c.tenant_id == tenant_id && c.id == contact_id)
            .cloned()
    }

    fn update_contact(&mut self, tenant_id: &str, contact_id: &str, name: String) -> Result<(), String> {
        if let Some(contact) = self.contacts.iter_mut().find(|c| c.tenant_id == tenant_id && c.id == contact_id) {
            contact.name = name;
            Ok(())
        } else {
            Err("Contact not found".to_string())
        }
    }

    fn delete_contact(&mut self, tenant_id: &str, contact_id: &str) -> Result<(), String> {
        let len_before = self.contacts.len();
        self.contacts.retain(|c| !(c.tenant_id == tenant_id && c.id == contact_id));
        if self.contacts.len() < len_before {
            Ok(())
        } else {
            Err("Contact not found".to_string())
        }
    }

    fn create_conversation(&mut self, conversation: Conversation) -> Result<(), String> {
        self.conversations.push(conversation);
        Ok(())
    }

    fn get_conversation(&self, tenant_id: &str, conversation_id: &str) -> Option<Conversation> {
        self.conversations
            .iter()
            .find(|c| c.tenant_id == tenant_id && c.id == conversation_id)
            .cloned()
    }

    fn update_conversation(&mut self, tenant_id: &str, conversation_id: &str, status: String) -> Result<(), String> {
        if let Some(conversation) = self.conversations.iter_mut().find(|c| c.tenant_id == tenant_id && c.id == conversation_id) {
            conversation.status = status;
            Ok(())
        } else {
            Err("Conversation not found".to_string())
        }
    }

    fn delete_conversation(&mut self, tenant_id: &str, conversation_id: &str) -> Result<(), String> {
        let len_before = self.conversations.len();
        self.conversations.retain(|c| !(c.tenant_id == tenant_id && c.id == conversation_id));
        if self.conversations.len() < len_before {
            Ok(())
        } else {
            Err("Conversation not found".to_string())
        }
    }

    fn create_message(&mut self, message: Message) -> Result<(), String> {
        self.messages.push(message);
        Ok(())
    }

    fn get_message(&self, tenant_id: &str, message_id: &str) -> Option<Message> {
        self.messages
            .iter()
            .find(|m| m.tenant_id == tenant_id && m.id == message_id)
            .cloned()
    }

    fn update_message_receipt(&mut self, tenant_id: &str, message_id: &str, status: crate::omnichannel::models::ReceiptStatus) -> Result<(), String> {
        if let Some(message) = self.messages.iter_mut().find(|m| m.tenant_id == tenant_id && m.id == message_id) {
            message.receipt_status = status;
            Ok(())
        } else {
            Err("Message not found".to_string())
        }
    }

    fn delete_message(&mut self, tenant_id: &str, message_id: &str) -> Result<(), String> {
        let len_before = self.messages.len();
        self.messages.retain(|m| !(m.tenant_id == tenant_id && m.id == message_id));
        if self.messages.len() < len_before {
            Ok(())
        } else {
            Err("Message not found".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::omnichannel::models::{MessageType, ReceiptStatus};

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
}
