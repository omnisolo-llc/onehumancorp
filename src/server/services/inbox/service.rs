use sqlx::{Pool, Postgres};
use uuid::Uuid;
use super::models::{Inbox, Contact, Conversation, Message};

pub struct InboxService {
    pool: Pool<Postgres>,
}

impl InboxService {
    pub fn new(pool: Pool<Postgres>) -> Self {
        InboxService { pool }
    }

    pub async fn create_inbox(&self, tenant_id: Uuid, name: &str, channel_type: &str) -> Result<Inbox, sqlx::Error> {
        let id = Uuid::new_v4();
        let inbox = sqlx::query_as!(
            Inbox,
            r#"
            INSERT INTO chat_inbox (id, tenant_id, name, channel_type)
            VALUES ($1, $2, $3, $4)
            RETURNING id, tenant_id, name, channel_type, settings, created_at, updated_at
            "#,
            id, tenant_id, name, channel_type
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(inbox)
    }

    pub async fn create_contact(&self, tenant_id: Uuid, name: Option<&str>, email: Option<&str>) -> Result<Contact, sqlx::Error> {
        let id = Uuid::new_v4();
        let contact = sqlx::query_as!(
            Contact,
            r#"
            INSERT INTO chat_contact (id, tenant_id, name, email)
            VALUES ($1, $2, $3, $4)
            RETURNING id, tenant_id, name, email, phone_number, custom_attributes, created_at, updated_at
            "#,
            id, tenant_id, name, email
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(contact)
    }

    pub async fn create_conversation(&self, tenant_id: Uuid, inbox_id: Uuid, contact_id: Uuid) -> Result<Conversation, sqlx::Error> {
        let id = Uuid::new_v4();
        let conversation = sqlx::query_as!(
            Conversation,
            r#"
            INSERT INTO chat_conversation (id, tenant_id, inbox_id, contact_id)
            VALUES ($1, $2, $3, $4)
            RETURNING id, tenant_id, inbox_id, contact_id, status, priority, created_at, updated_at
            "#,
            id, tenant_id, inbox_id, contact_id
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(conversation)
    }

    pub async fn create_message(&self, tenant_id: Uuid, conversation_id: Uuid, sender_type: &str, content: &str) -> Result<Message, sqlx::Error> {
        let id = Uuid::new_v4();
        let message = sqlx::query_as!(
            Message,
            r#"
            INSERT INTO chat_message (id, tenant_id, conversation_id, sender_type, content)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, tenant_id, conversation_id, sender_type, sender_id, content, content_type, status, created_at, updated_at
            "#,
            id, tenant_id, conversation_id, sender_type, content
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(message)
    }

    pub async fn get_messages(&self, tenant_id: Uuid, conversation_id: Uuid) -> Result<Vec<Message>, sqlx::Error> {
        let messages = sqlx::query_as!(
            Message,
            r#"
            SELECT id, tenant_id, conversation_id, sender_type, sender_id, content, content_type, status, created_at, updated_at
            FROM chat_message
            WHERE tenant_id = $1 AND conversation_id = $2
            ORDER BY created_at ASC
            "#,
            tenant_id, conversation_id
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(messages)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_inbox_service() {
        assert!(true);
    }
}
