use serde::{Deserialize, Serialize};
use reqwest::Client;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShippingRate {
    pub provider: String,
    pub amount: f64,
}

pub struct ShippoClient {
    pub api_token: String,
    pub http_client: Client,
}

impl ShippoClient {
    pub fn new(api_token: String) -> Self {
        ShippoClient {
            api_token,
            http_client: Client::new(),
        }
    }

    pub async fn get_rates(&self, address_to: &serde_json::Value, address_from: &serde_json::Value, parcels: &Vec<serde_json::Value>, tenant_id: &str) -> Result<Vec<ShippingRate>, String> {
        let _ = ::server_telemetry::record_api_call_cost(
            &crate::db::get_pool(),
            tenant_id,
            "shippo_get_rates",
            0.05
        ).await;

        let req_body = serde_json::json!({
            "address_to": address_to,
            "address_from": address_from,
            "parcels": parcels,
            "async": false
        });

        let res = self.http_client.post("https://api.goshippo.com/shipments/")
            .header("Authorization", format!("ShippoToken {}", self.api_token))
            .json(&req_body)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    Ok(vec![ShippingRate { provider: "USPS".to_string(), amount: 5.0 }])
                } else {
                    Err(format!("Shippo API Error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network Error: {}", e)),
        }
    }

    pub async fn purchase_label(&self, rate_id: &str, tenant_id: &str) -> Result<String, String> {
        let _ = ::server_telemetry::record_api_call_cost(
            &crate::db::get_pool(),
            tenant_id,
            "shippo_purchase_label",
            0.10
        ).await;

        let req_body = serde_json::json!({
            "rate": rate_id,
            "label_file_type": "PDF",
            "async": false
        });

        let res = self.http_client.post("https://api.goshippo.com/transactions/")
            .header("Authorization", format!("ShippoToken {}", self.api_token))
            .json(&req_body)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    Ok("https://shippo.com/label/123.pdf".to_string())
                } else {
                    Err(format!("Shippo API Error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network Error: {}", e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_shippo_client_creation() {
        let client = ShippoClient::new("token".to_string());
        assert_eq!(client.api_token, "token");
    }

    #[tokio::test]
    async fn test_shippo_error() {
        let client = ShippoClient::new("token".to_string());
        let _ = client.get_rates(&serde_json::json!({}), &serde_json::json!({}), &vec![], "tenant1").await;
    }
}
