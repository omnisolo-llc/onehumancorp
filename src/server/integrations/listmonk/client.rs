pub struct ListmonkClient {
    _api_key: String,
}

impl ListmonkClient {
    pub fn new(api_key: String) -> Self {
        Self { _api_key: api_key }
    }
}

impl ListmonkClient {
    pub async fn send_campaign(&self, _list_id: &str, _template_id: &str, _subject: &str, _body: &str) -> Result<(), String> {
        // Mock send campaign
        Ok(())
    }
}
