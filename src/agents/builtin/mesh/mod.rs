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
        self.transport.publish("system:state_handoff", Message {
            agent_id: "agent".to_string(),
            action: "system:state_handoff".to_string(),
            status: "ok".to_string(),
            payload,
            msg_id: uuid::Uuid::new_v4().to_string(),
        }).await
    }

    async fn subscribe_state_handoff(&self, handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        self.transport.subscribe("system:state_handoff", handler).await
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
        use prost::Message as ProstMessage;
        let node_id = uuid::Uuid::new_v4().to_string();
        let ping = crate::proto::interop::HealthPing {
            source_node_id: node_id.clone(),
            current_mode: 0,
            timestamp_ms: chrono::Utc::now().timestamp_millis(),
        };
        let mut buf = Vec::new();
        ping.encode(&mut buf).map_err(|e| e.to_string())?;

        let ack_topic = format!("system:health_ack:{}", node_id);
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
        let cancel = self.transport.subscribe(&ack_topic, Box::new(move |_msg| {
            let _ = tx.send(());
        })).await?;

        self.transport.publish("system:health_ping", Message {
            agent_id: "agent".to_string(),
            action: "system:health_ping".to_string(),
            status: "ok".to_string(),
            payload: buf,
            msg_id: uuid::Uuid::new_v4().to_string(),
        }).await?;

        match tokio::time::timeout(std::time::Duration::from_millis(500), rx.recv()).await {
            Ok(Some(_)) => {
                cancel();
                Ok(())
            }
            _ => {
                cancel();
                Err("Health ping timed out waiting for ack".to_string())
            }
        }
    }

    async fn start_health_responder(&self) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        let transport_clone = self.transport.clone();

        self.transport.subscribe("system:health_ping", Box::new(move |msg: Message| {
            use prost::Message as ProstMessage;
            if let Ok(ping) = crate::proto::interop::HealthPing::decode(&msg.payload[..]) {
                let ack_topic = format!("system:health_ack:{}", ping.source_node_id);

                let ack = crate::proto::interop::HealthAck {
                    source_node_id: "builtin_agent".to_string(),
                    target_node_id: ping.source_node_id.clone(),
                    timestamp_ms: chrono::Utc::now().timestamp_millis(),
                };
                let mut buf = Vec::new();
                if ack.encode(&mut buf).is_ok() {
                    let t_clone = transport_clone.clone();
                    let ack_topic_clone = ack_topic.clone();
                    tokio::spawn(async move {
                        let _ = t_clone.publish(&ack_topic_clone, Message {
                            agent_id: "health_responder".to_string(),
                            action: ack_topic_clone.clone(),
                            status: "ok".to_string(),
                            payload: buf,
                            msg_id: uuid::Uuid::new_v4().to_string(),
                        }).await;
                    });
                }
            }
        })).await
    }

    async fn publish_with_ack(&self, topic: &str, payload: Vec<u8>) -> Result<(), String> {
        use prost::Message as ProstMessage;
        let job_id = uuid::Uuid::new_v4().to_string();
        let ack_topic = format!("system:job_ack:{}", job_id);

        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

        let cancel = self.transport.subscribe(&ack_topic, Box::new(move |_msg| {
            let _ = tx.send(());
        })).await?;

        tokio::task::yield_now().await;

        let dispatch = crate::proto::interop::JobDispatch {
            job_id: job_id.clone(),
            tenant_id: "default".to_string(),
            action_name: topic.to_string(),
            payload: payload,
            timestamp_ms: chrono::Utc::now().timestamp_millis(),
        };

        let mut buf = Vec::new();
        dispatch.encode(&mut buf).map_err(|e| e.to_string())?;

        let mut retries = 0;
        let mut backoff = 200;

        loop {
            if retries > 10 {
                cancel();
                return Err("Failed to receive ack after retries".to_string());
            }

            let event = Message {
                agent_id: "agent".to_string(),
                action: topic.to_string(),
                status: "pending".to_string(),
                payload: buf.clone(),
                msg_id: job_id.clone(),
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
            backoff = std::cmp::min(backoff * 2, 2000);
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
    use crate::mesh::transport::InProcessTransport;
    use std::sync::atomic::{AtomicBool, Ordering};
    use tokio::time::{sleep, Duration};

    #[tokio::test]
    async fn test_teammate_mesh_client() {
        let transport: Arc<dyn MeshTransport> = Arc::new(InProcessTransport::new());
        let mesh = TeammateMeshClient::new(transport.clone());

        let tasks_received = Arc::new(AtomicBool::new(false));
        let coord_received = Arc::new(AtomicBool::new(false));

        let tasks_received_clone = tasks_received.clone();
        let coord_received_clone = coord_received.clone();

        let _task_cancel = mesh.subscribe_tasks(Box::new(move |msg| {
            if msg.payload == b"task_data" && msg.action == "mesh:tasks" {
                tasks_received_clone.store(true, Ordering::SeqCst);
            }
        })).await.unwrap();

        let _coord_cancel = mesh.subscribe_coordination(Box::new(move |msg| {
            if msg.payload == b"coord_data" && msg.action == "mesh:coordination" {
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
    async fn test_mesh_tasks_and_coordination_topics() {
        // Double check that the exact topics mesh:tasks and mesh:coordination are published to.
        let transport: Arc<dyn MeshTransport> = Arc::new(InProcessTransport::new());
        let mesh = TeammateMeshClient::new(transport.clone());
        let tasks_received = Arc::new(AtomicBool::new(false));
        let tasks_received_clone = tasks_received.clone();
        let _ = transport.subscribe("mesh:tasks", Box::new(move |_| {
            tasks_received_clone.store(true, Ordering::SeqCst);
        })).await;

        let coord_received = Arc::new(AtomicBool::new(false));
        let coord_received_clone = coord_received.clone();
        let _ = transport.subscribe("mesh:coordination", Box::new(move |_| {
            coord_received_clone.store(true, Ordering::SeqCst);
        })).await;

        mesh.publish_task(b"test".to_vec()).await.unwrap();
        mesh.publish_coordination(b"test".to_vec()).await.unwrap();

        sleep(Duration::from_millis(50)).await;

        assert!(tasks_received.load(Ordering::SeqCst), "Publish to mesh:tasks failed");
        assert!(coord_received.load(Ordering::SeqCst), "Publish to mesh:coordination failed");
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

        assert!(received.load(Ordering::SeqCst), "Fallback InProcessTransport should successfully process messages");
    }

    #[tokio::test]
    async fn test_mesh_acquire_lock() {
        let transport: Arc<dyn MeshTransport> = Arc::new(InProcessTransport::new());
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
        let transport: Arc<dyn MeshTransport> = Arc::new(InProcessTransport::new());
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
        let transport: Arc<dyn MeshTransport> = Arc::new(InProcessTransport::new());
        let mesh = TeammateMeshClient::new(transport);

        let _cancel_responder = mesh.start_health_responder().await.unwrap();

        // Give the responder a moment to subscribe
        sleep(Duration::from_millis(50)).await;

        let result = mesh.ping().await;
        assert!(result.is_ok(), "Ping should receive an ack successfully");
    }

    #[tokio::test]
    async fn test_mesh_state_handoff() {
        let transport: Arc<dyn MeshTransport> = Arc::new(InProcessTransport::new());
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
        let transport: Arc<dyn MeshTransport> = Arc::new(InProcessTransport::new());
        let mesh = TeammateMeshClient::new(transport.clone());

        let transport_clone = transport.clone();
        tokio::spawn(async move {
            let _ = transport_clone.subscribe("test_ack_topic", Box::new({
                let t = transport_clone.clone();
                move |msg: crate::mesh::transport::Message| {
                    use prost::Message as ProstMessage;
                    let dispatch = crate::proto::interop::JobDispatch::decode(&msg.payload[..]).unwrap();
                    let ack_topic = format!("system:job_ack:{}", dispatch.job_id);
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
// dummy validation
