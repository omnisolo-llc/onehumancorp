use super::{repository::OmnichannelRepository, Conversation, Inbox, Message};
use server_common::error::Result;
use sqlx::PgPool;
use uuid::Uuid;

pub struct OmnichannelService {
    repo: OmnichannelRepository,
}

impl OmnichannelService {
    pub fn new(pool: PgPool) -> Self {
        Self {
            repo: OmnichannelRepository::new(pool),
        }
    }

    pub async fn create_inbox(&self, tenant_id: &str, name: &str) -> Result<Inbox> {
        self.repo.create_inbox(tenant_id, name).await
    }

    pub async fn list_inboxes(&self, tenant_id: &str) -> Result<Vec<Inbox>> {
        self.repo.list_inboxes(tenant_id).await
    }

    pub async fn create_conversation(
        &self,
        tenant_id: &str,
        inbox_id: Uuid,
        contact_id: Uuid,
    ) -> Result<Conversation> {
        self.repo
            .create_conversation(tenant_id, inbox_id, contact_id)
            .await
    }

    pub async fn create_message(
        &self,
        tenant_id: &str,
        conversation_id: Uuid,
        content: &str,
        sender_type: &str,
    ) -> Result<Message> {
        self.repo
            .create_message(tenant_id, conversation_id, content, sender_type)
            .await
    }

    pub async fn list_messages(
        &self,
        tenant_id: &str,
        conversation_id: Uuid,
    ) -> Result<Vec<Message>> {
        self.repo.list_messages(tenant_id, conversation_id).await
    }
}
