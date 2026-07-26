use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Channel {
    WebWidget,
    WhatsApp,
    Email,
    Instagram,
    FacebookPage,
    Sms,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Inbox {
    pub id: String,
    pub tenant_id: String,
    pub name: String,
    pub channel: Channel,
}

impl Inbox {
    pub fn new(id: String, tenant_id: String, name: String, channel: Channel) -> Self {
        Self {
            id,
            tenant_id,
            name,
            channel,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inbox_new() {
        let inbox = Inbox::new(
            "inbox_1".to_string(),
            "tenant_1".to_string(),
            "Main Support".to_string(),
            Channel::WebWidget,
        );

        assert_eq!(inbox.id, "inbox_1");
        assert_eq!(inbox.tenant_id, "tenant_1");
        assert_eq!(inbox.name, "Main Support");
        assert_eq!(inbox.channel, Channel::WebWidget);
    }

    #[test]
    fn test_inbox_serialization() {
        let inbox = Inbox::new(
            "inbox_1".to_string(),
            "tenant_1".to_string(),
            "Instagram DMs".to_string(),
            Channel::Instagram,
        );

        let serialized = serde_json::to_string(&inbox).unwrap();
        let deserialized: Inbox = serde_json::from_str(&serialized).unwrap();

        assert_eq!(inbox, deserialized);
    }

    #[test]
    fn test_channel_other_serialization() {
        let inbox = Inbox::new(
            "inbox_1".to_string(),
            "tenant_1".to_string(),
            "Custom Channel".to_string(),
            Channel::Other("custom_api".to_string()),
        );

        let serialized = serde_json::to_string(&inbox).unwrap();
        let deserialized: Inbox = serde_json::from_str(&serialized).unwrap();

        assert_eq!(inbox, deserialized);
    }
}
