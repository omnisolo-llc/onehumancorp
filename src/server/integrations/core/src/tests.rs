#[cfg(test)]
mod tests {
    use crate::models::*;
    use crate::repository::OmnichannelRepository;
    use crate::service::OmnichannelService;
    use async_trait::async_trait;
    use uuid::Uuid;
    use std::sync::Arc;
    use std::sync::Mutex;

    struct MockRepo {
        messages: Mutex<Vec<Message>>,
        outbox: Mutex<Vec<OutboxMessage>>,
    }

    impl MockRepo {
        fn new() -> Self {
            Self {
                messages: Mutex::new(Vec::new()),
                outbox: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl OmnichannelRepository for MockRepo {
        async fn create_contact(&self, _tenant_id: Uuid, contact: Contact) -> Result<Contact, String> { Ok(contact) }
        async fn get_contact(&self, _tenant_id: Uuid, _id: Uuid) -> Result<Option<Contact>, String> { Ok(None) }
        async fn create_inbox(&self, _tenant_id: Uuid, inbox: Inbox) -> Result<Inbox, String> { Ok(inbox) }
        async fn get_inbox(&self, _tenant_id: Uuid, _id: Uuid) -> Result<Option<Inbox>, String> { Ok(None) }
        async fn create_conversation(&self, _tenant_id: Uuid, conversation: Conversation) -> Result<Conversation, String> { Ok(conversation) }
        async fn get_conversation(&self, _tenant_id: Uuid, _id: Uuid) -> Result<Option<Conversation>, String> { Ok(None) }

        async fn create_message(&self, _tenant_id: Uuid, message: Message) -> Result<Message, String> {
            self.messages.lock().unwrap().push(message.clone());
            Ok(message)
        }

        async fn get_message(&self, _tenant_id: Uuid, _id: Uuid) -> Result<Option<Message>, String> { Ok(None) }
        async fn get_messages_for_conversation(&self, _tenant_id: Uuid, _conversation_id: Uuid) -> Result<Vec<Message>, String> { Ok(Vec::new()) }

        async fn enqueue_outbox_message(&self, _tenant_id: Uuid, outbox_msg: OutboxMessage) -> Result<OutboxMessage, String> {
            self.outbox.lock().unwrap().push(outbox_msg.clone());
            Ok(outbox_msg)
        }

        async fn fetch_pending_outbox_messages(&self, _limit: i64) -> Result<Vec<OutboxMessage>, String> { Ok(Vec::new()) }
        async fn mark_outbox_message_completed(&self, _tenant_id: Uuid, _id: Uuid) -> Result<(), String> { Ok(()) }
        async fn mark_outbox_message_failed(&self, _tenant_id: Uuid, _id: Uuid, _attempt_increment: bool) -> Result<(), String> { Ok(()) }
    }

    #[tokio::test]
    async fn test_handle_incoming_message() {
        let repo = Arc::new(MockRepo::new());
        let service = OmnichannelService::new(repo.clone());

        let tenant_id = Uuid::new_v4();
        let inbox_id = Uuid::new_v4();
        let contact_id = Uuid::new_v4();
        let content = "Hello".to_string();

        let msg = service.handle_incoming_message(tenant_id, inbox_id, contact_id, content.clone()).await.unwrap();

        assert_eq!(msg.content, content);
        assert_eq!(msg.tenant_id, tenant_id);
        assert_eq!(msg.sender_type, "contact");
    }

    #[tokio::test]
    async fn test_draft_and_send_reply() {
        let repo = Arc::new(MockRepo::new());
        let service = OmnichannelService::new(repo.clone());

        let tenant_id = Uuid::new_v4();
        let conversation_id = Uuid::new_v4();
        let content = "Reply".to_string();

        let msg = service.draft_and_send_reply(tenant_id, conversation_id, content.clone(), "instagram".to_string()).await.unwrap();

        assert_eq!(msg.content, content);
        assert_eq!(msg.status, "pending");
    }
}
