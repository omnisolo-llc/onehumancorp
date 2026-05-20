use super::client::MercadoPagoClient;
use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct MercadoPagoProvider {
    _client: Arc<MercadoPagoClient>,
    metadata: ProviderMetadata,
}

impl MercadoPagoProvider {
    pub fn new(access_token: String) -> Self {
        let client = MercadoPagoClient::new(access_token);

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
}

impl MercadoPagoProvider {
    pub async fn create_checkout_preference(&self, price_id: &str, tenant_id: &str) -> Result<String, String> {
        self._client.create_checkout_preference(price_id, tenant_id).await
    }

    pub async fn create_payment(&self, amount: f64, description: &str, payer_email: &str) -> Result<String, String> {
        self._client.create_payment(amount, description, payer_email).await
    }
}
