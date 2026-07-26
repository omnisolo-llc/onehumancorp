pub mod models;
pub mod adapter;
pub mod service;

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_chat_service_e2e() {
        let adapter = Arc::new(adapter::LocalApiAdapter::new());
        let service = service::ChatService::new(adapter.clone());

        let tenant_id = Uuid::new_v4();
        let contact_id = Uuid::new_v4();
        let inbox_id = Uuid::new_v4();

        // 1. Create a conversation
        let conversation = service
            .create_conversation(tenant_id, contact_id, inbox_id)
            .await
            .unwrap();

        assert_eq!(conversation.tenant_id, tenant_id);
        assert_eq!(conversation.contact_id, contact_id);
        assert_eq!(conversation.inbox_id, inbox_id);
        assert_eq!(conversation.status, "open");

        // 2. Process an incoming webhook
        let incoming_msg = service
            .process_incoming_webhook(tenant_id, conversation.id, "Hello, I need help!".to_string(), contact_id)
            .await
            .unwrap();

        assert_eq!(incoming_msg.tenant_id, tenant_id);
        assert_eq!(incoming_msg.conversation_id, conversation.id);
        assert_eq!(incoming_msg.content, "Hello, I need help!");
        assert_eq!(incoming_msg.sender_type, "contact");
        assert_eq!(incoming_msg.sender_id, contact_id);

        // 3. Dispatch an outgoing message
        let agent_id = Uuid::new_v4();
        let outgoing_msg = service
            .dispatch_outgoing_message(tenant_id, conversation.id, "Hi, how can I assist you?".to_string(), agent_id)
            .await
            .unwrap();

        assert_eq!(outgoing_msg.tenant_id, tenant_id);
        assert_eq!(outgoing_msg.conversation_id, conversation.id);
        assert_eq!(outgoing_msg.content, "Hi, how can I assist you?");
        assert_eq!(outgoing_msg.sender_type, "agent");
        assert_eq!(outgoing_msg.sender_id, agent_id);

        // Verify the message was sent via the adapter
        let sent_messages = adapter.sent_messages.lock().await;
        assert_eq!(sent_messages.len(), 1);
        assert_eq!(sent_messages[0].id, outgoing_msg.id);
        assert_eq!(sent_messages[0].content, "Hi, how can I assist you?");
    }
}
