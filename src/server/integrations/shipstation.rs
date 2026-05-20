use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};

pub struct ShipStationProvider {
    pub metadata: ProviderMetadata,
}

impl ShipStationProvider {
    pub fn new() -> Self {
        Self {
            metadata: ProviderMetadata {
                id: "shipstation".to_string(),
                name: "ShipStation".to_string(),
                category: "shipping".to_string(),
                base_url: "https://ssapi.shipstation.com".to_string(),
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
