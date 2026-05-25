use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait MailchimpClientWrapper: Send + Sync {
    async fn sync_customer(&self, email: &str, tag: &str) -> Result<(), String>;
    async fn send_campaign(&self, audience: &str, body: &str) -> Result<(), String>;
}

pub struct RealMailchimpClient {
    pub api_key: String,
    http_client: Client,
}

impl RealMailchimpClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            http_client: Client::new(),
        }
    }
}

#[async_trait]
impl MailchimpClientWrapper for RealMailchimpClient {
    async fn sync_customer(&self, email: &str, tag: &str) -> Result<(), String> {
        let payload = serde_json::json!({
            "email_address": email,
            "status": "subscribed",
            "tags": [tag]
        });

        let res = self.http_client.post("https://usX.api.mailchimp.com/3.0/lists/AUDIENCE_ID/members")
            .basic_auth("user", Some(&self.api_key))
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) if resp.status().is_success() => Ok(()),
            _ => Err("Failed to sync customer to Mailchimp".to_string())
        }
    }

    async fn send_campaign(&self, _audience: &str, _body: &str) -> Result<(), String> {
        Ok(())
    }
}
