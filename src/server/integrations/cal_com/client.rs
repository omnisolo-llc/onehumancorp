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

    pub async fn get_upcoming_appointments(&self) -> Result<Vec<String>, String> {
        Ok(vec!["Appointment 1".to_string(), "Appointment 2".to_string()])
    }

    pub async fn set_availability(&self, _availability: &str) -> Result<(), String> {
        Ok(())
    }

    pub async fn block_time_slot(&self, _start_time: &str, _end_time: &str) -> Result<(), String> {
        Ok(())
    }
}
