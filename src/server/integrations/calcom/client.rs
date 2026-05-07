use async_trait::async_trait;
use reqwest::Client;

#[async_trait]
pub trait CalComClientWrapper: Send + Sync {
    async fn create_booking_link(&self, event_type_id: i32, name: &str, email: &str) -> Result<String, String>;
}

pub struct RealCalComClient {
    api_key: String,
    http_client: Client,
}

impl RealCalComClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            http_client: Client::new(),
        }
    }
}

#[async_trait]
impl CalComClientWrapper for RealCalComClient {
    async fn create_booking_link(&self, event_type_id: i32, _name: &str, _email: &str) -> Result<String, String> {
        // Cal.com API v1: POST /bookings
        // For OHC prototype, we return a pre-formatted link if API key is present
        if self.api_key.is_empty() {
             return Err("API key is required".to_string());
        }

        Ok(format!("https://cal.com/ohc-user/booking?eventTypeId={}", event_type_id))
    }
}
