pub mod models;
pub mod services;
pub mod adapters;
pub mod gateway;
pub mod api;

#[cfg(test)]
mod tests {
    use super::*;
    use models::{ChannelType, MessageType};
    use services::ChatService;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_chat_service_flow() {
        let service = ChatService::new();
        let tenant_id = Uuid::new_v4();

        // 1. Create Inbox
        let inbox = service.create_inbox(tenant_id, "Main Inbox".to_string(), ChannelType::WebWidget).await;
        assert_eq!(inbox.name, "Main Inbox");

        // 2. Create Contact
        let contact = service.create_contact(tenant_id, Some("Alice".to_string()), None, None).await;
        assert_eq!(contact.name.unwrap(), "Alice");

        // 3. Create Conversation
        let conversation = service.create_conversation(tenant_id, inbox.id, contact.id).await;
        assert_eq!(conversation.inbox_id, inbox.id);

        // 4. Create Message
        let message = service.create_message(
            tenant_id,
            conversation.id,
            Some(contact.id),
            "Hello!".to_string(),
            MessageType::Incoming,
        ).await;
        assert_eq!(message.content, "Hello!");

        // 5. Query
        let msgs = service.get_messages_for_conversation(tenant_id, conversation.id).await;
        assert_eq!(msgs.len(), 1);
        assert_eq!(msgs[0].content, "Hello!");
    }
}
