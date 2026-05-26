/// SOTA Harness Patterns (2025-2026): 4. Scalable multi-agent -> single-user CLI to 1000+ agent cloud deployments
use std::sync::Arc;



use crate::mesh::transport::{MeshTransport, Message as MeshMessage};
use uuid::Uuid;

pub struct ScalableCloudDeployer {
    pub mesh: Arc<dyn MeshTransport>,
    pub max_agents: usize,
}

impl ScalableCloudDeployer {
    pub fn new(mesh: Arc<dyn MeshTransport>, max_agents: usize) -> Self {
        Self { mesh, max_agents }
    }

    /// Deploys up to 1000+ agents via the CLI configuration across the mesh network.
    pub async fn deploy_swarm(&self, task_definition: &str, num_agents: usize) -> Result<String, String> {
        if num_agents > self.max_agents {
            return Err(format!("Requested {} agents exceeds max cloud deployment limit of {}", num_agents, self.max_agents));
        }

        let mut job_ids = Vec::new();
        for i in 0..num_agents {
            let job_id = format!("cloud_agent_run_{}_{}", Uuid::new_v4(), i);
            let payload = format!("{{\"task\": \"{}\", \"index\": {}}}", task_definition, i);
            self.mesh.publish("system:cloud_deploy:jobs", MeshMessage {
                agent_id: format!("deployer_{}", i),


                msg_id: uuid::Uuid::new_v4().to_string(),
                action: "deploy".to_string(),
                status: "pending".to_string(),
                payload: payload.into_bytes(),
            }).await?;
            job_ids.push(job_id);
        }

        Ok(format!("Successfully scaled out {} agents to the cloud via mesh.", job_ids.len()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mesh::transport::InProcessTransport;

    #[tokio::test]
    async fn test_scalable_multi_agent_deploy_mesh() {
        let transport: Arc<dyn MeshTransport> = Arc::new(InProcessTransport::new());
        let deployer = ScalableCloudDeployer::new(transport, 1500);

        let res = deployer.deploy_swarm("Analyze logs", 1000).await.unwrap();
        assert!(res.contains("Successfully scaled out 1000 agents"));

        let err = deployer.deploy_swarm("Overflow", 2000).await;
        assert!(err.is_err());
    }
}
