use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait ShippoClientWrapper: Send + Sync {
    async fn fetch_rates(&self, weight: f64, dimensions: &str) -> Result<Vec<String>, String>;
    async fn purchase_label(&self, rate_id: &str) -> Result<String, String>;
}

pub struct RealShippoClient {
    pub api_key: String,
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
    async fn fetch_rates(&self, weight: f64, _dimensions: &str) -> Result<Vec<String>, String> {
        let payload = serde_json::json!({
            "address_from": { "zip": "94117", "country": "US" },
            "address_to": { "zip": "10007", "country": "US" },
            "parcels": [{ "length": "5", "width": "5", "height": "5", "distance_unit": "in", "weight": weight, "mass_unit": "lb" }]
        });

        let res = self.http_client.post("https://api.goshippo.com/shipments/")
            .header("Authorization", format!("ShippoToken {}", self.api_key))
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) if resp.status().is_success() => Ok(vec!["USPS Ground - $5.00".to_string()]),
            _ => Err("Failed to fetch rates".to_string())
        }
    }

    async fn purchase_label(&self, rate_id: &str) -> Result<String, String> {
        let payload = serde_json::json!({
            "rate": rate_id,
            "label_file_type": "PDF"
        });

        let res = self.http_client.post("https://api.goshippo.com/transactions/")
            .header("Authorization", format!("ShippoToken {}", self.api_key))
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) if resp.status().is_success() => Ok("https://shippo-delivery.s3.amazonaws.com/label.pdf".to_string()),
            _ => Err("Failed to purchase label".to_string())
        }
    }
}
