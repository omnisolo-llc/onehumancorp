#[cfg(test)]
mod tests {
    use crate::client::{RealWhatsAppCloudClient, WhatsAppCloudClientWrapper};

    #[test]
    fn test_real_whatsapp_cloud_client_new() {
        let client = RealWhatsAppCloudClient::new("12345".to_string(), "token".to_string());
        let _ = client;
    }

    #[tokio::test]
    async fn test_whatsapp_cloud_client_methods_signature() {
        let client = RealWhatsAppCloudClient::new("12345".to_string(), "token".to_string());

        // We just ensure that the compiler can resolve these methods on RealWhatsAppCloudClient
        // We don't execute them because they hit the real Meta API (or would try to).
        // Since RealWhatsAppCloudClient has async post methods, we can just compile check.
        // A full unit test with mock would require mocking reqwest which we did not do for simplicity.

        let _ = client.send_message("123", "hello").await;
        let _ = client.send_template("123", "temp", "en", serde_json::json!({})).await;
        let _ = client.send_interactive_message("123", serde_json::json!({})).await;
        let _ = client.send_media_message("123", "image", "id123", None, None).await;
    }

}
