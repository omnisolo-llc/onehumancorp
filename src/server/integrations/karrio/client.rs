pub struct KarrioClient {
    pub api_key: String,
}

impl KarrioClient {
    pub fn new(api_key: String) -> Self {
        KarrioClient { api_key }
    }

    pub async fn get_rates(&self, _order_id: &str) -> Result<String, String> {
        Ok("mock_rates_from_karrio".to_string())
    }
}
