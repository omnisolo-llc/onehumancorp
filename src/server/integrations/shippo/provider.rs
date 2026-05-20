use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};

pub struct ShippoProvider {
    metadata: ProviderMetadata,
}

impl ShippoProvider {
    pub fn new() -> Self {
        Self {
            metadata: ProviderMetadata {
                id: "shippo".to_string(),
                name: "Shippo Shipping".to_string(),
                category: "shipping".to_string(),
                base_url: "https://api.goshippo.com".to_string(),
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
            },
        }
    }
}
