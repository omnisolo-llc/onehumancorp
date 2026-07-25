#[cfg(test)]
mod tests {
    use crate::client::{RealWhatsAppCloudClient, WhatsAppCloudClientWrapper};
    use serde_json::json;

    #[test]
    fn test_real_whatsapp_cloud_client_new() {
        let client = RealWhatsAppCloudClient::new("12345".to_string(), "token".to_string());
        let _ = client;
    }

    #[tokio::test]
    async fn test_send_interactive_message_does_not_panic() {
        let client = RealWhatsAppCloudClient::new("12345".to_string(), "token".to_string());

        let payload = json!({
            "type": "button",
            "body": {
                "text": "Hello"
            }
        });

        // This will fail because the token is fake, but we want to make sure it runs and builds the request
        let _res = client.send_interactive_message("1234567890", payload).await;
    }

    #[tokio::test]
    async fn test_send_template_message_does_not_panic() {
        let client = RealWhatsAppCloudClient::new("12345".to_string(), "token".to_string());

        let components = vec![json!({"type": "body", "parameters": []})];

        let _res = client.send_template_message("1234567890", "hello_world", "en_US", components).await;
    }

    #[tokio::test]
    async fn test_send_media_message_does_not_panic() {
        let client = RealWhatsAppCloudClient::new("12345".to_string(), "token".to_string());

        let _res = client.send_media_message("1234567890", "image", "http://example.com/image.png", Some("Caption")).await;
    }
}
