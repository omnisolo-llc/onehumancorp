use crate::models::{Message, WsEvent};
use chrono::Utc;
use tokio::sync::broadcast;
use uuid::Uuid;

// In a real application, this would talk to PostgreSQL using sqlx.
// For the foundational setup and acceptance criteria, we simulate it.
pub struct ChatService {
    tx: broadcast::Sender<WsEvent>,
}

impl ChatService {
    pub fn new(tx: broadcast::Sender<WsEvent>) -> Self {
        Self { tx }
    }

    pub async fn create_message(
        &self,
        tenant_id: Uuid,
        conversation_id: Uuid,
        content: String,
        sender_type: String,
    ) -> Result<Message, String> {
        // Enforce RLS logically here before DB interaction in real implementation
        // e.g. sqlx::query!("INSERT ... WHERE tenant_id = $1", tenant_id)

        let msg = Message {
            id: Uuid::new_v4(),
            tenant_id,
            conversation_id,
            sender_type,
            content,
            created_at: Utc::now(),
        };

        // Broadcast the event
        let _ = self.tx.send(WsEvent::MessageCreated {
            message: msg.clone(),
        });

        // Simulating Redis pub/sub for AI agents
        // redis_conn.publish("message.created", json!(msg)).await?;

        Ok(msg)
    }
}
