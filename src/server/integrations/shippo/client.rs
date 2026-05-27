pub struct ShippoClient {
    pub api_key: String,
}

impl ShippoClient {
    pub fn new(api_key: String) -> Self {
        ShippoClient { api_key }
    }

    pub async fn fetch_rates(&self, _weight: f64, _dimensions: &str) -> Result<Vec<String>, String> {
        Ok(vec!["USPS - $5.00".to_string()])
    }

    pub async fn purchase_label(&self, _rate_id: &str) -> Result<String, String> {
        Ok("https://api.goshippo.com/v1/mock_label.pdf".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shippo_client_new() {
        let client = ShippoClient::new("dummy_token".to_string());
        assert_eq!(client.api_key, "dummy_token");
    }

    #[tokio::test]
    async fn test_shippo_fetch_rates() {
        let client = ShippoClient::new("dummy_token".to_string());
        let res = client.fetch_rates(1.0, "1x1x1").await;
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), vec!["USPS - $5.00".to_string()]);
    }

    #[tokio::test]
    async fn test_shippo_purchase_label() {
        let client = ShippoClient::new("dummy_token".to_string());
        let res = client.purchase_label("rate_123").await;
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), "https://api.goshippo.com/v1/mock_label.pdf".to_string());
    }
}
