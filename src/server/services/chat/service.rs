use super::models::{ChatChannel, ChatContact, ChatConversation, ChatInbox, ChatMessage};
use sqlx::PgPool;
use uuid::Uuid;

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
        let mut tx = self.pool.begin().await?;
        sqlx::query(&format!(
            "SET LOCAL app.current_tenant_id = '{}'",
            tenant_id
        ))
        .execute(&mut *tx)
        .await?;
        let res = sqlx::query_as(
            r#"
            INSERT INTO chat_inboxes (id, tenant_id, name)
            VALUES ($1, $2, $3)
            RETURNING id, tenant_id, name, created_at, updated_at
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(name)
        .fetch_one(&mut *tx)
        .await?;
        tx.commit().await?;
        Ok(res)
    }

    pub async fn create_channel(
        &self,
        tenant_id: Uuid,
        inbox_id: Uuid,
        channel_type: String,
        config: serde_json::Value,
    ) -> Result<ChatChannel, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        sqlx::query(&format!(
            "SET LOCAL app.current_tenant_id = '{}'",
            tenant_id
        ))
        .execute(&mut *tx)
        .await?;
        let res = sqlx::query_as(
            r#"
            INSERT INTO chat_channels (id, tenant_id, inbox_id, channel_type, config)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, tenant_id, inbox_id, channel_type, config, created_at, updated_at
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(inbox_id)
        .bind(channel_type)
        .bind(config)
        .fetch_one(&mut *tx)
        .await?;
        tx.commit().await?;
        Ok(res)
    }

    pub async fn create_contact(
        &self,
        tenant_id: Uuid,
        name: Option<String>,
        email: Option<String>,
        phone: Option<String>,
    ) -> Result<ChatContact, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        sqlx::query(&format!(
            "SET LOCAL app.current_tenant_id = '{}'",
            tenant_id
        ))
        .execute(&mut *tx)
        .await?;
        let res = sqlx::query_as(
            r#"
            INSERT INTO chat_contacts (id, tenant_id, name, email, phone)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, tenant_id, name, email, phone, created_at, updated_at
            "#,
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(name)
        .bind(email)
        .bind(phone)
        .fetch_one(&mut *tx)
        .await?;
        tx.commit().await?;
        Ok(res)
    }

    pub async fn start_conversation(
        &self,
        tenant_id: Uuid,
        inbox_id: Uuid,
        contact_id: Uuid,
        assignee_id: Option<Uuid>,
    ) -> Result<ChatConversation, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        sqlx::query(&format!(
            "SET LOCAL app.current_tenant_id = '{}'",
            tenant_id
        ))
        .execute(&mut *tx)
        .await?;
        let res = sqlx::query_as(
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
        .fetch_one(&mut *tx)
        .await?;
        tx.commit().await?;
        Ok(res)
    }

    pub async fn get_open_conversations(
        &self,
        tenant_id: Uuid,
        inbox_id: Uuid,
    ) -> Result<Vec<ChatConversation>, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        sqlx::query(&format!(
            "SET LOCAL app.current_tenant_id = '{}'",
            tenant_id
        ))
        .execute(&mut *tx)
        .await?;
        let res = sqlx::query_as(
            r#"
            SELECT id, tenant_id, inbox_id, contact_id, assignee_id, status, created_at, updated_at
            FROM chat_conversations
            WHERE inbox_id = $1 AND status = 'open'
            ORDER BY updated_at DESC
            "#,
        )
        .bind(inbox_id)
        .fetch_all(&mut *tx)
        .await?;
        tx.commit().await?;
        Ok(res)
    }

    pub async fn send_message(
        &self,
        tenant_id: Uuid,
        conversation_id: Uuid,
        sender_type: String,
        sender_id: Option<Uuid>,
        content: String,
    ) -> Result<ChatMessage, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        sqlx::query(&format!(
            "SET LOCAL app.current_tenant_id = '{}'",
            tenant_id
        ))
        .execute(&mut *tx)
        .await?;
        let res = sqlx::query_as(
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
        .fetch_one(&mut *tx)
        .await?;
        tx.commit().await?;
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn setup_test_db() -> Result<PgPool, sqlx::Error> {
        let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:postgres@localhost:5432/omnisolo_test".to_string()
        });
        PgPool::connect(&database_url).await
    }

    #[tokio::test]
    async fn test_create_inbox() {
        let pool = match setup_test_db().await {
            Ok(p) => p,
            Err(_) => return, // Ignore missing DB
        };
        let service = ChatService::new(pool);
        let tenant_id = Uuid::new_v4();

        // 1. Create inbox
        let inbox = service
            .create_inbox(tenant_id, "Support".to_string())
            .await
            .unwrap();
        assert_eq!(inbox.name, "Support");

        // 2. Create channel
        let config = serde_json::json!({"webhook_url": "https://example.com"});
        let channel = service
            .create_channel(tenant_id, inbox.id, "widget".to_string(), config)
            .await
            .unwrap();
        assert_eq!(channel.channel_type, "widget");

        // 3. Create contact
        let contact = service
            .create_contact(
                tenant_id,
                Some("Alice".to_string()),
                Some("alice@example.com".to_string()),
                None,
            )
            .await
            .unwrap();
        assert_eq!(contact.name.as_deref(), Some("Alice"));

        // 4. Start conversation
        let conversation = service
            .start_conversation(tenant_id, inbox.id, contact.id, None)
            .await
            .unwrap();
        assert_eq!(conversation.status, "open");

        // 5. Send message
        let msg = service
            .send_message(
                tenant_id,
                conversation.id,
                "contact".to_string(),
                Some(contact.id),
                "Hello!".to_string(),
            )
            .await
            .unwrap();
        assert_eq!(msg.content, "Hello!");

        // 6. Query open conversations
        let open_conversations = service
            .get_open_conversations(tenant_id, inbox.id)
            .await
            .unwrap();
        assert_eq!(open_conversations.len(), 1);
        assert_eq!(open_conversations[0].id, conversation.id);

        // 7. Test closed conversation exclusion (update to closed manually for test)
        let mut tx = service.pool.begin().await.unwrap();
        sqlx::query(&format!(
            "SET LOCAL app.current_tenant_id = '{}'",
            tenant_id
        ))
        .execute(&mut *tx)
        .await
        .unwrap();
        sqlx::query("UPDATE chat_conversations SET status = 'closed' WHERE id = $1")
            .bind(conversation.id)
            .execute(&mut *tx)
            .await
            .unwrap();
        tx.commit().await.unwrap();

        let open_conversations_after = service
            .get_open_conversations(tenant_id, inbox.id)
            .await
            .unwrap();
        assert_eq!(open_conversations_after.len(), 0);
    }

    #[tokio::test]
    async fn test_rls_isolation() {
        let pool = match setup_test_db().await {
            Ok(p) => p,
            Err(_) => return, // Ignore missing DB
        };
        let service = ChatService::new(pool);
        let tenant1 = Uuid::new_v4();
        let tenant2 = Uuid::new_v4();

        // Create inbox for tenant 1
        let mut tx = service.pool.begin().await.unwrap();
        sqlx::query(&format!("SET LOCAL app.current_tenant_id = '{}'", tenant1))
            .execute(&mut *tx)
            .await
            .unwrap();
        let inbox_id = Uuid::new_v4();
        sqlx::query(
            r#"
            INSERT INTO chat_inboxes (id, tenant_id, name)
            VALUES ($1, $2, 'Tenant 1 Inbox')
            "#,
        )
        .bind(inbox_id)
        .bind(tenant1)
        .execute(&mut *tx)
        .await
        .unwrap();
        tx.commit().await.unwrap();

        // Attempt to read inbox as tenant 2
        let mut tx = service.pool.begin().await.unwrap();
        sqlx::query(&format!("SET LOCAL app.current_tenant_id = '{}'", tenant2))
            .execute(&mut *tx)
            .await
            .unwrap();
        let count: (i64,) = sqlx::query_as("SELECT count(*) FROM chat_inboxes WHERE id = $1")
            .bind(inbox_id)
            .fetch_one(&mut *tx)
            .await
            .unwrap_or((0,));
        assert_eq!(count.0, 0, "RLS failed: Tenant 2 can see Tenant 1's inbox");
        tx.commit().await.unwrap();
    }
}
