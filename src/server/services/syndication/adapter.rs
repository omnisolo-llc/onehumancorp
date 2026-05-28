use async_trait::async_trait;
use serde_json::Value;

#[async_trait]
pub trait SalesChannelAdapter: Send + Sync {
    fn platform_name(&self) -> &str;
    async fn push_product_update(&self, product: &Value) -> Result<(), String>;
}
