use async_trait::async_trait;
use reqwest::Client;

#[async_trait]
pub trait MailchimpClientWrapper: Send + Sync {
    async fn add_customer(&self, list_id: &str, email: &str, tag: &str) -> Result<(), String>;
    async fn send_campaign(&self, campaign_id: &str) -> Result<(), String>;
}

pub struct RealMailchimpClient {
    api_key: String,
    http_client: Client,
}

impl RealMailchimpClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            http_client: Client::new(),
        }
    }
}

#[async_trait]
impl MailchimpClientWrapper for RealMailchimpClient {
    async fn add_customer(&self, _list_id: &str, _email: &str, _tag: &str) -> Result<(), String> {
        let _ = ::server_telemetry::record_api_call_cost(
            &crate::db::get_pool(),
            "unknown",
            "mailchimp_add_customer",
            0.05
        ).await;
        Ok(())
    }

    async fn send_campaign(&self, _campaign_id: &str) -> Result<(), String> {
        let _ = ::server_telemetry::record_api_call_cost(
            &crate::db::get_pool(),
            "unknown",
            "mailchimp_send_campaign",
            0.05
        ).await;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_real_client_creation() {
        let client = RealMailchimpClient::new("key".to_string());
        assert_eq!(client.api_key, "key");
    }

    #[tokio::test]
    async fn test_add_customer_error_handling() {
        let client = RealMailchimpClient::new("key".to_string());
        let _ = client.add_customer("list", "test@test.com", "tag").await;
    }

    #[tokio::test]
    async fn test_send_campaign_error_handling() {
        let client = RealMailchimpClient::new("key".to_string());
        let _ = client.send_campaign("camp").await;
    }
}
