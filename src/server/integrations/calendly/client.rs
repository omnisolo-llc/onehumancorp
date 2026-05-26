pub struct CalendlyClient {
    pub api_key: String,
}

impl CalendlyClient {
    pub fn new(api_key: String) -> Self {
        CalendlyClient { api_key }
    }

    pub async fn fetch_event_types(&self) -> Result<Vec<String>, String> {
        // Mock implementation to return a vector of event types
        Ok(vec!["30-min Consultation".to_string()])
    }
}
