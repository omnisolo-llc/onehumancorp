use crate::mesh::protocol::TeammateMessage;
use async_trait::async_trait;

#[async_trait]
pub trait MeshTransport: Send + Sync {
    async fn publish(&self, message: TeammateMessage) -> Result<(), String>;
    async fn subscribe(&self, tenant_id: &str, handler: Box<dyn Fn(TeammateMessage) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String>;
}
