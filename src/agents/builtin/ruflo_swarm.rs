use std::collections::HashMap;

/// Ruflo Unique Harness Innovations: Swarm coordination topologies: Hierarchical, mesh, adaptive with consensus.

pub trait Topology {
    fn route_task(&self, task: &str) -> String;
}

pub struct HierarchicalTopology {
    pub leader: String,
    // capability/domain -> sub-agent name
    pub sub_agents: HashMap<String, String>,
}

impl Topology for HierarchicalTopology {
    fn route_task(&self, task: &str) -> String {
        let lowercase_task = task.to_lowercase();
        let mut chosen_subagent = "default_worker".to_string();
        for (domain, sub_agent) in &self.sub_agents {
            if lowercase_task.contains(domain) {
                chosen_subagent = sub_agent.clone();
                break;
            }
        }
        format!("[Leader: {}] Routed task to {}: {}", self.leader, chosen_subagent, task)
    }
}

pub struct MeshTopology {
    pub agents: Vec<String>,
}

impl Topology for MeshTopology {
    fn route_task(&self, task: &str) -> String {
        let mut results = Vec::new();
        for agent in &self.agents {
            results.push(format!("[Mesh Agent: {}] Processed: {}", agent, task));
        }
        format!("Mesh Broadcast Results:\n{}", results.join("\n"))
    }
}

pub struct AdaptiveConsensusTopology {
    pub agents: Vec<String>,
}

impl Topology for AdaptiveConsensusTopology {
    fn route_task(&self, task: &str) -> String {
        // Simulate a voting process based on length. Usually this would be LLM evaluations.
        let mut votes: HashMap<String, usize> = HashMap::new();
        let mut agent_answers = Vec::new();

        // Simulate each agent generating an answer
        for agent in &self.agents {
            let answer = format!("Consensus Answer for '{}'", task);
            let entry = votes.entry(answer.clone()).or_insert(0);
            *entry += 1;
            agent_answers.push(format!("[Agent: {}] Voted: {}", agent, answer));
        }

        // Add a dissenting vote just to show consensus logic works if there's disagreement.
        let dissenting_answer = "Different Answer".to_string();
        votes.insert(dissenting_answer, 1);

        let mut majority_answer = String::new();
        let mut max_votes = 0;
        for (answer, count) in votes {
            if count > max_votes {
                max_votes = count;
                majority_answer = answer;
            }
        }

        format!("Adaptive Consensus Reached ({} votes): {}", max_votes, majority_answer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hierarchical_routing() {
        let mut sub_agents = HashMap::new();
        sub_agents.insert("finance".to_string(), "finance_agent".to_string());
        sub_agents.insert("marketing".to_string(), "marketing_agent".to_string());

        let topology = HierarchicalTopology {
            leader: "lead_agent".to_string(),
            sub_agents,
        };

        let result_finance = topology.route_task("Please analyze the finance budget");
        assert!(result_finance.contains("[Leader: lead_agent]"));
        assert!(result_finance.contains("finance_agent"));

        let result_marketing = topology.route_task("We need a marketing campaign");
        assert!(result_marketing.contains("marketing_agent"));
    }

    #[test]
    fn test_mesh_broadcast() {
        let topology = MeshTopology {
            agents: vec!["agent_1".to_string(), "agent_2".to_string(), "agent_3".to_string()],
        };

        let result = topology.route_task("System update required");
        assert!(result.contains("Mesh Broadcast Results"));
        assert!(result.contains("[Mesh Agent: agent_1] Processed: System update required"));
        assert!(result.contains("[Mesh Agent: agent_2] Processed: System update required"));
        assert!(result.contains("[Mesh Agent: agent_3] Processed: System update required"));
    }

    #[test]
    fn test_adaptive_consensus() {
        let topology = AdaptiveConsensusTopology {
            agents: vec!["agent_1".to_string(), "agent_2".to_string(), "agent_3".to_string()],
        };

        let result = topology.route_task("What is the capital of France?");
        // We know from the mock logic that it generates "Consensus Answer for '<task>'" 3 times and "Different Answer" 1 time.
        // So the consensus answer should have 3 votes.
        assert!(result.contains("Adaptive Consensus Reached (3 votes): Consensus Answer for 'What is the capital of France?'"));
    }
}
