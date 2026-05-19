use async_trait::async_trait;
use ohc_builtin_agent::mesh::transport::{MeshTransport, Message};
use opentelemetry::global;
use opentelemetry::metrics::Counter;
use opentelemetry::trace::{Tracer, TraceContextExt};
use opentelemetry::KeyValue;

pub struct RedisMeshTransport {
    inner: ohc_builtin_agent::mesh::transport::RedisTransport,
    publish_counter: Counter<u64>,
    receive_counter: Counter<u64>,
}

impl RedisMeshTransport {
    pub async fn new(url: &str) -> Result<Self, String> {
        let inner = ohc_builtin_agent::mesh::transport::RedisTransport::new(url).await
            .map_err(|e| format!("Failed to create RedisTransport: {}", e))?;

        let meter = global::meter("ohc.orchestration.hub");
        let publish_counter = meter.u64_counter("hub.messages.published").build();
        let receive_counter = meter.u64_counter("hub.messages.received").build();

        Ok(Self { inner, publish_counter, receive_counter })
    }
}

#[async_trait]
impl MeshTransport for RedisMeshTransport {
    async fn publish(&self, topic: &str, message: ::server_ohc::orchestration::TeammateMeshEvent) -> Result<(), String> {
        let tracer = global::tracer("ohc.orchestration.hub");
        let _span = tracer.start("publish");
        self.publish_counter.add(1, &[KeyValue::new("topic", topic.to_string())]);
        self.inner.publish(topic, message).await
    }

    async fn subscribe(&self, topic: &str, handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        let receive_counter = self.receive_counter.clone();
        let topic_str = topic.to_string();

        let wrapped_handler = Box::new(move |msg: Message| {
            receive_counter.add(1, &[KeyValue::new("topic", topic_str.clone())]);
            handler(msg);
        });

        self.inner.subscribe(topic, wrapped_handler).await
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
}

pub struct MemoryMeshTransport {
    inner: ohc_builtin_agent::mesh::transport::MemoryTransport,
    publish_counter: Counter<u64>,
    receive_counter: Counter<u64>,
}

impl MemoryMeshTransport {
    pub fn new() -> Self {
        let meter = global::meter("ohc.orchestration.hub");
        let publish_counter = meter.u64_counter("hub.messages.published").build();
        let receive_counter = meter.u64_counter("hub.messages.received").build();

        Self {
            inner: ohc_builtin_agent::mesh::transport::MemoryTransport::new(),
            publish_counter,
            receive_counter,
        }
    }
}

#[async_trait]
impl MeshTransport for MemoryMeshTransport {
    async fn publish(&self, topic: &str, message: ::server_ohc::orchestration::TeammateMeshEvent) -> Result<(), String> {
        let tracer = global::tracer("ohc.orchestration.hub");
        let _span = tracer.start("publish");
        self.publish_counter.add(1, &[KeyValue::new("topic", topic.to_string())]);
        self.inner.publish(topic, message).await
    }

    async fn subscribe(&self, topic: &str, handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        let receive_counter = self.receive_counter.clone();
        let topic_str = topic.to_string();

        let wrapped_handler = Box::new(move |msg: Message| {
            receive_counter.add(1, &[KeyValue::new("topic", topic_str.clone())]);
            handler(msg);
        });

        self.inner.subscribe(topic, wrapped_handler).await
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
}
// dummy validation comment

#[cfg(test)]
mod tests {
    use super::*;
    use ::server_ohc::orchestration::TeammateMeshEvent;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::Arc;
    use tokio::time::{sleep, Duration};

    #[tokio::test]
    async fn test_memory_mesh_transport_publish_subscribe() {
        let transport = MemoryMeshTransport::new();
        let received = Arc::new(AtomicBool::new(false));
        let received_clone = received.clone();

        let _cancel = transport.subscribe("test_topic", Box::new(move |msg: Message| {
            if msg.action == "test_action" {
                received_clone.store(true, Ordering::SeqCst);
            }
        })).await.unwrap();

        let msg = TeammateMeshEvent {
            agent_id: "agent_1".to_string(),
            action: "test_action".to_string(),
            status: "ok".to_string(),
            payload: b"test payload".to_vec(),
            msg_id: "msg_1".to_string(),
        };

        transport.publish("test_topic", msg).await.unwrap();

        sleep(Duration::from_millis(50)).await;

        assert!(received.load(Ordering::SeqCst), "Should receive message published via MemoryMeshTransport");
    }

    #[tokio::test]
    async fn test_memory_mesh_transport_locks() {
        let transport = MemoryMeshTransport::new();

        let acquired = transport.acquire_lock("test_resource", "owner_1", 10).await.unwrap();
        assert!(acquired);

        let acquired_again = transport.acquire_lock("test_resource", "owner_2", 10).await.unwrap();
        assert!(!acquired_again);

        transport.release_lock("test_resource", "owner_1").await.unwrap();

        let acquired_after_release = transport.acquire_lock("test_resource", "owner_2", 10).await.unwrap();
        assert!(acquired_after_release);
    }
}
