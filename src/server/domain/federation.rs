use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FederatedAgent {
    pub id: String,
    pub home_cluster: String,
    pub status: String, // e.g., GLOBAL_IDLE, BUSY
    pub latency_ms: i32,
}

pub struct FederatedRegistry {
    agents: RwLock<HashMap<String, FederatedAgent>>,
}

impl FederatedRegistry {
    pub fn new() -> Self {
        FederatedRegistry {
            agents: RwLock::new(HashMap::new()),
        }
    }

    pub fn register_agent(&self, agent: FederatedAgent) -> Result<(), String> {
        if agent.id.is_empty() {
            return Err("agent ID cannot be empty".to_string());
        }
        if agent.home_cluster.is_empty() {
            return Err("home cluster cannot be empty".to_string());
        }

        let mut agents = self.agents.write().map_err(|e| e.to_string())?;
        if agents.contains_key(&agent.id) {
            return Err("agent already registered".to_string());
        }
        agents.insert(agent.id.clone(), agent);
        Ok(())
    }

    pub fn get_agent(&self, agent_id: &str) -> Option<FederatedAgent> {
        let agents = self.agents.read().ok()?;
        agents.get(agent_id).cloned()
    }

    pub fn update_agent_status(&self, agent_id: &str, status: &str) -> Result<(), String> {
        let mut agents = self.agents.write().map_err(|e| e.to_string())?;
        if let Some(agent) = agents.get_mut(agent_id) {
            agent.status = status.to_string();
            Ok(())
        } else {
            Err("agent not found".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_federated_registry() {
        let registry = FederatedRegistry::new();

        // Register Valid Agent
        let agent = FederatedAgent {
            id: "agent-1".to_string(),
            home_cluster: "eu-central-1".to_string(),
            status: "GLOBAL_IDLE".to_string(),
            latency_ms: 10,
        };
        assert!(registry.register_agent(agent.clone()).is_ok());

        // Register Missing ID
        let bad_agent = FederatedAgent {
            id: "".to_string(),
            home_cluster: "us-east-1".to_string(),
            status: "".to_string(),
            latency_ms: 0,
        };
        assert!(registry.register_agent(bad_agent).is_err());

        // Register Missing HomeCluster
        let bad_agent2 = FederatedAgent {
            id: "agent-2".to_string(),
            home_cluster: "".to_string(),
            status: "".to_string(),
            latency_ms: 0,
        };
        assert!(registry.register_agent(bad_agent2).is_err());

        // Register Duplicate Agent
        assert!(registry.register_agent(agent).is_err());

        // Get Existing Agent
        let got = registry.get_agent("agent-1");
        assert!(got.is_some());
        assert_eq!(got.unwrap().home_cluster, "eu-central-1");

        // Get Missing Agent
        assert!(registry.get_agent("agent-not-found").is_none());

        // Update Existing Agent Status
        assert!(registry.update_agent_status("agent-1", "BUSY").is_ok());
        let got = registry.get_agent("agent-1").unwrap();
        assert_eq!(got.status, "BUSY");

        // Update Missing Agent Status
        assert!(registry.update_agent_status("agent-not-found", "BUSY").is_err());
    }
}
