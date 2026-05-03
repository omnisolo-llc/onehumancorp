pub mod transport;

use async_trait::async_trait;
use std::sync::Arc;
use crate::mesh::transport::{MeshTransport, Message};

#[async_trait]
pub trait TeammateMesh: Send + Sync {
    async fn publish_task(&self, payload: Vec<u8>) -> Result<(), String>;
    async fn publish_coordination(&self, payload: Vec<u8>) -> Result<(), String>;
    async fn subscribe_tasks(&self, handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String>;
    async fn subscribe_coordination(&self, handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String>;
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
            topic: "mesh:tasks".to_string(),
            payload,
        }).await
    }

    async fn publish_coordination(&self, payload: Vec<u8>) -> Result<(), String> {
        self.transport.publish("mesh:coordination", Message {
            topic: "mesh:coordination".to_string(),
            payload,
        }).await
    }

    async fn subscribe_tasks(&self, handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        self.transport.subscribe("mesh:tasks", handler).await
    }

    async fn subscribe_coordination(&self, handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        self.transport.subscribe("mesh:coordination", handler).await
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
    use std::sync::atomic::{AtomicBool, Ordering};
    use tokio::time::{sleep, Duration};
    use crate::mesh::transport::MemoryTransport;

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
}
