#[cfg(test)]
mod tests {
    use crate::agent::AmbassadorAgent;
    use server_domain_inbox::models::{IncomingMessage, UnifiedCustomerGraph};

    #[test]
    fn test_process_message_with_context() {
        let message = IncomingMessage {
            tenant_id: "tenant-1".to_string(),
            source_channel: "instagram".to_string(),
            sender_id: "user123".to_string(),
            message_content: "Do you have vegan cakes?".to_string(),
            timestamp: 1678886400,
        };

        let context = UnifiedCustomerGraph {
            tenant_id: "tenant-1".to_string(),
            customer_id: "cust-1".to_string(),
            name: "Sarah".to_string(),
            phone_number: None,
            email: None,
            instagram_handle: Some("user123".to_string()),
            whatsapp_number: None,
            past_orders: vec!["Vegan Chocolate Cake".to_string()],
            tags: vec!["vegan".to_string()],
        };

        let draft = AmbassadorAgent::process_message(&message, Some(&context));

        assert_eq!(draft.tenant_id, "tenant-1");
        assert_eq!(draft.customer_id, "cust-1");
        assert_eq!(draft.status, "pending_approval");
        assert!(draft.drafted_reply.contains("Hi Sarah!"));
    }

    #[test]
    fn test_process_message_without_context() {
        let message = IncomingMessage {
            tenant_id: "tenant-1".to_string(),
            source_channel: "whatsapp".to_string(),
            sender_id: "9876543210".to_string(),
            message_content: "How much is a repair?".to_string(),
            timestamp: 1678886400,
        };

        let draft = AmbassadorAgent::process_message(&message, None);

        assert_eq!(draft.tenant_id, "tenant-1");
        assert_eq!(draft.customer_id, "unknown");
        assert_eq!(draft.status, "pending_approval");
        assert!(draft.drafted_reply.contains("Hi there!"));
    }
}
