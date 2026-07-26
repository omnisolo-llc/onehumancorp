use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ChannelType {
    WebWidget,
    WhatsApp,
    Email,
    Instagram,
    Sms,
    FacebookPage,
    Telegram,
    Line,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inbox {
    pub id: Uuid,
    pub tenant_id: String,
    pub name: String,
    pub channel_type: ChannelType,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Inbox {
    pub fn new(tenant_id: String, name: String, channel_type: ChannelType) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            tenant_id,
            name,
            channel_type,
            is_active: true,
            created_at: now,
            updated_at: now,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_inbox() {
        let tenant_id = "tenant_1".to_string();
        let name = "Support".to_string();
        let channel_type = ChannelType::WebWidget;

        let inbox = Inbox::new(tenant_id.clone(), name.clone(), channel_type.clone());

        assert_eq!(inbox.tenant_id, tenant_id);
        assert_eq!(inbox.name, name);
        assert_eq!(inbox.channel_type, channel_type);
        assert!(inbox.is_active);
    }
}
