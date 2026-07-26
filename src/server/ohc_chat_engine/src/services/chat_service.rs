use crate::models::{Conversation, Message};
use async_trait::async_trait;
use uuid::Uuid;
use chrono::Utc;

#[async_trait]
pub trait ChatEngine {
    async fn create_conversation(&self, tenant_id: Uuid, channel_id: Uuid) -> Result<Conversation, String>;
    async fn receive_message(&self, tenant_id: Uuid, conversation_id: Uuid, content: String) -> Result<Message, String>;
    async fn get_feed(&self, tenant_id: Uuid) -> Result<Vec<Conversation>, String>;
}

pub struct MockChatEngine;

#[async_trait]
impl ChatEngine for MockChatEngine {
    async fn create_conversation(&self, tenant_id: Uuid, channel_id: Uuid) -> Result<Conversation, String> {
        Ok(Conversation {
            id: Uuid::new_v4(),
            tenant_id,
            channel_id,
            status: "open".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    async fn receive_message(&self, tenant_id: Uuid, conversation_id: Uuid, content: String) -> Result<Message, String> {
        Ok(Message {
            id: Uuid::new_v4(),
            tenant_id,
            conversation_id,
            sender_type: "customer".to_string(),
            content,
            created_at: Utc::now(),
        })
    }

    async fn get_feed(&self, _tenant_id: Uuid) -> Result<Vec<Conversation>, String> {
        Ok(vec![])
    }
}
