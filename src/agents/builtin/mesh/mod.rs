pub mod transport;

use async_trait::async_trait;
use std::sync::Arc;
use crate::mesh::transport::{MeshTransport, Message};

#[async_trait]
pub trait TeammateMesh: Send + Sync {
    async fn publish_task(&self, payload: Vec<u8>) -> Result<(), String>;
    async fn publish_coordination(&self, payload: Vec<u8>) -> Result<(), String>;
    async fn publish_with_ack(&self, topic: &str, payload: Vec<u8>) -> Result<(), String>;
    async fn subscribe_tasks(&self, handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String>;
    async fn subscribe_coordination(&self, handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String>;

    async fn acquire_lock(&self, resource: &str, owner: &str, ttl_seconds: u64) -> Result<bool, String>;
    async fn release_lock(&self, resource: &str, owner: &str) -> Result<(), String>;

    async fn register_presence(&self, agent_id: &str, status: &str, ttl_seconds: u64) -> Result<(), String>;
    async fn get_active_agents(&self) -> Result<Vec<(String, String)>, String>;

    async fn ping(&self) -> Result<(), String>;
    async fn start_health_responder(&self) -> Result<Box<dyn Fn() + Send + Sync>, String>;

    async fn publish_state_handoff(&self, payload: Vec<u8>) -> Result<(), String>;
    async fn subscribe_state_handoff(&self, handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String>;
}

pub struct TeammateMeshClient {
    transport: Arc<dyn MeshTransport>,
}

impl TeammateMeshClient {
    pub fn new(transport: Arc<dyn MeshTransport>) -> Self {
        TeammateMeshClient { transport }
    }
}

#[async_trait]
impl TeammateMesh for TeammateMeshClient {
    async fn publish_task(&self, payload: Vec<u8>) -> Result<(), String> {
        self.transport.publish("mesh:tasks", Message {
            agent_id: "agent".to_string(),
            action: "mesh:tasks".to_string(),
            status: "ok".to_string(),
            payload,
            msg_id: uuid::Uuid::new_v4().to_string(),
        }).await
    }

    async fn publish_coordination(&self, payload: Vec<u8>) -> Result<(), String> {
        self.transport.publish("mesh:coordination", Message {
            agent_id: "agent".to_string(),
            action: "mesh:coordination".to_string(),
            status: "ok".to_string(),
            payload,
            msg_id: uuid::Uuid::new_v4().to_string(),
        }).await
    }

    async fn subscribe_tasks(&self, handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        self.transport.subscribe("mesh:tasks", handler).await
    }

    async fn subscribe_coordination(&self, handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        self.transport.subscribe("mesh:coordination", handler).await
    }

    async fn publish_state_handoff(&self, payload: Vec<u8>) -> Result<(), String> {
        self.transport.publish("mesh:state:handoff", Message {
            agent_id: "agent".to_string(),
            action: "mesh:state:handoff".to_string(),
            status: "ok".to_string(),
            payload,
            msg_id: uuid::Uuid::new_v4().to_string(),
        }).await
    }

    async fn subscribe_state_handoff(&self, handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        self.transport.subscribe("mesh:state:handoff", handler).await
    }

    async fn acquire_lock(&self, resource: &str, owner: &str, ttl_seconds: u64) -> Result<bool, String> {
        self.transport.acquire_lock(resource, owner, ttl_seconds).await
    }

    async fn release_lock(&self, resource: &str, owner: &str) -> Result<(), String> {
        self.transport.release_lock(resource, owner).await
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
                let _ = t_clone.publish(&ack_topic, Message {
                    agent_id: "health_responder".to_string(),
                    action: ack_topic.clone(),
                    status: "ok".to_string(),
                    payload: b"pong".to_vec(),
                    msg_id: uuid::Uuid::new_v4().to_string(),
                }).await;
            });
        })).await
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

            let event = Message {
                agent_id: "agent".to_string(),
                action: topic.to_string(),
                status: "pending".to_string(),
                payload: payload.clone(),
                msg_id: msg_id.clone(),
            };

            // In a real implementation we would attach the msg_id to the event,
            // but the proto might not have it. Let's send it anyway.
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

}

pub async fn create_teammate_mesh(redis_url: Option<&str>, is_cloud: bool) -> Result<Arc<dyn TeammateMesh>, String> {
    match crate::mesh::transport::create_transport(redis_url, is_cloud).await {
        Ok(transport) => {
            Ok(Arc::new(TeammateMeshClient::new(transport)))
        }
        Err(e) => {
            Err(format!("Failed to create TeammateMesh: {}", e))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mesh::transport::MemoryTransport;
    use std::sync::atomic::{AtomicBool, Ordering};
    use tokio::time::{sleep, Duration};

    #[tokio::test]
    async fn test_teammate_mesh_client() {
        let transport: Arc<dyn MeshTransport> = Arc::new(MemoryTransport::new());
        let mesh = TeammateMeshClient::new(transport);

        let tasks_received = Arc::new(AtomicBool::new(false));
        let coord_received = Arc::new(AtomicBool::new(false));

        let tasks_received_clone = tasks_received.clone();
        let coord_received_clone = coord_received.clone();

        let _task_cancel = mesh.subscribe_tasks(Box::new(move |msg| {
            if msg.payload == b"task_data" {
                tasks_received_clone.store(true, Ordering::SeqCst);
            }
        })).await.unwrap();

        let _coord_cancel = mesh.subscribe_coordination(Box::new(move |msg| {
            if msg.payload == b"coord_data" {
                coord_received_clone.store(true, Ordering::SeqCst);
            }
        })).await.unwrap();

        mesh.publish_task(b"task_data".to_vec()).await.unwrap();
        mesh.publish_coordination(b"coord_data".to_vec()).await.unwrap();

        sleep(Duration::from_millis(50)).await;

        assert!(tasks_received.load(Ordering::SeqCst), "Should receive task message");
        assert!(coord_received.load(Ordering::SeqCst), "Should receive coordination message");
    }

    #[tokio::test]
    async fn test_create_teammate_mesh_fallback() {
        // Test fallback behavior with an invalid Redis URL
        // We set is_cloud to false to allow fallback to work (since we don't have a valid redis and we aren't cloud)
        let mesh = create_teammate_mesh(Some("redis://invalid-host:9999"), false).await.unwrap();

        let received = Arc::new(AtomicBool::new(false));
        let received_clone = received.clone();

        let _cancel = mesh.subscribe_tasks(Box::new(move |msg| {
            if msg.payload == b"test" {
                received_clone.store(true, Ordering::SeqCst);
            }
        })).await.unwrap();

        mesh.publish_task(b"test".to_vec()).await.unwrap();
        sleep(Duration::from_millis(50)).await;

        assert!(received.load(Ordering::SeqCst), "Fallback MemoryTransport should successfully process messages");
    }

    #[tokio::test]
    async fn test_mesh_acquire_lock() {
        let transport: Arc<dyn MeshTransport> = Arc::new(MemoryTransport::new());
        let mesh = TeammateMeshClient::new(transport);

        let acquired = mesh.acquire_lock("test_resource", "agent_1", 10).await.unwrap();
        assert!(acquired);

        let acquired_again = mesh.acquire_lock("test_resource", "agent_2", 10).await.unwrap();
        assert!(!acquired_again);

        mesh.release_lock("test_resource", "agent_1").await.unwrap();

        let acquired_after_release = mesh.acquire_lock("test_resource", "agent_2", 10).await.unwrap();
        assert!(acquired_after_release);
    }

    #[tokio::test]
    async fn test_mesh_register_presence() {
        let transport: Arc<dyn MeshTransport> = Arc::new(MemoryTransport::new());
        let mesh = TeammateMeshClient::new(transport);

        mesh.register_presence("agent_1", "online", 10).await.unwrap();
        mesh.register_presence("agent_2", "busy", 10).await.unwrap();

        let mut agents = mesh.get_active_agents().await.unwrap();
        agents.sort();

        assert_eq!(agents.len(), 2);
        assert_eq!(agents[0], ("agent_1".to_string(), "online".to_string()));
        assert_eq!(agents[1], ("agent_2".to_string(), "busy".to_string()));
    }

    #[tokio::test]
    async fn test_mesh_ping_pong() {
        let transport: Arc<dyn MeshTransport> = Arc::new(MemoryTransport::new());
        let mesh = TeammateMeshClient::new(transport);

        let _cancel_responder = mesh.start_health_responder().await.unwrap();

        // Give the responder a moment to subscribe
        sleep(Duration::from_millis(50)).await;

        let result = mesh.ping().await;
        assert!(result.is_ok(), "Ping should receive an ack successfully");
    }

    #[tokio::test]
    async fn test_mesh_state_handoff() {
        let transport: Arc<dyn MeshTransport> = Arc::new(MemoryTransport::new());
        let mesh = TeammateMeshClient::new(transport);

        let received = Arc::new(AtomicBool::new(false));
        let received_clone = received.clone();

        let _cancel = mesh.subscribe_state_handoff(Box::new(move |msg| {
            if msg.payload == b"state_data" {
                received_clone.store(true, Ordering::SeqCst);
            }
        })).await.unwrap();

        mesh.publish_state_handoff(b"state_data".to_vec()).await.unwrap();
        sleep(Duration::from_millis(50)).await;

        assert!(received.load(Ordering::SeqCst), "Should receive state handoff message");
    }

    #[tokio::test]
    async fn test_mesh_publish_with_ack() {
        let transport: Arc<dyn MeshTransport> = Arc::new(MemoryTransport::new());
        let mesh = TeammateMeshClient::new(transport.clone());

        let transport_clone = transport.clone();
        tokio::spawn(async move {
            let _ = transport_clone.subscribe("test_ack_topic", Box::new({
                let t = transport_clone.clone();
                move |msg: crate::mesh::transport::Message| {
                    let msg_id = msg.msg_id.clone();
                    let ack_topic = format!("mesh:ack:{}", msg_id);
                    let t_clone = t.clone();
                    tokio::spawn(async move {
                        let _ = t_clone.publish(&ack_topic, crate::mesh::transport::Message {
                            agent_id: "test".to_string(),
                            action: ack_topic.clone(),
                            status: "ok".to_string(),
                            payload: b"ack".to_vec(),
                            msg_id: uuid::Uuid::new_v4().to_string(),
                        }).await;
                    });
                }
            })).await;
        });

        sleep(Duration::from_millis(50)).await;
        let result = mesh.publish_with_ack("test_ack_topic", b"payload".to_vec()).await;
        assert!(result.is_ok());
    }
}
