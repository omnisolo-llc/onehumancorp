#[cfg(test)]
mod tests {
    use crate::client::RealWhatsAppCloudClient;

    #[test]
    fn test_real_whatsapp_cloud_client_new() {
        let client = RealWhatsAppCloudClient::new("12345".to_string(), "token".to_string());
        let _ = client;
    }

    use crate::client::{InteractiveMessage, InteractiveBody, InteractiveAction, InteractiveButton, InteractiveButtonReply, LocationPayload};

    #[test]
    fn test_interactive_message_serialization() {
        let msg = InteractiveMessage {
            interactive_type: "button".to_string(),
            body: InteractiveBody {
                text: "test".to_string(),
            },
            action: InteractiveAction {
                buttons: vec![
                    InteractiveButton {
                        button_type: "reply".to_string(),
                        reply: InteractiveButtonReply {
                            id: "1".to_string(),
                            title: "Yes".to_string(),
                        },
                    }
                ]
            }
        };

        let serialized = serde_json::to_string(&msg).unwrap();
        assert!(serialized.contains("button"));
        assert!(serialized.contains("Yes"));
    }

    #[test]
    fn test_location_payload_serialization() {
        let loc = LocationPayload {
            latitude: 12.34,
            longitude: 56.78,
            name: Some("Store".to_string()),
            address: None,
        };

        let serialized = serde_json::to_string(&loc).unwrap();
        assert!(serialized.contains("12.34"));
        assert!(serialized.contains("Store"));
    }
}
