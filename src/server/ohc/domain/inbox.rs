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
        let name = "Support".to_string();
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
    fn test_inbox_serialization() {
        let inbox = Inbox::new("tenant-123".to_string(), "Support".to_string(), Channel::WhatsApp);
        let serialized = serde_json::to_string(&inbox).unwrap();
        let deserialized: Inbox = serde_json::from_str(&serialized).unwrap();

        assert_eq!(inbox, deserialized);
    }

    #[test]
    fn test_channel_serialization() {
        let channels = vec![
            Channel::WebWidget,
            Channel::Email,
            Channel::WhatsApp,
            Channel::Instagram,
            Channel::FacebookPage,
            Channel::Sms,
        ];

        for channel in channels {
            let serialized = serde_json::to_string(&channel).unwrap();
            let deserialized: Channel = serde_json::from_str(&serialized).unwrap();
            assert_eq!(channel, deserialized);
        }
    }
}
