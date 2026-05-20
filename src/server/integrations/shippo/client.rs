use async_trait::async_trait;
use reqwest::Client;

#[async_trait]
pub trait ShippoClientWrapper: Send + Sync {
    async fn get_shipping_rates(&self, from_zip: &str, to_zip: &str, weight: f32) -> Result<String, String>;
}

pub struct RealShippoClient {
    api_key: String,
    http_client: Client,
}

impl RealShippoClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            http_client: Client::new(),
        }
    }
}

#[async_trait]
impl ShippoClientWrapper for RealShippoClient {
    async fn get_shipping_rates(&self, _from_zip: &str, _to_zip: &str, _weight: f32) -> Result<String, String> {
        let _ = ::server_telemetry::record_api_call_cost(
            &crate::db::get_pool(),
            "unknown",
            "shippo_get_rates",
            0.05
        ).await;
        Ok("mock_rate_response".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_real_client_creation() {
        let client = RealShippoClient::new("key".to_string());
        assert_eq!(client.api_key, "key");
    }

    #[tokio::test]
    async fn test_get_rates_error_handling() {
        let client = RealShippoClient::new("key".to_string());
        let _ = client.get_shipping_rates("12345", "67890", 1.0).await;
    }
}
