#[cfg(test)]
mod tests {
    use crate::client::{RealWhatsAppCloudClient, Template, TemplateLanguage, Interactive, InteractiveBody, InteractiveAction, Media, WhatsAppCloudClientWrapper};
    use crate::provider::WhatsAppCloudProvider;

    #[test]
    fn test_real_whatsapp_cloud_client_new() {
        let client = RealWhatsAppCloudClient::new("12345".to_string(), "token".to_string());
        let _ = client;
    }

    #[tokio::test]
    async fn test_provider_setup_webhook_error() {
        let provider = WhatsAppCloudProvider::new("phone_id".to_string(), "test_token".to_string());
        let result = provider.setup_webhook("invalid_id", "invalid_secret", "https://example.com/webhook", "verify").await;
        // This will fail because we are providing invalid credentials to Meta API
        assert!(result.is_err());
    }

    #[test]
    fn test_payload_structures_compile() {
        let _template = Template {
            name: "hello_world".to_string(),
            language: TemplateLanguage { code: "en_US".to_string() },
            components: None,
        };

        let _interactive = Interactive {
            interactive_type: "button".to_string(),
            header: None,
            body: InteractiveBody { text: "Hello".to_string() },
            footer: None,
            action: InteractiveAction {
                button: None,
                buttons: None,
                sections: None,
            },
        };

        let _media = Media {
            id: None,
            link: Some("https://example.com/image.png".to_string()),
            caption: None,
            filename: None,
        };
    }
}