#[cfg(test)]
mod tests {
    use crate::client::{RealWhatsAppCloudClient, MessagePayload};
    use serde_json::json;

    #[test]
    fn test_real_whatsapp_cloud_client_new() {
        let client = RealWhatsAppCloudClient::new("12345".to_string(), "token".to_string());
        let _ = client;
    }

    #[test]
    fn test_message_payload_serialization() {
        let payload = MessagePayload {
            messaging_product: "whatsapp".to_string(),
            to: "1234567890".to_string(),
            msg_type: "text".to_string(),
            text: Some(json!({ "body": "Hello World" })),
            template: None,
            interactive: None,
            image: None,
            audio: None,
            video: None,
            document: None,
            location: None,
        };

        let json_str = serde_json::to_string(&payload).unwrap();
        assert!(json_str.contains(r#""messaging_product":"whatsapp""#));
        assert!(json_str.contains(r#""to":"1234567890""#));
        assert!(json_str.contains(r#""type":"text""#));
        assert!(json_str.contains(r#""text":{"body":"Hello World"}"#));

        // Ensure skipped Options are not serialized
        assert!(!json_str.contains("template"));
        assert!(!json_str.contains("interactive"));
    }
}
