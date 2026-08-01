pub struct Conversation {
    pub id: String,
    pub tenant_id: String,
}

pub struct Message {
    pub id: String,
    pub conversation_id: String,
    pub body: String,
}

pub struct Inbox {
    pub id: String,
    pub tenant_id: String,
}

pub trait ChannelAdapter {
    fn normalize_payload(&self, payload: &str) -> Message;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockAdapter;
    impl ChannelAdapter for MockAdapter {
        fn normalize_payload(&self, payload: &str) -> Message {
            Message {
                id: "msg_mock".into(),
                conversation_id: "conv_mock".into(),
                body: payload.into(),
            }
        }
    }

    #[test]
    fn test_conversation_creation() {
        let conv = Conversation {
            id: "conv_1".into(),
            tenant_id: "tenant_1".into(),
        };
        assert_eq!(conv.id, "conv_1");
        assert_eq!(conv.tenant_id, "tenant_1");
    }

    #[test]
    fn test_inbox_creation() {
        let inbox = Inbox {
            id: "inbox_1".into(),
            tenant_id: "tenant_1".into(),
        };
        assert_eq!(inbox.id, "inbox_1");
        assert_eq!(inbox.tenant_id, "tenant_1");
    }

    #[test]
    fn test_message_creation() {
        let msg = Message {
            id: "msg_1".into(),
            conversation_id: "conv_1".into(),
            body: "hello world".into(),
        };
        assert_eq!(msg.id, "msg_1");
        assert_eq!(msg.conversation_id, "conv_1");
        assert_eq!(msg.body, "hello world");
    }

    #[test]
    fn test_channel_adapter() {
        let adapter = MockAdapter;
        let msg = adapter.normalize_payload("test payload");
        assert_eq!(msg.body, "test payload");
    }
}
