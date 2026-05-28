use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;

/// Scalable multi-agent: single-user CLI to 1000+ agent cloud deployments
pub struct AgentCloudDeployment {
    pub agents: Mutex<HashMap<String, Arc<crate::agent::Agent>>>,
}

impl AgentCloudDeployment {
    pub fn new() -> Self {
        Self {
            agents: Mutex::new(HashMap::new()),
        }
    }

    pub async fn deploy_agent(&self, id: String, agent: Arc<crate::agent::Agent>) {
        let mut map = self.agents.lock().await;
        map.insert(id, agent);
    }

    pub async fn get_agent(&self, id: &str) -> Option<Arc<crate::agent::Agent>> {
        let map = self.agents.lock().await;
        map.get(id).cloned()
    }
}
