use uuid::Uuid;

use sqlx::PgPool;
use super::models::{Inbox, Contact, Conversation, Message, ChannelType, SenderType};

pub struct OmnichannelRepo {
    pool: PgPool,
}

impl OmnichannelRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_inbox(&self, tenant_id: Uuid, name: String, channel_type: ChannelType) -> Result<Inbox, sqlx::Error> {
        let id = Uuid::new_v4();
        let record = sqlx::query_as::<_, Inbox>(
            r#"
            INSERT INTO chat_inboxes (id, tenant_id, name, channel_type)
            VALUES ($1, $2, $3, $4)
            RETURNING id, tenant_id, name, channel_type, auto_assignment_config, created_at, updated_at
            "#
        )
        .bind(id)
        .bind(tenant_id)
        .bind(name)
        .bind(channel_type)
        .fetch_one(&self.pool)
        .await?;
        Ok(record)
    }

    pub async fn create_contact(&self, tenant_id: Uuid, name: Option<String>, email: Option<String>, phone_number: Option<String>) -> Result<Contact, sqlx::Error> {
        let id = Uuid::new_v4();
        let record = sqlx::query_as::<_, Contact>(
            r#"
            INSERT INTO chat_contacts (id, tenant_id, name, email, phone)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, tenant_id, name, email, phone as "phone_number", identifier, created_at, updated_at
            "#
        )
        .bind(id)
        .bind(tenant_id)
        .bind(name)
        .bind(email)
        .bind(phone_number)
        .fetch_one(&self.pool)
        .await?;
        Ok(record)
    }

    pub async fn create_conversation(&self, tenant_id: Uuid, inbox_id: Uuid, contact_id: Uuid) -> Result<Conversation, sqlx::Error> {
        let id = Uuid::new_v4();
        let record = sqlx::query_as::<_, Conversation>(
            r#"
            INSERT INTO chat_conversations (id, tenant_id, inbox_id, contact_id, status)
            VALUES ($1, $2, $3, $4, 'open')
            RETURNING id, tenant_id, inbox_id, contact_id, status, assignee_id, unread_count, created_at, updated_at
            "#
        )
        .bind(id)
        .bind(tenant_id)
        .bind(inbox_id)
        .bind(contact_id)
        .fetch_one(&self.pool)
        .await?;
        Ok(record)
    }

    pub async fn create_message(&self, tenant_id: Uuid, conversation_id: Uuid, inbox_id: Uuid, sender_type: SenderType, content: String) -> Result<Message, sqlx::Error> {
        let id = Uuid::new_v4();
        let record = sqlx::query_as::<_, Message>(
            r#"
            INSERT INTO chat_messages (id, tenant_id, conversation_id, inbox_id, sender_type, content)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, tenant_id, conversation_id, inbox_id, sender_type, sender_id, content, content_type, created_at, updated_at
            "#
        )
        .bind(id)
        .bind(tenant_id)
        .bind(conversation_id)
        .bind(inbox_id)
        .bind(sender_type)
        .bind(content)
        .fetch_one(&self.pool)
        .await?;
        Ok(record)
    }
}
