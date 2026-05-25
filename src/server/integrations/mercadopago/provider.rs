use super::client::{MercadoPagoClientWrapper, MercadoPagoClient};
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct MercadoPagoProvider {
    _client: Arc<dyn MercadoPagoClientWrapper>,
    pub metadata: ProviderMetadata,
}

impl MercadoPagoProvider {
    pub fn new(api_key: String) -> Self {
        let client = MercadoPagoClient::new(api_key);

        Self {
            _client: Arc::new(client),
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
        self._client.create_checkout_preference(price_id, tenant_id).await
    }

    pub async fn get_oauth_url(&self, redirect_uri: &str) -> String {
        self._client.get_oauth_url(redirect_uri).await
    }

    pub async fn exchange_token(&self, code: &str, redirect_uri: &str) -> Result<String, String> {
        self._client.exchange_token(code, redirect_uri).await
    }
}
