use std::collections::{HashMap, HashSet};
use std::fmt;

/// Ruflo Unique Harness Innovations: Swarm coordination topologies
/// Hierarchical, mesh, adaptive with consensus.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TopologyType {
    Hierarchical,
    Mesh,
    Adaptive,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SwarmError {
    UnknownNode(String),
    RoutingNotAllowed(String),
    PendingConsensus { current_votes: usize, required: usize },
}

impl fmt::Display for SwarmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SwarmError::UnknownNode(msg) => write!(f, "Unknown node: {}", msg),
            SwarmError::RoutingNotAllowed(msg) => write!(f, "Routing not allowed: {}", msg),
            SwarmError::PendingConsensus { current_votes, required } => {
                write!(f, "Pending consensus. Current votes: {}, Required: {}", current_votes, required)
            }
        }
    }
}

pub struct SwarmCoordinator {
    pub topology: TopologyType,
    pub nodes: HashMap<String, String>, // Agent ID to Role
    pub consensus_threshold: usize,
    pub pending_consensus: HashMap<String, HashSet<String>>, // message_id -> set of agent IDs that voted
}

impl SwarmCoordinator {
    pub fn new(topology: TopologyType, consensus_threshold: usize) -> Self {
        Self {
            topology,
            nodes: HashMap::new(),
            consensus_threshold,
            pending_consensus: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, agent_id: &str, role: &str) {
        self.nodes.insert(agent_id.to_string(), role.to_string());
    }

    pub fn route_message(&mut self, from: &str, to: &str, message_id: &str, _message: &str) -> Result<(), SwarmError> {
        if !self.nodes.contains_key(from) {
            return Err(SwarmError::UnknownNode(from.to_string()));
        }
        if !self.nodes.contains_key(to) {
            return Err(SwarmError::UnknownNode(to.to_string()));
        }

        match self.topology {
            TopologyType::Hierarchical => {
                // In hierarchical, all messages must pass through a leader (e.g. "leader" node)
                // If neither is leader, reject direct message
                let from_role = self.nodes.get(from).unwrap();
                let to_role = self.nodes.get(to).unwrap();

                if from_role != "leader" && to_role != "leader" {
                    return Err(SwarmError::RoutingNotAllowed("Hierarchical topology requires routing through leader".to_string()));
                }
                Ok(())
            }
            TopologyType::Mesh => {
                // In mesh, any node can talk to any node directly
                Ok(())
            }
            TopologyType::Adaptive => {
                // Adaptive logic requires consensus check before routing
                let voters = self.pending_consensus.entry(message_id.to_string()).or_insert_with(HashSet::new);
                voters.insert(from.to_string());
                let current_votes = voters.len();

                if current_votes >= self.consensus_threshold {
                    // Consensus reached, clear pending and route
                    self.pending_consensus.remove(message_id);
                    Ok(())
                } else {
                    Err(SwarmError::PendingConsensus {
                        current_votes,
                        required: self.consensus_threshold,
                    })
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hierarchical_topology() {
        let mut swarm = SwarmCoordinator::new(TopologyType::Hierarchical, 1);
        swarm.add_node("agent_A", "leader");
        swarm.add_node("agent_B", "worker");
        swarm.add_node("agent_C", "worker");

        // Worker to Leader is OK
        assert!(swarm.route_message("agent_B", "agent_A", "msg1", "hello").is_ok());

        // Leader to Worker is OK
        assert!(swarm.route_message("agent_A", "agent_C", "msg2", "do this").is_ok());

        // Worker to Worker is not OK
        let err = swarm.route_message("agent_B", "agent_C", "msg3", "secret").unwrap_err();
        assert_eq!(err, SwarmError::RoutingNotAllowed("Hierarchical topology requires routing through leader".to_string()));
    }

    #[test]
    fn test_mesh_topology() {
        let mut swarm = SwarmCoordinator::new(TopologyType::Mesh, 1);
        swarm.add_node("agent_A", "leader");
        swarm.add_node("agent_B", "worker");
        swarm.add_node("agent_C", "worker");

        // Any to Any is OK
        assert!(swarm.route_message("agent_B", "agent_C", "msg1", "secret").is_ok());
        assert!(swarm.route_message("agent_A", "agent_B", "msg2", "hello").is_ok());
    }

    #[test]
    fn test_adaptive_consensus_topology() {
        let mut swarm = SwarmCoordinator::new(TopologyType::Adaptive, 2);
        swarm.add_node("agent_A", "peer");
        swarm.add_node("agent_B", "peer");
        swarm.add_node("agent_C", "peer");

        // First attempt (Vote 1)
        let res1 = swarm.route_message("agent_A", "agent_C", "msg1", "proposal");
        assert!(res1.is_err());
        assert_eq!(res1.unwrap_err(), SwarmError::PendingConsensus { current_votes: 1, required: 2 });

        // Second attempt (Vote 2) -> Consensus reached
        let res2 = swarm.route_message("agent_B", "agent_C", "msg1", "proposal");
        assert!(res2.is_ok());

        // A new message requires consensus again
        let res3 = swarm.route_message("agent_C", "agent_A", "msg2", "new proposal");
        assert!(res3.is_err());
    }

    #[test]
    fn test_adaptive_consensus_double_vote() {
        let mut swarm = SwarmCoordinator::new(TopologyType::Adaptive, 2);
        swarm.add_node("agent_A", "peer");
        swarm.add_node("agent_B", "peer");

        // First attempt by agent_A (Vote 1)
        let res1 = swarm.route_message("agent_A", "agent_B", "msg1", "proposal");
        assert!(res1.is_err());
        assert_eq!(res1.unwrap_err(), SwarmError::PendingConsensus { current_votes: 1, required: 2 });

        // Second attempt by agent_A (Double Vote, should still be Vote 1)
        let res2 = swarm.route_message("agent_A", "agent_B", "msg1", "proposal_again");
        assert!(res2.is_err());
        assert_eq!(res2.unwrap_err(), SwarmError::PendingConsensus { current_votes: 1, required: 2 });

        // Attempt by agent_B (Vote 2) -> Consensus reached
        let res3 = swarm.route_message("agent_B", "agent_A", "msg1", "proposal_agree");
        assert!(res3.is_ok());
    }

    #[test]
    fn test_unknown_node() {
        let mut swarm = SwarmCoordinator::new(TopologyType::Mesh, 1);
        swarm.add_node("agent_A", "peer");

        let err = swarm.route_message("agent_A", "agent_Z", "msg", "data").unwrap_err();
        assert_eq!(err, SwarmError::UnknownNode("agent_Z".to_string()));

        let err2 = swarm.route_message("agent_Y", "agent_A", "msg", "data").unwrap_err();
        assert_eq!(err2, SwarmError::UnknownNode("agent_Y".to_string()));
    }
}
