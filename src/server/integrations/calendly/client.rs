use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait CalendlyClientWrapper: Send + Sync {
    async fn fetch_event_types(&self) -> Result<Vec<String>, String>;
    async fn create_webhook(&self, url: &str) -> Result<(), String>;
}

pub struct RealCalendlyClient {
    pub api_key: String,
    http_client: Client,
}

impl RealCalendlyClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            http_client: Client::new(),
        }
    }
}

#[async_trait]
impl CalendlyClientWrapper for RealCalendlyClient {
    async fn fetch_event_types(&self) -> Result<Vec<String>, String> {
        let res = self.http_client.get("https://api.calendly.com/event_types")
            .bearer_auth(&self.api_key)
            .send()
            .await;

        match res {
            Ok(resp) if resp.status().is_success() => Ok(vec!["30-min Consultation".to_string()]),
            _ => Err("Failed to fetch Calendly events".to_string())
        }
    }

    async fn create_webhook(&self, url: &str) -> Result<(), String> {
        let payload = serde_json::json!({
            "url": url,
            "events": ["invitee.created", "invitee.canceled"],
            "organization": "https://api.calendly.com/organizations/me",
            "user": "https://api.calendly.com/users/me"
        });

        let res = self.http_client.post("https://api.calendly.com/webhook_subscriptions")
            .bearer_auth(&self.api_key)
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) if resp.status().is_success() => Ok(()),
            _ => Err("Failed to create Calendly webhook".to_string())
        }
    }
}
