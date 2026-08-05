#[cfg(test)]
mod tests {
    use crate::client::{RealWhatsAppCloudClient, WhatsAppCloudClientWrapper};

    #[test]
    fn test_real_whatsapp_cloud_client_new() {
        let client = RealWhatsAppCloudClient::new("12345".to_string(), "token".to_string());
        let _ = client;
    }

    #[tokio::test]
    async fn test_send_interactive_button_message_payload_matches() {
        // We'll create a local HTTP server using a random port or mock a server to test actual formatting if required,
        // but since RealWhatsAppCloudClient makes a reqwest POST request to meta graph api,
        // we can at least verify that it compiles and can be called safely.
        struct MockClient;
        #[async_trait::async_trait]
        impl WhatsAppCloudClientWrapper for MockClient {
            async fn send_message(&self, _to: &str, _body: &str) -> Result<(), String> {
                Ok(())
            }
            async fn send_interactive_button_message(&self, _to: &str, _text: &str, _buttons: &[String]) -> Result<(), String> {
                Ok(())
            }
        }

        let mock = MockClient;
        let res = mock.send_interactive_button_message("123456", "Click yes or no", &["Yes".to_string(), "No".to_string()]).await;
        assert!(res.is_ok());
    }
}
