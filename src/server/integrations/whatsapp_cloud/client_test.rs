#[cfg(test)]
mod tests {
    use crate::client::{RealWhatsAppCloudClient, WhatsAppCloudClientWrapper};

    #[test]
    fn test_real_whatsapp_cloud_client_new() {
        let client = RealWhatsAppCloudClient::new("12345".to_string(), "token".to_string());
        let _ = client;
    }

    #[tokio::test]
    async fn test_send_media_missing_id_and_link() {
        let client = RealWhatsAppCloudClient::new("12345".to_string(), "token".to_string());
        let res = client.send_media("123", "image", None, None).await;
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Either media_id or media_link must be provided");
    }
}
