#[cfg(test)]
mod tests {
    use super::super::client::RealWhatsAppCloudClient;

    #[test]
    fn test_real_whatsapp_cloud_client_new() {
        let client = RealWhatsAppCloudClient::new("12345".to_string(), "token".to_string());
        let _ = client;
    }
}

#[cfg(test)]
mod payload_tests {
    use super::super::client::build_whatsapp_payload;

    #[test]
    fn test_standard_phone_payload() {
        let payload = build_whatsapp_payload("1234567890", "hello");
        assert_eq!(payload["to"], "1234567890");
        assert!(payload.get("recipient").is_none());
        assert!(payload.get("recipient_type").is_none());
    }

    #[test]
    fn test_bsuid_payload() {
        let payload = build_whatsapp_payload("BR.ENT.123456789", "hello");
        assert_eq!(payload["recipient"], "BR.ENT.123456789");
        assert_eq!(payload["recipient_type"], "individual");
        assert!(payload.get("to").is_none());

        let payload2 = build_whatsapp_payload("US.987654321", "hello");
        assert_eq!(payload2["recipient"], "US.987654321");
        assert_eq!(payload2["recipient_type"], "individual");
        assert!(payload2.get("to").is_none());
    }
}
