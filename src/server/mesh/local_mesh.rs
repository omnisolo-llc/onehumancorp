use std::sync::Arc;
use crate::mesh::transport::{MeshTransport, Message};

pub struct LocalTeammateMesh {
    transport: Arc<dyn MeshTransport>,
}

impl LocalTeammateMesh {
    pub fn new(transport: Arc<dyn MeshTransport>) -> Self {
        LocalTeammateMesh { transport }
    }

    pub async fn publish_task(&self, payload: Vec<u8>) -> Result<(), String> {
        self.transport.publish("mesh:tasks", Message {
            topic: "mesh:tasks".to_string(),
            payload,
        }).await
    }

    pub async fn publish_coordination(&self, payload: Vec<u8>) -> Result<(), String> {
        self.transport.publish("mesh:coordination", Message {
            topic: "mesh:coordination".to_string(),
            payload,
        }).await
    }

    pub async fn subscribe_tasks(&self, handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        self.transport.subscribe("mesh:tasks", handler).await
    }

    pub async fn subscribe_coordination(&self, handler: Box<dyn Fn(Message) + Send + Sync>) -> Result<Box<dyn Fn() + Send + Sync>, String> {
        self.transport.subscribe("mesh:coordination", handler).await
    }
}
