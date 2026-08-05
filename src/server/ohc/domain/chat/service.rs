use uuid::Uuid;
use sqlx::{PgPool, Error};

use super::chat::{Contact, Conversation, Message};

pub struct ChatService {
    pool: PgPool,
}

impl ChatService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn process_incoming_message(
        &self,
        tenant_id: Uuid,
        inbox_id: Uuid,
        contact_name: Option<String>,
        contact_email: Option<String>,
        contact_phone: Option<String>,
        content: String,
    ) -> Result<(Contact, Conversation, Message), Error> {

        let contact = self.find_or_create_contact(tenant_id, contact_name, contact_email, contact_phone).await?;
        let conversation = self.find_or_create_conversation(tenant_id, inbox_id, contact.id).await?;
        let message = self.create_message(tenant_id, conversation.id, "contact".to_string(), None, content).await?;

        // Here we could broadcast real-time events via Redis/NATS or trigger AI Agent Queue

        Ok((contact, conversation, message))
    }

    async fn find_or_create_contact(
        &self,
        tenant_id: Uuid,
        name: Option<String>,
        email: Option<String>,
        phone: Option<String>,
    ) -> Result<Contact, Error> {
        if let Some(ref e) = email {
            if let Ok(contact) = sqlx::query_as::<_, Contact>(
                "SELECT id, tenant_id, name, email, phone, created_at, updated_at FROM chat_contacts WHERE tenant_id = $1 AND email = $2"
            )
            .bind(tenant_id)
            .bind(e)
            .fetch_one(&self.pool)
            .await {
                return Ok(contact);
            }
        }

        if let Some(ref p) = phone {
            if let Ok(contact) = sqlx::query_as::<_, Contact>(
                "SELECT id, tenant_id, name, email, phone, created_at, updated_at FROM chat_contacts WHERE tenant_id = $1 AND phone = $2"
            )
            .bind(tenant_id)
            .bind(p)
            .fetch_one(&self.pool)
            .await {
                return Ok(contact);
            }
        }

        let id = Uuid::new_v4();
        sqlx::query_as::<_, Contact>(
            "INSERT INTO chat_contacts (id, tenant_id, name, email, phone) VALUES ($1, $2, $3, $4, $5) RETURNING id, tenant_id, name, email, phone, created_at, updated_at"
        )
        .bind(id)
        .bind(tenant_id)
        .bind(name)
        .bind(email)
        .bind(phone)
        .fetch_one(&self.pool)
        .await
    }

    async fn find_or_create_conversation(
        &self,
        tenant_id: Uuid,
        inbox_id: Uuid,
        contact_id: Uuid,
    ) -> Result<Conversation, Error> {
        let conversation_result = sqlx::query_as::<_, Conversation>(
            "SELECT id, tenant_id, inbox_id, contact_id, assignee_id, status, created_at, updated_at FROM chat_conversations WHERE tenant_id = $1 AND inbox_id = $2 AND contact_id = $3 AND status = 'open' LIMIT 1"
        )
        .bind(tenant_id)
        .bind(inbox_id)
        .bind(contact_id)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(conversation) = conversation_result {
            return Ok(conversation);
        }

        let id = Uuid::new_v4();
        sqlx::query_as::<_, Conversation>(
            "INSERT INTO chat_conversations (id, tenant_id, inbox_id, contact_id, status) VALUES ($1, $2, $3, $4, 'open') RETURNING id, tenant_id, inbox_id, contact_id, assignee_id, status, created_at, updated_at"
        )
        .bind(id)
        .bind(tenant_id)
        .bind(inbox_id)
        .bind(contact_id)
        .fetch_one(&self.pool)
        .await
    }

    async fn create_message(
        &self,
        tenant_id: Uuid,
        conversation_id: Uuid,
        sender_type: String,
        sender_id: Option<Uuid>,
        content: String,
    ) -> Result<Message, Error> {
        let id = Uuid::new_v4();

        sqlx::query_as::<_, Message>(
            "INSERT INTO chat_messages (id, tenant_id, conversation_id, sender_type, sender_id, content) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id, tenant_id, conversation_id, sender_type, sender_id, content, created_at, updated_at"
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;

    #[tokio::test]
    async fn test_process_incoming_message() {
        let database_url = match std::env::var("DATABASE_URL") {
            Ok(url) => url,
            Err(_) => return, // Skip test if no DB url
        };

        let pool = match PgPool::connect(&database_url).await {
            Ok(pool) => pool,
            Err(_) => return, // Skip test if DB connection fails
        };

        let service = ChatService::new(pool.clone());
        let tenant_id = Uuid::new_v4();
        let inbox_id = Uuid::new_v4();

        // Ensure tables exist for test
        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS chat_inboxes (
                id UUID PRIMARY KEY, tenant_id UUID NOT NULL, name TEXT NOT NULL, created_at TIMESTAMPTZ DEFAULT NOW(), updated_at TIMESTAMPTZ DEFAULT NOW()
            );
            CREATE TABLE IF NOT EXISTS chat_contacts (
                id UUID PRIMARY KEY, tenant_id UUID NOT NULL, name TEXT, email TEXT, phone TEXT, created_at TIMESTAMPTZ DEFAULT NOW(), updated_at TIMESTAMPTZ DEFAULT NOW()
            );
            CREATE TABLE IF NOT EXISTS chat_conversations (
                id UUID PRIMARY KEY, tenant_id UUID NOT NULL, inbox_id UUID NOT NULL, contact_id UUID NOT NULL, assignee_id UUID, status TEXT NOT NULL DEFAULT 'open', created_at TIMESTAMPTZ DEFAULT NOW(), updated_at TIMESTAMPTZ DEFAULT NOW()
            );
            CREATE TABLE IF NOT EXISTS chat_messages (
                id UUID PRIMARY KEY, tenant_id UUID NOT NULL, conversation_id UUID NOT NULL, sender_type TEXT NOT NULL, sender_id UUID, content TEXT NOT NULL, created_at TIMESTAMPTZ DEFAULT NOW(), updated_at TIMESTAMPTZ DEFAULT NOW()
            );
        ").execute(&pool).await;

        // Create mock inbox
        let _ = sqlx::query("INSERT INTO chat_inboxes (id, tenant_id, name) VALUES ($1, $2, 'Test Inbox')")
            .bind(inbox_id)
            .bind(tenant_id)
            .execute(&pool)
            .await;

        let result = service.process_incoming_message(
            tenant_id,
            inbox_id,
            Some("Maya".to_string()),
            Some("maya@example.com".to_string()),
            None,
            "I need a cake".to_string()
        ).await;

        assert!(result.is_ok());

        let (contact, conversation, message) = result.unwrap();

        assert_eq!(contact.name, Some("Maya".to_string()));
        assert_eq!(contact.email, Some("maya@example.com".to_string()));
        assert_eq!(conversation.inbox_id, inbox_id);
        assert_eq!(conversation.contact_id, contact.id);
        assert_eq!(message.content, "I need a cake");
        assert_eq!(message.sender_type, "contact".to_string());
    }
}
