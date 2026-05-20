use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};

pub struct AlipayProvider {
    pub metadata: ProviderMetadata,
}

impl AlipayProvider {
    pub fn new() -> Self {
        Self {
            metadata: ProviderMetadata {
                id: "alipay".to_string(),
                name: "Alipay".to_string(),
                category: "payment".to_string(),
                base_url: "https://openapi.alipay.com/gateway.do".to_string(),
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
