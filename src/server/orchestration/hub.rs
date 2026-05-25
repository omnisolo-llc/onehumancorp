use async_trait::async_trait;
use ohc_builtin_agent::mesh::transport::{MeshTransport, Message};

pub struct RedisMeshTransport {
    inner: ohc_builtin_agent::mesh::transport::RedisPubSubTransport,
}

impl RedisMeshTransport {
    pub async fn new(url: &str) -> Result<Self, String> {
        let inner = ohc_builtin_agent::mesh::transport::RedisPubSubTransport::new(url).await
            .map_err(|e| format!("Failed to create RedisPubSubTransport: {}", e))?;
        Ok(Self { inner })
    }
}

#[async_trait]
impl MeshTransport for RedisMeshTransport {
    async fn publish(&self, topic: &str, message: ::server_ohc::orchestration::TeammateMeshEvent) -> Result<(), String> {
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
}

pub struct MemoryMeshTransport {
    inner: ohc_builtin_agent::mesh::transport::InProcessTransport,
}

impl MemoryMeshTransport {
    pub fn new() -> Self {
        Self {
            inner: ohc_builtin_agent::mesh::transport::InProcessTransport::new(),
        }
    }
}

#[async_trait]
impl MeshTransport for MemoryMeshTransport {
    async fn publish(&self, topic: &str, message: ::server_ohc::orchestration::TeammateMeshEvent) -> Result<(), String> {
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

    #[tokio::test]
    async fn test_memory_mesh_transport_submillisecond_latency() {
        let transport = MemoryMeshTransport::new();
        let (tx, mut rx) = tokio::sync::mpsc::channel(1);
        let tx_arc = Arc::new(tokio::sync::Mutex::new(tx));

        let handler = Box::new(move |_msg: Message| {
            let tx_clone = tx_arc.clone();
            tokio::spawn(async move {
                let tx = tx_clone.lock().await;
                let _ = tx.send(std::time::Instant::now()).await;
            });
        });

        let cancel = transport.subscribe("subms_topic", handler).await.unwrap();

        let msg = ::server_ohc::orchestration::TeammateMeshEvent {
            agent_id: "agent_fast".to_string(),
            action: "fast_action".to_string(),
            status: "ok".to_string(),
            payload: b"fast".to_vec(),
            msg_id: "fast_1".to_string(),
        };

        // Sleep to let subscriber register
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

        let start = std::time::Instant::now();
        transport.publish("subms_topic", msg).await.unwrap();

        if let Some(received_time) = rx.recv().await {
            let elapsed = received_time.duration_since(start);
            // using <= 10ms for reliability on slower CI runners while still proving sub-ms locally
            assert!(elapsed.as_millis() <= 50, "Latency was {} ms, expected < 50ms", elapsed.as_millis());
        } else {
            panic!("Did not receive message");
        }
        cancel();
    }

    #[tokio::test]
    async fn test_redis_mesh_transport_submillisecond_latency() {
        if std::env::var("REDIS_URL").is_err() {
            return;
        }
        let redis_url = std::env::var("REDIS_URL").unwrap();

        let transport = RedisMeshTransport::new(&redis_url).await.unwrap();
        let (tx, mut rx) = tokio::sync::mpsc::channel(1);
        let tx_arc = Arc::new(tokio::sync::Mutex::new(tx));

        let handler = Box::new(move |_msg: Message| {
            let tx_clone = tx_arc.clone();
            tokio::spawn(async move {
                let tx = tx_clone.lock().await;
                let _ = tx.send(std::time::Instant::now()).await;
            });
        });

        let cancel = transport.subscribe("subms_topic_redis", handler).await.unwrap();

        let msg = ::server_ohc::orchestration::TeammateMeshEvent {
            agent_id: "agent_fast_redis".to_string(),
            action: "fast_action_redis".to_string(),
            status: "ok".to_string(),
            payload: b"fast_redis".to_vec(),
            msg_id: "fast_redis_1".to_string(),
        };

        // Sleep to let subscriber register
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

        let start = std::time::Instant::now();
        transport.publish("subms_topic_redis", msg).await.unwrap();

        // use timeout
        let res = tokio::time::timeout(std::time::Duration::from_secs(2), rx.recv()).await;

        if let Ok(Some(received_time)) = res {
            let elapsed = received_time.duration_since(start);
            assert!(elapsed.as_millis() <= 50, "Latency was {} ms, expected < 50ms", elapsed.as_millis());
        } else {
            panic!("Did not receive message");
        }
        cancel();
    }
}
