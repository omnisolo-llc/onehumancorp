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
        let id = Uuid::new_v5(&Uuid::NAMESPACE_DNS, tenant_id.to_string().as_bytes());
        sqlx::query_as(
            r#"
            INSERT INTO chat_inboxes (id, tenant_id, name)
            VALUES ($1, $2, $3)
            ON CONFLICT (id) DO UPDATE SET name = EXCLUDED.name
            RETURNING id, tenant_id, name, created_at, updated_at
            "#
        )
        .bind(id)
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

    pub async fn get_conversations(
        &self,
        tenant_id: Uuid,
        inbox_id: Uuid,
    ) -> Result<Vec<ChatConversation>, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT id, tenant_id, inbox_id, contact_id, assignee_id, status, created_at, updated_at
            FROM chat_conversations
            WHERE tenant_id = $1 AND inbox_id = $2
            ORDER BY updated_at DESC
            "#
        )
        .bind(tenant_id)
        .bind(inbox_id)
        .fetch_all(&self.pool)
        .await
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
            WHERE tenant_id = $1 AND conversation_id = $2
            ORDER BY created_at ASC
            "#
        )
        .bind(tenant_id)
        .bind(conversation_id)
        .fetch_all(&self.pool)
        .await
    }

    pub async fn resolve_or_create_contact(
        &self,
        tenant_id: Uuid,
        channel_type: &str,
        channel_identity: &str,
    ) -> Result<ChatContact, sqlx::Error> {
        // Simple heuristic for phone/email
        let email = if channel_identity.contains('@') { Some(channel_identity.to_string()) } else { None };
        let phone = if !channel_identity.contains('@') && channel_identity.chars().any(|c| c.is_digit(10)) { Some(channel_identity.to_string()) } else { None };

        let existing: Option<ChatContact> = if let Some(e) = &email {
            sqlx::query_as(
                "SELECT id, tenant_id, name, email, phone, created_at, updated_at FROM chat_contacts WHERE tenant_id = $1 AND email = $2"
            ).bind(tenant_id).bind(e).fetch_optional(&self.pool).await?
        } else if let Some(p) = &phone {
            sqlx::query_as(
                "SELECT id, tenant_id, name, email, phone, created_at, updated_at FROM chat_contacts WHERE tenant_id = $1 AND phone = $2"
            ).bind(tenant_id).bind(p).fetch_optional(&self.pool).await?
        } else {
            None
        };

        if let Some(contact) = existing {
            return Ok(contact);
        }

        self.create_contact(tenant_id, Some(format!("{} User", channel_type)), email, phone).await
    }

    pub async fn resolve_or_create_conversation(
        &self,
        tenant_id: Uuid,
        inbox_id: Uuid,
        contact_id: Uuid,
    ) -> Result<ChatConversation, sqlx::Error> {
        let existing: Option<ChatConversation> = sqlx::query_as(
            "SELECT id, tenant_id, inbox_id, contact_id, assignee_id, status, created_at, updated_at FROM chat_conversations WHERE tenant_id = $1 AND inbox_id = $2 AND contact_id = $3 AND status = 'open' ORDER BY created_at DESC LIMIT 1"
        )
        .bind(tenant_id)
        .bind(inbox_id)
        .bind(contact_id)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(conv) = existing {
            Ok(conv)
        } else {
            self.start_conversation(tenant_id, inbox_id, contact_id, None).await
        }
    }
}
