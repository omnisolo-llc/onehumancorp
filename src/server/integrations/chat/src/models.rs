use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Inbox {
    pub id: i64,
    pub tenant_id: i64,
    pub channel_type: String,
    pub endpoint: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Contact {
    pub id: i64,
    pub tenant_id: i64,
    pub name: String,
    pub identifier: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Conversation {
    pub id: i64,
    pub tenant_id: i64,
    pub contact_id: i64,
    pub inbox_id: i64,
    pub status: ConversationStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ConversationStatus {
    Open,
    Snoozed,
    Resolved,
    AssignedToAgent,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Message {
    pub id: i64,
    pub tenant_id: i64,
    pub conversation_id: i64,
    pub content: String,
    pub message_type: MessageType,
    pub content_type: ContentType,
    pub status: MessageStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MessageType {
    Incoming,
    Outgoing,
    Template,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ContentType {
    Text,
    Image,
    Audio,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum MessageStatus {
    Sent,
    Delivered,
    Read,
    Unassigned,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_models_derive() {
        let status = ConversationStatus::Open;
        assert_eq!(status, ConversationStatus::Open);
    }
}
