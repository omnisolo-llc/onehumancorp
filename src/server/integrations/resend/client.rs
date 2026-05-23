pub struct ResendClient {
    pub api_key: String,
}

impl ResendClient {
    pub fn new(api_key: String) -> Self {
        ResendClient { api_key }
    }

    pub async fn send_email(&self, _to: &str, _subject: &str, _body: &str) -> Result<(), String> {
        // Mock send email
        Ok(())
    }
}
