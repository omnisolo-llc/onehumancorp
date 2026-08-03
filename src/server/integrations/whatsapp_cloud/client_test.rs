#[cfg(test)]
mod tests {
    use crate::client::RealWhatsAppCloudClient;

    #[test]
    fn test_real_whatsapp_cloud_client_new() {
        let client = RealWhatsAppCloudClient::new("12345".to_string(), "token".to_string());
        let _ = client;
    }

    #[test]
    fn test_generate_payload_phone_number() {
        let payload = RealWhatsAppCloudClient::generate_payload("1234567890", "Hello");
        assert_eq!(payload["messaging_product"], "whatsapp");
        assert_eq!(payload["to"], "1234567890");
        assert_eq!(payload["type"], "text");
        assert_eq!(payload["text"]["body"], "Hello");
        assert!(payload.get("recipient_type").is_none());
        assert!(payload.get("recipient").is_none());
    }

    #[test]
    fn test_generate_payload_bsuid() {
        let payload = RealWhatsAppCloudClient::generate_payload("BR.ENT.12345abc", "Hello BSUID");
        assert_eq!(payload["messaging_product"], "whatsapp");
        assert_eq!(payload["recipient_type"], "individual");
        assert_eq!(payload["recipient"], "BR.ENT.12345abc");
        assert_eq!(payload["type"], "text");
        assert_eq!(payload["text"]["body"], "Hello BSUID");
        assert!(payload.get("to").is_none());
    }

    #[test]
    fn test_generate_payload_bsuid_no_ent() {
        let payload = RealWhatsAppCloudClient::generate_payload("US.98765xyz", "Hello BSUID No ENT");
        assert_eq!(payload["messaging_product"], "whatsapp");
        assert_eq!(payload["recipient_type"], "individual");
        assert_eq!(payload["recipient"], "US.98765xyz");
        assert_eq!(payload["type"], "text");
        assert_eq!(payload["text"]["body"], "Hello BSUID No ENT");
        assert!(payload.get("to").is_none());
    }
}
