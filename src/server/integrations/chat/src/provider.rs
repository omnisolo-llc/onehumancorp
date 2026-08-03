use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: String,
    pub tenant_id: Uuid,
    pub channel_id: String,
    pub contact_id: String,
    pub content: String,
    pub role: String,
    pub created_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub id: String,
    pub tenant_id: Uuid,
    pub contact_id: String,
    pub status: String,
    pub unread_count: i32,
    pub created_at: i64,
    pub updated_at: i64,
}

#[async_trait::async_trait]
pub trait ChatProvider: Send + Sync {
    async fn receive_message(&self, message: ChatMessage) -> Result<(), String>;
    async fn send_message(&self, tenant_id: &Uuid, channel_id: &str, contact_id: &str, content: &str) -> Result<String, String>;
    async fn get_conversations(&self, tenant_id: &Uuid) -> Result<Vec<Conversation>, String>;
    async fn resolve_conversation(&self, tenant_id: &Uuid, conversation_id: &str) -> Result<(), String>;
}

pub struct InMemoryChatProvider {
    pub messages: Arc<Mutex<Vec<ChatMessage>>>,
    pub conversations: Arc<Mutex<Vec<Conversation>>>,
}

impl InMemoryChatProvider {
    pub fn new() -> Self {
        Self {
            messages: Arc::new(Mutex::new(Vec::new())),
            conversations: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

#[async_trait::async_trait]
impl ChatProvider for InMemoryChatProvider {
    async fn receive_message(&self, message: ChatMessage) -> Result<(), String> {
        let mut msgs = self.messages.lock().await;
        msgs.push(message);
        Ok(())
    }

    async fn send_message(&self, tenant_id: &Uuid, channel_id: &str, contact_id: &str, content: &str) -> Result<String, String> {
        let id = Uuid::new_v4().to_string();
        let message = ChatMessage {
            id: id.clone(),
            tenant_id: tenant_id.clone(),
            channel_id: channel_id.to_string(),
            contact_id: contact_id.to_string(),
            content: content.to_string(),
            role: "agent".to_string(),
            created_at: 0,
        };
        let mut msgs = self.messages.lock().await;
        msgs.push(message);
        Ok(id)
    }

    async fn get_conversations(&self, _tenant_id: &Uuid) -> Result<Vec<Conversation>, String> {
        let convs = self.conversations.lock().await;
        Ok(convs.clone())
    }

    async fn resolve_conversation(&self, _tenant_id: &Uuid, conversation_id: &str) -> Result<(), String> {
        let mut convs = self.conversations.lock().await;
        for c in convs.iter_mut() {
            if c.id == conversation_id {
                c.status = "resolved".to_string();
                break;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_in_memory_provider() {
        let provider = InMemoryChatProvider::new();
        let tenant_id = Uuid::new_v4();

        let msg = ChatMessage {
            id: "msg1".to_string(),
            tenant_id: tenant_id.clone(),
            channel_id: "whatsapp".to_string(),
            contact_id: "user1".to_string(),
            content: "Hello".to_string(),
            role: "user".to_string(),
            created_at: 0,
        };

        provider.receive_message(msg).await.unwrap();

        let sent_id = provider.send_message(&tenant_id, "whatsapp", "user1", "Hi there!").await.unwrap();

        let msgs = provider.messages.lock().await;
        assert_eq!(msgs.len(), 2);
        assert_eq!(msgs[1].id, sent_id);
    }
}
