use sqlx::PgPool;
use uuid::Uuid;
use super::models::{ChatInbox, ChatChannel, ChatContact, ChatConversation, ChatMessage};
use crate::msgbus::{Bus, Message};
use std::sync::Arc;

pub struct ChatService {
    pool: PgPool,
    bus: Arc<dyn Bus>,
}

impl ChatService {
    pub fn new(pool: PgPool, bus: Arc<dyn Bus>) -> Self {
        Self { pool, bus }
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

        // Publish event for real-time updates
        let topic = format!("tenant:{}:chat:conversation:{}", tenant_id, conversation_id);
        if let Ok(payload) = serde_json::to_vec(&msg) {
            let bus_msg = Message {
                topic,
                payload,
            };
            let _ = self.bus.publish(bus_msg).await;
        }

        Ok(msg)
    }

    pub async fn get_conversations(
        &self,
        tenant_id: Uuid,
    ) -> Result<Vec<ChatConversation>, sqlx::Error> {
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
    use crate::msgbus::{MemoryBus, Bus, Message};
    use std::sync::atomic::{AtomicBool, Ordering};

    #[sqlx::test]
    async fn test_chat_service_flow(pool: PgPool) {
        let bus = Arc::new(MemoryBus::new());
        let service = ChatService::new(pool.clone(), bus.clone());
        let tenant_id = Uuid::new_v4();

        // 1. Create Inbox
        let inbox = service.create_inbox(tenant_id, "Test Inbox".to_string()).await.expect("Failed to create inbox");
        assert_eq!(inbox.name, "Test Inbox");

        // 2. Create Contact
        let contact = service.create_contact(tenant_id, Some("Test Contact".to_string()), None, None).await.expect("Failed to create contact");

        // 3. Start Conversation
        let conv = service.start_conversation(tenant_id, inbox.id, contact.id, None).await.expect("Failed to start conv");

        // Subscribe to bus
        let received = Arc::new(AtomicBool::new(false));
        let rx = received.clone();
        let topic = format!("tenant:{}:chat:conversation:{}", tenant_id, conv.id);

        let _cancel = bus.subscribe(topic.clone(), Box::new(move |_msg: Message| {
            rx.store(true, Ordering::SeqCst);
        })).await.unwrap();

        // 4. Send Message
        let msg = service.send_message(tenant_id, conv.id, "agent".to_string(), None, "Hello!".to_string()).await.expect("Failed to send message");
        assert_eq!(msg.content, "Hello!");

        // Give bus a tiny moment to process
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        assert!(received.load(Ordering::SeqCst), "Did not receive bus event");

        // 5. Get Conversations
        let convs = service.get_conversations(tenant_id).await.unwrap();
        assert_eq!(convs.len(), 1);

        // 6. Get Messages
        let msgs = service.get_messages(tenant_id, conv.id).await.unwrap();
        assert_eq!(msgs.len(), 1);
        assert_eq!(msgs[0].content, "Hello!");
    }
}
