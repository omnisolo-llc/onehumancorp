use sqlx::PgPool;
use uuid::Uuid;
use super::models::{ChatInbox, ChatChannel, ChatContact, ChatConversation, ChatMessage};
use std::sync::Arc;
use crate::msgbus::{Bus, Message};

pub struct ConversationService {
    pool: PgPool,
}

impl ConversationService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_inbox(
        &self,
        tenant_id: Uuid,
        name: String,
    ) -> Result<ChatInbox, sqlx::Error> {
        sqlx::query_as(
            r#"
            INSERT INTO chat_inboxes (id, tenant_id, name)
            VALUES ($1, $2, $3)
            RETURNING id, tenant_id, name, created_at, updated_at
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(name)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn create_channel(
        &self,
        tenant_id: Uuid,
        inbox_id: Uuid,
        channel_type: String,
        config: serde_json::Value,
    ) -> Result<ChatChannel, sqlx::Error> {
        sqlx::query_as(
            r#"
            INSERT INTO chat_channels (id, tenant_id, inbox_id, channel_type, config)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, tenant_id, inbox_id, channel_type, config, created_at, updated_at
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(inbox_id)
        .bind(channel_type)
        .bind(config)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn create_contact(
        &self,
        tenant_id: Uuid,
        name: Option<String>,
        email: Option<String>,
        phone: Option<String>,
    ) -> Result<ChatContact, sqlx::Error> {
        sqlx::query_as(
            r#"
            INSERT INTO chat_contacts (id, tenant_id, name, email, phone)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, tenant_id, name, email, phone, created_at, updated_at
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(name)
        .bind(email)
        .bind(phone)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn start_conversation(
        &self,
        tenant_id: Uuid,
        inbox_id: Uuid,
        contact_id: Uuid,
        assignee_id: Option<Uuid>,
    ) -> Result<ChatConversation, sqlx::Error> {
        sqlx::query_as(
            r#"
            INSERT INTO chat_conversations (id, tenant_id, inbox_id, contact_id, assignee_id, status)
            VALUES ($1, $2, $3, $4, $5, 'open')
            RETURNING id, tenant_id, inbox_id, contact_id, assignee_id, status, created_at, updated_at
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(inbox_id)
        .bind(contact_id)
        .bind(assignee_id)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn get_conversation(
        &self,
        tenant_id: Uuid,
        conversation_id: Uuid,
    ) -> Result<ChatConversation, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT id, tenant_id, inbox_id, contact_id, assignee_id, status, created_at, updated_at
            FROM chat_conversations
            WHERE id = $1 AND tenant_id = $2
            "#
        )
        .bind(conversation_id)
        .bind(tenant_id)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn update_conversation_status(
        &self,
        tenant_id: Uuid,
        conversation_id: Uuid,
        status: String,
    ) -> Result<ChatConversation, sqlx::Error> {
        sqlx::query_as(
            r#"
            UPDATE chat_conversations
            SET status = $1, updated_at = NOW()
            WHERE id = $2 AND tenant_id = $3
            RETURNING id, tenant_id, inbox_id, contact_id, assignee_id, status, created_at, updated_at
            "#
        )
        .bind(status)
        .bind(conversation_id)
        .bind(tenant_id)
        .fetch_one(&self.pool)
        .await
    }
}

pub struct MessageService {
    pool: PgPool,
    bus: Option<Arc<dyn Bus>>,
}

impl MessageService {
    pub fn new(pool: PgPool, bus: Option<Arc<dyn Bus>>) -> Self {
        Self { pool, bus }
    }

    pub async fn send_message(
        &self,
        tenant_id: Uuid,
        conversation_id: Uuid,
        sender_type: String,
        sender_id: Option<Uuid>,
        content: String,
    ) -> Result<ChatMessage, sqlx::Error> {
        let msg = sqlx::query_as::<_, ChatMessage>(
            r#"
            INSERT INTO chat_messages (id, tenant_id, conversation_id, sender_type, sender_id, content)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, tenant_id, conversation_id, sender_type, sender_id, content, created_at, updated_at
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(conversation_id)
        .bind(sender_type)
        .bind(sender_id)
        .bind(content)
        .fetch_one(&self.pool)
        .await?;

        if let Some(bus) = &self.bus {
            if let Ok(payload) = serde_json::to_vec(&msg) {
                let _ = bus.publish(Message {
                    topic: "tenant.omnichannel.message.received".to_string(),
                    payload,
                }).await;
            }
        }

        Ok(msg)
    }

    pub async fn get_messages(
        &self,
        tenant_id: Uuid,
        conversation_id: Uuid,
    ) -> Result<Vec<ChatMessage>, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT id, tenant_id, conversation_id, sender_type, sender_id, content, created_at, updated_at
            FROM chat_messages
            WHERE conversation_id = $1 AND tenant_id = $2
            ORDER BY created_at ASC
            "#
        )
        .bind(conversation_id)
        .bind(tenant_id)
        .fetch_all(&self.pool)
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;
    use crate::msgbus::MemoryBus;

    #[tokio::test]
    async fn test_conversation_and_message_services() {
        let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());
        let maybe_pool = PgPool::connect(&database_url).await;
        if maybe_pool.is_err() {
            return;
        }
        let pool = maybe_pool.unwrap();

        let tenant_id = Uuid::new_v4();

        // Ensure tables exist for test
        // Ensure tables exist for test
        let _ = sqlx::query("
            CREATE TABLE IF NOT EXISTS chat_inboxes (
                id UUID PRIMARY KEY,
                tenant_id UUID NOT NULL,
                name TEXT NOT NULL,
                created_at TIMESTAMPTZ DEFAULT NOW(),
                updated_at TIMESTAMPTZ DEFAULT NOW()
            );
            ALTER TABLE chat_inboxes ENABLE ROW LEVEL SECURITY;

            CREATE TABLE IF NOT EXISTS chat_channels (
                id UUID PRIMARY KEY,
                tenant_id UUID NOT NULL,
                inbox_id UUID NOT NULL REFERENCES chat_inboxes(id) ON DELETE CASCADE,
                channel_type TEXT NOT NULL,
                config JSONB DEFAULT '{}'::jsonb,
                created_at TIMESTAMPTZ DEFAULT NOW(),
                updated_at TIMESTAMPTZ DEFAULT NOW()
            );
            ALTER TABLE chat_channels ENABLE ROW LEVEL SECURITY;

            CREATE TABLE IF NOT EXISTS chat_contacts (
                id UUID PRIMARY KEY,
                tenant_id UUID NOT NULL,
                name TEXT,
                email TEXT,
                phone TEXT,
                created_at TIMESTAMPTZ DEFAULT NOW(),
                updated_at TIMESTAMPTZ DEFAULT NOW()
            );
            ALTER TABLE chat_contacts ENABLE ROW LEVEL SECURITY;

            CREATE TABLE IF NOT EXISTS chat_conversations (
                id UUID PRIMARY KEY,
                tenant_id UUID NOT NULL,
                inbox_id UUID NOT NULL REFERENCES chat_inboxes(id) ON DELETE CASCADE,
                contact_id UUID NOT NULL REFERENCES chat_contacts(id) ON DELETE CASCADE,
                assignee_id UUID,
                status TEXT NOT NULL DEFAULT 'open',
                created_at TIMESTAMPTZ DEFAULT NOW(),
                updated_at TIMESTAMPTZ DEFAULT NOW()
            );
            ALTER TABLE chat_conversations ENABLE ROW LEVEL SECURITY;

            CREATE TABLE IF NOT EXISTS chat_messages (
                id UUID PRIMARY KEY,
                tenant_id UUID NOT NULL,
                conversation_id UUID NOT NULL REFERENCES chat_conversations(id) ON DELETE CASCADE,
                sender_type TEXT NOT NULL,
                sender_id UUID,
                content TEXT NOT NULL,
                created_at TIMESTAMPTZ DEFAULT NOW(),
                updated_at TIMESTAMPTZ DEFAULT NOW()
            );
            ALTER TABLE chat_messages ENABLE ROW LEVEL SECURITY;
        ").execute(&pool).await;

        let conv_service = ConversationService::new(pool.clone());
        let inbox = conv_service.create_inbox(tenant_id, "Test Inbox".to_string()).await.unwrap();
        assert_eq!(inbox.name, "Test Inbox");

        let channel = conv_service.create_channel(tenant_id, inbox.id, "web".to_string(), serde_json::json!({})).await.unwrap();
        assert_eq!(channel.channel_type, "web");

        let contact = conv_service.create_contact(tenant_id, Some("John Doe".to_string()), Some("john@example.com".to_string()), None).await.unwrap();
        assert_eq!(contact.name.unwrap(), "John Doe");

        let conversation = conv_service.start_conversation(tenant_id, inbox.id, contact.id, None).await.unwrap();
        assert_eq!(conversation.status, "open");

        let fetched_conversation = conv_service.get_conversation(tenant_id, conversation.id).await.unwrap();
        assert_eq!(fetched_conversation.id, conversation.id);

        let updated_conversation = conv_service.update_conversation_status(tenant_id, conversation.id, "resolved".to_string()).await.unwrap();
        assert_eq!(updated_conversation.status, "resolved");

        let bus = Arc::new(MemoryBus::new());
        let msg_service = MessageService::new(pool.clone(), Some(bus.clone()));

        let message = msg_service.send_message(tenant_id, conversation.id, "contact".to_string(), Some(contact.id), "Hello!".to_string()).await.unwrap();
        assert_eq!(message.content, "Hello!");

        let messages = msg_service.get_messages(tenant_id, conversation.id).await.unwrap();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].id, message.id);
    }
}
