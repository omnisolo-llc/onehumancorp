// Temporarily using String instead of Uuid and Chrono to avoid workspace issues for now.
#[derive(Debug, Clone, PartialEq)]
pub struct Inbox {
    pub id: String,
    pub tenant_id: String,
    pub name: String,
    pub channel_type: String, // e.g., "instagram", "whatsapp"
}

#[derive(Debug, Clone, PartialEq)]
pub struct Conversation {
    pub id: String,
    pub tenant_id: String,
    pub inbox_id: String,
    pub contact_id: String,
    pub status: String, // e.g., "open", "resolved", "waiting_for_customer"
}

#[derive(Debug, Clone, PartialEq)]
pub struct Message {
    pub id: String,
    pub tenant_id: String,
    pub conversation_id: String,
    pub sender_type: String, // e.g., "customer", "agent", "bot"
    pub content: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Contact {
    pub id: String,
    pub tenant_id: String,
    pub name: String,
    pub identifier: String, // e.g., phone number, email, IG handle
}

impl Inbox {
    pub fn new(tenant_id: String, name: String, channel_type: String) -> Self {
        Self {
            id: "uuid-placeholder".to_string(),
            tenant_id,
            name,
            channel_type,
        }
    }
}

impl Conversation {
    pub fn new(tenant_id: String, inbox_id: String, contact_id: String) -> Self {
        Self {
            id: "uuid-placeholder".to_string(),
            tenant_id,
            inbox_id,
            contact_id,
            status: "open".to_string(),
        }
    }
}

impl Message {
    pub fn new(tenant_id: String, conversation_id: String, sender_type: String, content: String) -> Self {
        Self {
            id: "uuid-placeholder".to_string(),
            tenant_id,
            conversation_id,
            sender_type,
            content,
        }
    }
}

impl Contact {
    pub fn new(tenant_id: String, name: String, identifier: String) -> Self {
        Self {
            id: "uuid-placeholder".to_string(),
            tenant_id,
            name,
            identifier,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inbox_creation() {
        let tenant_id = "tenant-123".to_string();
        let name = "IG Messages".to_string();
        let channel_type = "instagram".to_string();

        let inbox = Inbox::new(tenant_id.clone(), name.clone(), channel_type.clone());

        assert_eq!(inbox.tenant_id, tenant_id);
        assert_eq!(inbox.name, name);
        assert_eq!(inbox.channel_type, channel_type);
    }

    #[test]
    fn test_conversation_creation() {
        let tenant_id = "tenant-123".to_string();
        let inbox_id = "inbox-1".to_string();
        let contact_id = "contact-1".to_string();

        let conversation = Conversation::new(tenant_id.clone(), inbox_id.clone(), contact_id.clone());

        assert_eq!(conversation.tenant_id, tenant_id);
        assert_eq!(conversation.inbox_id, inbox_id);
        assert_eq!(conversation.contact_id, contact_id);
        assert_eq!(conversation.status, "open");
    }

    #[test]
    fn test_message_creation() {
        let tenant_id = "tenant-123".to_string();
        let conversation_id = "conv-1".to_string();
        let sender_type = "agent".to_string();
        let content = "Hello there!".to_string();

        let message = Message::new(tenant_id.clone(), conversation_id.clone(), sender_type.clone(), content.clone());

        assert_eq!(message.tenant_id, tenant_id);
        assert_eq!(message.conversation_id, conversation_id);
        assert_eq!(message.sender_type, sender_type);
        assert_eq!(message.content, content);
    }

    #[test]
    fn test_contact_creation() {
        let tenant_id = "tenant-123".to_string();
        let name = "Maya Customer".to_string();
        let identifier = "@maya_customer".to_string();

        let contact = Contact::new(tenant_id.clone(), name.clone(), identifier.clone());

        assert_eq!(contact.tenant_id, tenant_id);
        assert_eq!(contact.name, name);
        assert_eq!(contact.identifier, identifier);
    }
}
