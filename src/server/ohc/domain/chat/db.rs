use sqlx::{Error, Postgres, Transaction};
use uuid::Uuid;
use super::models::{ChatInbox, ChatContact, ChatConversation, ChatMessage};

pub struct ChatRepository;

impl ChatRepository {
    pub fn new() -> Self {
        Self
    }

    pub async fn create_inbox<'a>(&self, tx: &mut Transaction<'a, Postgres>, inbox: &ChatInbox) -> Result<ChatInbox, Error> {
        let rec = sqlx::query_as::<_, ChatInbox>(
            r#"
            INSERT INTO chat_inboxes (id, tenant_id, name, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, tenant_id, name, created_at, updated_at
            "#
        )
        .bind(inbox.id)
        .bind(inbox.tenant_id)
        .bind(&inbox.name)
        .bind(inbox.created_at)
        .bind(inbox.updated_at)
        .fetch_one(&mut **tx)
        .await?;

        Ok(rec)
    }

    pub async fn get_inbox<'a>(&self, tx: &mut Transaction<'a, Postgres>, tenant_id: Uuid, id: Uuid) -> Result<ChatInbox, Error> {
        let rec = sqlx::query_as::<_, ChatInbox>(
            r#"
            SELECT id, tenant_id, name, created_at, updated_at
            FROM chat_inboxes
            WHERE tenant_id = $1 AND id = $2
            "#
        )
        .bind(tenant_id)
        .bind(id)
        .fetch_one(&mut **tx)
        .await?;
        Ok(rec)
    }

    pub async fn create_contact<'a>(&self, tx: &mut Transaction<'a, Postgres>, contact: &ChatContact) -> Result<ChatContact, Error> {
        let rec = sqlx::query_as::<_, ChatContact>(
            r#"
            INSERT INTO chat_contacts (id, tenant_id, name, email, phone, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, tenant_id, name, email, phone, created_at, updated_at
            "#
        )
        .bind(contact.id)
        .bind(contact.tenant_id)
        .bind(&contact.name)
        .bind(&contact.email)
        .bind(&contact.phone)
        .bind(contact.created_at)
        .bind(contact.updated_at)
        .fetch_one(&mut **tx)
        .await?;

        Ok(rec)
    }

    pub async fn get_contact<'a>(&self, tx: &mut Transaction<'a, Postgres>, tenant_id: Uuid, id: Uuid) -> Result<ChatContact, Error> {
        let rec = sqlx::query_as::<_, ChatContact>(
            r#"
            SELECT id, tenant_id, name, email, phone, created_at, updated_at
            FROM chat_contacts
            WHERE tenant_id = $1 AND id = $2
            "#
        )
        .bind(tenant_id)
        .bind(id)
        .fetch_one(&mut **tx)
        .await?;
        Ok(rec)
    }

    pub async fn create_conversation<'a>(&self, tx: &mut Transaction<'a, Postgres>, conv: &ChatConversation) -> Result<ChatConversation, Error> {
        let rec = sqlx::query_as::<_, ChatConversation>(
            r#"
            INSERT INTO chat_conversations (id, tenant_id, inbox_id, contact_id, assignee_id, status, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id, tenant_id, inbox_id, contact_id, assignee_id, status, created_at, updated_at
            "#
        )
        .bind(conv.id)
        .bind(conv.tenant_id)
        .bind(conv.inbox_id)
        .bind(conv.contact_id)
        .bind(conv.assignee_id)
        .bind(&conv.status)
        .bind(conv.created_at)
        .bind(conv.updated_at)
        .fetch_one(&mut **tx)
        .await?;

        Ok(rec)
    }

    pub async fn get_conversation<'a>(&self, tx: &mut Transaction<'a, Postgres>, tenant_id: Uuid, id: Uuid) -> Result<ChatConversation, Error> {
        let rec = sqlx::query_as::<_, ChatConversation>(
            r#"
            SELECT id, tenant_id, inbox_id, contact_id, assignee_id, status, created_at, updated_at
            FROM chat_conversations
            WHERE tenant_id = $1 AND id = $2
            "#
        )
        .bind(tenant_id)
        .bind(id)
        .fetch_one(&mut **tx)
        .await?;
        Ok(rec)
    }

    pub async fn create_message<'a>(&self, tx: &mut Transaction<'a, Postgres>, message: &ChatMessage) -> Result<ChatMessage, Error> {
        let rec = sqlx::query_as::<_, ChatMessage>(
            r#"
            INSERT INTO chat_messages (id, tenant_id, conversation_id, sender_type, sender_id, content, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id, tenant_id, conversation_id, sender_type, sender_id, content, created_at, updated_at
            "#
        )
        .bind(message.id)
        .bind(message.tenant_id)
        .bind(message.conversation_id)
        .bind(&message.sender_type)
        .bind(message.sender_id)
        .bind(&message.content)
        .bind(message.created_at)
        .bind(message.updated_at)
        .fetch_one(&mut **tx)
        .await?;

        Ok(rec)
    }

    pub async fn get_messages<'a>(&self, tx: &mut Transaction<'a, Postgres>, tenant_id: Uuid, conversation_id: Uuid) -> Result<Vec<ChatMessage>, Error> {
        let messages = sqlx::query_as::<_, ChatMessage>(
            r#"
            SELECT id, tenant_id, conversation_id, sender_type, sender_id, content, created_at, updated_at
            FROM chat_messages
            WHERE tenant_id = $1 AND conversation_id = $2
            ORDER BY created_at ASC
            "#
        )
        .bind(tenant_id)
        .bind(conversation_id)
        .fetch_all(&mut **tx)
        .await?;

        Ok(messages)
    }
}
