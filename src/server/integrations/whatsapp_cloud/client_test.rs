#[cfg(test)]
mod tests {
    use crate::client::{build_message_payload, RealWhatsAppCloudClient};

    #[test]
    fn test_real_whatsapp_cloud_client_new() {
        let client = RealWhatsAppCloudClient::new("12345".to_string(), "token".to_string());
        let _ = client;
    }

    #[test]
    fn test_build_message_payload_phone_number() {
        let payload = build_message_payload("1234567890", "Hello world");
        assert_eq!(payload["messaging_product"], "whatsapp");
        assert_eq!(payload["to"], "1234567890");
        assert_eq!(payload["type"], "text");
        assert_eq!(payload["text"]["body"], "Hello world");
        assert!(payload.get("recipient_type").is_none());
        assert!(payload.get("recipient").is_none());
    }

    #[test]
    fn test_build_message_payload_bsuid() {
        let payload = build_message_payload("BR.123456", "Hello BSUID");
        assert_eq!(payload["messaging_product"], "whatsapp");
        assert_eq!(payload["recipient_type"], "individual");
        assert_eq!(payload["recipient"], "BR.123456");
        assert_eq!(payload["type"], "text");
        assert_eq!(payload["text"]["body"], "Hello BSUID");
        assert!(payload.get("to").is_none());
    }

    #[test]
    fn test_build_message_payload_bsuid_ent() {
        let payload = build_message_payload("BR.ENT.123456", "Hello ENT BSUID");
        assert_eq!(payload["messaging_product"], "whatsapp");
        assert_eq!(payload["recipient_type"], "individual");
        assert_eq!(payload["recipient"], "BR.ENT.123456");
        assert_eq!(payload["type"], "text");
        assert_eq!(payload["text"]["body"], "Hello ENT BSUID");
        assert!(payload.get("to").is_none());
    }
}
