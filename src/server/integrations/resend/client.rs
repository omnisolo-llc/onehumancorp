use reqwest::Client;

pub struct ResendClient {
    pub api_key: String,
    http_client: Client,
}

impl ResendClient {
    pub fn new(api_key: String) -> Self {
        ResendClient {
            api_key,
            http_client: Client::new(),
        }
    }

    pub async fn send_email(&self, to: &str, subject: &str, body: &str) -> Result<(), String> {
        let url = "https://api.resend.com/emails";

        let payload = serde_json::json!({
            "from": "onboarding@resend.dev", // Typically configured per tenant, using mock default
            "to": [to],
            "subject": subject,
            "html": body
        });

        let res = self.http_client.post(url)
            .bearer_auth(&self.api_key)
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let _ = ::server_telemetry::record_api_call_cost(
                        &crate::db::get_pool(),
                        "unknown",
                        "resend_send_email",
                        0.001
                    ).await;
                    Ok(())
                } else {
                    Err(format!("Resend API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}
