use async_trait::async_trait;
use reqwest::Client;

#[async_trait]
pub trait ShippoClientWrapper: Send + Sync {
    async fn create_shipment(&self, address_to: &str, address_from: &str, parcel: &str) -> Result<String, String>;
}

pub struct RealShippoClient {
    pub api_key: String,
    http_client: Client,
}

impl RealShippoClient {
    pub fn new(api_key: String) -> Self {
        Self { api_key, http_client: Client::new() }
    }
}

#[async_trait]
impl ShippoClientWrapper for RealShippoClient {
    async fn create_shipment(&self, address_to: &str, address_from: &str, parcel: &str) -> Result<String, String> {
        let url = "https://api.goshippo.com/shipments/";
        let res = self.http_client.post(url)
            .header("Authorization", format!("ShippoToken {}", self.api_key))
            .json(&serde_json::json!({
                "address_to": address_to,
                "address_from": address_from,
                "parcels": [parcel]
            }))
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let _ = ::server_telemetry::record_api_call_cost(
                        &crate::db::get_pool(),
                        "unknown",
                        "shippo_create_shipment",
                        0.05
                    ).await;
                    let body = resp.text().await.unwrap_or_default();
                    let v: serde_json::Value = serde_json::from_str(&body).unwrap_or(serde_json::Value::Null);
                    let id = v.get("object_id").and_then(|id| id.as_str()).unwrap_or("").to_string();
                    Ok(id)
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
    fn test_client_creation() {
        let client = RealShippoClient::new("key".to_string());
        assert_eq!(client.api_key, "key");
    }

    #[tokio::test]
    async fn test_create_shipment_error_handling() {
        let client = RealShippoClient::new("key".to_string());
        let _ = client.create_shipment("address_to", "address_from", "parcel").await;
    }
}
