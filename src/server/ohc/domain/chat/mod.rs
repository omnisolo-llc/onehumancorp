pub mod models;
pub mod repository;
pub mod service;

#[cfg(test)]
mod tests {
    use super::models::{Conversation, ConversationStatus, Message};
    use super::repository::ChatRepository;
    use super::service::ChatService;
    use async_trait::async_trait;
    use chrono::Utc;
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};
    use uuid::Uuid;

    struct MockChatRepository {
        conversations: Mutex<HashMap<Uuid, Conversation>>,
        messages: Mutex<Vec<Message>>,
    }

    impl MockChatRepository {
        fn new() -> Self {
            Self {
                conversations: Mutex::new(HashMap::new()),
                messages: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl ChatRepository for MockChatRepository {
        async fn create_conversation(
            &self,
            _tenant_id: Uuid,
            conversation: Conversation,
        ) -> Result<Conversation, String> {
            let mut convs = self.conversations.lock().unwrap();
            convs.insert(conversation.id, conversation.clone());
            Ok(conversation)
        }

        async fn get_conversation(
            &self,
            tenant_id: Uuid,
            conversation_id: Uuid,
        ) -> Result<Option<Conversation>, String> {
            let convs = self.conversations.lock().unwrap();
            if let Some(conv) = convs.get(&conversation_id) {
                if conv.tenant_id == tenant_id {
                    return Ok(Some(conv.clone()));
                }
            }
            Ok(None)
        }

        async fn add_message(
            &self,
            _tenant_id: Uuid,
            message: Message,
        ) -> Result<Message, String> {
            let mut msgs = self.messages.lock().unwrap();
            msgs.push(message.clone());
            Ok(message)
        }

        async fn get_messages_for_conversation(
            &self,
            tenant_id: Uuid,
            conversation_id: Uuid,
        ) -> Result<Vec<Message>, String> {
            let msgs = self.messages.lock().unwrap();
            let filtered: Vec<Message> = msgs
                .iter()
                .filter(|m| m.conversation_id == conversation_id && m.tenant_id == tenant_id)
                .cloned()
                .collect();
            Ok(filtered)
        }

        async fn create_inbox(
            &self,
            _tenant_id: Uuid,
            inbox: super::models::Inbox,
        ) -> Result<super::models::Inbox, String> {
            Ok(inbox)
        }

        async fn create_contact(
            &self,
            _tenant_id: Uuid,
            contact: super::models::Contact,
        ) -> Result<super::models::Contact, String> {
            Ok(contact)
        }
    }

    #[tokio::test]
    async fn test_start_conversation() {
        let repo = Arc::new(MockChatRepository::new());
        let service = ChatService::new(repo);

        let tenant_id = Uuid::new_v4();
        let conversation = Conversation {
            id: Uuid::new_v4(),
            tenant_id,
            inbox_id: Uuid::new_v4(),
            contact_id: Uuid::new_v4(),
            status: ConversationStatus::Open,
            priority: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let result = service.start_conversation(tenant_id, conversation.clone()).await;
        assert!(result.is_ok());
        let saved = result.unwrap();
        assert_eq!(saved.id, conversation.id);
    }

    #[tokio::test]
    async fn test_start_conversation_tenant_mismatch() {
        let repo = Arc::new(MockChatRepository::new());
        let service = ChatService::new(repo);

        let tenant_id1 = Uuid::new_v4();
        let tenant_id2 = Uuid::new_v4();
        let conversation = Conversation {
            id: Uuid::new_v4(),
            tenant_id: tenant_id2,
            inbox_id: Uuid::new_v4(),
            contact_id: Uuid::new_v4(),
            status: ConversationStatus::Open,
            priority: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let result = service.start_conversation(tenant_id1, conversation).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Tenant ID mismatch");
    }

    #[tokio::test]
    async fn test_add_message_to_conversation() {
        let repo = Arc::new(MockChatRepository::new());
        let service = ChatService::new(repo);

        let tenant_id = Uuid::new_v4();
        let conversation = Conversation {
            id: Uuid::new_v4(),
            tenant_id,
            inbox_id: Uuid::new_v4(),
            contact_id: Uuid::new_v4(),
            status: ConversationStatus::Open,
            priority: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        service.start_conversation(tenant_id, conversation.clone()).await.unwrap();

        let message = Message {
            id: Uuid::new_v4(),
            tenant_id,
            conversation_id: conversation.id,
            sender_type: "customer".to_string(),
            content: "Hello!".to_string(),
            created_at: Utc::now(),
        };

        let result = service.add_message_to_conversation(tenant_id, message.clone()).await;
        assert!(result.is_ok());
        let saved = result.unwrap();
        assert_eq!(saved.id, message.id);

        let history = service.get_conversation_history(tenant_id, conversation.id).await.unwrap();
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].id, message.id);
    }

    #[tokio::test]
    async fn test_add_message_tenant_mismatch() {
        let repo = Arc::new(MockChatRepository::new());
        let service = ChatService::new(repo);

        let tenant_id1 = Uuid::new_v4();
        let tenant_id2 = Uuid::new_v4();

        let message = Message {
            id: Uuid::new_v4(),
            tenant_id: tenant_id2,
            conversation_id: Uuid::new_v4(),
            sender_type: "customer".to_string(),
            content: "Hello!".to_string(),
            created_at: Utc::now(),
        };

        let result = service.add_message_to_conversation(tenant_id1, message).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Tenant ID mismatch");
    }

    #[tokio::test]
    async fn test_add_message_conversation_not_found() {
        let repo = Arc::new(MockChatRepository::new());
        let service = ChatService::new(repo);

        let tenant_id = Uuid::new_v4();

        let message = Message {
            id: Uuid::new_v4(),
            tenant_id,
            conversation_id: Uuid::new_v4(),
            sender_type: "customer".to_string(),
            content: "Hello!".to_string(),
            created_at: Utc::now(),
        };

        let result = service.add_message_to_conversation(tenant_id, message).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Conversation not found");
    }
}
