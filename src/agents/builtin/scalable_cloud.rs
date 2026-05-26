use std::sync::Arc;
use crate::mesh::transport::MeshTransport;
use crate::proto::hub::TeammateMeshEvent;

/// SOTA Harness Patterns (2025-2026): 4. Scalable multi-agent -> single-user CLI to 1000+ agent cloud deployments
/// This module implements the orchestration logic to dispatch 1000+ tasks concurrently to a
/// distributed cloud cluster of agent workers using the underlying MeshTransport.

pub struct CloudOrchestrator {
    transport: Arc<dyn MeshTransport>,
}

impl CloudOrchestrator {
    pub fn new(transport: Arc<dyn MeshTransport>) -> Self {
        Self { transport }
    }

    /// Deploys and orchestrates tasks across up to 1000+ distributed agents.
    pub async fn dispatch_cloud_cluster(&self, base_task: &str, num_agents: usize) -> Result<(), String> {
        let mut futures = Vec::new();
        for i in 0..num_agents {
            let task_id = format!("cloud-task-{}-{}", uuid::Uuid::new_v4(), i);
            let payload = format!("{} (Agent Index {})", base_task, i);

            // In a real cloud environment, `agent_jobs` is subscribed to by N workers.
            let msg = TeammateMeshEvent {
                event_id: task_id.clone(),
                task_id,
                event_type: "DISPATCH".to_string(),
                payload,
                timestamp: 0,
            };

            let transport_clone = self.transport.clone();
            futures.push(tokio::spawn(async move {
                transport_clone.publish("agent_jobs", msg).await
            }));
        }

        // Wait for all dispatches to finish
        let results = futures::future::join_all(futures).await;
        for r in results {
            if let Ok(Err(e)) = r {
                return Err(format!("Failed to dispatch task: {}", e));
            } else if r.is_err() {
                return Err("Failed to join dispatch task".to_string());
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mesh::transport::InProcessTransport;

    #[tokio::test]
    async fn test_cloud_dispatch_1000_agents() {
        let transport = Arc::new(InProcessTransport::new());
        let orchestrator = CloudOrchestrator::new(transport.clone());

        let sub_handle = transport.subscribe("agent_jobs", Box::new(|_| {})).await.unwrap();

        // Dispatch 1005 tasks to simulate 1000+ cloud agents scale
        let res = orchestrator.dispatch_cloud_cluster("Compute large data", 1005).await;
        assert!(res.is_ok());
    }
}
