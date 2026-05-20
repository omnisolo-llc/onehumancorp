use super::client::MercadoPagoClient;
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct MercadoPagoProvider {
    client: Arc<MercadoPagoClient>,
    metadata: ProviderMetadata,
}

impl MercadoPagoProvider {
    pub fn new(access_token: String) -> Self {
        let client = MercadoPagoClient::new(access_token);

        Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "mercadopago".to_string(),
                name: "Mercado Pago".to_string(),
                category: "payment".to_string(),
                base_url: "https://api.mercadopago.com".to_string(),
            },
        }
    }

    pub fn to_integration_provider(&self) -> IntegrationProvider {
        IntegrationProvider {
            metadata: ProviderMetadata {
                id: self.metadata.id.clone(),
                name: self.metadata.name.clone(),
                category: self.metadata.category.clone(),
                base_url: self.metadata.base_url.clone(),
            }
        }
    }

    pub async fn create_checkout_preference(&self, price_id: &str, tenant_id: &str) -> Result<String, String> {
        self.client.create_checkout_preference(price_id, tenant_id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mercadopago_provider_new() {
        let provider = MercadoPagoProvider::new("token".to_string());
        assert_eq!(provider.metadata.id, "mercadopago");
        assert_eq!(provider.metadata.category, "payment");
    }

    #[test]
    fn test_mercadopago_provider_to_integration_provider() {
        let provider = MercadoPagoProvider::new("token".to_string());
        let integration = provider.to_integration_provider();
        assert_eq!(integration.metadata.id, "mercadopago");
    }

    #[tokio::test]
    async fn test_create_checkout_preference() {
        let provider = MercadoPagoProvider::new("token".to_string());
        let result = provider.create_checkout_preference("price_123", "tenant_1").await;
        assert!(result.is_ok());
    }
}
