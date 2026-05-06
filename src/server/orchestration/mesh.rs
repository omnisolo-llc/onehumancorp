use ohc_builtin_agent::mesh::transport::{MeshTransport, Message};
use crate::ohc::orchestration::TeammateMeshEvent;
use opentelemetry::global;
use opentelemetry::metrics::Counter;
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
        self.publish_counter.add(1, &[KeyValue::new("topic", topic.to_string())]);
        self.transport.publish(topic, TeammateMeshEvent {
            agent_id: "sys".to_string(),
            action: topic.to_string(),
            status: "ok".to_string(),
            payload,
            msg_id: uuid::Uuid::new_v4().to_string(),
        }).await
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
        let msg_id = uuid::Uuid::new_v4().to_string();
        let ack_topic = format!("mesh:ack:{}", msg_id);

        let (tx, mut rx) = tokio::sync::mpsc::channel(1);

        let cancel = self.transport.subscribe(&ack_topic, Box::new(move |_msg| {
            let _ = tx.try_send(());
        })).await?;

        let mut retries = 0;
        let mut backoff = 200;

        loop {
            if retries > 5 {
                cancel();
                return Err("Failed to receive ack after retries".to_string());
            }

            let event = TeammateMeshEvent {
                agent_id: "sys".to_string(),
                action: topic.to_string(),
                status: "pending".to_string(),
                payload: payload.clone(),
                msg_id: msg_id.clone(),
            };

            if let Err(e) = self.transport.publish(topic, event).await {
                cancel();
                return Err(e);
            }

            if let Ok(Some(())) = tokio::time::timeout(tokio::time::Duration::from_millis(backoff), rx.recv()).await {
                cancel();
                return Ok(());
            }

            retries += 1;
            backoff *= 2;
        }
    }

    async fn register_presence(&self, agent_id: &str, status: &str, ttl_seconds: u64) -> Result<(), String> {
        self.transport.register_presence(agent_id, status, ttl_seconds).await
    }

    async fn get_active_agents(&self) -> Result<Vec<(String, String)>, String> {
        self.transport.get_active_agents().await
    }

    async fn ping(&self) -> Result<(), String> {
        self.publish_with_ack("mesh:health:ping", b"ping".to_vec()).await
    }

    async fn start_health_responder(&self) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        let transport_clone = self.transport.clone();

        self.transport.subscribe("mesh:health:ping", Box::new(move |msg: Message| {
            let msg_id = msg.msg_id.clone();
            let ack_topic = format!("mesh:ack:{}", msg_id);

            let t_clone = transport_clone.clone();
            tokio::spawn(async move {
                let _ = t_clone.publish(&ack_topic, TeammateMeshEvent {
                    agent_id: "health_responder".to_string(),
                    action: ack_topic.clone(),
                    status: "ok".to_string(),
                    payload: b"pong".to_vec(),
                    msg_id: uuid::Uuid::new_v4().to_string(),
                }).await;
            });
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
            let connect_options = pool.connect_options();
            let db_path = connect_options.get_filename();
            let db_url = format!("sqlite://{}", db_path.to_string_lossy());

            if !db_path.to_string_lossy().is_empty() && db_path.to_string_lossy() != ":memory:" {
                match ohc_builtin_agent::mesh::transport::IpcTransport::new(&db_url, "server_node").await {
                    Ok(transport) => {
                        let t_clone = transport.clone();
                        tokio::spawn(async move { t_clone.start_worker().await; });
                        return Ok(Arc::new(CentrifugeNode::new(Arc::new(transport))));
                    }
                    Err(e) => {
                        return Err(format!("Failed to initialize IpcTransport for Sqlite: {}", e));
                    }
                }
            }

            let transport = ohc_builtin_agent::mesh::transport::MemoryTransport::new();
            Ok(Arc::new(CentrifugeNode::new(Arc::new(transport))))
        }
    }
}
