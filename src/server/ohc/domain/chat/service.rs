use crate::domain::chat::models::{Conversation, ConversationStatus, Message, MessageStatus, MessageType, Contact};
use chrono::Utc;
use serde_json::Value;
use uuid::Uuid;

pub struct ChatService {}

impl ChatService {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn create_contact(
        &self,
        tenant_id: Uuid,
        account_id: Uuid,
        name: Option<String>,
        email: Option<String>,
        phone_number: Option<String>,
        identifier: Option<String>,
    ) -> Result<Contact, String> {
        let contact = Contact {
            id: Uuid::new_v4(),
            tenant_id,
            account_id,
            name,
            email,
            phone_number,
            avatar_url: None,
            identifier,
            additional_attributes: None,
            custom_attributes: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // TODO: Wire up to Postgres via main repo
        Ok(contact)
    }

    pub async fn create_conversation(
        &self,
        tenant_id: Uuid,
        account_id: Uuid,
        inbox_id: Uuid,
        contact_id: Uuid,
        custom_attributes: Option<Value>,
    ) -> Result<Conversation, String> {
        let conversation = Conversation {
            id: Uuid::new_v4(),
            tenant_id,
            account_id,
            inbox_id,
            contact_id,
            assignee_id: None,
            status: ConversationStatus::Open,
            additional_attributes: None,
            custom_attributes,
            snoozed_until: None,
            last_activity_at: Utc::now(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // TODO: Wire up to Postgres via main repo
        Ok(conversation)
    }

    pub async fn send_message(
        &self,
        tenant_id: Uuid,
        account_id: Uuid,
        inbox_id: Uuid,
        conversation_id: Uuid,
        content: String,
        sender_id: Option<Uuid>,
        is_incoming: bool,
    ) -> Result<Message, String> {
        let message_type = if is_incoming { MessageType::Incoming } else { MessageType::Outgoing };
        let sender_type = if is_incoming { "contact" } else { "agent" };

        let message = Message {
            id: Uuid::new_v4(),
            tenant_id,
            account_id,
            inbox_id,
            conversation_id,
            message_type,
            content: Some(content),
            status: MessageStatus::Sent,
            sender_id,
            sender_type: Some(sender_type.to_string()),
            source_id: None,
            additional_attributes: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // TODO: Wire up to Postgres via main repo and WebSockets
        Ok(message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_contact() {
        let service = ChatService::new();
        let tenant_id = Uuid::new_v4();
        let contact = futures::executor::block_on(service.create_contact(tenant_id, Uuid::new_v4(), Some("Test".to_string()), None, None, None)).unwrap();
        assert_eq!(contact.tenant_id, tenant_id);
    }

    #[test]
    fn test_create_conversation() {
        let service = ChatService::new();
        let tenant_id = Uuid::new_v4();
        let conversation = futures::executor::block_on(service.create_conversation(tenant_id, Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4(), None)).unwrap();
        assert_eq!(conversation.tenant_id, tenant_id);
    }

    #[test]
    fn test_send_message() {
        let service = ChatService::new();
        let tenant_id = Uuid::new_v4();
        let message = futures::executor::block_on(service.send_message(tenant_id, Uuid::new_v4(), Uuid::new_v4(), Uuid::new_v4(), "Hello".to_string(), None, true)).unwrap();
        assert_eq!(message.tenant_id, tenant_id);
        assert_eq!(message.message_type, MessageType::Incoming);
    }
}
