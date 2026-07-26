
use uuid::Uuid;
use chrono::Utc;
use sqlx::{Pool, Postgres};
use crate::models::{Conversation, Message, MessageType, ConversationStatus};

#[async_trait::async_trait]
pub trait ChatEngineService: Send + Sync {
    async fn create_conversation(&self, tenant_id: &str, contact_id: Uuid) -> Result<Conversation, String>;
    async fn send_message(&self, tenant_id: &str, conv_id: Uuid, content: &str, msg_type: MessageType, sender: Option<Uuid>) -> Result<Message, String>;
    async fn auto_respond(&self, tenant_id: &str, conv_id: Uuid, content: &str) -> Result<Message, String>;
    async fn assign_human_agent(&self, tenant_id: &str, conv_id: Uuid, agent_id: Uuid) -> Result<Conversation, String>;
    async fn resolve_conversation(&self, tenant_id: &str, conv_id: Uuid) -> Result<Conversation, String>;
}

pub struct PgChatService {
    pub pool: Pool<Postgres>,
}

impl PgChatService {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl ChatEngineService for PgChatService {
    async fn create_conversation(&self, tenant_id: &str, contact_id: Uuid) -> Result<Conversation, String> {
        let conv = Conversation {
            id: Uuid::new_v4(),
            tenant_id: tenant_id.to_string(),
            contact_id,
            assignee_id: None,
            status: ConversationStatus::Open,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        // Simulated DB insert
        Ok(conv)
    }

    async fn send_message(&self, tenant_id: &str, conv_id: Uuid, content: &str, msg_type: MessageType, sender: Option<Uuid>) -> Result<Message, String> {
        let msg = Message {
            id: Uuid::new_v4(),
            tenant_id: tenant_id.to_string(),
            conversation_id: conv_id,
            content: content.to_string(),
            message_type: msg_type,
            sender_id: sender,
            is_private: false,
            created_at: Utc::now(),
        };
        // Simulated DB insert
        Ok(msg)
    }

    async fn auto_respond(&self, tenant_id: &str, conv_id: Uuid, content: &str) -> Result<Message, String> {
        self.send_message(tenant_id, conv_id, content, MessageType::Outgoing, None).await
    }

    async fn assign_human_agent(&self, tenant_id: &str, conv_id: Uuid, agent_id: Uuid) -> Result<Conversation, String> {
        let conv = Conversation {
            id: conv_id,
            tenant_id: tenant_id.to_string(),
            contact_id: Uuid::new_v4(),
            assignee_id: Some(agent_id),
            status: ConversationStatus::Open,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        // Simulated DB update
        Ok(conv)
    }

    async fn resolve_conversation(&self, tenant_id: &str, conv_id: Uuid) -> Result<Conversation, String> {
        let conv = Conversation {
            id: conv_id,
            tenant_id: tenant_id.to_string(),
            contact_id: Uuid::new_v4(),
            assignee_id: None,
            status: ConversationStatus::Resolved,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        // Simulated DB update
        Ok(conv)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // Need to test PgChatService, but we would need a DB pool. We can at least write structural tests or logic flow tests.
    // For now, since the actual DB interactions are mocked out in PgChatService anyway, we will verify its dummy return values.

    #[tokio::test]
    async fn test_pg_chat_service_create() {
        let pool = sqlx::PgPool::connect_lazy("postgres://postgres:postgres@localhost:5432/dummy").unwrap();
        let service = PgChatService::new(pool);
        let tenant = "tenant1";
        let contact = Uuid::new_v4();

        let conv = service.create_conversation(tenant, contact).await.unwrap();
        assert_eq!(conv.tenant_id, tenant);
        assert_eq!(conv.status, ConversationStatus::Open);
    }

    #[tokio::test]
    async fn test_pg_chat_service_resolve() {
        let pool = sqlx::PgPool::connect_lazy("postgres://postgres:postgres@localhost:5432/dummy").unwrap();
        let service = PgChatService::new(pool);
        let tenant = "tenant1";
        let conv_id = Uuid::new_v4();

        let conv = service.resolve_conversation(tenant, conv_id).await.unwrap();
        assert_eq!(conv.tenant_id, tenant);
        assert_eq!(conv.status, ConversationStatus::Resolved);
    }
}
