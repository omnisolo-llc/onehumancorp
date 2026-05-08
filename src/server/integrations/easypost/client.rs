use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;

#[async_trait]
pub trait EasyPostClientWrapper: Send + Sync {
    async fn buy_label(&self, shipment_id: &str, rate_id: &str) -> Result<(), String>;
}

pub struct RealEasyPostClient {
    api_key: String,
    http_client: Client,
}

impl RealEasyPostClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            http_client: Client::new(),
        }
    }
}

#[async_trait]
impl EasyPostClientWrapper for RealEasyPostClient {
    async fn buy_label(&self, shipment_id: &str, rate_id: &str) -> Result<(), String> {
        let url = format!("https://api.easypost.com/v2/shipments/{}/buy", shipment_id);
        let res = self.http_client.post(&url)
            .basic_auth(&self.api_key, Some(""))
            .json(&json!({"rate": {"id": rate_id}}))
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    Ok(())
                } else {
                    Err(format!("EasyPost API error: {}", resp.status()))
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
    fn test_real_client_creation() {
        let client = RealEasyPostClient::new("api_key".to_string());
        assert_eq!(client.api_key, "api_key");
    }

    #[tokio::test]
    async fn test_buy_label_error_handling() {
        let client = RealEasyPostClient::new("api_key".to_string());
        let _ = client.buy_label("shp_123", "rate_123").await;
    }
}
