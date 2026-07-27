use sqlx::PgPool;
use uuid::Uuid;
use super::db::ChatRepository;
use super::models::{ChatInbox, ChatConversation, ChatMessage};

pub struct ChatService {
    repo: ChatRepository,
}

impl ChatService {
    pub fn new(pool: PgPool) -> Self {
        Self {
            repo: ChatRepository::new(pool),
        }
    }

    pub async fn create_inbox(&self, tenant_id: Uuid, name: &str) -> Result<ChatInbox, sqlx::Error> {
        self.repo.create_inbox(tenant_id, name).await
    }

    pub async fn get_inboxes(&self, tenant_id: Uuid) -> Result<Vec<ChatInbox>, sqlx::Error> {
        self.repo.get_inboxes(tenant_id).await
    }

    pub async fn create_conversation(&self, tenant_id: Uuid, inbox_id: Uuid, contact_id: Uuid, status: &str) -> Result<ChatConversation, sqlx::Error> {
        self.repo.create_conversation(tenant_id, inbox_id, contact_id, status).await
    }

    pub async fn get_conversations(&self, tenant_id: Uuid, inbox_id: Uuid) -> Result<Vec<ChatConversation>, sqlx::Error> {
        self.repo.get_conversations(tenant_id, inbox_id).await
    }

    pub async fn add_message(&self, tenant_id: Uuid, conversation_id: Uuid, content: &str, message_type: &str, sender_id: Option<Uuid>) -> Result<ChatMessage, sqlx::Error> {
        self.repo.add_message(tenant_id, conversation_id, content, message_type, sender_id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(1, 1);
    }
}
