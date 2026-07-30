use sqlx::PgPool;
use uuid::Uuid;
use super::models::{ChatInbox, ChatChannel, ChatContact, ChatConversation, ChatMessage, ChatContactInbox, ChatCannedResponse};

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

    pub async fn create_canned_response(
        &self,
        tenant_id: Uuid,
        short_code: String,
        content: String,
    ) -> Result<ChatCannedResponse, sqlx::Error> {
        sqlx::query_as(
            r#"
            INSERT INTO chat_canned_responses (id, tenant_id, short_code, content)
            VALUES ($1, $2, $3, $4)
            RETURNING id, tenant_id, short_code, content, created_at, updated_at
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(short_code)
        .bind(content)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn get_canned_responses(
        &self,
        tenant_id: Uuid,
    ) -> Result<Vec<ChatCannedResponse>, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT id, tenant_id, short_code, content, created_at, updated_at
            FROM chat_canned_responses
            WHERE tenant_id = $1
            ORDER BY created_at DESC
            "#
        )
        .bind(tenant_id)
        .fetch_all(&self.pool)
        .await
    }

    pub async fn create_contact_inbox(
        &self,
        tenant_id: Uuid,
        contact_id: Uuid,
        inbox_id: Uuid,
        source_id: String,
    ) -> Result<ChatContactInbox, sqlx::Error> {
        sqlx::query_as(
            r#"
            INSERT INTO chat_contact_inboxes (id, tenant_id, contact_id, inbox_id, source_id)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, tenant_id, contact_id, inbox_id, source_id, created_at, updated_at
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(contact_id)
        .bind(inbox_id)
        .bind(source_id)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn get_contact_inboxes(
        &self,
        tenant_id: Uuid,
        contact_id: Uuid,
    ) -> Result<Vec<ChatContactInbox>, sqlx::Error> {
        sqlx::query_as(
            r#"
            SELECT id, tenant_id, contact_id, inbox_id, source_id, created_at, updated_at
            FROM chat_contact_inboxes
            WHERE tenant_id = $1 AND contact_id = $2
            ORDER BY created_at DESC
            "#
        )
        .bind(tenant_id)
        .bind(contact_id)
        .fetch_all(&self.pool)
        .await
    }

    pub async fn get_inboxes(
        &self,
        tenant_id: Uuid,
    ) -> Result<Vec<ChatInbox>, sqlx::Error> {
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
            ORDER BY created_at DESC
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;

    #[sqlx::test]
    async fn test_chat_service_crud(pool: PgPool) {
        let service = ChatService::new(pool);
        let tenant_id = Uuid::new_v4();

        // 1. Create Inbox
        let inbox = service.create_inbox(tenant_id, "Support".to_string()).await.unwrap();
        assert_eq!(inbox.name, "Support");
        assert_eq!(inbox.tenant_id, tenant_id);

        let inboxes = service.get_inboxes(tenant_id).await.unwrap();
        assert_eq!(inboxes.len(), 1);

        // 2. Create Channel
        let config = serde_json::json!({"webhook_url": "http://localhost"});
        let channel = service.create_channel(tenant_id, inbox.id, "webhook".to_string(), config).await.unwrap();
        assert_eq!(channel.channel_type, "webhook");

        // 3. Create Contact
        let contact = service.create_contact(tenant_id, Some("Alice".to_string()), Some("alice@example.com".to_string()), None).await.unwrap();
        assert_eq!(contact.name.unwrap(), "Alice");

        // 4. Create Contact Inbox
        let contact_inbox = service.create_contact_inbox(tenant_id, contact.id, inbox.id, "ext-123".to_string()).await.unwrap();
        assert_eq!(contact_inbox.source_id, "ext-123");

        let contact_inboxes = service.get_contact_inboxes(tenant_id, contact.id).await.unwrap();
        assert_eq!(contact_inboxes.len(), 1);

        // 5. Start Conversation
        let conversation = service.start_conversation(tenant_id, inbox.id, contact.id, None).await.unwrap();
        assert_eq!(conversation.status, "open");

        let conversations = service.get_conversations(tenant_id, inbox.id).await.unwrap();
        assert_eq!(conversations.len(), 1);

        // 6. Send Message
        let message = service.send_message(tenant_id, conversation.id, "customer".to_string(), None, "Hello!".to_string()).await.unwrap();
        assert_eq!(message.content, "Hello!");

        let messages = service.get_messages(tenant_id, conversation.id).await.unwrap();
        assert_eq!(messages.len(), 1);

        // 7. Canned Responses
        let canned = service.create_canned_response(tenant_id, "greet".to_string(), "Hi there!".to_string()).await.unwrap();
        assert_eq!(canned.short_code, "greet");

        let canned_responses = service.get_canned_responses(tenant_id).await.unwrap();
        assert_eq!(canned_responses.len(), 1);
    }
}
