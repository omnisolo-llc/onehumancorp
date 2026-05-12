use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;
use reqwest::Client;

#[async_trait::async_trait]
pub trait MailchimpClientWrapper: Send + Sync {
    async fn add_list_member(&self, server_prefix: &str, list_id: &str, email: &str, status: &str) -> Result<(), String>;
}

pub struct RealMailchimpClient {
    api_key: String,
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

#[async_trait::async_trait]
impl MailchimpClientWrapper for RealMailchimpClient {
    async fn add_list_member(&self, server_prefix: &str, list_id: &str, email: &str, status: &str) -> Result<(), String> {
        let url = format!("https://{}.api.mailchimp.com/3.0/lists/{}/members", server_prefix, list_id);
        let payload = serde_json::json!({
            "email_address": email,
            "status": status
        });

        let res = self.http_client.post(&url)
            .basic_auth("anystring", Some(&self.api_key))
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let _ = ::server_telemetry::record_api_call_cost(
                        &crate::db::get_pool(),
                        "unknown",
                        "mailchimp_add_member",
                        0.01
                    ).await;
                    Ok(())
                } else {
                    Err(format!("Mailchimp API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}

pub struct MailchimpProvider {
    client: Arc<dyn MailchimpClientWrapper>,
    metadata: ProviderMetadata,
}

impl MailchimpProvider {
    pub fn new(api_key: String) -> Self {
        let client = RealMailchimpClient::new(api_key);
        Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "mailchimp".to_string(),
                name: "Mailchimp Integration".to_string(),
                category: "marketing".to_string(),
                base_url: "https://api.mailchimp.com".to_string(),
            },
        }
    }

    pub fn with_client(client: Arc<dyn MailchimpClientWrapper>) -> Self {
        Self {
            client,
            metadata: ProviderMetadata {
                id: "mailchimp".to_string(),
                name: "Mailchimp Integration".to_string(),
                category: "marketing".to_string(),
                base_url: "https://api.mailchimp.com".to_string(),
            },
        }
    }

    pub fn into_integration_provider(self) -> IntegrationProvider {
        IntegrationProvider {
            metadata: self.metadata,
        }
    }

    pub async fn add_list_member(&self, server_prefix: &str, list_id: &str, email: &str, status: &str) -> Result<(), String> {
        self.client.add_list_member(server_prefix, list_id, email, status).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct MockClient { calls: Arc<AtomicUsize> }
    #[async_trait::async_trait]
    impl MailchimpClientWrapper for MockClient {
        async fn add_list_member(&self, _1: &str, _2: &str, _3: &str, _4: &str) -> Result<(), String> {
            self.calls.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_add() {
        let calls = Arc::new(AtomicUsize::new(0));
        let p = MailchimpProvider::with_client(Arc::new(MockClient{ calls: calls.clone() }));
        p.add_list_member("1", "2", "3", "4").await.unwrap();
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }
}
