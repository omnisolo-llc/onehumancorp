pub struct BufferClient {
    _api_key: String,
}

impl BufferClient {
    pub fn new(api_key: String) -> Self {
        Self { _api_key: api_key }
    }
}

impl BufferClient {
    pub async fn mock_method(&self) -> Result<(), String> {
        Ok(())
    }
}
