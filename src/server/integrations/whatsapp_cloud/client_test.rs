#[cfg(test)]
mod tests {
    use crate::client::RealWhatsAppCloudClient;

    #[test]
    fn test_real_whatsapp_cloud_client_new() {
        let client = RealWhatsAppCloudClient::new("12345".to_string(), "token".to_string());
        let _ = client;
    }
}
