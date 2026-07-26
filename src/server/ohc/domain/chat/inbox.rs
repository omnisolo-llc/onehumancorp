use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ChannelType {
    WebWidget,
    Email,
    WhatsApp,
    Instagram,
    FacebookPage,
    Sms,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Inbox {
    pub id: String,
    pub tenant_id: String,
    pub name: String,
    pub channel_type: ChannelType,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Inbox {
    pub fn new(tenant_id: String, name: String, channel_type: ChannelType) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            tenant_id,
            name,
            channel_type,
            created_at: now,
            updated_at: now,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inbox_creation() {
        let tenant_id = "tenant-123".to_string();
        let name = "Support Team".to_string();
        let channel_type = ChannelType::WebWidget;

        let inbox = Inbox::new(tenant_id.clone(), name.clone(), channel_type.clone());

        assert!(!inbox.id.is_empty());
        assert_eq!(inbox.tenant_id, tenant_id);
        assert_eq!(inbox.name, name);
        assert_eq!(inbox.channel_type, channel_type);
        assert!(inbox.created_at <= Utc::now());
        assert_eq!(inbox.created_at, inbox.updated_at);
    }
}
