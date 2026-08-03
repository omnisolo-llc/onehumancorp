use crate::domain::chat::models::{Conversation, Message};

// Mock gRPC API service
pub struct ChatEngineApi;

impl ChatEngineApi {
    pub fn create_conversation(tenant_id: String, inbox_id: String, contact_id: String) -> Conversation {
        Conversation::new(tenant_id, inbox_id, contact_id)
    }

    pub fn create_message(tenant_id: String, conversation_id: String, sender_type: String, content: String) -> Message {
        Message::new(tenant_id, conversation_id, sender_type, content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_conversation_api() {
        let tenant_id = "tenant-123".to_string();
        let inbox_id = "inbox-1".to_string();
        let contact_id = "contact-1".to_string();

        let conversation = ChatEngineApi::create_conversation(tenant_id.clone(), inbox_id.clone(), contact_id.clone());

        assert_eq!(conversation.tenant_id, tenant_id);
        assert_eq!(conversation.inbox_id, inbox_id);
        assert_eq!(conversation.contact_id, contact_id);
    }

    #[test]
    fn test_create_message_api() {
        let tenant_id = "tenant-123".to_string();
        let conversation_id = "conv-1".to_string();
        let sender_type = "customer".to_string();
        let content = "Is this vegan?".to_string();

        let message = ChatEngineApi::create_message(tenant_id.clone(), conversation_id.clone(), sender_type.clone(), content.clone());

        assert_eq!(message.tenant_id, tenant_id);
        assert_eq!(message.conversation_id, conversation_id);
        assert_eq!(message.sender_type, sender_type);
        assert_eq!(message.content, content);
    }
}
