use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PackageDimensions {
    pub length: f64,
    pub width: f64,
    pub height: f64,
    pub weight_oz: f64,
}

#[async_trait]
pub trait EasyPostClientWrapper: Send + Sync {
    async fn calculate_shipping_rates(&self, destination_zip: &str, dimensions: &PackageDimensions) -> Result<f64, String>;
    async fn create_shipping_label(&self, order_id: &str, dimensions: &PackageDimensions) -> Result<String, String>;
}

pub struct RealEasyPostClient {
    api_key: String,
}

impl RealEasyPostClient {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}

#[async_trait]
impl EasyPostClientWrapper for RealEasyPostClient {
    async fn calculate_shipping_rates(&self, _destination_zip: &str, dimensions: &PackageDimensions) -> Result<f64, String> {
        // Mocking the EasyPost rate calculation logic
        let base_rate = 5.0;
        let weight_rate = dimensions.weight_oz * 0.1;
        let volume = dimensions.length * dimensions.width * dimensions.height;
        let volume_rate = volume * 0.01;
        Ok(base_rate + weight_rate + volume_rate)
    }

    async fn create_shipping_label(&self, order_id: &str, _dimensions: &PackageDimensions) -> Result<String, String> {
        // Mocking EasyPost shipping label creation
        Ok(format!("https://easypost.com/labels/{}.pdf", order_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_real_client_creation() {
        let client = RealEasyPostClient::new("key".to_string());
        assert_eq!(client.api_key, "key");
    }

    #[tokio::test]
    async fn test_calculate_shipping_rates() {
        let client = RealEasyPostClient::new("key".to_string());
        let dimensions = PackageDimensions {
            length: 10.0,
            width: 10.0,
            height: 10.0,
            weight_oz: 16.0,
        };
        let rate = client.calculate_shipping_rates("90210", &dimensions).await.unwrap();
        assert_eq!(rate, 5.0 + 1.6 + 10.0);
    }

    #[tokio::test]
    async fn test_create_shipping_label() {
        let client = RealEasyPostClient::new("key".to_string());
        let dimensions = PackageDimensions {
            length: 10.0,
            width: 10.0,
            height: 10.0,
            weight_oz: 16.0,
        };
        let label_url = client.create_shipping_label("order_123", &dimensions).await.unwrap();
        assert_eq!(label_url, "https://easypost.com/labels/order_123.pdf");
    }
}
