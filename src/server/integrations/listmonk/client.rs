pub struct ListmonkClient {
    api_key: String,
}

impl ListmonkClient {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }
}

impl ListmonkClient {
    pub async fn send_campaign(&self, list_id: &str, template_id: &str, subject: &str, body: &str) -> Result<(), String> {
        // Mock send campaign
        Ok(())
    }
}
