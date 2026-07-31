use sqlx::PgPool;
use uuid::Uuid;
use super::models::{ChatInbox, ChatChannel, ChatContact, ChatConversation, ChatMessage};
use redis::AsyncCommands;

pub struct ChatService {
    pool: PgPool,
    redis_client: Option<redis::Client>,
}

impl ChatService {
    pub fn new(pool: PgPool, redis_url: Option<String>) -> Self {
        let redis_client = redis_url.and_then(|url| redis::Client::open(url).ok());
        Self { pool, redis_client }
    }

    async fn publish_event(&self, channel: &str, event_type: &str, payload: serde_json::Value) {
        if let Some(client) = &self.redis_client {
            if let Ok(mut con) = client.get_multiplexed_async_connection().await {
                let event = serde_json::json!({
                    "event_type": event_type,
                    "payload": payload,
                });
                let _: Result<(), _> = con.publish(channel, event.to_string()).await;
            }
        }
    }

    // --- INBOXES ---

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

    pub async fn get_inboxes(&self, tenant_id: Uuid) -> Result<Vec<ChatInbox>, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT id, tenant_id, name, created_at, updated_at
            FROM chat_inboxes
            WHERE tenant_id = $1
            ORDER BY created_at DESC
            "#
        )
        .bind(tenant_id)
        .fetch_all(&self.pool)
        .await
    }

    pub async fn get_inbox(&self, tenant_id: Uuid, id: Uuid) -> Result<Option<ChatInbox>, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT id, tenant_id, name, created_at, updated_at
            FROM chat_inboxes
            WHERE tenant_id = $1 AND id = $2
            "#
        )
        .bind(tenant_id)
        .bind(id)
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn update_inbox(&self, tenant_id: Uuid, id: Uuid, name: String) -> Result<ChatInbox, sqlx::Error> {
        sqlx::query_as(
            r#"
            UPDATE chat_inboxes
            SET name = $1, updated_at = NOW()
            WHERE tenant_id = $2 AND id = $3
            RETURNING id, tenant_id, name, created_at, updated_at
            "#
        )
        .bind(name)
        .bind(tenant_id)
        .bind(id)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn delete_inbox(&self, tenant_id: Uuid, id: Uuid) -> Result<u64, sqlx::Error> {
        let result = sqlx::query(
            r#"
            DELETE FROM chat_inboxes
            WHERE tenant_id = $1 AND id = $2
            "#
        )
        .bind(tenant_id)
        .bind(id)
        .execute(&self.pool)
        .await?;
        Ok(result.rows_affected())
    }

    // --- CHANNELS ---

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

    pub async fn get_channels(&self, tenant_id: Uuid, inbox_id: Uuid) -> Result<Vec<ChatChannel>, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT id, tenant_id, inbox_id, channel_type, config, created_at, updated_at
            FROM chat_channels
            WHERE tenant_id = $1 AND inbox_id = $2
            "#
        )
        .bind(tenant_id)
        .bind(inbox_id)
        .fetch_all(&self.pool)
        .await
    }

    // --- CONTACTS ---

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

    // --- CONVERSATIONS ---

    pub async fn start_conversation(
        &self,
        tenant_id: Uuid,
        inbox_id: Uuid,
        contact_id: Uuid,
        assignee_id: Option<Uuid>,
    ) -> Result<ChatConversation, sqlx::Error> {
        let conv: ChatConversation = sqlx::query_as(
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
        .await?;

        let channel = format!("chat_events:{}", tenant_id);
        self.publish_event(&channel, "conversation.updated", serde_json::to_value(&conv).unwrap()).await;

        Ok(conv)
    }

    pub async fn get_conversations(&self, tenant_id: Uuid, inbox_id: Option<Uuid>) -> Result<Vec<ChatConversation>, sqlx::Error> {
        if let Some(inbox) = inbox_id {
            sqlx::query_as(
                r#"
                SELECT id, tenant_id, inbox_id, contact_id, assignee_id, status, created_at, updated_at
                FROM chat_conversations
                WHERE tenant_id = $1 AND inbox_id = $2
                ORDER BY updated_at DESC
                "#
            )
            .bind(tenant_id)
            .bind(inbox)
            .fetch_all(&self.pool)
            .await
        } else {
            sqlx::query_as(
                r#"
                SELECT id, tenant_id, inbox_id, contact_id, assignee_id, status, created_at, updated_at
                FROM chat_conversations
                WHERE tenant_id = $1
                ORDER BY updated_at DESC
                "#
            )
            .bind(tenant_id)
            .fetch_all(&self.pool)
            .await
        }
    }

    pub async fn get_conversation(&self, tenant_id: Uuid, id: Uuid) -> Result<Option<ChatConversation>, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT id, tenant_id, inbox_id, contact_id, assignee_id, status, created_at, updated_at
            FROM chat_conversations
            WHERE tenant_id = $1 AND id = $2
            "#
        )
        .bind(tenant_id)
        .bind(id)
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn update_conversation(&self, tenant_id: Uuid, id: Uuid, status: String, assignee_id: Option<Uuid>) -> Result<ChatConversation, sqlx::Error> {
        let conv: ChatConversation = sqlx::query_as(
            r#"
            UPDATE chat_conversations
            SET status = $1, assignee_id = $2, updated_at = NOW()
            WHERE tenant_id = $3 AND id = $4
            RETURNING id, tenant_id, inbox_id, contact_id, assignee_id, status, created_at, updated_at
            "#
        )
        .bind(status)
        .bind(assignee_id)
        .bind(tenant_id)
        .bind(id)
        .fetch_one(&self.pool)
        .await?;

        let channel = format!("chat_events:{}", tenant_id);
        self.publish_event(&channel, "conversation.updated", serde_json::to_value(&conv).unwrap()).await;

        Ok(conv)
    }

    pub async fn delete_conversation(&self, tenant_id: Uuid, id: Uuid) -> Result<u64, sqlx::Error> {
        let result = sqlx::query(
            r#"
            DELETE FROM chat_conversations
            WHERE tenant_id = $1 AND id = $2
            "#
        )
        .bind(tenant_id)
        .bind(id)
        .execute(&self.pool)
        .await?;
        Ok(result.rows_affected())
    }


    // --- MESSAGES ---

    pub async fn send_message(
        &self,
        tenant_id: Uuid,
        conversation_id: Uuid,
        sender_type: String,
        sender_id: Option<Uuid>,
        content: String,
    ) -> Result<ChatMessage, sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        let msg: ChatMessage = sqlx::query_as(
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
        .bind(&content)
        .fetch_one(&mut *tx)
        .await?;

        // Update conversation updated_at
        sqlx::query(
            r#"
            UPDATE chat_conversations
            SET updated_at = NOW()
            WHERE tenant_id = $1 AND id = $2
            "#
        )
        .bind(tenant_id)
        .bind(conversation_id)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        let channel = format!("chat_events:{}", tenant_id);
        self.publish_event(&channel, "message.created", serde_json::to_value(&msg).unwrap()).await;

        Ok(msg)
    }

    pub async fn get_messages(&self, tenant_id: Uuid, conversation_id: Uuid) -> Result<Vec<ChatMessage>, sqlx::Error> {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;
    use sqlx::PgPool;

    async fn setup_db() -> PgPool {
        let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());
        PgPool::connect(&db_url).await.unwrap()
    }

    #[tokio::test]
    async fn test_inbox_crud() {
        let pool = setup_db().await;
        let service = ChatService::new(pool.clone(), None);
        let tenant_id = Uuid::new_v4();

        // Create inbox
        let inbox = service.create_inbox(tenant_id, "Support".to_string()).await.unwrap();
        assert_eq!(inbox.name, "Support");
        assert_eq!(inbox.tenant_id, tenant_id);

        // Get inboxes
        let inboxes = service.get_inboxes(tenant_id).await.unwrap();
        assert_eq!(inboxes.len(), 1);
        assert_eq!(inboxes[0].name, "Support");

        // Update inbox
        let updated = service.update_inbox(tenant_id, inbox.id, "Help Desk".to_string()).await.unwrap();
        assert_eq!(updated.name, "Help Desk");

        // Delete inbox
        let deleted = service.delete_inbox(tenant_id, inbox.id).await.unwrap();
        assert_eq!(deleted, 1);

        let inboxes_after = service.get_inboxes(tenant_id).await.unwrap();
        assert!(inboxes_after.is_empty());
    }

    #[tokio::test]
    async fn test_conversation_and_messages() {
        let pool = setup_db().await;
        let service = ChatService::new(pool.clone(), None);
        let tenant_id = Uuid::new_v4();

        let inbox = service.create_inbox(tenant_id, "Sales".to_string()).await.unwrap();
        let contact = service.create_contact(tenant_id, Some("Alice".to_string()), None, None).await.unwrap();

        // Start conversation
        let conv = service.start_conversation(tenant_id, inbox.id, contact.id, None).await.unwrap();
        assert_eq!(conv.inbox_id, inbox.id);
        assert_eq!(conv.status, "open");

        // Send message
        let msg = service.send_message(tenant_id, conv.id, "contact".to_string(), Some(contact.id), "Hello!".to_string()).await.unwrap();
        assert_eq!(msg.content, "Hello!");
        assert_eq!(msg.sender_type, "contact");

        // Get messages
        let msgs = service.get_messages(tenant_id, conv.id).await.unwrap();
        assert_eq!(msgs.len(), 1);
        assert_eq!(msgs[0].content, "Hello!");

        // Test tenant isolation
        let other_tenant = Uuid::new_v4();
        let other_msgs = service.get_messages(other_tenant, conv.id).await.unwrap();
        assert!(other_msgs.is_empty());
    }
}
