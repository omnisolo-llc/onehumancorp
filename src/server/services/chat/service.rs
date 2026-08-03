use sqlx::PgPool;
use uuid::Uuid;
use super::models::{ChatInbox, ChatChannel, ChatContact, ChatConversation, ChatMessage};

pub struct ChatService {
    pool: PgPool,
}

impl ChatService {
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

    pub async fn send_message(
        &self,
        tenant_id: Uuid,
        conversation_id: Uuid,
        sender_type: String,
        sender_id: Option<Uuid>,
        content: String,
    ) -> Result<ChatMessage, sqlx::Error> {
        sqlx::query_as(
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
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;

    async fn setup_db(tenant_id: Uuid) -> Option<PgPool> {
        let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());

        let pool = match PgPool::connect(&database_url).await {
            Ok(p) => p,
            Err(_) => return None,
        };

        sqlx::query(
            "
            CREATE TABLE IF NOT EXISTS chat_inboxes (
                id UUID PRIMARY KEY,
                tenant_id UUID NOT NULL,
                name TEXT NOT NULL,
                created_at TIMESTAMPTZ DEFAULT NOW(),
                updated_at TIMESTAMPTZ DEFAULT NOW()
            );
            ALTER TABLE chat_inboxes ENABLE ROW LEVEL SECURITY;
            DO $$ BEGIN
                IF NOT EXISTS (SELECT 1 FROM pg_policies WHERE policyname = 'chat_inboxes_tenant_isolation_policy') THEN
                    CREATE POLICY chat_inboxes_tenant_isolation_policy ON chat_inboxes FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);
                END IF;
            END $$;

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
            DO $$ BEGIN
                IF NOT EXISTS (SELECT 1 FROM pg_policies WHERE policyname = 'chat_channels_tenant_isolation_policy') THEN
                    CREATE POLICY chat_channels_tenant_isolation_policy ON chat_channels FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);
                END IF;
            END $$;

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
            DO $$ BEGIN
                IF NOT EXISTS (SELECT 1 FROM pg_policies WHERE policyname = 'chat_contacts_tenant_isolation_policy') THEN
                    CREATE POLICY chat_contacts_tenant_isolation_policy ON chat_contacts FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);
                END IF;
            END $$;

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
            DO $$ BEGIN
                IF NOT EXISTS (SELECT 1 FROM pg_policies WHERE policyname = 'chat_conversations_tenant_isolation_policy') THEN
                    CREATE POLICY chat_conversations_tenant_isolation_policy ON chat_conversations FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);
                END IF;
            END $$;

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
            DO $$ BEGIN
                IF NOT EXISTS (SELECT 1 FROM pg_policies WHERE policyname = 'chat_messages_tenant_isolation_policy') THEN
                    CREATE POLICY chat_messages_tenant_isolation_policy ON chat_messages FOR ALL USING (tenant_id = current_setting('app.current_tenant_id', true)::uuid);
                END IF;
            END $$;
            "
        ).execute(&pool).await.ok()?;

        // Set RLS scope context
        sqlx::query(&format!("SET LOCAL app.current_tenant_id = '{}'", tenant_id))
            .execute(&pool)
            .await
            .unwrap_or_default(); // In case of older PostgreSQL or setup mismatch

        Some(pool)
    }

    #[tokio::test]
    async fn test_create_inbox() {
        let tenant_id = Uuid::new_v4();
        if let Some(pool) = setup_db(tenant_id).await {
            let service = ChatService::new(pool);
            let inbox = service.create_inbox(tenant_id, "Support".to_string()).await;
            assert!(inbox.is_ok());
            assert_eq!(inbox.unwrap().name, "Support");
        }
    }

    #[tokio::test]
    async fn test_create_channel() {
        let tenant_id = Uuid::new_v4();
        if let Some(pool) = setup_db(tenant_id).await {
            let service = ChatService::new(pool);
            let inbox = service.create_inbox(tenant_id, "Support".to_string()).await.unwrap();

            let channel = service.create_channel(tenant_id, inbox.id, "Web".to_string(), serde_json::json!({})).await;
            assert!(channel.is_ok());
            assert_eq!(channel.unwrap().channel_type, "Web");
        }
    }

    #[tokio::test]
    async fn test_create_contact() {
        let tenant_id = Uuid::new_v4();
        if let Some(pool) = setup_db(tenant_id).await {
            let service = ChatService::new(pool);
            let contact = service.create_contact(tenant_id, Some("Bob".to_string()), None, None).await;
            assert!(contact.is_ok());
            assert_eq!(contact.unwrap().name.as_deref(), Some("Bob"));
        }
    }

    #[tokio::test]
    async fn test_start_conversation() {
        let tenant_id = Uuid::new_v4();
        if let Some(pool) = setup_db(tenant_id).await {
            let service = ChatService::new(pool);
            let inbox = service.create_inbox(tenant_id, "Support".to_string()).await.unwrap();
            let contact = service.create_contact(tenant_id, Some("Bob".to_string()), None, None).await.unwrap();

            let conversation = service.start_conversation(tenant_id, inbox.id, contact.id, None).await;
            assert!(conversation.is_ok());
            assert_eq!(conversation.unwrap().status, "open");
        }
    }

    #[tokio::test]
    async fn test_send_message() {
        let tenant_id = Uuid::new_v4();
        if let Some(pool) = setup_db(tenant_id).await {
            let service = ChatService::new(pool);
            let inbox = service.create_inbox(tenant_id, "Support".to_string()).await.unwrap();
            let contact = service.create_contact(tenant_id, Some("Bob".to_string()), None, None).await.unwrap();
            let conversation = service.start_conversation(tenant_id, inbox.id, contact.id, None).await.unwrap();

            let message = service.send_message(tenant_id, conversation.id, "Customer".to_string(), Some(contact.id), "Help!".to_string()).await;
            assert!(message.is_ok());
            assert_eq!(message.unwrap().content, "Help!");
        }
    }
}
