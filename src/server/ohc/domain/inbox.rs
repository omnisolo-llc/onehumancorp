use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Channel {
    WebWidget,
    WhatsApp,
    Email,
    Instagram,
    FacebookPage,
    Sms,
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
    fn test_inbox_creation() {
        let inbox = Inbox::new(
            "inbox_1".to_string(),
            "tenant_1".to_string(),
            "Support Inbox".to_string(),
            Channel::WebWidget,
        );

        assert_eq!(inbox.id, "inbox_1");
        assert_eq!(inbox.tenant_id, "tenant_1");
        assert_eq!(inbox.name, "Support Inbox");
        assert_eq!(inbox.channel, Channel::WebWidget);
    }
}
