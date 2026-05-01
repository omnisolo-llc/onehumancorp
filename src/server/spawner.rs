#![allow(dead_code, unused_mut, unused_variables, unused_imports, deprecated)]
use async_trait::async_trait;
use std::sync::Arc;
use crate::hub::Hub;
use crate::tasks::SharedTask;

#[async_trait]
pub trait SubAgentSpawner: Send + Sync {
    async fn spawn(&self, task: SharedTask) -> Result<(), String>;
}

use ohc_builtin_agent::mesh::transport::MeshTransport;

pub struct DefaultSubAgentSpawner {
    hub: Arc<Hub>,
    transport: Arc<dyn MeshTransport>,
}

impl DefaultSubAgentSpawner {
    pub fn new(hub: Arc<Hub>, transport: Arc<dyn MeshTransport>) -> Self {
        DefaultSubAgentSpawner { hub, transport }
    }
}

#[async_trait]
impl SubAgentSpawner for DefaultSubAgentSpawner {
    async fn spawn(&self, task: SharedTask) -> Result<(), String> {
        println!("Spawning sub-agent for task via MeshTransport: {}", task.id);
        
        use crate::ohc::agent::service::RunTaskRequest;
        use prost::Message;

        let req = RunTaskRequest {
            task_id: task.id.clone(),
            task: task.title.clone(),
            model: String::new(),
            llm_provider: String::new(),
            department: "system".to_string(),
            ..Default::default()
        };

        let mut buf = Vec::new();
        req.encode(&mut buf).map_err(|e| format!("encode failed: {}", e))?;

        self.transport.publish("agent_jobs", ohc_builtin_agent::mesh::transport::Message {
            topic: "agent_jobs".to_string(),
            payload: buf,
        }).await?;

        Ok(())
    }
}
