pub mod memory;
pub mod redis;
pub mod ipc;
pub mod nats;
pub mod state_handoff;

pub use memory::MemoryBus;
pub use redis::RedisBus;
pub use ipc::IpcBus;
pub use nats::NatsBus;
pub use state_handoff::StateHandoffManager;

use async_trait::async_trait;

#[derive(Clone, prost::Message)]
#[allow(dead_code)]
pub struct Message {
    #[prost(string, tag = "1")]
    pub topic: String,
    #[prost(bytes, tag = "2")]
    pub payload: Vec<u8>,
}

#[async_trait]
pub trait DistributedLock: Send + Sync {
    async fn acquire_lock(&self, resource: &str, owner: &str, ttl_seconds: u64) -> Result<bool, String>;
    async fn release_lock(&self, resource: &str, owner: &str) -> Result<(), String>;
}

#[async_trait]
#[allow(dead_code)]
pub trait Bus: Send + Sync {
    async fn publish(&self, msg: Message) -> Result<(), String>;
    async fn subscribe(&self, topic: String, handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String>;
}
