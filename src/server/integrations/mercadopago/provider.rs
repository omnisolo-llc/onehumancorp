use super::client::{MercadoPagoClientWrapper, RealMercadoPagoClient};
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct MercadoPagoProvider {
    client: Arc<dyn MercadoPagoClientWrapper>,
    metadata: ProviderMetadata,
}

impl MercadoPagoProvider {
    pub fn new(access_token: String) -> Self {
        let client = RealMercadoPagoClient::new(access_token);

        Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "mercadopago".to_string(),
                name: "Mercado Pago".to_string(),
                category: "payments".to_string(),
                base_url: "https://api.mercadopago.com".to_string(),
            },
        }
    }

    pub fn with_client(client: Arc<dyn MercadoPagoClientWrapper>) -> Self {
        Self {
            client,
            metadata: ProviderMetadata {
                id: "mercadopago".to_string(),
                name: "Mercado Pago".to_string(),
                category: "payments".to_string(),
                base_url: "https://api.mercadopago.com".to_string(),
            },
        }
    }

    pub fn into_integration_provider(self) -> IntegrationProvider {
        IntegrationProvider {
            metadata: self.metadata,
        }
    }

    pub async fn process_payment(&self, amount: f64, method: &str) -> Result<String, String> {
        self.client.process_payment(amount, method).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    struct MockMercadoPagoClient {
        payments_processed: Arc<AtomicUsize>,
    }

    #[async_trait]
    impl MercadoPagoClientWrapper for MockMercadoPagoClient {
        async fn process_payment(&self, _amount: f64, _method: &str) -> Result<String, String> {
            self.payments_processed.fetch_add(1, Ordering::SeqCst);
            Ok("mock_id".to_string())
        }
    }

    #[tokio::test]
    async fn test_mercadopago_provider_integration() {
        let processed = Arc::new(AtomicUsize::new(0));
        let mock = Arc::new(MockMercadoPagoClient { payments_processed: processed.clone() });
        let provider = MercadoPagoProvider::with_client(mock);

        let res = provider.process_payment(150.0, "pix").await.unwrap();
        assert_eq!(res, "mock_id");
        assert_eq!(processed.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_mercadopago_provider_new() {
        let provider = MercadoPagoProvider::new("token".to_string());
        assert_eq!(provider.metadata.id, "mercadopago");
        assert_eq!(provider.metadata.category, "payments");
    }

    #[test]
    fn test_mercadopago_provider_into() {
        let provider = MercadoPagoProvider::new("token".to_string());
        let integration = provider.into_integration_provider();
        assert_eq!(integration.metadata.id, "mercadopago");
    }
}
