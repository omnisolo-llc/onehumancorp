pub mod memory;
pub mod redis;

use async_trait::async_trait;

#[async_trait]
pub trait MeshTransport: Send + Sync {
    async fn publish(&self, channel: String, payload: Vec<u8>) -> Result<(), String>;
    async fn subscribe(&self, channel: String, handler: Box<dyn Fn(Vec<u8>) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String>;
    async fn broadcast_presence(&self, agent_id: String, status: String) -> Result<(), String>;
}
