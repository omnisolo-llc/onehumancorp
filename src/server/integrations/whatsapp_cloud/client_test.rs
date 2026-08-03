#[cfg(test)]
mod tests {
    use crate::client::{RealWhatsAppCloudClient, build_message_payload};
    use serde_json::json;

    #[test]
    fn test_real_whatsapp_cloud_client_new() {
        let client = RealWhatsAppCloudClient::new("12345".to_string(), "token".to_string());
        let _ = client;
    }

    #[test]
    fn test_build_message_payload_phone_number() {
        let payload = build_message_payload("1234567890", "Hello");
        assert_eq!(payload, json!({
            "messaging_product": "whatsapp",
            "to": "1234567890",
            "type": "text",
            "text": {
                "body": "Hello"
            }
        }));
    }

    #[test]
    fn test_build_message_payload_bsuid() {
        let payload = build_message_payload("BR.1234567890", "Hello");
        assert_eq!(payload, json!({
            "messaging_product": "whatsapp",
            "recipient_type": "individual",
            "recipient": "BR.1234567890",
            "type": "text",
            "text": {
                "body": "Hello"
            }
        }));

        let payload_ent = build_message_payload("US.ENT.abcdef123456", "Hello ENT");
        assert_eq!(payload_ent, json!({
            "messaging_product": "whatsapp",
            "recipient_type": "individual",
            "recipient": "US.ENT.abcdef123456",
            "type": "text",
            "text": {
                "body": "Hello ENT"
            }
        }));
    }
}
