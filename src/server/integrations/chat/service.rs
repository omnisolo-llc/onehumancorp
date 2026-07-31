use sqlx::PgPool;
use uuid::Uuid;
use super::db::{ChatDb, ChatMessage, ChatConversation};

pub struct ChatService {
    db: ChatDb,
    redis: redis::Client,
}

impl ChatService {
    pub fn new(pool: PgPool, redis_url: &str) -> Result<Self, redis::RedisError> {
        let redis = redis::Client::open(redis_url)?;
        Ok(Self {
            db: ChatDb::new(pool),
            redis,
        })
    }

    pub async fn process_incoming_message(
        &self,
        tenant_id: Uuid,
        inbox_id: Uuid,
        contact_phone: &str,
        content: &str,
    ) -> Result<ChatMessage, sqlx::Error> {
        let contact = self.db.get_or_create_contact_by_phone(tenant_id, contact_phone).await?;

        let conversation = match self.db.get_conversation_by_contact_and_inbox(tenant_id, inbox_id, contact.id).await? {
            Some(c) => c,
            None => {
                let new_conv = ChatConversation {
                    id: Uuid::new_v4(),
                    tenant_id,
                    inbox_id,
                    contact_id: contact.id,
                    assignee_id: None,
                    status: "open".to_string(),
                    created_at: None,
                    updated_at: None,
                };
                self.db.create_conversation(new_conv).await?
            }
        };

        let message = ChatMessage {
            id: Uuid::new_v4(),
            tenant_id,
            conversation_id: conversation.id,
            sender_type: "contact".to_string(),
            sender_id: Some(contact.id),
            content: content.to_string(),
            created_at: None,
            updated_at: None,
        };

        let saved_message = self.db.create_message(message).await?;

        // Broadcast to redis pub/sub
        if let Ok(mut con) = self.redis.get_multiplexed_async_connection().await {
            let channel = format!("chat:tenant:{}", tenant_id);
            let payload = serde_json::to_string(&saved_message).unwrap_or_default();
            let _ : Result<(), _> = redis::cmd("PUBLISH")
                .arg(channel)
                .arg(payload)
                .query_async(&mut con).await;
        }

        Ok(saved_message)
    }
}
