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
