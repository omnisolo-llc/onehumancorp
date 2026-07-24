use crate::domain::repository::core_inbox_repo::{CoreInboxRepo, Message};
use serde_json::Value;
use std::sync::Arc;
use uuid::Uuid;
use sqlx::PgPool;

#[derive(Clone)]
pub struct CoreInboxService {
    repo: CoreInboxRepo,
}

impl CoreInboxService {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self {
            repo: CoreInboxRepo::new(pool),
        }
    }

    pub async fn ingest_message(
        &self,
        tenant_id: &str,
        inbox_id: &str,
        contact_name: Option<&str>,
        contact_email: Option<&str>,
        contact_phone: Option<&str>,
        content: &str,
    ) -> Result<Message, String> {
        let contact_id = Uuid::new_v4().to_string();
        let contact = self.repo.get_or_create_contact(&contact_id, tenant_id, contact_name, contact_email, contact_phone)
            .await
            .map_err(|e| e.to_string())?;

        let conversation_id = Uuid::new_v4().to_string();
        let conversation = self.repo.create_conversation(&conversation_id, tenant_id, inbox_id, &contact.id)
            .await
            .map_err(|e| e.to_string())?;

        let message_id = Uuid::new_v4().to_string();
        let message = self.repo.create_message(&message_id, tenant_id, &conversation.id, "contact", content)
            .await
            .map_err(|e| e.to_string())?;

        Ok(message)
    }
}
