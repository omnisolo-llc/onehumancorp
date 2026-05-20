pub struct ResendClient {
    pub api_key: String,
}

impl ResendClient {
    pub fn new(api_key: String) -> Self {
        ResendClient { api_key }
    }

    pub async fn send_email(&self, to: &str, from: &str, subject: &str, html: &str) -> Result<String, String> {
        let _ = ::server_telemetry::record_api_call_cost(
            &crate::db::get_pool(),
            "unknown",
            "resend_send_email",
            0.01
        ).await;
        Ok("mock_email_id".to_string())
    }
}
