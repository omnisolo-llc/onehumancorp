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
        sqlx::query("SET LOCAL app.current_tenant_id = $1")
            .bind(tenant_id)
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
        sqlx::query("SET LOCAL app.current_tenant_id = $1")
            .bind(tenant_id)
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
        sqlx::query("SET LOCAL app.current_tenant_id = $1")
            .bind(tenant_id)
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
        sqlx::query("SET LOCAL app.current_tenant_id = $1")
            .bind(tenant_id)
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

    pub async fn send_message(
        &self,
        tenant_id: Uuid,
        conversation_id: Uuid,
        sender_type: String,
        sender_id: Option<Uuid>,
        content: String,
    ) -> Result<ChatMessage, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        sqlx::query("SET LOCAL app.current_tenant_id = $1")
            .bind(tenant_id)
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
    use sqlx::PgPool;
    use uuid::Uuid;

    async fn get_test_pool() -> PgPool {
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/omnisolo".to_string());
        PgPool::connect(&database_url).await.unwrap()
    }

    #[tokio::test]
    async fn test_create_inbox() {
        let pool = get_test_pool().await;
        let tenant_id = Uuid::new_v4();
        let service = ChatService::new(pool);

        let inbox = service
            .create_inbox(tenant_id, "Test Inbox".to_string())
            .await
            .unwrap();

        assert_eq!(inbox.name, "Test Inbox");
        assert_eq!(inbox.tenant_id, tenant_id);
    }

    #[tokio::test]
    async fn test_create_channel() {
        let pool = get_test_pool().await;
        let tenant_id = Uuid::new_v4();
        let service = ChatService::new(pool);

        let inbox = service
            .create_inbox(tenant_id, "Test Inbox for Channel".to_string())
            .await
            .unwrap();

        let channel = service
            .create_channel(
                tenant_id,
                inbox.id,
                "web".to_string(),
                serde_json::json!({"theme": "dark"}),
            )
            .await
            .unwrap();

        assert_eq!(channel.channel_type, "web");
        assert_eq!(channel.tenant_id, tenant_id);
        assert_eq!(channel.inbox_id, inbox.id);
    }

    #[tokio::test]
    async fn test_create_contact() {
        let pool = get_test_pool().await;
        let tenant_id = Uuid::new_v4();
        let service = ChatService::new(pool);

        let contact = service
            .create_contact(
                tenant_id,
                Some("John Doe".to_string()),
                Some("john@example.com".to_string()),
                Some("1234567890".to_string()),
            )
            .await
            .unwrap();

        assert_eq!(contact.name.unwrap(), "John Doe");
        assert_eq!(contact.tenant_id, tenant_id);
    }

    #[tokio::test]
    async fn test_start_conversation() {
        let pool = get_test_pool().await;
        let tenant_id = Uuid::new_v4();
        let service = ChatService::new(pool);

        let inbox = service
            .create_inbox(tenant_id, "Inbox for Conv".to_string())
            .await
            .unwrap();
        let contact = service
            .create_contact(tenant_id, Some("Jane Doe".to_string()), None, None)
            .await
            .unwrap();

        let conversation = service
            .start_conversation(tenant_id, inbox.id, contact.id, None)
            .await
            .unwrap();

        assert_eq!(conversation.status, "open");
        assert_eq!(conversation.tenant_id, tenant_id);
        assert_eq!(conversation.inbox_id, inbox.id);
        assert_eq!(conversation.contact_id, contact.id);
    }

    #[tokio::test]
    async fn test_send_message() {
        let pool = get_test_pool().await;
        let tenant_id = Uuid::new_v4();
        let service = ChatService::new(pool);

        let inbox = service
            .create_inbox(tenant_id, "Inbox for Msg".to_string())
            .await
            .unwrap();
        let contact = service
            .create_contact(tenant_id, Some("Alice".to_string()), None, None)
            .await
            .unwrap();
        let conversation = service
            .start_conversation(tenant_id, inbox.id, contact.id, None)
            .await
            .unwrap();

        let message = service
            .send_message(
                tenant_id,
                conversation.id,
                "contact".to_string(),
                Some(contact.id),
                "Hello world!".to_string(),
            )
            .await
            .unwrap();

        assert_eq!(message.content, "Hello world!");
        assert_eq!(message.tenant_id, tenant_id);
        assert_eq!(message.conversation_id, conversation.id);
        assert_eq!(message.sender_id, Some(contact.id));
    }
}
