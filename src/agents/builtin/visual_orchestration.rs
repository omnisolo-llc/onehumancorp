use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::agent::{Agent, AgentRunConfig};

/// SOTA Harness Patterns (2025-2026): 3. Visual/low-code orchestration -> democratizing agent construction
///
/// This provides a programmatic representation of a visual, block-based flow where
/// users can connect LLM nodes, Tool nodes, and Logic nodes together without writing Rust.

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum OrchestrationNode {
    /// Simulates a user input block
    Input { name: String },
    /// Simulates an LLM block
    Llm { prompt_template: String },
    /// Simulates a Tool Execution block
    Tool { tool_name: String, args_template: String },
    /// Simulates a conditional routing block
    Condition { condition_expression: String, true_target: String, false_target: String },
    /// Simulates the final output block
    Output,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestrationGraph {
    pub nodes: HashMap<String, OrchestrationNode>,
    pub edges: Vec<OrchestrationEdge>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrchestrationEdge {
    pub source: String,
    pub target: String,
}

pub struct VisualOrchestrator {
    pub graph: OrchestrationGraph,
    pub agent: Arc<Agent>,
    pub config: AgentRunConfig,
}

impl VisualOrchestrator {
    pub fn new(graph: OrchestrationGraph, agent: Arc<Agent>, config: AgentRunConfig) -> Self {
        Self { graph, agent, config }
    }

    /// Evaluates the visual orchestration graph
    pub async fn execute(&self, mut inputs: HashMap<String, String>) -> Result<String, String> {
        let mut state = inputs.clone();

        // 1. Find start node (Input)
        let mut current_node_id = self.graph.nodes.iter()
            .find(|(_, n)| matches!(n, OrchestrationNode::Input { .. }))
            .map(|(id, _)| id.clone())
            .ok_or_else(|| "Graph must contain an Input node to begin orchestration".to_string())?;

        // 2. Build outgoing edges map
        let mut outgoing: HashMap<String, Vec<String>> = HashMap::new();
        for e in &self.graph.edges {
            outgoing.entry(e.source.clone()).or_default().push(e.target.clone());
        }

        let mut visited = std::collections::HashSet::new();

        // 3. Traverse the graph
        while let Some(node) = self.graph.nodes.get(&current_node_id) {
            if visited.contains(&current_node_id) {
                return Err("Visual Orchestrator cycle detected".to_string());
            }
            visited.insert(current_node_id.clone());

            match node {
                OrchestrationNode::Input { name } => {
                    // Start block; variables should be in state already.
                }
                OrchestrationNode::Llm { prompt_template } => {
                    let mut prompt = prompt_template.clone();
                    for (k, v) in &state {
                        prompt = prompt.replace(&format!("{{{{{}}}}}", k), v);
                    }
                    let mut on_event = |_| {};
                    let res = self.agent.run(&self.config, &prompt, &mut on_event).await
                        .map_err(|e| format!("LLM execution failed: {}", e))?;
                    state.insert(current_node_id.clone(), res);
                }
                OrchestrationNode::Tool { tool_name, args_template } => {
                    let mut args_json = args_template.clone();
                    for (k, v) in &state {
                        args_json = args_json.replace(&format!("{{{{{}}}}}", k), v);
                    }
                    let args: serde_json::Value = serde_json::from_str(&args_json)
                        .map_err(|e| format!("Invalid JSON args for tool {}: {}", tool_name, e))?;

                    let tool = self.agent.tools.iter().find(|t| t.name == *tool_name)
                        .ok_or_else(|| format!("Tool {} not available in Agent", tool_name))?;

                    let res = tool.execute.execute(args).await
                        .map_err(|e| format!("Tool {} execution failed: {}", tool_name, e))?;

                    state.insert(current_node_id.clone(), res);
                }
                OrchestrationNode::Condition { condition_expression, true_target, false_target } => {
                    let mut expr = condition_expression.clone();
                    for (k, v) in &state {
                        expr = expr.replace(&format!("{{{{{}}}}}", k), v);
                    }
                    // For demo of visual logic:
                    let is_true = expr.contains("true") || expr.contains("success");
                    current_node_id = if is_true { true_target.clone() } else { false_target.clone() };
                    // Condition nodes dictate explicit routing, skip edge traversal
                    continue;
                }
                OrchestrationNode::Output => {
                    let in_edges: Vec<_> = self.graph.edges.iter().filter(|e| e.target == current_node_id).collect();
                    if let Some(edge) = in_edges.first() {
                        if let Some(val) = state.get(&edge.source) {
                            return Ok(val.clone());
                        }
                    }
                    return Ok("Visual orchestration completed with no data".to_string());
                }
            }

            // Standard edge traversal
            if let Some(next_nodes) = outgoing.get(&current_node_id) {
                if !next_nodes.is_empty() {
                    current_node_id = next_nodes[0].clone();
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        Ok("Execution halted without reaching output".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::llm::LlmClient;
    use crate::types::{ChatRequest, ChatResponse, Message, Role, Usage, ToolCall, ToolError};
    use crate::tools::{Tool, ToolExecutor};

    struct MockVisualLlmClient;
    #[async_trait::async_trait]
    impl LlmClient for MockVisualLlmClient {
        async fn chat(&self, req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            let last_user = req.messages.last().unwrap().content.clone();
            Ok(ChatResponse {
                message: Message::assistant(format!("Orchestrated: {}", last_user)),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("id1".to_string()),
            })
        }
    }

    struct MockAddTool;
    #[async_trait::async_trait]
    impl ToolExecutor for MockAddTool {
        async fn execute(&self, args: serde_json::Value) -> Result<String, ToolError> {
            let val = args["val"].as_str().unwrap_or("");
            Ok(format!("Tool processed: {}", val))
        }
    }

    #[tokio::test]
    async fn test_visual_orchestrator_execution() {
        let mut nodes = HashMap::new();
        nodes.insert("n_in".to_string(), OrchestrationNode::Input { name: "input_data".to_string() });
        nodes.insert("n_llm".to_string(), OrchestrationNode::Llm { prompt_template: "Please summarize: {{n_in}}".to_string() });
        nodes.insert("n_tool".to_string(), OrchestrationNode::Tool { tool_name: "add_prefix".to_string(), args_template: r#"{"val": "{{n_llm}}"}"#.to_string() });
        nodes.insert("n_out".to_string(), OrchestrationNode::Output);

        let edges = vec![
            OrchestrationEdge { source: "n_in".to_string(), target: "n_llm".to_string() },
            OrchestrationEdge { source: "n_llm".to_string(), target: "n_tool".to_string() },
            OrchestrationEdge { source: "n_tool".to_string(), target: "n_out".to_string() },
        ];

        let graph = OrchestrationGraph { nodes, edges };

        let tools = vec![
            Tool {
                name: "add_prefix".to_string(),
                description: "".to_string(),
                is_read_only: true,
                parameters: serde_json::json!({}),
                execute: Arc::new(MockAddTool),
            }
        ];

        let agent = Arc::new(Agent::new(Arc::new(MockVisualLlmClient), tools));
        let config = AgentRunConfig::default();
        let orchestrator = VisualOrchestrator::new(graph, agent, config);

        let mut inputs = HashMap::new();
        inputs.insert("n_in".to_string(), "raw text block".to_string());

        let result = orchestrator.execute(inputs).await.unwrap();
        assert_eq!(result, "Tool processed: Orchestrated: Please summarize: raw text block");
    }
}
