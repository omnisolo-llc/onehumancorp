pub struct MailerliteClient {
    pub api_key: String,
}

impl MailerliteClient {
    pub fn new(api_key: String) -> Self {
        MailerliteClient { api_key }
    }

    pub async fn sync_customer(&self, _email: &str, _tag: &str) -> Result<(), String> {
        // Mock sync customer
        Ok(())
    }

    pub async fn send_campaign(&self, _audience: &str, _body: &str) -> Result<(), String> {
        // Mock send campaign
        Ok(())
    }
}
