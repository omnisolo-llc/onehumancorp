use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;
use reqwest::Client;

#[async_trait::async_trait]
pub trait CalendlyClientWrapper: Send + Sync {
    async fn create_webhook_subscription(&self, webhook_url: &str, org: &str) -> Result<(), String>;
}

pub struct RealCalendlyClient {
    personal_access_token: String,
    http_client: Client,
}

impl RealCalendlyClient {
    pub fn new(personal_access_token: String) -> Self {
        Self {
            personal_access_token,
            http_client: Client::new(),
        }
    }
}

#[async_trait::async_trait]
impl CalendlyClientWrapper for RealCalendlyClient {
    async fn create_webhook_subscription(&self, webhook_url: &str, org: &str) -> Result<(), String> {
        let url = "https://api.calendly.com/webhook_subscriptions";
        let payload = serde_json::json!({
            "url": webhook_url,
            "events": ["invitee.created", "invitee.canceled"],
            "organization": org,
            "scope": "organization"
        });

        let res = self.http_client.post(url)
            .header("Authorization", format!("Bearer {}", self.personal_access_token))
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let _ = ::server_telemetry::record_api_call_cost(
                        &crate::db::get_pool(),
                        "unknown",
                        "calendly_create_webhook",
                        0.01
                    ).await;
                    Ok(())
                } else {
                    Err(format!("Calendly API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}

pub struct CalendlyProvider {
    client: Arc<dyn CalendlyClientWrapper>,
    metadata: ProviderMetadata,
}

impl CalendlyProvider {
    pub fn new(personal_access_token: String) -> Self {
        let client = RealCalendlyClient::new(personal_access_token);
        Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "calendly".to_string(),
                name: "Calendly Integration".to_string(),
                category: "scheduling".to_string(),
                base_url: "https://api.calendly.com".to_string(),
            },
        }
    }

    pub fn with_client(client: Arc<dyn CalendlyClientWrapper>) -> Self {
        Self {
            client,
            metadata: ProviderMetadata {
                id: "calendly".to_string(),
                name: "Calendly Integration".to_string(),
                category: "scheduling".to_string(),
                base_url: "https://api.calendly.com".to_string(),
            },
        }
    }

    pub fn into_integration_provider(self) -> IntegrationProvider {
        IntegrationProvider {
            metadata: self.metadata,
        }
    }

    pub async fn create_webhook_subscription(&self, webhook_url: &str, org: &str) -> Result<(), String> {
        self.client.create_webhook_subscription(webhook_url, org).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct MockClient { calls: Arc<AtomicUsize> }
    #[async_trait::async_trait]
    impl CalendlyClientWrapper for MockClient {
        async fn create_webhook_subscription(&self, _u: &str, _o: &str) -> Result<(), String> {
            self.calls.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_create() {
        let calls = Arc::new(AtomicUsize::new(0));
        let p = CalendlyProvider::with_client(Arc::new(MockClient{ calls: calls.clone() }));
        p.create_webhook_subscription("url", "org").await.unwrap();
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }
}
