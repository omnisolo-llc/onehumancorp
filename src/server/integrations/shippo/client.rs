pub struct ShippoClient {
    pub api_key: String,
}

impl ShippoClient {
    pub fn new(api_key: String) -> Self {
        ShippoClient { api_key }
    }

    pub async fn fetch_rates(&self, _weight: f64, _dimensions: &str) -> Result<Vec<String>, String> {
        Ok(vec!["USPS Priority Mail - $8.50".to_string(), "USPS First-Class Mail - $4.20".to_string(), "UPS Ground - $9.75".to_string()])
    }

    pub async fn purchase_label(&self, _rate_id: &str) -> Result<String, String> {
        Ok("https://api.goshippo.com/v1/mock_label.pdf".to_string())
    }
}
