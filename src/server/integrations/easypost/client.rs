use serde::{Deserialize, Serialize};
use reqwest::Client;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EasyPostLabel {
    pub id: String,
    pub tracking_code: String,
    pub label_url: String,
}

pub struct EasyPostClient {
    pub api_key: String,
    http_client: Client,
}

impl EasyPostClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            http_client: Client::new(),
        }
    }

    pub async fn create_label(&self, address_to: &str, weight: f64, organization_id: &str) -> Result<EasyPostLabel, String> {
        let url = "https://api.easypost.com/v2/shipments";
        let res = self.http_client.post(url)
            .basic_auth(&self.api_key, Some(""))
            .json(&serde_json::json!({
                "shipment": {
                    "to_address": {
                        "street1": address_to,
                    },
                    "parcel": {
                        "weight": weight,
                    }
                }
            }))
            .send()
            .await;

        match res {
            Ok(resp) if resp.status().is_success() => {
                let _ = ::server_telemetry::record_api_call_cost(
                    &crate::db::get_pool(),
                    organization_id,
                    "easypost_create_label",
                    0.05
                ).await;
                let data: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;
                Ok(EasyPostLabel {
                    id: data["id"].as_str().unwrap_or_default().to_string(),
                    tracking_code: data["tracking_code"].as_str().unwrap_or_default().to_string(),
                    label_url: data["postage_label"]["label_url"].as_str().unwrap_or_default().to_string(),
                })
            }
            Ok(resp) => Err(format!("EasyPost API error: {}", resp.status())),
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_easypost_creation() {
        let _client = EasyPostClient::new("key".to_string());
    }
}
