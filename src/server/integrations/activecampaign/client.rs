pub struct ActiveCampaignClient {
    _api_key: String,
}

impl ActiveCampaignClient {
    pub fn new(api_key: String) -> Self {
        Self { _api_key: api_key }
    }
}

impl ActiveCampaignClient {
    pub async fn mock_method(&self) -> Result<(), String> {
        Ok(())
    }
}
