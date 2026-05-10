use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShippoAddress {
    pub name: String,
    pub street1: String,
    pub city: String,
    pub state: String,
    pub zip: String,
    pub country: String,
    pub phone: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShippoParcel {
    pub length: String,
    pub width: String,
    pub height: String,
    pub distance_unit: String,
    pub weight: String,
    pub mass_unit: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShippoShipmentRequest {
    pub address_from: ShippoAddress,
    pub address_to: ShippoAddress,
    pub parcels: Vec<ShippoParcel>,
    pub async_: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShippoRate {
    pub object_id: String,
    pub amount: String,
    pub currency: String,
    pub provider: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShippoShipmentResponse {
    pub object_id: String,
    pub rates: Vec<ShippoRate>,
}

pub struct ShippoClient {
    pub api_token: String,
    pub http_client: Client,
}

impl ShippoClient {
    pub fn new(api_token: String) -> Self {
        Self {
            api_token,
            http_client: Client::new(),
        }
    }

    pub async fn create_shipment(&self, request: &ShippoShipmentRequest) -> Result<ShippoShipmentResponse, String> {
        let url = "https://api.goshippo.com/shipments/";
        let res = self.http_client.post(url)
            .header("Authorization", format!("ShippoToken {}", self.api_token))
            .json(request)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let shipment: ShippoShipmentResponse = resp.json().await.map_err(|e| e.to_string())?;
                    Ok(shipment)
                } else {
                    Err(format!("Shippo API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shippo_client_creation() {
        let client = ShippoClient::new("test_token".to_string());
        assert_eq!(client.api_token, "test_token");
    }

    #[tokio::test]
    async fn test_create_shipment_compiles() {
        let client = ShippoClient::new("test_token".to_string());
        assert_eq!(client.api_token, "test_token");
    }
}
