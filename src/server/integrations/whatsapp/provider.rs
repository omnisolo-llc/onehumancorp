use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;
use reqwest::Client;

#[async_trait::async_trait]
pub trait WhatsappClientWrapper: Send + Sync {
    async fn send_message(&self, phone_number_id: &str, to: &str, body: &str) -> Result<(), String>;
}

pub struct RealWhatsappClient {
    access_token: String,
    http_client: Client,
}

impl RealWhatsappClient {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            http_client: Client::new(),
        }
    }
}

#[async_trait::async_trait]
impl WhatsappClientWrapper for RealWhatsappClient {
    async fn send_message(&self, phone_number_id: &str, to: &str, body: &str) -> Result<(), String> {
        let url = format!("https://graph.facebook.com/v17.0/{}/messages", phone_number_id);
        let payload = serde_json::json!({
            "messaging_product": "whatsapp",
            "to": to,
            "type": "text",
            "text": { "body": body }
        });

        let res = self.http_client.post(&url)
            .header("Authorization", format!("Bearer {}", self.access_token))
            .json(&payload)
            .send()
            .await;

        match res {
            Ok(resp) => {
                if resp.status().is_success() {
                    let _ = ::server_telemetry::record_api_call_cost(
                        &crate::db::get_pool(),
                        "unknown",
                        "whatsapp_send_message",
                        0.05
                    ).await;
                    Ok(())
                } else {
                    Err(format!("WhatsApp API error: {}", resp.status()))
                }
            }
            Err(e) => Err(format!("Network error: {}", e)),
        }
    }
}

pub struct WhatsappProvider {
    client: Arc<dyn WhatsappClientWrapper>,
    metadata: ProviderMetadata,
}

impl WhatsappProvider {
    pub fn new(access_token: String) -> Self {
        let client = RealWhatsappClient::new(access_token);
        Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "whatsapp".to_string(),
                name: "WhatsApp Business Integration".to_string(),
                category: "messaging".to_string(),
                base_url: "https://graph.facebook.com/v17.0".to_string(),
            },
        }
    }

    pub fn with_client(client: Arc<dyn WhatsappClientWrapper>) -> Self {
        Self {
            client,
            metadata: ProviderMetadata {
                id: "whatsapp".to_string(),
                name: "WhatsApp Business Integration".to_string(),
                category: "messaging".to_string(),
                base_url: "https://graph.facebook.com/v17.0".to_string(),
            },
        }
    }

    pub fn into_integration_provider(self) -> IntegrationProvider {
        IntegrationProvider {
            metadata: self.metadata,
        }
    }

    pub async fn send_message(&self, phone_number_id: &str, to: &str, body: &str) -> Result<(), String> {
        self.client.send_message(phone_number_id, to, body).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct MockClient { calls: Arc<AtomicUsize> }
    #[async_trait::async_trait]
    impl WhatsappClientWrapper for MockClient {
        async fn send_message(&self, _id: &str, _to: &str, _body: &str) -> Result<(), String> {
            self.calls.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_send() {
        let calls = Arc::new(AtomicUsize::new(0));
        let p = WhatsappProvider::with_client(Arc::new(MockClient{ calls: calls.clone() }));
        p.send_message("123", "to", "hi").await.unwrap();
        assert_eq!(calls.load(Ordering::SeqCst), 1);
    }
}
