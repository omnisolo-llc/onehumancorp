use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use sqlx::FromRow;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Inbox {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Channel {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub inbox_id: Uuid,
    #[sqlx(rename = "type")]
    pub channel_type: String, // 'type' is a reserved keyword in Rust
    pub config: sqlx::types::Json<HashMap<String, String>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Contact {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Conversation {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub inbox_id: Uuid,
    pub contact_id: Uuid,
    pub channel_id: Uuid,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Message {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub conversation_id: Uuid,
    pub content: String,
    pub message_type: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub trait ChannelAdapter {
    fn send_message(&self, message: &Message) -> Result<(), Box<dyn std::error::Error>>;
    fn receive_webhook(&self, payload: &str) -> Result<Message, Box<dyn std::error::Error>>;
    fn get_metadata(&self) -> HashMap<String, String>;
}

pub struct WebWidgetAdapter {
    pub config: HashMap<String, String>,
}

impl ChannelAdapter for WebWidgetAdapter {
    fn send_message(&self, _message: &Message) -> Result<(), Box<dyn std::error::Error>> {
        // Implementation for WebWidget
        Ok(())
    }

    fn receive_webhook(&self, _payload: &str) -> Result<Message, Box<dyn std::error::Error>> {
        // Implementation for WebWidget
        unimplemented!()
    }

    fn get_metadata(&self) -> HashMap<String, String> {
        self.config.clone()
    }
}
