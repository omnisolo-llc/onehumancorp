use sqlx::{Pool, Postgres};
use uuid::Uuid;
use crate::domain::omnichannel::{Inbox, Contact, Conversation, Message};

#[derive(Clone)]
pub struct OmnichannelRepo {
    pool: Pool<Postgres>,
}

impl OmnichannelRepo {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn create_inbox(&self, tenant_id: Uuid, name: String) -> Result<Inbox, sqlx::Error> {
        let id = Uuid::new_v4();
        let record = sqlx::query_as::<_, Inbox>(
            "INSERT INTO chat_inboxes (id, tenant_id, name) VALUES ($1, $2, $3) RETURNING *"
        )
        .bind(id)
        .bind(tenant_id)
        .bind(name)
        .fetch_one(&self.pool)
        .await?;
        Ok(record)
    }

    pub async fn get_inboxes(&self, tenant_id: Uuid) -> Result<Vec<Inbox>, sqlx::Error> {
        sqlx::query_as::<_, Inbox>("SELECT * FROM chat_inboxes WHERE tenant_id = $1")
            .bind(tenant_id)
            .fetch_all(&self.pool)
            .await
    }

    pub async fn create_contact(&self, tenant_id: Uuid, name: Option<String>, email: Option<String>, phone: Option<String>) -> Result<Contact, sqlx::Error> {
        let id = Uuid::new_v4();
        sqlx::query_as::<_, Contact>(
            "INSERT INTO chat_contacts (id, tenant_id, name, email, phone) VALUES ($1, $2, $3, $4, $5) RETURNING *"
        )
        .bind(id)
        .bind(tenant_id)
        .bind(name)
        .bind(email)
        .bind(phone)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn get_contacts(&self, tenant_id: Uuid) -> Result<Vec<Contact>, sqlx::Error> {
        sqlx::query_as::<_, Contact>("SELECT * FROM chat_contacts WHERE tenant_id = $1")
            .bind(tenant_id)
            .fetch_all(&self.pool)
            .await
    }

    pub async fn create_conversation(&self, tenant_id: Uuid, inbox_id: Uuid, contact_id: Uuid, assignee_id: Option<Uuid>, status: String) -> Result<Conversation, sqlx::Error> {
        let id = Uuid::new_v4();
        sqlx::query_as::<_, Conversation>(
            "INSERT INTO chat_conversations (id, tenant_id, inbox_id, contact_id, assignee_id, status) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *"
        )
        .bind(id)
        .bind(tenant_id)
        .bind(inbox_id)
        .bind(contact_id)
        .bind(assignee_id)
        .bind(status)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn get_conversations(&self, tenant_id: Uuid, inbox_id: Uuid) -> Result<Vec<Conversation>, sqlx::Error> {
        sqlx::query_as::<_, Conversation>("SELECT * FROM chat_conversations WHERE tenant_id = $1 AND inbox_id = $2")
            .bind(tenant_id)
            .bind(inbox_id)
            .fetch_all(&self.pool)
            .await
    }

    pub async fn get_conversation_by_id(&self, tenant_id: Uuid, id: Uuid) -> Result<Option<Conversation>, sqlx::Error> {
        sqlx::query_as::<_, Conversation>("SELECT * FROM chat_conversations WHERE tenant_id = $1 AND id = $2")
            .bind(tenant_id)
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }

    pub async fn create_message(&self, tenant_id: Uuid, conversation_id: Uuid, sender_type: String, sender_id: Option<Uuid>, content: String) -> Result<Message, sqlx::Error> {
        let id = Uuid::new_v4();
        sqlx::query_as::<_, Message>(
            "INSERT INTO chat_messages (id, tenant_id, conversation_id, sender_type, sender_id, content) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *"
        )
        .bind(id)
        .bind(tenant_id)
        .bind(conversation_id)
        .bind(sender_type)
        .bind(sender_id)
        .bind(content)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn get_messages(&self, tenant_id: Uuid, conversation_id: Uuid) -> Result<Vec<Message>, sqlx::Error> {
        sqlx::query_as::<_, Message>("SELECT * FROM chat_messages WHERE tenant_id = $1 AND conversation_id = $2 ORDER BY created_at ASC")
            .bind(tenant_id)
            .bind(conversation_id)
            .fetch_all(&self.pool)
            .await
    }
}
