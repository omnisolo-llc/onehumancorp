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

    async fn publish_event(&self, tenant_id: Uuid, inbox_id: Uuid, event_type: &str, data: serde_json::Value) {
        if let Some(client) = crate::redis_pool::get_redis_client() {
            if let Ok(mut conn) = client.get_multiplexed_async_connection().await {
                let channel = format!("tenant:{}:inbox:{}", tenant_id, inbox_id);
                let payload = serde_json::json!({
                    "event": event_type,
                    "data": data,
                });
                if let Ok(payload_str) = serde_json::to_string(&payload) {
                    use redis::AsyncCommands;
                    let _: Result<(), _> = conn.publish(channel, payload_str).await;
                }
            }
        }
    }

    pub async fn create_inbox(
        &self,
        tenant_id: Uuid,
        name: String,
        auto_assignment: Option<bool>,
    ) -> Result<ChatInbox, sqlx::Error> {
        sqlx::query_as(
            r#"
            INSERT INTO chat_inboxes (id, tenant_id, name, auto_assignment)
            VALUES ($1, $2, $3, $4)
            RETURNING id, tenant_id, name, auto_assignment, created_at, updated_at
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(name)
        .bind(auto_assignment.unwrap_or(true))
        .fetch_one(&self.pool)
        .await
    }

    pub async fn get_inbox(
        &self,
        tenant_id: Uuid,
        inbox_id: Uuid,
    ) -> Result<ChatInbox, sqlx::Error> {
        sqlx::query_as(
            "SELECT id, tenant_id, name, auto_assignment, created_at, updated_at
             FROM chat_inboxes
             WHERE tenant_id = $1 AND id = $2"
        )
        .bind(tenant_id)
        .bind(inbox_id)
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
        custom_attributes: Option<serde_json::Value>,
    ) -> Result<ChatContact, sqlx::Error> {
        sqlx::query_as(
            r#"
            INSERT INTO chat_contacts (id, tenant_id, name, email, phone, custom_attributes)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, tenant_id, name, email, phone, custom_attributes, created_at, updated_at
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(name)
        .bind(email)
        .bind(phone)
        .bind(custom_attributes.unwrap_or_else(|| serde_json::json!({})))
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
        let conversation: ChatConversation = sqlx::query_as(
            r#"
            INSERT INTO chat_conversations (id, tenant_id, inbox_id, contact_id, assignee_id, status)
            VALUES ($1, $2, $3, $4, $5, 'open')
            RETURNING id, tenant_id, inbox_id, contact_id, assignee_id, status, last_activity_at, created_at, updated_at
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(inbox_id)
        .bind(contact_id)
        .bind(assignee_id)
        .fetch_one(&self.pool)
        .await?;

        self.publish_event(tenant_id, inbox_id, "conversation.created", serde_json::to_value(&conversation).unwrap_or_default()).await;

        Ok(conversation)
    }

    pub async fn get_conversation(
        &self,
        tenant_id: Uuid,
        conversation_id: Uuid,
    ) -> Result<ChatConversation, sqlx::Error> {
        sqlx::query_as(
            "SELECT id, tenant_id, inbox_id, contact_id, assignee_id, status, last_activity_at, created_at, updated_at
             FROM chat_conversations
             WHERE tenant_id = $1 AND id = $2"
        )
        .bind(tenant_id)
        .bind(conversation_id)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn get_conversations_for_inbox(
        &self,
        tenant_id: Uuid,
        inbox_id: Uuid,
    ) -> Result<Vec<ChatConversation>, sqlx::Error> {
        sqlx::query_as(
            "SELECT id, tenant_id, inbox_id, contact_id, assignee_id, status, last_activity_at, created_at, updated_at
             FROM chat_conversations
             WHERE tenant_id = $1 AND inbox_id = $2
             ORDER BY last_activity_at DESC"
        )
        .bind(tenant_id)
        .bind(inbox_id)
        .fetch_all(&self.pool)
        .await
    }

    pub async fn update_conversation_status(
        &self,
        tenant_id: Uuid,
        conversation_id: Uuid,
        status: String,
    ) -> Result<ChatConversation, sqlx::Error> {
        let conv: ChatConversation = sqlx::query_as(
            r#"
            UPDATE chat_conversations
            SET status = $1, last_activity_at = NOW(), updated_at = NOW()
            WHERE tenant_id = $2 AND id = $3
            RETURNING id, tenant_id, inbox_id, contact_id, assignee_id, status, last_activity_at, created_at, updated_at
            "#
        )
        .bind(status)
        .bind(tenant_id)
        .bind(conversation_id)
        .fetch_one(&self.pool)
        .await?;

        self.publish_event(tenant_id, conv.inbox_id, "conversation.updated", serde_json::to_value(&conv).unwrap_or_default()).await;

        Ok(conv)
    }

    pub async fn update_conversation_assignee(
        &self,
        tenant_id: Uuid,
        conversation_id: Uuid,
        assignee_id: Option<Uuid>,
    ) -> Result<ChatConversation, sqlx::Error> {
        let conv: ChatConversation = sqlx::query_as(
            r#"
            UPDATE chat_conversations
            SET assignee_id = $1, last_activity_at = NOW(), updated_at = NOW()
            WHERE tenant_id = $2 AND id = $3
            RETURNING id, tenant_id, inbox_id, contact_id, assignee_id, status, last_activity_at, created_at, updated_at
            "#
        )
        .bind(assignee_id)
        .bind(tenant_id)
        .bind(conversation_id)
        .fetch_one(&self.pool)
        .await?;

        self.publish_event(tenant_id, conv.inbox_id, "conversation.updated", serde_json::to_value(&conv).unwrap_or_default()).await;

        Ok(conv)
    }

    pub async fn send_message(
        &self,
        tenant_id: Uuid,
        conversation_id: Uuid,
        sender_type: String,
        sender_id: Option<Uuid>,
        content: String,
        message_type: Option<String>,
        is_private: Option<bool>,
    ) -> Result<ChatMessage, sqlx::Error> {
        let msg_type = message_type.unwrap_or_else(|| "incoming".to_string());
        let private = is_private.unwrap_or(false);

        let conv = self.get_conversation(tenant_id, conversation_id).await?;

        let mut tx = self.pool.begin().await?;

        let message: ChatMessage = sqlx::query_as(
            r#"
            INSERT INTO chat_messages (id, tenant_id, conversation_id, sender_type, sender_id, content, message_type, is_private)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id, tenant_id, conversation_id, sender_type, sender_id, content, message_type, is_private, created_at, updated_at
            "#
        )
        .bind(Uuid::new_v4())
        .bind(tenant_id)
        .bind(conversation_id)
        .bind(sender_type)
        .bind(sender_id)
        .bind(content)
        .bind(msg_type)
        .bind(private)
        .fetch_one(&mut *tx)
        .await?;

        sqlx::query(
            "UPDATE chat_conversations
             SET last_activity_at = NOW(), updated_at = NOW()
             WHERE tenant_id = $1 AND id = $2"
        )
        .bind(tenant_id)
        .bind(conversation_id)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        self.publish_event(tenant_id, conv.inbox_id, "message.created", serde_json::to_value(&message).unwrap_or_default()).await;

        Ok(message)
    }

    pub async fn get_messages_for_conversation(
        &self,
        tenant_id: Uuid,
        conversation_id: Uuid,
    ) -> Result<Vec<ChatMessage>, sqlx::Error> {
        sqlx::query_as(
            "SELECT id, tenant_id, conversation_id, sender_type, sender_id, content, message_type, is_private, created_at, updated_at
             FROM chat_messages
             WHERE tenant_id = $1 AND conversation_id = $2
             ORDER BY created_at ASC"
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

    async fn get_test_pool() -> Option<PgPool> {
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/ohc".to_string());
        PgPool::connect(&database_url).await.ok()
    }

    #[tokio::test]
    async fn test_chat_service_full_flow_and_isolation() {
        let Some(pool) = get_test_pool().await else {
            println!("Skipping chat service integration test: no DB pool available");
            return;
        };

        // Create the service
        let service = ChatService::new(pool);

        let tenant_a = Uuid::new_v4();
        let tenant_b = Uuid::new_v4();

        // 1. Create Inboxes
        let inbox_a = service.create_inbox(tenant_a, "Maya Inbox A".to_string(), Some(true)).await.unwrap();
        assert_eq!(inbox_a.tenant_id, tenant_a);
        assert_eq!(inbox_a.name, "Maya Inbox A");
        assert!(inbox_a.auto_assignment);

        let inbox_b = service.create_inbox(tenant_b, "Carlos Inbox B".to_string(), Some(false)).await.unwrap();
        assert_eq!(inbox_b.tenant_id, tenant_b);
        assert_eq!(inbox_b.name, "Carlos Inbox B");
        assert!(!inbox_b.auto_assignment);

        // 2. Tenant isolation on get_inbox
        let fetched_a = service.get_inbox(tenant_a, inbox_a.id).await.unwrap();
        assert_eq!(fetched_a.name, "Maya Inbox A");

        // Tenant B should not be able to fetch Tenant A's inbox
        let fetched_b_fail = service.get_inbox(tenant_b, inbox_a.id).await;
        assert!(fetched_b_fail.is_err());

        // 3. Create Channels
        let channel_a = service.create_channel(
            tenant_a,
            inbox_a.id,
            "Instagram".to_string(),
            serde_json::json!({"handle": "maya_bakes"}),
        )
        .await
        .unwrap();
        assert_eq!(channel_a.tenant_id, tenant_a);
        assert_eq!(channel_a.inbox_id, inbox_a.id);
        assert_eq!(channel_a.channel_type, "Instagram");

        // 4. Create Contacts
        let contact_a = service.create_contact(
            tenant_a,
            Some("John Doe".to_string()),
            Some("john@example.com".to_string()),
            None,
            Some(serde_json::json!({"preferred_color": "pink"})),
        )
        .await
        .unwrap();
        assert_eq!(contact_a.tenant_id, tenant_a);
        assert_eq!(contact_a.name, Some("John Doe".to_string()));
        assert_eq!(contact_a.email, Some("john@example.com".to_string()));
        assert_eq!(contact_a.custom_attributes["preferred_color"], "pink");

        // 5. Start Conversation
        let conv_a = service.start_conversation(
            tenant_a,
            inbox_a.id,
            contact_a.id,
            None,
        )
        .await
        .unwrap();
        assert_eq!(conv_a.tenant_id, tenant_a);
        assert_eq!(conv_a.inbox_id, inbox_a.id);
        assert_eq!(conv_a.contact_id, contact_a.id);
        assert_eq!(conv_a.status, "open");

        // 6. Tenant isolation on get_conversation
        let fetched_conv = service.get_conversation(tenant_a, conv_a.id).await.unwrap();
        assert_eq!(fetched_conv.id, conv_a.id);

        let fetched_conv_fail = service.get_conversation(tenant_b, conv_a.id).await;
        assert!(fetched_conv_fail.is_err());

        // 7. Update status and assignee
        let agent_id = Uuid::new_v4();
        let conv_updated = service.update_conversation_assignee(tenant_a, conv_a.id, Some(agent_id)).await.unwrap();
        assert_eq!(conv_updated.assignee_id, Some(agent_id));

        let conv_status_updated = service.update_conversation_status(tenant_a, conv_a.id, "resolved".to_string()).await.unwrap();
        assert_eq!(conv_status_updated.status, "resolved");

        // 8. Send Messages
        let msg1 = service.send_message(
            tenant_a,
            conv_a.id,
            "Contact".to_string(),
            Some(contact_a.id),
            "I want a vanilla cake".to_string(),
            Some("incoming".to_string()),
            Some(false),
        )
        .await
        .unwrap();
        assert_eq!(msg1.tenant_id, tenant_a);
        assert_eq!(msg1.conversation_id, conv_a.id);
        assert_eq!(msg1.content, "I want a vanilla cake");
        assert_eq!(msg1.message_type, "incoming");
        assert!(!msg1.is_private);

        let _msg2 = service.send_message(
            tenant_a,
            conv_a.id,
            "Agent".to_string(),
            Some(agent_id),
            "Sure, we can do that!".to_string(),
            Some("outgoing".to_string()),
            Some(false),
        )
        .await
        .unwrap();

        // 9. List messages & check tenant isolation
        let msgs = service.get_messages_for_conversation(tenant_a, conv_a.id).await.unwrap();
        assert_eq!(msgs.len(), 2);
        assert_eq!(msgs[0].content, "I want a vanilla cake");
        assert_eq!(msgs[1].content, "Sure, we can do that!");

        let msgs_fail = service.get_messages_for_conversation(tenant_b, conv_a.id).await.unwrap();
        assert!(msgs_fail.is_empty());

        // 10. List conversations for inbox
        let convs = service.get_conversations_for_inbox(tenant_a, inbox_a.id).await.unwrap();
        assert_eq!(convs.len(), 1);
        assert_eq!(convs[0].id, conv_a.id);

        let convs_fail = service.get_conversations_for_inbox(tenant_b, inbox_a.id).await.unwrap();
        assert!(convs_fail.is_empty());
    }
}
