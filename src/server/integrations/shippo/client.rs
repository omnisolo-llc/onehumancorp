use serde::{Deserialize, Serialize};
use reqwest::Client;
use async_trait::async_trait;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShippingRate {
    pub id: String,
    pub amount: String,
    pub provider: String,
}

#[async_trait]
pub trait ShippoClientWrapper: Send + Sync {
    async fn get_rates(&self, address_to: &str, parcel: &str) -> Result<Vec<ShippingRate>, String>;
}

pub struct RealShippoClient {
    api_token: String,
    http_client: Client,
}

impl RealShippoClient {
    pub fn new(api_token: String) -> Self {
        Self {
            api_token,
            http_client: Client::new(),
        }
    }
}

#[async_trait]
impl ShippoClientWrapper for RealShippoClient {
    async fn get_rates(&self, _address_to: &str, _parcel: &str) -> Result<Vec<ShippingRate>, String> {
        let url = "https://api.goshippo.com/shipments/";
        let res = self.http_client.post(url)
            .header("Authorization", format!("ShippoToken {}", self.api_token))
            .json(&serde_json::json!({
                "address_from": "mock_addr_id",
                "address_to": "mock_addr_id",
                "parcels": ["mock_parcel_id"],
                "async": false
            }))
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let body: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;
                    let rates_val = body["rates"].as_array().ok_or("No rates in response")?;
                    let mut rates = Vec::new();
                    for r in rates_val {
                        rates.push(ShippingRate {
                            id: r["object_id"].as_str().unwrap_or_default().to_string(),
                            amount: r["amount"].as_str().unwrap_or_default().to_string(),
                            provider: r["provider"].as_str().unwrap_or_default().to_string(),
                        });
                    }
                    Ok(rates)
                } else {
                    Err(format!("Shippo API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}
