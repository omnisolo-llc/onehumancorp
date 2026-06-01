pub struct TeamsClient {
    _api_key: String,
}

impl TeamsClient {
    pub fn new(api_key: String) -> Self {
        Self { _api_key: api_key }
    }
}

impl TeamsClient {
    pub async fn mock_method(&self) -> Result<(), String> {
        Ok(())
    }
}
