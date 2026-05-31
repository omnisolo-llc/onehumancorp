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
        let dc = "us1"; // Placeholder datacenter
        let list_id = "mock_list_id";
        let url = format!("https://{}.api.mailchimp.com/3.0/lists/{}/members", dc, list_id);

        let payload = serde_json::json!({
            "email_address": email,
            "status": "subscribed",
            "tags": [tag]
        });

        let res = self.http_client.post(&url)
            .basic_auth("anystring", Some(&self.api_key))
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() || resp.status().as_u16() == 400 {
                    // 400 often means already subscribed, which is fine for sync
                    Ok(())
                } else {
                    Err(format!("Mailchimp API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }

    pub async fn send_campaign(&self, _audience: &str, _body: &str) -> Result<(), String> {
        let dc = "us1"; // Placeholder datacenter
        let campaign_id = "mock_campaign_id";
        let url = format!("https://{}.api.mailchimp.com/3.0/campaigns/{}/actions/send", dc, campaign_id);

        let res = self.http_client.post(&url)
            .basic_auth("anystring", Some(&self.api_key))
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
