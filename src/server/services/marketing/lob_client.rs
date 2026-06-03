pub struct LobClient {
    api_key: String,
    base_url: String,
}

impl LobClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            base_url: "https://api.lob.com/v1".to_string(),
        }
    }

    pub async fn dispatch_postcard(&self, radius: u32, location: &str, content: &str) -> Result<String, String> {
        // Implementation stub for dispatching postcard
        Ok(format!("Dispatched postcard to {} homes around {}", radius, location))
    }
}
