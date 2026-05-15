use ohc_builtin_agent::mesh::transport::{MeshTransport, Message};

use opentelemetry::global;
use opentelemetry::metrics::Counter;
use opentelemetry::trace::{Tracer, TraceContextExt};
use std::sync::Arc;
use async_trait::async_trait;
use opentelemetry::KeyValue;

#[async_trait]
pub trait TeammateMesh: Send + Sync {
    async fn publish(&self, topic: &str, payload: Vec<u8>) -> Result<(), String>;
    async fn publish_with_ack(&self, topic: &str, payload: Vec<u8>) -> Result<(), String>;
    async fn subscribe(&self, topic: &str, handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String>;

    async fn acquire_lock(&self, resource: &str, owner: &str, ttl_seconds: u64) -> Result<bool, String>;
    async fn release_lock(&self, resource: &str, owner: &str) -> Result<(), String>;

    async fn register_presence(&self, agent_id: &str, status: &str, ttl_seconds: u64) -> Result<(), String>;
    async fn get_active_agents(&self) -> Result<Vec<(String, String)>, String>;

    async fn ping(&self) -> Result<(), String>;
    async fn start_health_responder(&self) -> Result<Box<dyn Fn() + Send + Sync>, String>;

    async fn publish_state_handoff(&self, payload: Vec<u8>) -> Result<(), String>;
    async fn subscribe_state_handoff(&self, handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String>;
}

pub struct CentrifugeNode {
    transport: Arc<dyn MeshTransport>,
    publish_counter: Counter<u64>,
    receive_counter: Counter<u64>,
}

impl CentrifugeNode {
    pub fn new(transport: Arc<dyn MeshTransport>) -> Self {
        let meter = global::meter("ohc.orchestration.mesh");
        let publish_counter = meter.u64_counter("mesh.messages.published").build();
        let receive_counter = meter.u64_counter("mesh.messages.received").build();
        Self { transport, publish_counter, receive_counter }
    }
}

#[async_trait]
impl TeammateMesh for CentrifugeNode {
    async fn publish(&self, topic: &str, payload: Vec<u8>) -> Result<(), String> {
        let tracer = global::tracer("ohc.orchestration.mesh");
        let _span = tracer.start("publish");
        self.publish_counter.add(1, &[KeyValue::new("topic", topic.to_string())]);
        self.transport.publish(topic, ohc_builtin_agent::mesh::transport::Message { topic: topic.to_string(), payload }).await
    }

    async fn subscribe(&self, topic: &str, handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        let receive_counter = self.receive_counter.clone();
        let topic_str = topic.to_string();

        let wrapped_handler = Box::new(move |msg: Message| {
            receive_counter.add(1, &[KeyValue::new("topic", topic_str.clone())]);
            handler(msg);
        });

        self.transport.subscribe(topic, wrapped_handler).await
    }

    async fn acquire_lock(&self, resource: &str, owner: &str, ttl_seconds: u64) -> Result<bool, String> {
        self.transport.acquire_lock(resource, owner, ttl_seconds).await
    }

    async fn release_lock(&self, resource: &str, owner: &str) -> Result<(), String> {
        self.transport.release_lock(resource, owner).await
    }

    async fn publish_with_ack(&self, topic: &str, payload: Vec<u8>) -> Result<(), String> {
        use prost::Message as ProstMessage;
        let job_id = if let Ok(dispatch) = crate::interop::protocol::proto::JobDispatch::decode(&payload[..]) {
            dispatch.job_id
        } else if let Ok(ping) = crate::interop::protocol::proto::HealthPing::decode(&payload[..]) {
            ping.source_node_id
        } else {
            "unknown".to_string()
        };
        let ack_topic = format!("mesh:ack:{}", job_id);

        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

        let cancel = self.transport.subscribe(&ack_topic, Box::new(move |_msg| {
            let _ = tx.send(());
        })).await?;

        let mut retries = 0;
        let mut backoff = 200;

        loop {
            if retries > 10 {
                cancel();
                return Err("Failed to receive ack after retries".to_string());
            }

            let event = ohc_builtin_agent::mesh::transport::Message { topic: topic.to_string(), payload: payload.clone() };

            if let Err(e) = self.transport.publish(topic, event).await {
                cancel();
                return Err(e);
            }

            if let Ok(Some(())) = tokio::time::timeout(tokio::time::Duration::from_millis(backoff), rx.recv()).await {
                cancel();
                return Ok(());
            }

            retries += 1;
            backoff = std::cmp::min(backoff * 2, 2000);
        }
    }

    async fn register_presence(&self, agent_id: &str, status: &str, ttl_seconds: u64) -> Result<(), String> {
        self.transport.register_presence(agent_id, status, ttl_seconds).await
    }

    async fn get_active_agents(&self) -> Result<Vec<(String, String)>, String> {
        self.transport.get_active_agents().await
    }

    async fn ping(&self) -> Result<(), String> {
        use prost::Message as ProstMessage;
        let node_id = uuid::Uuid::new_v4().to_string();
        let ping = crate::interop::protocol::proto::HealthPing {
            source_node_id: node_id.clone(),
            current_mode: 0,
            timestamp_ms: chrono::Utc::now().timestamp_millis(),
        };
        let mut buf = Vec::new();
        ping.encode(&mut buf).unwrap();

        let ack_topic = format!("mesh:ack:{}", node_id);
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
        let cancel = self.transport.subscribe(&ack_topic, Box::new(move |_msg| {
            let _ = tx.send(());
        })).await?;

        let msg = ohc_builtin_agent::mesh::transport::Message { topic: "mesh:health:ping".to_string(), payload: buf };
        self.transport.publish("mesh:health:ping", msg).await?;

        match tokio::time::timeout(std::time::Duration::from_millis(500), rx.recv()).await {
            Ok(Some(_)) => { cancel(); Ok(()) }
            _ => { cancel(); Err("Timeout".to_string()) }
        }
    }

    async fn start_health_responder(&self) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        let transport_clone = self.transport.clone();

        self.transport.subscribe("mesh:health:ping", Box::new(move |msg: Message| {
            use prost::Message as ProstMessage;
            if let Ok(ping) = crate::interop::protocol::proto::HealthPing::decode(&msg.payload[..]) {
                let ack_topic = format!("mesh:ack:{}", ping.source_node_id);
                let t_clone = transport_clone.clone();
                tokio::spawn(async move {
                    let _ = t_clone.publish(&ack_topic, ohc_builtin_agent::mesh::transport::Message { topic: ack_topic.clone(), payload: b"pong".to_vec() }).await;
                });
            }
        })).await
    }

    async fn publish_state_handoff(&self, payload: Vec<u8>) -> Result<(), String> {
        self.publish("mesh:state:handoff", payload).await
    }

    async fn subscribe_state_handoff(&self, handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        self.subscribe("mesh:state:handoff", handler).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent::mesh::transport::MemoryTransport;
    use std::sync::atomic::{AtomicBool, Ordering};
    use tokio::time::{sleep, Duration};

    #[tokio::test]
    async fn test_centrifuge_node_pubsub() {
        let transport: Arc<dyn MeshTransport> = Arc::new(MemoryTransport::new());
        let node = CentrifugeNode::new(transport);

        let received = Arc::new(AtomicBool::new(false));
        let received_clone = received.clone();

        let _cancel = node.subscribe("test_topic", Box::new(move |msg: Message| {
            if msg.payload == b"hello world" {
                received_clone.store(true, Ordering::SeqCst);
            }
        })).await.unwrap();

        node.publish("test_topic", b"hello world".to_vec()).await.unwrap();

        sleep(Duration::from_millis(50)).await;

        assert!(received.load(Ordering::SeqCst), "Should receive message published via CentrifugeNode");
    }

    #[tokio::test]
    async fn test_mesh_acquire_lock() {
        let transport: Arc<dyn MeshTransport> = Arc::new(MemoryTransport::new());
        let node = CentrifugeNode::new(transport);

        let acquired = node.acquire_lock("test_resource", "agent_1", 10).await.unwrap();
        assert!(acquired);

        let acquired_again = node.acquire_lock("test_resource", "agent_2", 10).await.unwrap();
        assert!(!acquired_again);

        node.release_lock("test_resource", "agent_1").await.unwrap();

        let acquired_after_release = node.acquire_lock("test_resource", "agent_2", 10).await.unwrap();
        assert!(acquired_after_release);
    }

    #[tokio::test]
    async fn test_mesh_register_presence() {
        let transport: Arc<dyn MeshTransport> = Arc::new(MemoryTransport::new());
        let node = CentrifugeNode::new(transport);

        node.register_presence("agent_1", "online", 10).await.unwrap();
        node.register_presence("agent_2", "busy", 10).await.unwrap();

        let mut agents = node.get_active_agents().await.unwrap();
        agents.sort();

        assert_eq!(agents.len(), 2);
        assert_eq!(agents[0], ("agent_1".to_string(), "online".to_string()));
        assert_eq!(agents[1], ("agent_2".to_string(), "busy".to_string()));
    }

    #[tokio::test]
    async fn test_mesh_ping_pong() {
        let transport: Arc<dyn MeshTransport> = Arc::new(MemoryTransport::new());
        let node = CentrifugeNode::new(transport);

        let _cancel_responder = node.start_health_responder().await.unwrap();

        // Give the responder a moment to subscribe
        sleep(Duration::from_millis(50)).await;

        let result = node.ping().await;
        assert!(result.is_ok(), "Ping should receive an ack successfully");
    }

    #[tokio::test]
    async fn test_mesh_state_handoff() {
        let transport: Arc<dyn MeshTransport> = Arc::new(MemoryTransport::new());
        let node = CentrifugeNode::new(transport);

        let received = Arc::new(AtomicBool::new(false));
        let received_clone = received.clone();

        let _cancel = node.subscribe_state_handoff(Box::new(move |msg| {
            if msg.payload == b"state_data" {
                received_clone.store(true, Ordering::SeqCst);
            }
        })).await.unwrap();

        node.publish_state_handoff(b"state_data".to_vec()).await.unwrap();
        sleep(Duration::from_millis(50)).await;

        assert!(received.load(Ordering::SeqCst), "Should receive state handoff message");
    }
    #[tokio::test]
    async fn test_get_mesh_transport_sqlite_memory() {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();
        let db_store = crate::db::DbStore::Sqlite(pool);

        let mesh_res = super::get_mesh_transport(&db_store).await;
        assert!(mesh_res.is_ok());
    }

    #[tokio::test]
    async fn test_get_mesh_transport_sqlite_file() {
        if std::env::var("NATS_URL").is_ok() {
            return;
        }

        let tmp_dir = std::env::var("TEST_TMPDIR").unwrap_or_else(|_| "/tmp".to_string());
        let db_path = format!("{}/test_mesh_ipc_file_{}.sqlite", tmp_dir, std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos());
        let db_url = format!("sqlite://{}", db_path);

        std::fs::File::create(&db_path).unwrap();

        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .connect(&db_url)
            .await
            .unwrap();

        let db_store = crate::db::DbStore::Sqlite(pool);

        let mesh_res = super::get_mesh_transport(&db_store).await;
        assert!(mesh_res.is_ok());
    }
}


pub async fn get_mesh_transport(db_store: &crate::db::DbStore) -> Result<Arc<dyn TeammateMesh>, String> {
    if let Ok(nats_url) = std::env::var("NATS_URL") {
        if let Ok(transport) = ohc_builtin_agent::mesh::transport::NatsTransport::new(&nats_url).await {
            return Ok(Arc::new(CentrifugeNode::new(Arc::new(transport))));
        }
    }

    match db_store {
        crate::db::DbStore::Postgres => {
            let redis_url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://localhost:6379".to_string());
            let transport = ohc_builtin_agent::mesh::transport::RedisTransport::new(&redis_url).await
                .map_err(|e| format!("Failed to create RedisTransport: {}", e))?;
            Ok(Arc::new(CentrifugeNode::new(Arc::new(transport))))
        }
        crate::db::DbStore::Sqlite(pool) => {
            if let Ok(pg_url) = std::env::var("DATABASE_URL") {
                if pg_url.starts_with("postgres://") || pg_url.starts_with("postgresql://") {
                    match ohc_builtin_agent::mesh::transport::PgTransport::new(&pg_url).await {
                        Ok(transport) => {
                            let t_clone = transport.clone();
                            tokio::spawn(async move { t_clone.start_worker().await; });
                            return Ok(Arc::new(CentrifugeNode::new(Arc::new(transport))));
                        }
                        Err(e) => {
                            tracing::error!("Failed to initialize PgTransport fallback: {}", e);
                            // Fallback to memory
                        }
                    }
                }
            }

            match ohc_builtin_agent::mesh::transport::SqliteTransport::new(pool.clone()).await {
                Ok(transport) => {
                    let t_clone = transport.clone();
                    tokio::spawn(async move { t_clone.start_worker().await; });
                    Ok(Arc::new(CentrifugeNode::new(Arc::new(transport))))
                }
                Err(e) => {
                    tracing::error!("Failed to initialize SqliteTransport: {}. Falling back to MemoryTransport.", e);
                    let transport = ohc_builtin_agent::mesh::transport::MemoryTransport::new();
                    Ok(Arc::new(CentrifugeNode::new(Arc::new(transport))))
                }
            }
        }
    }
}
// dummy validation comment
