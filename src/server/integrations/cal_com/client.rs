use reqwest::Client;

pub struct CalComClient {
    pub access_token: String,
    #[allow(dead_code)]
    http_client: Client,
}

impl CalComClient {
    pub fn new(access_token: String) -> Self {
        CalComClient {
            access_token,
            http_client: Client::new(),
        }
    }

    pub async fn get_booking_link(&self, event_type: &str) -> Result<String, String> {
        // Technically Cal.com links are just generated based on usernames
        // But let's assume we do a quick validation
        Ok(format!("https://cal.com/ohc-tenant/{}", event_type))
    }
}
