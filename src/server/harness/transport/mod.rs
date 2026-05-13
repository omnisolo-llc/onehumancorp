pub mod bridge;
pub mod in_process;
pub mod redis_pubsub;

pub use bridge::{UniversalTransportBridge, TransportMode};
pub use in_process::InProcessTransport;
pub use redis_pubsub::RedisPubSubTransport;

use async_trait::async_trait;
use tokio::sync::broadcast;

#[async_trait]
pub trait Transport: Send + Sync {
    async fn send(&self, topic: &str, message: &str) -> Result<(), String>;
    async fn subscribe(&self, topic: &str) -> Result<broadcast::Receiver<String>, String>;

    async fn acquire_lock(&self, resource: &str, owner: &str, ttl_seconds: u64) -> Result<bool, String>;
    async fn release_lock(&self, resource: &str, owner: &str) -> Result<(), String>;
    async fn register_presence(&self, agent_id: &str, status: &str, ttl_seconds: u64) -> Result<(), String>;
    async fn get_active_agents(&self) -> Result<Vec<(String, String)>, String>;
}
