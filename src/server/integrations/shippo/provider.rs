use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;
use reqwest::Client;

#[derive(serde::Deserialize)]
struct ShippoTransactionResponse {
    label_url: String,
}

#[async_trait::async_trait]
pub trait ShippoClientWrapper: Send + Sync {
    async fn create_transaction(&self, rate_object_id: &str) -> Result<String, String>;
}

pub struct RealShippoClient {
    api_token: String,
    http_client: Client,
}

impl RealShippoClient {
    pub fn new(api_token: String) -> Self {
        Self {
            api_token,
            http_client: Client::new(),
        }
    }
}

#[async_trait::async_trait]
impl ShippoClientWrapper for RealShippoClient {
    async fn create_transaction(&self, rate_object_id: &str) -> Result<String, String> {
        let url = "https://api.goshippo.com/transactions/";
        let payload = serde_json::json!({
            "rate": rate_object_id,
            "label_file_type": "PDF",
            "async": false
        });

        let res = self.http_client.post(url)
            .header("Authorization", format!("ShippoToken {}", self.api_token))
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let _ = ::server_telemetry::record_api_call_cost(
                        &crate::db::get_pool(),
                        "unknown",
                        "shippo_create_transaction",
                        0.05
                    ).await;

                    if let Ok(data) = resp.json::<ShippoTransactionResponse>().await {
                        Ok(data.label_url)
                    } else {
                        Err("Failed to parse label URL".to_string())
                    }
                } else {
                    Err(format!("Shippo API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}

pub struct ShippoProvider {
    client: Arc<dyn ShippoClientWrapper>,
    metadata: ProviderMetadata,
}

impl ShippoProvider {
    pub fn new(api_token: String) -> Self {
        let client = RealShippoClient::new(api_token);
        Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "shippo".to_string(),
                name: "Shippo Integration".to_string(),
                category: "logistics".to_string(),
                base_url: "https://api.goshippo.com".to_string(),
            },
        }
    }

    pub fn with_client(client: Arc<dyn ShippoClientWrapper>) -> Self {
        Self {
            client,
            metadata: ProviderMetadata {
                id: "shippo".to_string(),
                name: "Shippo Integration".to_string(),
                category: "logistics".to_string(),
                base_url: "https://api.goshippo.com".to_string(),
            },
        }
    }

    pub fn into_integration_provider(self) -> IntegrationProvider {
        IntegrationProvider {
            metadata: self.metadata,
        }
    }

    pub async fn create_transaction(&self, rate_object_id: &str) -> Result<String, String> {
        self.client.create_transaction(rate_object_id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct MockClient { calls: Arc<AtomicUsize> }
    #[async_trait::async_trait]
    impl ShippoClientWrapper for MockClient {
        async fn create_transaction(&self, _1: &str) -> Result<String, String> {
            self.calls.fetch_add(1, Ordering::SeqCst);
            Ok("url".to_string())
        }
    }

    #[tokio::test]
    async fn test_create() {
        let calls = Arc::new(AtomicUsize::new(0));
        let p = ShippoProvider::with_client(Arc::new(MockClient{ calls: calls.clone() }));
        let res = p.create_transaction("1").await.unwrap();
        assert_eq!(res, "url");
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }
}
