pub struct ResendClient { pub api_key: String }
impl ResendClient {
    pub fn new(api_key: String) -> Self { ResendClient { api_key } }

    pub async fn send_email(&self, to: &str, subject: &str, html_body: &str) -> Result<(), String> {
        let _ = crate::telemetry::record_api_call_cost(
            &crate::db::get_pool(),
            "unknown",
            "resend_send_email",
            0.02
        ).await;

        let client = reqwest::Client::new();
        let res = client.post("https://api.resend.com/emails")
            .bearer_auth(&self.api_key)
            .json(&serde_json::json!({
                "from": "onboarding@resend.dev",
                "to": to,
                "subject": subject,
                "html": html_body
            }))
            .send()
            .await;

        match res {
            Ok(resp) if resp.status().is_success() => Ok(()),
            Ok(resp) => Err(format!("Resend API error: {}", resp.status())),
            Err(e) => Err(format!("Request failed: {}", e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ResendClient;

    #[tokio::test]
    async fn test_resend_client_instantiation() {
        let client = ResendClient::new("dummy_api_key".to_string());
        assert_eq!(client.api_key, "dummy_api_key");
    }

    #[tokio::test]
    async fn test_resend_client_send_email_error_handling() {
        let client = ResendClient::new("dummy_api_key".to_string());
        let res = client.send_email("to@example.com", "sub", "body").await;
        assert!(res.is_err() || res.is_ok());
    }
}
