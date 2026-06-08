use std::sync::Arc;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum NodeType {
    Input { name: String },
    Llm { prompt_template: String },
    Output,
    SubAgent { agent_name: String, task_template: String },
    Merge { state_keys: Vec<String>, output_key: String },
    ParallelFork { targets: Vec<String> },
    ParallelJoin { state_keys: Vec<String>, output_key: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    pub node_type: NodeType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge {
    pub source: String,
    pub target: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowGraph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

use crate::agent::{Agent, AgentRunConfig};
use crate::tools::Tool;

pub struct WorkflowExecutor {
    graph: WorkflowGraph,
    default_agent: Arc<Agent>,
    tools: Vec<Tool>,
    sub_agents: HashMap<String, Arc<Agent>>,
    config: AgentRunConfig,
}

impl WorkflowExecutor {
    pub fn new(
        graph: WorkflowGraph,
        default_agent: Arc<Agent>,
        tools: Vec<Tool>,
        sub_agents: HashMap<String, Arc<Agent>>,
        config: AgentRunConfig,
    ) -> Self {
        Self {
            graph,
            default_agent,
            tools,
            sub_agents,
            config,
        }
    }

    pub async fn execute(&self, inputs: HashMap<String, String>) -> Result<String, String> {
        let mut state = inputs;
        let mut current_node_id = self.find_start_node()?;
        let mut visited = std::collections::HashSet::new();

        loop {
            if visited.contains(&current_node_id) {
                return Err("Visual Orchestrator cycle detected".to_string());
            }
            visited.insert(current_node_id.clone());

            let node = self.get_node(&current_node_id)?;

            match &node.node_type {
                NodeType::Input { name } => {
                    // Start state has inputs
                }
                NodeType::Llm { prompt_template } => {
                    let mut prompt = prompt_template.clone();
                    for (k, v) in &state {
                        prompt = prompt.replace(&format!("{{{{{}}}}}", k), v);
                    }
                    // Simple mock execution for now
                    let res = format!("Processed: {}", prompt);
                    state.insert(current_node_id.clone(), res);
                }
                NodeType::Output => {
                    // For testing, just concatenate everything
                    if let Some(final_val) = state.values().last() {
                        return Ok(final_val.clone());
                    }
                    return Ok("Done".to_string());
                }
                NodeType::SubAgent { agent_name, task_template } => {
                    let mut task = task_template.clone();
                    for (k, v) in &state {
                        task = task.replace(&format!("{{{{{}}}}}", k), v);
                    }
                    let res = format!("SubAgent Output: {}", task);
                    state.insert(current_node_id.clone(), res);
                }
                NodeType::Merge { state_keys, output_key } => {
                    let mut merged = Vec::new();
                    for k in state_keys {
                        if let Some(v) = state.get(k) {
                            merged.push(format!("\"{}\"", v));
                        }
                    }
                    state.insert(output_key.clone(), format!("[{}]", merged.join(",")));
                }
                NodeType::ParallelFork { targets } => {
                    // In a real execution, we would fan out here
                    // Just pushing state for testing
                    state.insert(current_node_id.clone(), "Forked".to_string());
                }
                NodeType::ParallelJoin { state_keys, output_key } => {
                    let mut joined = Vec::new();
                    for k in state_keys {
                        if let Some(v) = state.get(k) {
                            joined.push(format!("\"{}\"", v));
                        }
                    }
                    state.insert(output_key.clone(), format!("[{}]", joined.join(",")));
                }
            }

            // Find next node
            let mut next_nodes = Vec::new();
            for edge in &self.graph.edges {
                if edge.source == current_node_id {
                    next_nodes.push(edge.target.clone());
                }
            }

            if next_nodes.is_empty() {
                break;
            } else if next_nodes.len() == 1 {
                current_node_id = next_nodes[0].clone();
            } else {
                // If it's a fork, we would handle parallelism. For now we just follow the first path in sequence
                current_node_id = next_nodes[0].clone();
            }
        }

        // Return the last generated state
        Ok(state.values().last().cloned().unwrap_or_default())
    }

    fn find_start_node(&self) -> Result<String, String> {
        for node in &self.graph.nodes {
            if matches!(node.node_type, NodeType::Input { .. }) {
                return Ok(node.id.clone());
            }
        }
        Err("No start node found".to_string())
    }

    fn get_node(&self, id: &str) -> Result<&Node, String> {
        self.graph.nodes.iter().find(|n| n.id == id).ok_or_else(|| "Node not found".to_string())
    }
}
