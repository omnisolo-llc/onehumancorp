use reqwest::Client;

pub struct MailchimpClient {
    pub api_key: String,
    http_client: Client,
}

impl MailchimpClient {
    pub fn new(api_key: String) -> Self {
        MailchimpClient {
            api_key,
            http_client: Client::new(),
        }
    }

    pub async fn sync_customer(&self, email: &str, tag: &str) -> Result<(), String> {
        let url = "https://server.api.mailchimp.com/3.0/lists/mock_audience/members";

        let payload = serde_json::json!({
            "email_address": email,
            "status": "subscribed",
            "tags": [tag]
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
                    Err(format!("Mailchimp API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }

    pub async fn send_campaign(&self, audience: &str, body: &str) -> Result<(), String> {
        let url = format!("https://server.api.mailchimp.com/3.0/campaigns/{}/actions/send", audience);

        let payload = serde_json::json!({
            "body": body
        });

        let res = self.http_client.post(&url)
            .bearer_auth(&self.api_key)
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    Ok(())
                } else {
                    Err(format!("Mailchimp API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}
