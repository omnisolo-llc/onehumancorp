pub mod provider {
    use crate::integrations::catalog::{IntegrationProvider, ProviderMetadata};

    pub struct MercadopagoProvider {}

    impl MercadopagoProvider {
        pub fn new() -> Self {
            Self {}
        }

        pub fn into_integration_provider(self) -> IntegrationProvider {
            IntegrationProvider {
                metadata: ProviderMetadata {
                    id: "mercadopago".to_string(),
                    name: "Mercado Pago".to_string(),
                    category: "Payment Processing".to_string(),
                    base_url: "https://mercadopago.com".to_string(),
                },
            }
        }

        pub async fn initialize_oauth_flow(&self) -> Result<String, String> {
            Ok("https://auth.mercadopago.com/authorization?client_id=MOCK".to_string())
        }

        pub async fn create_payment_intent(&self, _amount: f64) -> Result<String, String> {
            Ok("payment_intent_123".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::provider::*;

    #[test]
    fn test_mercadopago_provider_metadata() {
        let provider = MercadopagoProvider::new().into_integration_provider();
        assert_eq!(provider.metadata.id, "mercadopago");
        assert_eq!(provider.metadata.name, "Mercado Pago");
        assert_eq!(provider.metadata.category, "Payment Processing");
        assert_eq!(provider.metadata.base_url, "https://mercadopago.com");
    }

    #[tokio::test]
    async fn test_mercadopago_auth_flow() {
        let provider = MercadopagoProvider::new();
        let url = provider.initialize_oauth_flow().await.unwrap();
        assert!(url.contains("authorization"));
    }

    #[tokio::test]
    async fn test_mercadopago_payment() {
        let provider = MercadopagoProvider::new();
        let intent = provider.create_payment_intent(100.0).await.unwrap();
        assert_eq!(intent, "payment_intent_123");
    }
}
