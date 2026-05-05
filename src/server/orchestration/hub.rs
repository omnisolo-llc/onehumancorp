use async_trait::async_trait;
use ohc_builtin_agent::mesh::transport::{MeshTransport, Message};

pub struct RedisMeshTransport {
    inner: ohc_builtin_agent::mesh::transport::RedisTransport,
}

impl RedisMeshTransport {
    pub async fn new(url: &str) -> Result<Self, String> {
        let inner = ohc_builtin_agent::mesh::transport::RedisTransport::new(url).await
            .map_err(|e| format!("Failed to create RedisTransport: {}", e))?;
        Ok(Self { inner })
    }
}

#[async_trait]
impl MeshTransport for RedisMeshTransport {
    async fn publish(&self, topic: &str, message: crate::ohc::orchestration::TeammateMeshEvent) -> Result<(), String> {
        self.inner.publish(topic, message).await
    }

    async fn subscribe(&self, topic: &str, handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        self.inner.subscribe(topic, handler).await
    }

    async fn acquire_lock(&self, resource: &str, owner: &str, ttl_seconds: u64) -> Result<bool, String> {
        self.inner.acquire_lock(resource, owner, ttl_seconds).await
    }

    async fn release_lock(&self, resource: &str, owner: &str) -> Result<(), String> {
        self.inner.release_lock(resource, owner).await
    }

    async fn register_presence(&self, agent_id: &str, status: &str, ttl_seconds: u64) -> Result<(), String> {
        self.inner.register_presence(agent_id, status, ttl_seconds).await
    }

    async fn get_active_agents(&self) -> Result<Vec<(String, String)>, String> {
        self.inner.get_active_agents().await
    }

    async fn ack(&self, topic: &str, message_id: &str) -> Result<(), String> {
        self.inner.ack(topic, message_id).await
    }
}

pub struct MemoryMeshTransport {
    inner: ohc_builtin_agent::mesh::transport::MemoryTransport,
}

impl MemoryMeshTransport {
    pub fn new() -> Self {
        Self {
            inner: ohc_builtin_agent::mesh::transport::MemoryTransport::new(),
        }
    }
}

#[async_trait]
impl MeshTransport for MemoryMeshTransport {
    async fn publish(&self, topic: &str, message: crate::ohc::orchestration::TeammateMeshEvent) -> Result<(), String> {
        self.inner.publish(topic, message).await
    }

    async fn subscribe(&self, topic: &str, handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        self.inner.subscribe(topic, handler).await
    }

    async fn acquire_lock(&self, resource: &str, owner: &str, ttl_seconds: u64) -> Result<bool, String> {
        self.inner.acquire_lock(resource, owner, ttl_seconds).await
    }

    async fn release_lock(&self, resource: &str, owner: &str) -> Result<(), String> {
        self.inner.release_lock(resource, owner).await
    }

    async fn register_presence(&self, agent_id: &str, status: &str, ttl_seconds: u64) -> Result<(), String> {
        self.inner.register_presence(agent_id, status, ttl_seconds).await
    }

    async fn get_active_agents(&self) -> Result<Vec<(String, String)>, String> {
        self.inner.get_active_agents().await
    }

    async fn ack(&self, topic: &str, message_id: &str) -> Result<(), String> {
        self.inner.ack(topic, message_id).await
    }
}
