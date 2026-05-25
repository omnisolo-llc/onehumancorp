use reqwest::Client;

pub struct SendGridClient {
    pub api_key: String,
    http_client: Client,
}

impl SendGridClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            http_client: Client::new(),
        }
    }

    pub async fn send_email(&self, to: &str, subject: &str, body: &str) -> Result<(), String> {
        let url = "https://api.sendgrid.com/v3/mail/send";
        let payload = serde_json::json!({
            "personalizations": [{
                "to": [{"email": to}]
            }],
            "from": {"email": "no-reply@onehumancorp.com"},
            "subject": subject,
            "content": [{"type": "text/plain", "value": body}]
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
                    Err(format!("SendGrid API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}
