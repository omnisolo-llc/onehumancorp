#[cfg(test)]
mod tests {
    use super::super::models::{ChatInbox, ChatChannel, ChatContact, ChatConversation, ChatMessage};
    use uuid::Uuid;
    use chrono::Utc;

    #[tokio::test]
    async fn test_chat_models_and_constructors() {
        let tenant_id = Uuid::new_v4();
        let inbox_id = Uuid::new_v4();
        let contact_id = Uuid::new_v4();
        let conversation_id = Uuid::new_v4();
        let message_id = Uuid::new_v4();

        // 1. Validate ChatInbox
        let inbox = ChatInbox {
            id: inbox_id,
            tenant_id,
            name: "Default Website Widget".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        assert_eq!(inbox.id, inbox_id);
        assert_eq!(inbox.tenant_id, tenant_id);
        assert_eq!(inbox.name, "Default Website Widget");

        // 2. Validate ChatChannel
        let config = serde_json::json!({
            "allowed_origins": ["https://baker-maya.com"],
            "enable_ai_auto_reply": true,
            "theme_color": "#FF0000"
        });
        let channel = ChatChannel {
            id: Uuid::new_v4(),
            tenant_id,
            inbox_id,
            channel_type: "widget".to_string(),
            config: config.clone(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        assert_eq!(channel.tenant_id, tenant_id);
        assert_eq!(channel.inbox_id, inbox_id);
        assert_eq!(channel.channel_type, "widget");
        assert_eq!(channel.config["allowed_origins"][0], "https://baker-maya.com");

        // 3. Validate ChatContact
        let contact = ChatContact {
            id: contact_id,
            tenant_id,
            name: Some("Maya Baker".to_string()),
            email: Some("maya@customcakes.com".to_string()),
            phone: Some("+15551234567".to_string()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        assert_eq!(contact.id, contact_id);
        assert_eq!(contact.name.as_deref(), Some("Maya Baker"));
        assert_eq!(contact.email.as_deref(), Some("maya@customcakes.com"));

        // 4. Validate ChatConversation
        let conversation = ChatConversation {
            id: conversation_id,
            tenant_id,
            inbox_id,
            contact_id,
            assignee_id: None,
            status: "open".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        assert_eq!(conversation.id, conversation_id);
        assert_eq!(conversation.status, "open");
        assert!(conversation.assignee_id.is_none());

        // 5. Validate ChatMessage
        let message = ChatMessage {
            id: message_id,
            tenant_id,
            conversation_id,
            sender_type: "contact".to_string(),
            sender_id: Some(contact_id),
            content: "Hello, I would like to order a chocolate birthday cake!".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        assert_eq!(message.id, message_id);
        assert_eq!(message.sender_type, "contact");
        assert_eq!(message.content, "Hello, I would like to order a chocolate birthday cake!");
    }

    #[tokio::test]
    async fn test_table_driven_conversations_and_messages() {
        let tenant_id = Uuid::new_v4();
        let inbox_id = Uuid::new_v4();
        let contact_id = Uuid::new_v4();

        // Let's create an array of test cases to validate different channel connections
        let test_cases = vec![
            ("widget", serde_json::json!({"allowed_origins": ["https://baker-maya.com"]})),
            ("whatsapp", serde_json::json!({"phone_number": "+15551234567"})),
            ("facebook", serde_json::json!({"page_id": "page_1234"})),
            ("instagram", serde_json::json!({"account_id": "insta_5678"})),
            ("twilio_sms", serde_json::json!({"from_number": "+15559876543"})),
            ("twilio_voice", serde_json::json!({"sip_trunk": "trunk_abc"})),
            ("email_resend", serde_json::json!({"domain": "customcakes.com"})),
            ("api_inbox", serde_json::json!({"webhook_callback_url": "https://api.com/v1"})),
        ];

        for (channel_type, config) in test_cases {
            let channel = ChatChannel {
                id: Uuid::new_v4(),
                tenant_id,
                inbox_id,
                channel_type: channel_type.to_string(),
                config: config.clone(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            };
            assert_eq!(channel.channel_type, channel_type);
            assert!(channel.config.is_object());
        }

        // Additional edge case combinations for statuses and priorities
        let statuses = vec!["open", "snoozed", "resolved", "bot"];
        for status in statuses {
            let conv = ChatConversation {
                id: Uuid::new_v4(),
                tenant_id,
                inbox_id,
                contact_id,
                assignee_id: None,
                status: status.to_string(),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            };
            assert_eq!(conv.status, status);
        }
    }
}
