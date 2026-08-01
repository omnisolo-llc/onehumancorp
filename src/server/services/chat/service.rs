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
            RETURNING id, tenant_id, name, email, phone, source_id, created_at, updated_at
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
            RETURNING id, tenant_id, conversation_id, sender_type, sender_id, content, source_id, created_at, updated_at
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

    pub async fn process_webhook_payload(
        &self,
        tenant_id: Uuid,
        inbox_id: Uuid,
        _channel_type: String,
        contact_source_id: String,
        contact_name: Option<String>,
        message_source_id: String,
        message_content: String,
    ) -> Result<ChatMessage, sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        let contact = sqlx::query_as::<_, ChatContact>(
            r#"
            INSERT INTO chat_contacts (id, tenant_id, name, source_id)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (tenant_id, source_id) DO UPDATE
            SET name = COALESCE(EXCLUDED.name, chat_contacts.name)
            RETURNING id, tenant_id, name, email, phone, source_id, created_at, updated_at
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(contact_name)
        .bind(&contact_source_id)
        .fetch_one(&mut *tx)
        .await?;

        let conversation_opt = sqlx::query_as::<_, ChatConversation>(
            r#"
            SELECT id, tenant_id, inbox_id, contact_id, assignee_id, status, created_at, updated_at
            FROM chat_conversations
            WHERE tenant_id = $1 AND inbox_id = $2 AND contact_id = $3 AND status = 'open'
            LIMIT 1
            "#
        )
        .bind(tenant_id)
        .bind(inbox_id)
        .bind(contact.id)
        .fetch_optional(&mut *tx)
        .await?;

        let conversation = match conversation_opt {
            Some(c) => c,
            None => {
                sqlx::query_as::<_, ChatConversation>(
                    r#"
                    INSERT INTO chat_conversations (id, tenant_id, inbox_id, contact_id, status)
                    VALUES ($1, $2, $3, $4, 'open')
                    RETURNING id, tenant_id, inbox_id, contact_id, assignee_id, status, created_at, updated_at
                    "#
                )
                .bind(Uuid::new_v4())
                .bind(tenant_id)
                .bind(inbox_id)
                .bind(contact.id)
                .fetch_one(&mut *tx)
                .await?
            }
        };

        let message = sqlx::query_as::<_, ChatMessage>(
            r#"
            INSERT INTO chat_messages (id, tenant_id, conversation_id, sender_type, content, source_id)
            VALUES ($1, $2, $3, 'contact', $4, $5)
            ON CONFLICT (tenant_id, source_id) DO UPDATE SET content = EXCLUDED.content
            RETURNING id, tenant_id, conversation_id, sender_type, sender_id, content, source_id, created_at, updated_at
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(conversation.id)
        .bind(message_content)
        .bind(&message_source_id)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(message)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;
    use sqlx::PgPool;

    #[tokio::test]
    #[ignore] // Ignoring by default as it requires an active PgPool
    async fn test_process_webhook_payload_cuj() {
        let pool = PgPool::connect("postgres://postgres:postgres@localhost/ohc").await.unwrap();
        let service = ChatService::new(pool);
        let tenant_id = Uuid::new_v4();
        let inbox_id = Uuid::new_v4();

        // Ensure inbox exists for the test
        let _inbox = service.create_inbox(tenant_id, "Test Inbox".to_string()).await.unwrap();

        let contact_source_id = "whatsapp_12345".to_string();
        let contact_name = Some("John Doe".to_string());
        let message_source_id = "msg_001".to_string();
        let message_content = "Hello, world!".to_string();

        // Run the payload process first time
        let msg1 = service.process_webhook_payload(
            tenant_id,
            _inbox.id,
            "whatsapp".to_string(),
            contact_source_id.clone(),
            contact_name.clone(),
            message_source_id.clone(),
            message_content.clone(),
        ).await.expect("Failed first insert");

        assert_eq!(msg1.content, "Hello, world!");

        // Run the payload process a second time with same IDs to prove idempotency
        let msg2 = service.process_webhook_payload(
            tenant_id,
            _inbox.id,
            "whatsapp".to_string(),
            contact_source_id.clone(),
            contact_name.clone(),
            message_source_id.clone(),
            "Hello, world again!".to_string(), // new content should overwrite due to DO UPDATE
        ).await.expect("Failed second insert");

        assert_eq!(msg1.id, msg2.id); // Same row should be updated
        assert_eq!(msg2.content, "Hello, world again!"); // Should be updated
    }
}
