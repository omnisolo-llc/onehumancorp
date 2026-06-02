use crate::agent::{Agent, AgentRunConfig};

use std::collections::HashMap;
use std::sync::Arc;
use serde::{Deserialize, Serialize};

/// AutoGPT Unique Harness Innovations: Block-based visual workflow
/// No-code agent assembly via block-connect UI.

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum NodeType {
    Llm { prompt_template: String },
    Tool { tool_name: String, args_template: String },
    Condition { condition_expression: String, true_target: String, false_target: String },
    Input { name: String },
    Output,
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

pub struct WorkflowExecutor {
    pub graph: WorkflowGraph,
    pub agent: Arc<Agent>,
    pub tools: Vec<crate::tools::Tool>,
    pub config: AgentRunConfig,
}

impl WorkflowExecutor {
    pub fn new(graph: WorkflowGraph, agent: Arc<Agent>, tools: Vec<crate::tools::Tool>, config: AgentRunConfig) -> Self {
        Self { graph, agent, tools, config }
    }

    pub async fn execute(&self, input_vars: HashMap<String, String>) -> Result<String, String> {
        let mut state: HashMap<String, String> = input_vars.clone();

        // Find input node to start
        let mut current_node_id = self.graph.nodes.iter()
            .find(|n| matches!(n.node_type, NodeType::Input { .. }))
            .map(|n| n.id.clone())
            .ok_or_else(|| "No input node found in graph".to_string())?;

        let nodes_map: HashMap<String, Node> = self.graph.nodes.iter()
            .map(|n| (n.id.clone(), n.clone()))
            .collect();

        let mut outgoing_edges: HashMap<String, Vec<String>> = HashMap::new();
        for edge in &self.graph.edges {
            outgoing_edges.entry(edge.source.clone()).or_default().push(edge.target.clone());
        }

        loop {
            let node = nodes_map.get(&current_node_id).ok_or_else(|| format!("Node not found: {}", current_node_id))?;

            match &node.node_type {
                NodeType::Input { name: _ } => {
                    // Start execution
                }
                NodeType::Llm { prompt_template } => {
                    let mut prompt = prompt_template.clone();
                    for (k, v) in &state {
                        prompt = prompt.replace(&format!("{{{{{}}}}}", k), v);
                    }

                    let mut on_event = |_| {};
                    let result = self.agent.run(&self.config, &prompt, &mut on_event).await
                        .map_err(|e| format!("LLM node {} failed: {}", node.id, e))?;

                    state.insert(node.id.clone(), result);
                }
                NodeType::Tool { tool_name, args_template } => {
                    let mut args_json = args_template.clone();
                    for (k, v) in &state {
                        args_json = args_json.replace(&format!("{{{{{}}}}}", k), v);
                    }

                    let args: serde_json::Value = serde_json::from_str(&args_json)
                        .map_err(|e| format!("Tool node {} failed to parse args: {}", node.id, e))?;

                    let tool = self.tools.iter().find(|t| &t.name == tool_name)
                        .ok_or_else(|| format!("Tool {} not found", tool_name))?;

                    let result = tool.execute.execute(args).await
                        .map_err(|e| format!("Tool {} execution failed: {}", tool_name, e))?;

                    state.insert(node.id.clone(), result);
                }
                NodeType::Condition { condition_expression, true_target, false_target } => {
                    let mut expr = condition_expression.clone();
                    for (k, v) in &state {
                        expr = expr.replace(&format!("{{{{{}}}}}", k), v);
                    }

                    // A very naive evaluation for demo purposes: e.g. "success == success"
                    let parts: Vec<&str> = expr.split("==").collect();
                    let is_true = if parts.len() == 2 {
                        parts[0].trim() == parts[1].trim()
                    } else {
                        false
                    };

                    current_node_id = if is_true { true_target.clone() } else { false_target.clone() };
                    continue;
                }
                NodeType::Output => {
                    // For an output node, we find the state value from the node that points to it
                    // Wait, our loop just follows edges.
                    let incoming = self.graph.edges.iter().find(|e| e.target == current_node_id).map(|e| e.source.clone());
                    if let Some(src) = incoming {
                        if let Some(val) = state.get(&src) {
                            return Ok(val.clone());
                        }
                    }
                    return Ok("Execution finished but no output found".to_string());
                }
            }

            // Move to next node
            let next_nodes = outgoing_edges.get(&current_node_id);
            if let Some(nexts) = next_nodes {
                if !nexts.is_empty() {
                    current_node_id = nexts[0].clone();
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        Ok("Execution halted unexpectedly".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::llm::LlmClient;
    use crate::types::{ChatRequest, ChatResponse, Message, Usage, ToolError};
    use crate::tools::{Tool, ToolExecutor};

    struct MockVisualLlmClient;
    #[async_trait::async_trait]
    impl LlmClient for MockVisualLlmClient {
        async fn chat(&self, req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            let last_user = req.messages.last().unwrap().content.clone();
            Ok(ChatResponse {
                message: Message::assistant(format!("Processed: {}", last_user)),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("id1".to_string()),
            })
        }
    }

    struct MockEchoTool;
    #[async_trait::async_trait]
    impl ToolExecutor for MockEchoTool {
        async fn execute(&self, args: serde_json::Value) -> Result<String, ToolError> {
            Ok(format!("Tool echo: {}", args["val"].as_str().unwrap_or("")))
        }
    }

    #[tokio::test]
    async fn test_visual_workflow_linear() {
        let graph = WorkflowGraph {
            nodes: vec![
                Node { id: "in".to_string(), node_type: NodeType::Input { name: "input_var".to_string() } },
                Node { id: "llm1".to_string(), node_type: NodeType::Llm { prompt_template: "Please format: {{in}}".to_string() } },
                Node { id: "tool1".to_string(), node_type: NodeType::Tool { tool_name: "echo".to_string(), args_template: r#"{"val": "{{llm1}}"}"#.to_string() } },
                Node { id: "out".to_string(), node_type: NodeType::Output },
            ],
            edges: vec![
                Edge { source: "in".to_string(), target: "llm1".to_string() },
                Edge { source: "llm1".to_string(), target: "tool1".to_string() },
                Edge { source: "tool1".to_string(), target: "out".to_string() },
            ],
        };

        let tools = vec![
            Tool {
                name: "echo".to_string(),
                description: "".to_string(),
                is_read_only: true,
                parameters: serde_json::json!({}),
                execute: Arc::new(MockEchoTool),
            }
        ];

        let agent = Arc::new(Agent::new(Arc::new(MockVisualLlmClient), vec![]));
        let config = AgentRunConfig::default();

        let executor = WorkflowExecutor::new(graph, agent, tools, config);

        let mut inputs = HashMap::new();
        inputs.insert("in".to_string(), "raw_data".to_string());

        let result = executor.execute(inputs).await.unwrap();
        assert_eq!(result, "Tool echo: Processed: Please format: raw_data");
    }

    #[tokio::test]
    async fn test_visual_workflow_condition() {
        let graph = WorkflowGraph {
            nodes: vec![
                Node { id: "in".to_string(), node_type: NodeType::Input { name: "input_var".to_string() } },
                Node { id: "cond1".to_string(), node_type: NodeType::Condition {
                    condition_expression: "{{in}} == trigger".to_string(),
                    true_target: "out_true".to_string(),
                    false_target: "out_false".to_string()
                }},
                Node { id: "out_true".to_string(), node_type: NodeType::Llm { prompt_template: "True branch".to_string() } },
                Node { id: "out_false".to_string(), node_type: NodeType::Llm { prompt_template: "False branch".to_string() } },
                Node { id: "out".to_string(), node_type: NodeType::Output },
            ],
            edges: vec![
                Edge { source: "in".to_string(), target: "cond1".to_string() },
                Edge { source: "out_true".to_string(), target: "out".to_string() },
            ],
        };

        let agent = Arc::new(Agent::new(Arc::new(MockVisualLlmClient), vec![]));
        let config = AgentRunConfig::default();

        let executor = WorkflowExecutor::new(graph.clone(), agent.clone(), vec![], config.clone());
        let mut inputs = HashMap::new();
        inputs.insert("in".to_string(), "trigger".to_string());

        let result = executor.execute(inputs).await.unwrap();
        assert_eq!(result, "Processed: True branch");
    }
}
