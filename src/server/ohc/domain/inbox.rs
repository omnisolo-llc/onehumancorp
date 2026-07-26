use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChannelType {
    WebWidget,
    Email,
    WhatsApp,
    Instagram,
    FacebookPage,
    Sms,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Channel {
    pub id: String,
    pub tenant_id: String,
    pub channel_type: ChannelType,
    pub name: String,
}

impl Channel {
    pub fn new(id: impl Into<String>, tenant_id: impl Into<String>, channel_type: ChannelType, name: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            tenant_id: tenant_id.into(),
            channel_type,
            name: name.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Inbox {
    pub id: String,
    pub tenant_id: String,
    pub name: String,
    pub channels: Vec<Channel>,
}

impl Inbox {
    pub fn new(id: impl Into<String>, tenant_id: impl Into<String>, name: impl Into<String>, channels: Vec<Channel>) -> Self {
        Self {
            id: id.into(),
            tenant_id: tenant_id.into(),
            name: name.into(),
            channels,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_channel_creation() {
        let channel = Channel::new("ch1", "t1", ChannelType::WebWidget, "Support Widget");
        assert_eq!(channel.id, "ch1");
        assert_eq!(channel.tenant_id, "t1");
        assert_eq!(channel.channel_type, ChannelType::WebWidget);
        assert_eq!(channel.name, "Support Widget");
    }

    #[test]
    fn test_channel_serialization() {
        let channel = Channel::new("ch1", "t1", ChannelType::WhatsApp, "WA Support");
        let serialized = serde_json::to_string(&channel).unwrap();
        let deserialized: Channel = serde_json::from_str(&serialized).unwrap();
        assert_eq!(channel, deserialized);
    }

    #[test]
    fn test_inbox_creation() {
        let channel = Channel::new("ch1", "t1", ChannelType::WebWidget, "Widget");
        let inbox = Inbox::new("inbox1", "t1", "Main Inbox", vec![channel.clone()]);
        assert_eq!(inbox.id, "inbox1");
        assert_eq!(inbox.tenant_id, "t1");
        assert_eq!(inbox.name, "Main Inbox");
        assert_eq!(inbox.channels.len(), 1);
        assert_eq!(inbox.channels[0], channel);
    }

    #[test]
    fn test_inbox_serialization() {
        let channel = Channel::new("ch1", "t1", ChannelType::Email, "Support Email");
        let inbox = Inbox::new("inbox1", "t1", "Main Inbox", vec![channel]);
        let serialized = serde_json::to_string(&inbox).unwrap();
        let deserialized: Inbox = serde_json::from_str(&serialized).unwrap();
        assert_eq!(inbox, deserialized);
    }
}
