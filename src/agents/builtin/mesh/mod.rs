pub mod transport;

use async_trait::async_trait;
use std::sync::Arc;
use crate::mesh::transport::{MeshTransport, Message, MemoryTransport, RedisTransport};
use tracing::{info, warn};

#[async_trait]
pub trait TeammateMesh: Send + Sync {
    async fn publish_task(&self, payload: Vec<u8>) -> Result<(), String>;
    async fn publish_coordination(&self, payload: Vec<u8>) -> Result<(), String>;
    async fn subscribe_tasks(&self, handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String>;
    async fn subscribe_coordination(&self, handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String>;
}






pub struct TeammateMeshClient {
    transport: tokio::sync::RwLock<Arc<dyn MeshTransport>>,
    fallback_transport: Arc<dyn MeshTransport>,
    task_handlers: tokio::sync::RwLock<Vec<Arc<dyn Fn(Message) + Send + Sync>>>,
    coord_handlers: tokio::sync::RwLock<Vec<Arc<dyn Fn(Message) + Send + Sync>>>,
}

impl TeammateMeshClient {
    pub fn new(transport: Arc<dyn MeshTransport>, fallback_transport: Arc<dyn MeshTransport>) -> Self {
        TeammateMeshClient {
            transport: tokio::sync::RwLock::new(transport),
            fallback_transport,
            task_handlers: tokio::sync::RwLock::new(Vec::new()),
            coord_handlers: tokio::sync::RwLock::new(Vec::new()),
        }
    }

    pub async fn switch_to_fallback(&self) {
        tracing::warn!("TeammateMeshClient: Switching to local fallback transport due to cloud disconnect.");
        let mut t = self.transport.write().await;
        *t = self.fallback_transport.clone();

        let th = self.task_handlers.read().await;
        for handler in th.iter() {
            let h = handler.clone();
            let _ = self.fallback_transport.subscribe("mesh:tasks", Box::new(move |m| h(m))).await;
        }

        let ch = self.coord_handlers.read().await;
        for handler in ch.iter() {
            let h = handler.clone();
            let _ = self.fallback_transport.subscribe("mesh:coordination", Box::new(move |m| h(m))).await;
        }
    }
}

#[async_trait]
impl TeammateMesh for TeammateMeshClient {
    async fn publish_task(&self, payload: Vec<u8>) -> Result<(), String> {
        let t = self.transport.read().await.clone();
        if let Err(e) = t.publish("mesh:tasks", Message {
            topic: "mesh:tasks".to_string(),
            payload: payload.clone(),
        }).await {
            tracing::warn!("TeammateMeshClient: Failed to publish task, falling back. Error: {}", e);
            self.switch_to_fallback().await;
            self.fallback_transport.publish("mesh:tasks", Message {
                topic: "mesh:tasks".to_string(),
                payload,
            }).await
        } else {
            Ok(())
        }
    }

    async fn publish_coordination(&self, payload: Vec<u8>) -> Result<(), String> {
        let t = self.transport.read().await.clone();
        if let Err(e) = t.publish("mesh:coordination", Message {
            topic: "mesh:coordination".to_string(),
            payload: payload.clone(),
        }).await {
            tracing::warn!("TeammateMeshClient: Failed to publish coordination, falling back. Error: {}", e);
            self.switch_to_fallback().await;
            self.fallback_transport.publish("mesh:coordination", Message {
                topic: "mesh:coordination".to_string(),
                payload,
            }).await
        } else {
            Ok(())
        }
    }

    async fn subscribe_tasks(&self, handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        let arc_handler: Arc<dyn Fn(Message) + Send + Sync> = Arc::from(handler);
        let mut th = self.task_handlers.write().await;
        th.push(arc_handler.clone());

        let t = self.transport.read().await.clone();
        let h = arc_handler.clone();
        t.subscribe("mesh:tasks", Box::new(move |m| h(m))).await
    }

    async fn subscribe_coordination(&self, handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        let arc_handler: Arc<dyn Fn(Message) + Send + Sync> = Arc::from(handler);
        let mut ch = self.coord_handlers.write().await;
        ch.push(arc_handler.clone());

        let t = self.transport.read().await.clone();
        let h = arc_handler.clone();
        t.subscribe("mesh:coordination", Box::new(move |m| h(m))).await
    }
}



pub async fn create_teammate_mesh(redis_url: Option<&str>) -> Arc<dyn TeammateMesh> {
    let fallback = Arc::new(MemoryTransport::new()) as Arc<dyn MeshTransport>;

    if let Some(url) = redis_url {
        match RedisTransport::new(url).await {
            Ok(redis_transport) => {
                tracing::info!("Successfully connected to Redis for TeammateMesh.");
                return Arc::new(TeammateMeshClient::new(Arc::new(redis_transport) as Arc<dyn MeshTransport>, fallback));
            }
            Err(e) => {
                tracing::warn!("Failed to connect to Redis for TeammateMesh: {}. Falling back to MemoryTransport.", e);
            }
        }
    } else {
        tracing::info!("No Redis URL provided for TeammateMesh. Using MemoryTransport.");
    }

    Arc::new(TeammateMeshClient::new(Arc::new(MemoryTransport::new()) as Arc<dyn MeshTransport>, fallback))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicBool, Ordering};
    use tokio::time::{sleep, Duration};

    #[tokio::test]
    async fn test_teammate_mesh_client() {
        let transport: Arc<dyn MeshTransport> = Arc::new(MemoryTransport::new());
        let mesh = TeammateMeshClient::new(transport, Arc::new(MemoryTransport::new()) as Arc<dyn MeshTransport>);

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
        let mesh = create_teammate_mesh(Some("redis://invalid-host:9999")).await;

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
