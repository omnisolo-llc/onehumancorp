use async_trait::async_trait;
use ohc_builtin_agent::mesh::transport::{MeshTransport, Message};
use opentelemetry::global;
use opentelemetry::trace::Tracer;
use opentelemetry::metrics::Counter;
use opentelemetry::KeyValue;

pub struct RedisMeshTransport {
    inner: ohc_builtin_agent::mesh::transport::RedisPubSubTransport,
    publish_counter: Counter<u64>,
    receive_counter: Counter<u64>,
}

impl RedisMeshTransport {
    pub async fn new(url: &str) -> Result<Self, String> {
        let inner = ohc_builtin_agent::mesh::transport::RedisPubSubTransport::new(url).await
            .map_err(|e| format!("Failed to create RedisPubSubTransport: {}", e))?;
        let meter = global::meter("ohc.orchestration.hub");
        let publish_counter = meter.u64_counter("mesh.redis.publish.count").build();
        let receive_counter = meter.u64_counter("mesh.redis.receive.count").build();
        Ok(Self { inner, publish_counter, receive_counter })
    }
}

#[async_trait]
impl MeshTransport for RedisMeshTransport {
    async fn publish(&self, topic: &str, message: ::server_ohc::orchestration::TeammateMeshEvent) -> Result<(), String> {
        let tracer = global::tracer("ohc.orchestration.hub");
        let _span = tracer.start("redis_publish");
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
    inner: ohc_builtin_agent::mesh::transport::InProcessTransport,
    publish_counter: Counter<u64>,
    receive_counter: Counter<u64>,
}

impl MemoryMeshTransport {
    pub fn new() -> Self {
        let meter = global::meter("ohc.orchestration.hub");
        let publish_counter = meter.u64_counter("mesh.memory.publish.count").build();
        let receive_counter = meter.u64_counter("mesh.memory.receive.count").build();
        Self {
            inner: ohc_builtin_agent::mesh::transport::InProcessTransport::new(),
            publish_counter,
            receive_counter,
        }
    }
}

#[async_trait]
impl MeshTransport for MemoryMeshTransport {
    async fn publish(&self, topic: &str, message: ::server_ohc::orchestration::TeammateMeshEvent) -> Result<(), String> {
        let tracer = global::tracer("ohc.orchestration.hub");
        let _span = tracer.start("memory_publish");
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


#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent::mesh::transport::Message;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    #[tokio::test]
    async fn test_memory_mesh_transport_pubsub() {
        let transport = MemoryMeshTransport::new();
        let received = Arc::new(Mutex::new(Vec::new()));
        let received_clone = received.clone();

        let handler = Box::new(move |msg: Message| {
            let received = received_clone.clone();
            tokio::spawn(async move {
                received.lock().await.push(msg);
            });
        });

        let cancel = transport.subscribe("test_topic", handler).await.unwrap();

        let msg = ::server_ohc::orchestration::TeammateMeshEvent {
            agent_id: "agent_1".to_string(),
            action: "action_1".to_string(),
            status: "ok".to_string(),
            payload: b"hello".to_vec(),
            msg_id: "msg_1".to_string(),
        };

        transport.publish("test_topic", msg.clone()).await.unwrap();

        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

        let locked = received.lock().await;
        assert_eq!(locked.len(), 1);
        assert_eq!(locked[0].msg_id, "msg_1");
        drop(locked);

        cancel();
    }

    #[tokio::test]
    async fn test_memory_mesh_transport_locking() {
        let transport = MemoryMeshTransport::new();
        let resource_name = format!("test_resource_{}", uuid::Uuid::new_v4());

        let acq1 = transport.acquire_lock(&resource_name, "agent_1", 10).await.unwrap();
        assert!(acq1);

        let acq2 = transport.acquire_lock(&resource_name, "agent_2", 10).await.unwrap();
        assert!(!acq2);

        transport.release_lock(&resource_name, "agent_1").await.unwrap();

        let acq3 = transport.acquire_lock(&resource_name, "agent_2", 10).await.unwrap();
        assert!(acq3);
    }

    #[tokio::test]
    async fn test_memory_mesh_transport_presence() {
        let transport = MemoryMeshTransport::new();

        transport.register_presence("agent_1", "online", 10).await.unwrap();
        transport.register_presence("agent_2", "busy", 10).await.unwrap();

        let mut agents = transport.get_active_agents().await.unwrap();
        agents.sort();

        assert_eq!(agents.len(), 2);
        assert_eq!(agents[0], ("agent_1".to_string(), "online".to_string()));
        assert_eq!(agents[1], ("agent_2".to_string(), "busy".to_string()));
    }

    #[tokio::test]
    async fn test_redis_mesh_transport_pubsub() {
        if std::env::var("REDIS_URL").is_err() {
            return;
        }
        let redis_url = std::env::var("REDIS_URL").unwrap();

        let transport_res = RedisMeshTransport::new(&redis_url).await;
        let transport = transport_res.unwrap();
        let received = Arc::new(Mutex::new(Vec::new()));
        let received_clone = received.clone();

        let handler = Box::new(move |msg: Message| {
            let received = received_clone.clone();
            tokio::spawn(async move {
                received.lock().await.push(msg);
            });
        });

        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

        let cancel = transport.subscribe("test_topic_redis", handler).await.unwrap();

        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        let msg = ::server_ohc::orchestration::TeammateMeshEvent {
            agent_id: "agent_1".to_string(),
            action: "action_redis".to_string(),
            status: "ok".to_string(),
            payload: b"hello redis".to_vec(),
            msg_id: "msg_redis_1".to_string(),
        };

        transport.publish("test_topic_redis", msg.clone()).await.unwrap();

        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        let locked = received.lock().await;
        assert!(locked.len() >= 1);
        let found = locked.iter().any(|m| m.msg_id == "msg_redis_1");
        assert!(found);
        drop(locked);

        cancel();
    }

    #[tokio::test]
    async fn test_redis_mesh_transport_locking() {
        if std::env::var("REDIS_URL").is_err() {
            return;
        }
        let redis_url = std::env::var("REDIS_URL").unwrap();

        let transport = RedisMeshTransport::new(&redis_url).await.unwrap();

        let acq1 = transport.acquire_lock("test_resource_redis", "agent_1", 10).await.unwrap();
        assert!(acq1);

        let acq2 = transport.acquire_lock("test_resource_redis", "agent_2", 10).await.unwrap();
        assert!(!acq2);

        transport.release_lock("test_resource_redis", "agent_1").await.unwrap();

        let acq3 = transport.acquire_lock("test_resource_redis", "agent_2", 10).await.unwrap();
        assert!(acq3);
    }

    #[tokio::test]
    async fn test_redis_mesh_transport_presence() {
        if std::env::var("REDIS_URL").is_err() {
            return;
        }
        let redis_url = std::env::var("REDIS_URL").unwrap();

        let transport = RedisMeshTransport::new(&redis_url).await.unwrap();

        transport.register_presence("agent_redis_1", "online", 10).await.unwrap();
        transport.register_presence("agent_redis_2", "busy", 10).await.unwrap();

        let mut agents = transport.get_active_agents().await.unwrap();

        // Filter agents because other tests might run in parallel
        agents.retain(|(id, _)| id == "agent_redis_1" || id == "agent_redis_2");
        agents.sort();

        assert_eq!(agents.len(), 2);
        assert_eq!(agents[0], ("agent_redis_1".to_string(), "online".to_string()));
        assert_eq!(agents[1], ("agent_redis_2".to_string(), "busy".to_string()));
    }
}
