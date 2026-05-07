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

    pub fn into_integration_provider(self) -> IntegrationProvider {
        IntegrationProvider {
            metadata: self.metadata,
        }
    }

    pub async fn process_payment(&self, amount: f64, description: &str, payer_email: &str) -> Result<String, String> {
        self.client.create_payment(amount, description, payer_email).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;

    struct MockMercadoPagoClient;

    #[async_trait]
    impl MercadoPagoClientWrapper for MockMercadoPagoClient {
        async fn create_payment(&self, _amount: f64, _description: &str, _payer_email: &str) -> Result<String, String> {
            Ok("mp_test".to_string())
        }
    }

    #[tokio::test]
    async fn test_process_payment() {
        let provider = MercadoPagoProvider {
            client: Arc::new(MockMercadoPagoClient),
            metadata: ProviderMetadata {
                id: "mercadopago".to_string(),
                name: "Mercado Pago".to_string(),
                category: "payments".to_string(),
                base_url: "url".to_string(),
            },
        };
        let payment = provider.process_payment(10.0, "desc", "email").await.unwrap();
        assert_eq!(payment, "mp_test");
    }
}
