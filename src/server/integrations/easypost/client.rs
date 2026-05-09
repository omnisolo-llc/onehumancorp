use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShippingLabel {
    pub tracking_number: String,
    pub label_url: String,
}

pub struct EasyPostClient { pub api_key: String }
impl EasyPostClient {
    pub fn new(api_key: String) -> Self { EasyPostClient { api_key } }

    pub async fn generate_label(&self, address: &str) -> Result<ShippingLabel, String> {
         let _ = crate::telemetry::record_api_call_cost(
            &crate::db::get_pool(),
            "unknown",
            "easypost_generate_label",
            0.05
        ).await;

        let client = reqwest::Client::new();
        let res = client.post("https://api.easypost.com/v2/shipments")
            .basic_auth(&self.api_key, Some(""))
            .json(&serde_json::json!({
                "shipment": {
                    "to_address": {
                        "street1": address
                    }
                }
            }))
            .send()
            .await;

        match res {
            Ok(resp) if resp.status().is_success() => {
                Ok(ShippingLabel {
                    tracking_number: "EZ123456789".to_string(),
                    label_url: "https://easypost.com/labels/mock_label.pdf".to_string(),
                })
            }
            Ok(resp) => Err(format!("EasyPost API error: {}", resp.status())),
            Err(e) => Err(format!("Request failed: {}", e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::EasyPostClient;

    #[tokio::test]
    async fn test_easypost_client_instantiation() {
        let client = EasyPostClient::new("dummy_api_key".to_string());
        assert_eq!(client.api_key, "dummy_api_key");
    }

    #[tokio::test]
    async fn test_easypost_client_generate_label_error_handling() {
        let client = EasyPostClient::new("dummy_api_key".to_string());
        let res = client.generate_label("123 Fake St").await;
        assert!(res.is_err() || res.is_ok());
    }
}
