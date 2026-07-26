use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Channel {
    WebWidget,
    Email,
    WhatsApp,
    Instagram,
    FacebookPage,
    Sms,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Inbox {
    pub id: Uuid,
    pub tenant_id: String,
    pub name: String,
    pub channel: Channel,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Inbox {
    pub fn new(tenant_id: String, name: String, channel: Channel) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            tenant_id,
            name,
            channel,
            created_at: now,
            updated_at: now,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inbox_new() {
        let tenant_id = "tenant-123".to_string();
        let name = "Support Inbox".to_string();
        let channel = Channel::WebWidget;

        let inbox = Inbox::new(tenant_id.clone(), name.clone(), channel.clone());

        assert_eq!(inbox.tenant_id, tenant_id);
        assert_eq!(inbox.name, name);
        assert_eq!(inbox.channel, channel);
        assert!(!inbox.id.is_nil());
        assert!(inbox.created_at <= Utc::now());
        assert_eq!(inbox.created_at, inbox.updated_at);
    }

    #[test]
    fn test_channel_serialization() {
        let channel = Channel::WhatsApp;
        let json = serde_json::to_string(&channel).unwrap();
        assert_eq!(json, "\"WhatsApp\"");

        let deserialized: Channel = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, channel);
    }
}
