use super::client::{
    ShipdayClient, ShipdayCreateDeliveryRequest, ShipdayDelivery, ShipdayDeliveryStatus,
};
use ::server_integrations_core::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct ShipdayProvider {
    client: Arc<ShipdayClient>,
    metadata: ProviderMetadata,
}

impl ShipdayProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Arc::new(ShipdayClient::new(api_key)),
            metadata: ProviderMetadata {
                id: "shipday".to_string(),
                name: "Shipday Local Delivery".to_string(),
                category: "delivery".to_string(),
                base_url: "https://api.shipday.com".to_string(),
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

    pub async fn create_delivery(
        &self,
        request: ShipdayCreateDeliveryRequest,
    ) -> Result<ShipdayDelivery, String> {
        self.client.create_delivery(request).await
    }

    pub async fn delivery_status(
        &self,
        tracking_id: &str,
    ) -> Result<ShipdayDeliveryStatus, String> {
        self.client.delivery_status(tracking_id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provider_exports_shipday_delivery_metadata() {
        let provider = ShipdayProvider::new("live_shipday_key".to_string());
        let integration = provider.to_integration_provider();

        assert_eq!(integration.metadata.id, "shipday");
        assert_eq!(integration.metadata.name, "Shipday Local Delivery");
        assert_eq!(integration.metadata.category, "delivery");
        assert_eq!(integration.metadata.base_url, "https://api.shipday.com");
    }
}
