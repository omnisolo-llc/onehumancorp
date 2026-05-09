pub struct MailchimpClient {
    api_key: String,
    server_prefix: String,
}

impl MailchimpClient {
    pub fn new(api_key: String, server_prefix: String) -> Self {
        Self { api_key, server_prefix }
    }

    pub async fn get_audiences(&self) -> Result<String, String> {
        Ok("Mock audiences".to_string())
    }
}
