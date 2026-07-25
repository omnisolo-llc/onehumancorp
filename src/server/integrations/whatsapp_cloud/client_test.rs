#[cfg(test)]
mod tests {
    use crate::client::{RealWhatsAppCloudClient, WhatsAppCloudClientWrapper};

    #[test]
    fn test_real_whatsapp_cloud_client_new() {
        let client = RealWhatsAppCloudClient::new("12345".to_string(), "token".to_string());
        let _ = client;
    }

    #[tokio::test]
    async fn test_client_methods_signature_check() {
        let client = RealWhatsAppCloudClient::new("phone".to_string(), "token".to_string());

        // Assert trait is implemented and methods are accessible
        let _c: &dyn WhatsAppCloudClientWrapper = &client;
    }
}
