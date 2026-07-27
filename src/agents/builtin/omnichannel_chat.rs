use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Conversation {
    pub id: i32,
    pub account_id: i32,
    pub inbox_id: i32,
    pub contact_id: Option<i64>,
    pub status: i32,
    pub assignee_id: Option<i32>,
    pub uuid: String,
    pub priority: Option<i32>,
    pub team_id: Option<i64>,
    pub campaign_id: Option<i64>,
}

impl Default for Conversation {
    fn default() -> Self {
        Self {
            id: 0,
            account_id: 0,
            inbox_id: 0,
            contact_id: None,
            status: 0, // 0 = open
            assignee_id: None,
            uuid: String::new(),
            priority: None,
            team_id: None,
            campaign_id: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Message {
    pub id: i32,
    pub account_id: i32,
    pub conversation_id: i32,
    pub inbox_id: i32,
    pub content: String,
    pub message_type: i32,
    pub private: bool,
    pub sender_type: Option<String>,
    pub sender_id: Option<i64>,
}

impl Default for Message {
    fn default() -> Self {
        Self {
            id: 0,
            account_id: 0,
            conversation_id: 0,
            inbox_id: 0,
            content: String::new(),
            message_type: 0,
            private: false,
            sender_type: None,
            sender_id: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Inbox {
    pub id: i32,
    pub account_id: i32,
    pub channel_type: String,
    pub name: String,
}

impl Default for Inbox {
    fn default() -> Self {
        Self {
            id: 0,
            account_id: 0,
            channel_type: String::new(),
            name: String::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_conversation() {
        let conv = Conversation {
            id: 1,
            account_id: 100,
            inbox_id: 200,
            contact_id: Some(300),
            status: 0,
            uuid: "123e4567-e89b-12d3-a456-426614174000".to_string(),
            ..Default::default()
        };

        assert_eq!(conv.id, 1);
        assert_eq!(conv.status, 0);
        assert_eq!(conv.contact_id, Some(300));

        let json = serde_json::to_string(&conv).unwrap();
        assert!(json.contains("account_id\":100"));
    }

    #[test]
    fn test_add_message() {
        let msg = Message {
            id: 1,
            account_id: 100,
            conversation_id: 1,
            inbox_id: 200,
            content: "Hello!".to_string(),
            message_type: 1,
            private: false,
            sender_type: Some("User".to_string()),
            sender_id: Some(300),
        };

        assert_eq!(msg.content, "Hello!");
        assert_eq!(msg.message_type, 1);

        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("content\":\"Hello!\""));
    }
}
