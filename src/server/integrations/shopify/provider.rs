use super::client::ShopifyClient;
use ::server_integrations_core::{IntegrationProvider, ProviderMetadata};
use std::sync::Arc;

pub struct ShopifyProvider {
    client: Arc<ShopifyClient>,
    metadata: ProviderMetadata,
}

impl ShopifyProvider {
    pub fn new(store_url: String, access_token: String) -> Self {
        let client = ShopifyClient::new(store_url.clone(), access_token);
        let base_url = format!("{}/admin/api/2024-01/graphql.json", store_url);

        Self {
            client: Arc::new(client),
            metadata: ProviderMetadata {
                id: "shopify".to_string(),
                name: "Shopify".to_string(),
                category: "ecommerce".to_string(),
                base_url,
            },
        }
    }

    pub fn with_client(client: Arc<ShopifyClient>, store_url: String) -> Self {
        let base_url = format!("{}/admin/api/2024-01/graphql.json", store_url);

        Self {
            client,
            metadata: ProviderMetadata {
                id: "shopify".to_string(),
                name: "Shopify".to_string(),
                category: "ecommerce".to_string(),
                base_url,
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

    pub async fn get_products(
        &self,
        first: u32,
    ) -> Result<Vec<super::client::ShopifyProduct>, String> {
        self.client.get_products(first).await
    }

    pub async fn create_product(
        &self,
        title: &str,
        description: &str,
        price: &str,
    ) -> Result<super::client::ShopifyProduct, String> {
        self.client.create_product(title, description, price).await
    }

    pub async fn get_orders(
        &self,
        first: u32,
        status: Option<&str>,
    ) -> Result<Vec<super::client::ShopifyOrder>, String> {
        self.client.get_orders(first, status).await
    }

    pub async fn get_inventory_levels(
        &self,
        location_id: &str,
    ) -> Result<Vec<super::client::ShopifyInventoryItem>, String> {
        self.client.get_inventory_levels(location_id).await
    }

    pub async fn update_inventory(
        &self,
        inventory_item_id: &str,
        location_id: &str,
        available: i32,
    ) -> Result<(), String> {
        self.client
            .update_inventory(inventory_item_id, location_id, available)
            .await
    }

    pub async fn get_customers(
        &self,
        first: u32,
    ) -> Result<Vec<super::client::ShopifyCustomer>, String> {
        self.client.get_customers(first).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shopify_provider_new() {
        let provider =
            ShopifyProvider::new("my-store.myshopify.com".to_string(), "test-token".to_string());
        assert_eq!(provider.metadata.id, "shopify");
        assert_eq!(provider.metadata.category, "ecommerce");
    }

    #[test]
    fn test_shopify_provider_to_integration_provider() {
        let provider =
            ShopifyProvider::new("my-store.myshopify.com".to_string(), "test-token".to_string());
        let integration = provider.to_integration_provider();
        assert_eq!(integration.metadata.id, "shopify");
    }

    #[test]
    fn test_shopify_provider_with_client() {
        let client = Arc::new(ShopifyClient::new(
            "my-store.myshopify.com".to_string(),
            "test-token".to_string(),
        ));
        let provider = ShopifyProvider::with_client(
            client,
            "my-store.myshopify.com".to_string(),
        );
        assert_eq!(provider.metadata.id, "shopify");
    }
}
