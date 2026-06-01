pub struct AcuityClient {
    _api_key: String,
}

impl AcuityClient {
    pub fn new(api_key: String) -> Self {
        Self { _api_key: api_key }
    }
}

impl AcuityClient {
    pub async fn mock_method(&self) -> Result<(), String> {
        Ok(())
    }
}
