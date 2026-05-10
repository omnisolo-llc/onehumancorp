pub struct CalComClient {
    api_key: String,
}

impl CalComClient {
    pub fn new(api_key: String) -> Self {
        Self { api_key }
    }

    pub async fn create_booking_link(&self, event_type_id: &str, _duration_mins: i32) -> Result<String, String> {
        Ok(format!("https://cal.com/booking/{}", event_type_id))
    }

    pub async fn get_availability(&self, _date_from: &str, _date_to: &str) -> Result<Vec<String>, String> {
         Ok(vec!["2026-06-01T10:00:00Z".to_string(), "2026-06-01T14:00:00Z".to_string()])
    }
}
