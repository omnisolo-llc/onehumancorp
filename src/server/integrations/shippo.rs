pub mod provider {
    use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};

    pub struct ShippoProvider {}

    impl ShippoProvider {
        pub fn new() -> Self {
            Self {}
        }

        pub fn into_integration_provider(self) -> IntegrationProvider {
            IntegrationProvider {
                metadata: ProviderMetadata {
                    id: "shippo".to_string(),
                    name: "Shippo".to_string(),
                    category: "Shipping & Logistics".to_string(),
                    base_url: "https://goshippo.com".to_string(),
                },
            }
        }

        pub async fn initialize_oauth_flow(&self) -> Result<String, String> {
            Ok("https://goshippo.com/oauth/authorize?client_id=MOCK".to_string())
        }

        pub async fn get_rates(&self, _weight: f64) -> Result<Vec<String>, String> {
            Ok(vec!["USPS: $5.00".to_string(), "UPS: $8.00".to_string()])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::provider::*;

    #[test]
    fn test_shippo_provider_metadata() {
        let provider = ShippoProvider::new().into_integration_provider();
        assert_eq!(provider.metadata.id, "shippo");
        assert_eq!(provider.metadata.name, "Shippo");
        assert_eq!(provider.metadata.category, "Shipping & Logistics");
        assert_eq!(provider.metadata.base_url, "https://goshippo.com");
    }

    #[tokio::test]
    async fn test_shippo_auth_flow() {
        let provider = ShippoProvider::new();
        let url = provider.initialize_oauth_flow().await.unwrap();
        assert!(url.contains("oauth/authorize"));
    }

    #[tokio::test]
    async fn test_shippo_rates() {
        let provider = ShippoProvider::new();
        let rates = provider.get_rates(1.0).await.unwrap();
        assert_eq!(rates.len(), 2);
    }
}
