use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};
use super::client::MercadoPagoClient;
use std::sync::Arc;

pub struct MercadoPagoProvider {
    metadata: ProviderMetadata,
    client: Option<Arc<MercadoPagoClient>>,
}

impl MercadoPagoProvider {
    pub fn new() -> Self {
        Self {
            metadata: ProviderMetadata {
                id: "mercadopago".to_string(),
                name: "Mercado Pago".to_string(),
                category: "payment".to_string(),
                base_url: "https://api.mercadopago.com/v1".to_string(),
            },
            client: None,
        }
    }

    pub fn with_access_token(access_token: String) -> Self {
        let mut provider = Self::new();
        provider.client = Some(Arc::new(MercadoPagoClient::new(access_token)));
        provider
    }

    pub fn into_integration_provider(self) -> IntegrationProvider {
        IntegrationProvider {
            metadata: self.metadata,
        }
    }

    pub async fn create_checkout_preference(&self, price_id: &str, tenant_id: &str) -> Result<String, String> {
         if let Some(client) = &self.client {
            client.create_checkout_preference(price_id, tenant_id).await
        } else {
            Err("MercadoPago client not initialized".to_string())
        }
    }
}
