use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeliveryQuote {
    pub fee: f64,
    pub dropoff_eta: String,
    pub pickup_eta: String,
}

pub struct DoorDashClient {
    pub api_key: String,
}

impl DoorDashClient {
    pub fn new(api_key: String) -> Self {
        DoorDashClient { api_key }
    }

    pub async fn get_delivery_quote(&self, _pickup_address: &str, _dropoff_address: &str) -> Result<DeliveryQuote, String> {
        // Mock implementation for getting a delivery quote
        Ok(DeliveryQuote {
            fee: 8.50,
            dropoff_eta: "2024-05-20T12:30:00Z".to_string(),
            pickup_eta: "2024-05-20T12:00:00Z".to_string(),
        })
    }

    pub async fn dispatch_delivery(&self, _pickup_address: &str, _dropoff_address: &str, _order_id: &str) -> Result<String, String> {
        // Mock implementation for dispatching delivery
        Ok("mock_delivery_id_123".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_delivery_quote() {
        let client = DoorDashClient::new("dummy_key".to_string());
        let quote = client.get_delivery_quote("123 Pickup St", "456 Dropoff Ave").await.unwrap();
        assert_eq!(quote.fee, 8.50);
    }

    #[tokio::test]
    async fn test_dispatch_delivery() {
        let client = DoorDashClient::new("dummy_key".to_string());
        let delivery_id = client.dispatch_delivery("123 Pickup St", "456 Dropoff Ave", "ord_123").await.unwrap();
        assert_eq!(delivery_id, "mock_delivery_id_123");
    }
}
