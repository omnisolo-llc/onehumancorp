#[cfg(test)]
mod tests {
    use crate::chat::whatsapp_provider::WhatsAppCloudService;
    use serde_json::json;

    fn get_service() -> WhatsAppCloudService {
        WhatsAppCloudService::new(
            "phone_id".to_string(),
            "business_id".to_string(),
            "token".to_string(),
        )
    }

    #[test]
    fn test_recipient_params_phone() {
        let service = get_service();
        let payload = service.send_text_message("+1234567890", "Hello");
        assert_eq!(payload["to"], "+1234567890");
        assert!(payload.get("recipient_type").is_none());
    }

    #[test]
    fn test_recipient_params_bsuid() {
        let service = get_service();
        let payload = service.send_text_message("BR.ENT.1234567890", "Hello");
        assert_eq!(payload["recipient"], "BR.ENT.1234567890");
        assert_eq!(payload["recipient_type"], "individual");
        assert!(payload.get("to").is_none());
    }

    #[test]
    fn test_send_attachment() {
        let service = get_service();
        let payload = service.send_attachment_message(
            "+123",
            "document",
            "https://example.com/file.pdf",
            Some("Here is your file"),
            Some("file.pdf"),
        );
        assert_eq!(payload["messaging_product"], "whatsapp");
        assert_eq!(payload["type"], "document");
        assert_eq!(payload["to"], "+123");
        assert_eq!(payload["document"]["link"], "https://example.com/file.pdf");
        assert_eq!(payload["document"]["caption"], "Here is your file");
        assert_eq!(payload["document"]["filename"], "file.pdf");
    }

    #[test]
    fn test_send_template() {
        let service = get_service();
        let components = json!([{"type": "body", "parameters": [{"type": "text", "text": "param1"}]}]);
        let payload = service.send_template("+123", "hello_world", "en_US", components.clone());

        assert_eq!(payload["messaging_product"], "whatsapp");
        assert_eq!(payload["type"], "template");
        assert_eq!(payload["to"], "+123");
        assert_eq!(payload["recipient_type"], "individual");
        assert_eq!(payload["template"]["name"], "hello_world");
        assert_eq!(payload["template"]["language"]["code"], "en_US");
        assert_eq!(payload["template"]["components"], components);
    }
}
