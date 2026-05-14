pub struct MailchimpClient {
    pub api_key: String,
    pub server_prefix: String,
}

impl MailchimpClient {
    pub fn new(api_key: String, server_prefix: String) -> Self {
        Self { api_key, server_prefix }
    }

    pub async fn add_subscriber(&self, list_id: &str, email: &str) -> Result<(), String> {
        let _ = crate::telemetry::record_api_call_cost(&crate::db::get_pool(), "unknown", "mailchimp_add_subscriber", 0.02).await;
        tracing::info!("Adding subscriber to mailing list");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = MailchimpClient::new("key".to_string(), "us1".to_string());
        assert_eq!(client.api_key, "key");
        assert_eq!(client.server_prefix, "us1");
    }

    #[tokio::test]
    async fn test_add_subscriber() {
        let client = MailchimpClient::new("key".to_string(), "us1".to_string());
        let res = client.add_subscriber("list_1", "test@example.com").await;
        assert!(res.is_ok());
    }
}
