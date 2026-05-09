pub struct CalendlyClient {
    api_key: String,
}

impl CalendlyClient {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }

    pub async fn get_event_types(&self) -> Result<String, String> {
        Ok("Mock event types".to_string())
    }
}
