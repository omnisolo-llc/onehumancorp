pub struct CalendlyClient {
    pub api_key: String,
}

impl CalendlyClient {
    pub fn new(api_key: String) -> Self {
        CalendlyClient { api_key }
    }

    pub async fn fetch_event_types(&self) -> Result<Vec<String>, String> {
        // Mock implementation to return a vector of event types
        Ok(vec!["30-min Consultation".to_string()])
    }

    pub async fn handle_webhook(&self, _payload: &str) -> Result<(), String> {
        // Mock registering a booking
        let booking = crate::services::booking::BookingRecord {
            id: uuid::Uuid::new_v4().to_string(),
            tenant_id: "test_tenant".to_string(),
            customer_id: "test_customer".to_string(),
            product_id: "test_product".to_string(),
            start_time: chrono::Utc::now(),
            end_time: Some(chrono::Utc::now() + chrono::Duration::hours(1)),
            status: "confirmed".to_string(),
        };
        // Result ignored in mock client
        let _ = crate::services::booking::BookingService::create_booking(booking).await;
        Ok(())
    }
}
