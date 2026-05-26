use reqwest::Client;

pub struct ResendClient {
    pub api_key: String,
    http_client: Client,
}

impl ResendClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            http_client: Client::new(),
        }
    }

    pub async fn send_email(&self, to: &str, subject: &str, body: &str, tenant_domain_email: Option<&str>) -> Result<(), String> {
        let url = "https://api.resend.com/emails";
        let from_email = tenant_domain_email.unwrap_or("no-reply@onehumancorp.com");
        let payload = serde_json::json!({
            "from": from_email,
            "to": [to],
            "subject": subject,
            "text": body
        });

        let res = self.http_client.post(url)
            .bearer_auth(&self.api_key)
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    Ok(())
                } else {
                    Err(format!("Resend API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}
