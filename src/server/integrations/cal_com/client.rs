pub struct CalComClient {
    pub access_token: String,
}

impl CalComClient {
    pub fn new(access_token: String) -> Self {
        CalComClient { access_token }
    }
}

impl CalComClient {
    pub async fn get_booking_link(&self, event_type: &str) -> Result<String, String> {
        Ok(format!("https://cal.com/ohc-tenant/{}", event_type))
    }
}
