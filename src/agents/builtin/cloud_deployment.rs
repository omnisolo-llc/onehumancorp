use std::collections::HashMap;

/// SOTA Harness Patterns (2025-2026): 4. Scalable multi-agent -> single-user CLI to 1000+ agent cloud deployments
/// Implements a lightweight Kubernetes/Docker-Swarm like orchestrator mock for scaling agents to 1000+ in the cloud.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AgentState {
    Pending,
    Running,
    Terminated,
    Failed,
}

#[derive(Debug, Clone)]
pub struct CloudAgentInstance {
    pub id: String,
    pub state: AgentState,
    pub assigned_node: String,
}

pub struct CloudAgentOrchestrator {
    pub instances: std::sync::Mutex<HashMap<String, CloudAgentInstance>>,
    pub available_nodes: Vec<String>,
}

impl CloudAgentOrchestrator {
    pub fn new(nodes: Vec<String>) -> Self {
        Self {
            instances: std::sync::Mutex::new(HashMap::new()),
            available_nodes: nodes,
        }
    }

    pub fn deploy_agents(&self, count: usize, base_id: &str) -> Result<(), String> {
        if self.available_nodes.is_empty() {
            return Err("No nodes available for deployment.".to_string());
        }

        let mut instances = self.instances.lock().unwrap();

        for i in 0..count {
            let id = format!("{}-{}", base_id, i);
            let node = self.available_nodes[i % self.available_nodes.len()].clone();

            instances.insert(id.clone(), CloudAgentInstance {
                id,
                state: AgentState::Running,
                assigned_node: node,
            });
        }

        Ok(())
    }

    pub fn get_running_count(&self) -> usize {
        let instances = self.instances.lock().unwrap();
        instances.values().filter(|i| i.state == AgentState::Running).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cloud_agent_deployment_scale_to_1000() {
        let nodes = vec!["node-1".to_string(), "node-2".to_string(), "node-3".to_string(), "node-4".to_string()];
        let orchestrator = CloudAgentOrchestrator::new(nodes);

        // SOTA Harness Patterns (2025-2026): 4. Scalable multi-agent -> single-user CLI to 1000+ agent cloud deployments
        let deploy_result = orchestrator.deploy_agents(1005, "cloud-agent");
        assert!(deploy_result.is_ok());

        let running = orchestrator.get_running_count();
        assert_eq!(running, 1005);
    }
}
