use std::sync::Arc;
use sqlx::PgPool;
use uuid::Uuid;
use super::models::{ChatInbox, ChatChannel, ChatContact, ChatConversation, ChatMessage};
use super::repository::ChatRepository;
use crate::integrations::nats::client::NatsClientWrapper;

pub struct ChatService {
    repository: ChatRepository,
    nats_client: Arc<dyn NatsClientWrapper>,
}

impl ChatService {
    pub fn new(pool: PgPool, nats_client: Arc<dyn NatsClientWrapper>) -> Self {
        Self {
            repository: ChatRepository::new(pool),
            nats_client,
        }
    }

    pub async fn create_inbox(
        &self,
        tenant_id: Uuid,
        name: String,
    ) -> Result<ChatInbox, sqlx::Error> {
        self.repository.create_inbox(tenant_id, name).await
    }

    pub async fn create_channel(
        &self,
        tenant_id: Uuid,
        inbox_id: Uuid,
        channel_type: String,
        config: serde_json::Value,
    ) -> Result<ChatChannel, sqlx::Error> {
        self.repository.create_channel(tenant_id, inbox_id, channel_type, config).await
    }

    pub async fn create_contact(
        &self,
        tenant_id: Uuid,
        name: Option<String>,
        email: Option<String>,
        phone: Option<String>,
    ) -> Result<ChatContact, sqlx::Error> {
        self.repository.create_contact(tenant_id, name, email, phone).await
    }

    pub async fn start_conversation(
        &self,
        tenant_id: Uuid,
        inbox_id: Uuid,
        contact_id: Uuid,
        assignee_id: Option<Uuid>,
    ) -> Result<ChatConversation, sqlx::Error> {
        self.repository.start_conversation(tenant_id, inbox_id, contact_id, assignee_id).await
    }

    pub async fn send_message(
        &self,
        tenant_id: Uuid,
        conversation_id: Uuid,
        sender_type: String,
        sender_id: Option<Uuid>,
        content: String,
    ) -> Result<ChatMessage, sqlx::Error> {
        let message = self.repository.send_message(tenant_id, conversation_id, sender_type, sender_id, content).await?;

        let subject = format!("tenant.{}.chat.message_created", tenant_id);
        let payload = serde_json::to_vec(&message).unwrap_or_default();

        // Publish event for real-time clients. We log the error but don't fail the message creation.
        if let Err(e) = self.nats_client.publish(&subject, payload).await {
            tracing::error!("Failed to publish chat message event: {}", e);
        }

        Ok(message)
    }

    pub async fn get_conversations(
        &self,
        tenant_id: Uuid,
    ) -> Result<Vec<ChatConversation>, sqlx::Error> {
        self.repository.get_conversations(tenant_id).await
    }

    pub async fn get_messages(
        &self,
        tenant_id: Uuid,
        conversation_id: Uuid,
    ) -> Result<Vec<ChatMessage>, sqlx::Error> {
        self.repository.get_messages(tenant_id, conversation_id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::sync::Mutex;

    struct MockNatsClient {
        published_subjects: Mutex<Vec<String>>,
    }

    impl MockNatsClient {
        fn new() -> Self {
            Self {
                published_subjects: Mutex::new(Vec::new()),
            }
        }
    }

    #[async_trait]
    impl NatsClientWrapper for MockNatsClient {
        async fn publish(&self, subject: &str, _data: Vec<u8>) -> Result<(), String> {
            let mut subjects = self.published_subjects.lock().unwrap();
            subjects.push(subject.to_string());
            Ok(())
        }

        async fn subscribe(
            &self,
            _subject: &str,
            _handler: Box<dyn Fn(Vec<u8>) + Send + Sync>,
        ) -> Result<Box<dyn Fn() + Send + Sync>, String> {
            Ok(Box::new(|| {}))
        }
    }

    #[tokio::test]
    async fn test_service_dummy() {
        // Just checking that we can compile and write tests here
        assert_eq!(1, 1);
    }
}
