use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Inbox {
    pub id: String,
    pub tenant_id: String,
    pub name: String,
    pub description: Option<String>,
}

impl Inbox {
    pub fn new(id: String, tenant_id: String, name: String, description: Option<String>) -> Self {
        Self {
            id,
            tenant_id,
            name,
            description,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ChannelType {
    WebWidget,
    Email,
    WhatsApp,
    Instagram,
    FacebookPage,
    Sms,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Channel {
    pub id: String,
    pub tenant_id: String,
    pub inbox_id: String,
    pub channel_type: ChannelType,
    pub name: String,
    pub is_active: bool,
}

impl Channel {
    pub fn new(
        id: String,
        tenant_id: String,
        inbox_id: String,
        channel_type: ChannelType,
        name: String,
    ) -> Self {
        Self {
            id,
            tenant_id,
            inbox_id,
            channel_type,
            name,
            is_active: true,
        }
    }
}

#[async_trait::async_trait]
pub trait InboxService: Send + Sync {
    async fn create_inbox(
        &self,
        tenant_id: &str,
        name: String,
        description: Option<String>,
    ) -> Result<Inbox, String>;

    async fn get_inbox(&self, tenant_id: &str, id: &str) -> Result<Option<Inbox>, String>;

    async fn list_inboxes(&self, tenant_id: &str) -> Result<Vec<Inbox>, String>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inbox_creation_and_serde() {
        let inbox = Inbox::new(
            "inbox-1".to_string(),
            "tenant-abc".to_string(),
            "Customer Support".to_string(),
            Some("Main support inbox".to_string()),
        );

        assert_eq!(inbox.id, "inbox-1");
        assert_eq!(inbox.tenant_id, "tenant-abc");
        assert_eq!(inbox.name, "Customer Support");
        assert_eq!(inbox.description, Some("Main support inbox".to_string()));

        let serialized = serde_json::to_string(&inbox).unwrap();
        let deserialized: Inbox = serde_json::from_str(&serialized).unwrap();
        assert_eq!(inbox, deserialized);
    }

    #[test]
    fn test_channel_creation_and_serde() {
        let types = vec![
            ChannelType::WebWidget,
            ChannelType::Email,
            ChannelType::WhatsApp,
            ChannelType::Instagram,
            ChannelType::FacebookPage,
            ChannelType::Sms,
        ];

        for (i, &chan_type) in types.iter().enumerate() {
            let channel = Channel::new(
                format!("chan-{}", i),
                "tenant-abc".to_string(),
                "inbox-1".to_string(),
                chan_type,
                format!("Channel {:?}", chan_type),
            );

            assert_eq!(channel.id, format!("chan-{}", i));
            assert_eq!(channel.tenant_id, "tenant-abc");
            assert_eq!(channel.inbox_id, "inbox-1");
            assert_eq!(channel.channel_type, chan_type);
            assert_eq!(channel.name, format!("Channel {:?}", chan_type));
            assert!(channel.is_active);

            let serialized = serde_json::to_string(&channel).unwrap();
            let deserialized: Channel = serde_json::from_str(&serialized).unwrap();
            assert_eq!(channel, deserialized);
        }
    }
}
