use async_trait::async_trait;
use reqwest::Client;

#[async_trait]
pub trait ShippoClientWrapper: Send + Sync {
    async fn buy_shipping_label(&self, order_id: &str) -> Result<String, String>;
}

pub struct RealShippoClient {
    api_key: String,
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
    async fn buy_shipping_label(&self, order_id: &str) -> Result<String, String> {
        let url = "https://api.goshippo.com/transactions/";
        let res = self.http_client.post(url)
            .header("Authorization", format!("ShippoToken {}", self.api_key))
            .json(&serde_json::json!({
                "rate": format!("rate_{}", order_id),
                "label_file_type": "PDF",
                "async": false
            }))
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    Ok(format!("shippo_label_{}.pdf", order_id))
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
    fn test_real_client_creation() {
        let client = RealShippoClient::new("token".to_string());
        assert_eq!(client.api_key, "token");
    }

    #[tokio::test]
    async fn test_buy_shipping_label_error_handling() {
        let client = RealShippoClient::new("token".to_string());
        let _ = client.buy_shipping_label("1234").await;
    }
}
