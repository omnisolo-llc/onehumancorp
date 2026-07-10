#![allow(clippy::all)]
use crate::agent::{Agent, AgentRunConfig};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

/// AutoGPT Unique Harness Innovations: Block-based visual workflow
/// No-code agent assembly via block-connect UI.
/// SOTA Harness Patterns (2025-2026): 3. Visual/low-code orchestration -> democratizing agent construction

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum NodeType {
    Llm {
        prompt_template: String,
    },
    Tool {
        tool_name: String,
        args_template: String,
    },
    Condition {
        condition_expression: String,
        true_target: String,
        false_target: String,
    },
    Input {
        name: String,
    },
    Output,
    SubAgent {
        agent_name: String,
        task_template: String,
    },
    HumanInLoop {
        prompt_template: String,
    },
    Merge {
        state_keys: Vec<String>,
        output_key: String,
    },
    ParallelFork {
        targets: Vec<String>,
    },
    ParallelJoin {
        state_keys: Vec<String>,
        output_key: String,
    },
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
// Visual workflow orchestration
pub struct WorkflowGraph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

pub struct WorkflowExecutor {
    pub graph: WorkflowGraph,
    pub agent: Arc<Agent>,
    pub tools: Vec<crate::tools::Tool>,
    pub sub_agents: HashMap<String, Arc<Agent>>,
    pub config: AgentRunConfig,
    pub checkpointer: Option<Arc<dyn crate::checkpointer::CheckpointSaver>>,
    pub cache: Option<Arc<tokio::sync::Mutex<HashMap<String, String>>>>,
}

fn evaluate_condition(expr: &str) -> bool {
    let operators = ["==", "!=", ">=", "<=", ">", "<"];
    for op in operators {
        if let Some(idx) = expr.find(op) {
            let left = expr[..idx].trim();
            let right = expr[idx + op.len()..].trim();

            let num_left = left.parse::<f64>();
            let num_right = right.parse::<f64>();

            return match op {
                "==" => left == right,
                "!=" => left != right,
                ">" => {
                    if let (Ok(l), Ok(r)) = (&num_left, &num_right) {
                        l > r
                    } else {
                        left > right
                    }
                }
                "<" => {
                    if let (Ok(l), Ok(r)) = (&num_left, &num_right) {
                        l < r
                    } else {
                        left < right
                    }
                }
                ">=" => {
                    if let (Ok(l), Ok(r)) = (&num_left, &num_right) {
                        l >= r
                    } else {
                        left >= right
                    }
                }
                "<=" => {
                    if let (Ok(l), Ok(r)) = (&num_left, &num_right) {
                        l <= r
                    } else {
                        left <= right
                    }
                }
                _ => false,
            };
        }
    }
    expr.trim().eq_ignore_ascii_case("true")
}

pub trait BlockConnectUI: Send + Sync {
    fn generate_ui_schema(&self) -> String;
    fn validate_connection(&self, source_node: &Node, target_node: &Node) -> Result<(), String>;
}

impl BlockConnectUI for WorkflowGraph {
    fn generate_ui_schema(&self) -> String {
        let mut schema = serde_json::json!({
            "nodes": [],
            "edges": []
        });

        for node in &self.nodes {
            schema["nodes"].as_array_mut().unwrap().push(serde_json::json!({
                "id": node.id,
                "type": match node.node_type {
                    NodeType::Llm { .. } => "Llm",
                    NodeType::Tool { .. } => "Tool",
                    NodeType::Condition { .. } => "Condition",
                    NodeType::Input { .. } => "Input",
                    NodeType::Output => "Output",
                    NodeType::SubAgent { .. } => "SubAgent",
                    NodeType::HumanInLoop { .. } => "HumanInLoop",
                    NodeType::Merge { .. } => "Merge",
                    NodeType::ParallelFork { .. } => "ParallelFork",
                    NodeType::ParallelJoin { .. } => "ParallelJoin",
                }
            }));
        }

        for edge in &self.edges {
            schema["edges"].as_array_mut().unwrap().push(serde_json::json!({
                "source": edge.source,
                "target": edge.target
            }));
        }

        serde_json::to_string_pretty(&schema).unwrap_or_default()
    }

    fn validate_connection(&self, source_node: &Node, target_node: &Node) -> Result<(), String> {
        if matches!(source_node.node_type, NodeType::Output) {
            return Err("Output nodes cannot have outgoing connections.".to_string());
        }
        if matches!(target_node.node_type, NodeType::Input { .. }) {
            return Err("Input nodes cannot have incoming connections.".to_string());
        }
        Ok(())
    }
}

impl WorkflowExecutor {
    pub fn new(
        graph: WorkflowGraph,
        agent: Arc<Agent>,
        tools: Vec<crate::tools::Tool>,
        sub_agents: HashMap<String, Arc<Agent>>,
        config: AgentRunConfig,
        checkpointer: Option<Arc<dyn crate::checkpointer::CheckpointSaver>>,
    ) -> Self {
        Self {
            graph,
            agent,
            tools,
            sub_agents,
            config,
            checkpointer,
            cache: None,
        }
    }

    pub fn with_cache(mut self, cache: Arc<tokio::sync::Mutex<HashMap<String, String>>>) -> Self {
        self.cache = Some(cache);
        self
    }

    pub async fn execute(&self, input_vars: HashMap<String, String>) -> Result<String, String> {
        let state: HashMap<String, String> = input_vars.clone();

        // Find input node to start
        let start_node_id = self
            .graph
            .nodes
            .iter()
            .find(|n| matches!(n.node_type, NodeType::Input { .. }))
            .map(|n| n.id.clone())
            .ok_or_else(|| "No input node found in graph".to_string())?;

        let (final_state, stopped_at) = self.execute_from_node(start_node_id, state).await?;

        if let Some(last_node) = stopped_at {
            if let Some(node) = self.graph.nodes.iter().find(|n| n.id == last_node) {
                if matches!(node.node_type, NodeType::Output) {
                    if let Some(edge) = self.graph.edges.iter().find(|e| e.target == last_node)
                        && let Some(val) = final_state.get(&edge.source)
                    {
                        return Ok(val.clone());
                    }
                    return Ok("Visual orchestration completed with no data".to_string());
                }
            }
        }

        Ok("Execution halted without reaching output".to_string())
    }

    pub fn execute_from_node<'a>(
        &'a self,
        start_node_id: String,
        mut state: HashMap<String, String>,
    ) -> std::pin::Pin<
        Box<
            dyn std::future::Future<
                    Output = Result<(HashMap<String, String>, Option<String>), String>,
                > + Send
                + 'a,
        >,
    > {
        Box::pin(async move {
            let mut current_node_id = start_node_id;

            let nodes_map: HashMap<String, Node> = self
                .graph
                .nodes
                .iter()
                .map(|n| (n.id.clone(), n.clone()))
                .collect();

            let mut outgoing_edges: HashMap<String, Vec<String>> = HashMap::new();
            for edge in &self.graph.edges {
                outgoing_edges
                    .entry(edge.source.clone())
                    .or_default()
                    .push(edge.target.clone());
            }

            let mut visit_counts = std::collections::HashMap::new();
            let mut just_finished_fork = false;

            loop {
                let node = nodes_map
                    .get(&current_node_id)
                    .ok_or_else(|| format!("Node not found: {}", current_node_id))?;

                let count = visit_counts.entry(current_node_id.clone()).or_insert(0);

                // Master Catalog B.7. State Management: Checkpoint at super-step boundaries
                if let Some(cp) = &self.checkpointer {
                    let state_json = serde_json::to_value(&state).unwrap_or_default();
                    // We ignore errors in checkpointing so the workflow can continue
                    let _ = cp.put_checkpoint(crate::checkpointer::Checkpoint { thread_id: "visual-workflow".to_string(), checkpoint_id: current_node_id.clone(), data: state_json, parent_id: None, created_at: chrono::Utc::now(), metadata: serde_json::Value::Null });
                }

                *count += 1;

                if *count > self.config.max_workflow_cycles.unwrap_or(1) {
                    return Err("Visual Orchestrator cycle detected".to_string());
                }

                let current_just_finished_fork = just_finished_fork;
                just_finished_fork = false;

                match &node.node_type {
                    NodeType::ParallelJoin {
                        state_keys,
                        output_key,
                    } => {
                        if !current_just_finished_fork {
                            return Ok((state, Some(current_node_id)));
                        } else {
                            let mut merged_data = Vec::new();
                            for key in state_keys {
                                if let Some(val) = state.get(key) {
                                    merged_data.push(val.clone());
                                }
                            }
                            merged_data.sort();
                            let merged_string = serde_json::to_string(&merged_data)
                                .unwrap_or_else(|_| "[]".to_string());
                            state.insert(output_key.clone(), merged_string);
                        }
                    }
                    NodeType::Output => {
                        return Ok((state, Some(current_node_id)));
                    }
                    NodeType::Input { name: _ } => {}
                    NodeType::HumanInLoop { prompt_template } => {
                        let mut prompt = prompt_template.clone();
                        for (k, v) in &state {
                            prompt = prompt.replace(&format!("{{{{{}}}}}", k), v);
                        }
                        return Err(format!("USER_FIXABLE: Human in loop required: {}", prompt));
                    }
                    NodeType::Llm { prompt_template } => {
                        let mut prompt = prompt_template.clone();
                        for (k, v) in &state {
                            prompt = prompt.replace(&format!("{{{{{}}}}}", k), v);
                        }

                        let cache_key = format!("llm:{}", prompt);
                        let mut cached_result = None;

                        if let Some(c) = &self.cache {
                            let lock = c.lock().await;
                            if let Some(val) = lock.get(&cache_key) {
                                cached_result = Some(val.clone());
                            }
                        }

                        let result = if let Some(val) = cached_result {
                            val
                        } else {
                            let mut on_event = |_| {};
                            let res = self
                                .agent
                                .run(&self.config, &prompt, &mut on_event)
                                .await
                                .map_err(|e| format!("LLM node {} failed: {}", node.id, e))?;

                            if let Some(c) = &self.cache {
                                let mut lock = c.lock().await;
                                lock.insert(cache_key, res.clone());
                            }
                            res
                        };

                        state.insert(node.id.clone(), result);
                    }
                    NodeType::Tool {
                        tool_name,
                        args_template,
                    } => {
                        let mut args_json = args_template.clone();
                        for (k, v) in &state {
                            args_json = args_json.replace(&format!("{{{{{}}}}}", k), v);
                        }

                        let args: serde_json::Value =
                            serde_json::from_str(&args_json).map_err(|e| {
                                format!("Tool node {} failed to parse args: {}", node.id, e)
                            })?;

                        let tool = self
                            .tools
                            .iter()
                            .find(|t| &t.name == tool_name)
                            .ok_or_else(|| format!("Tool {} not found", tool_name))?;

                        let result = crate::tool_executor_engine::ToolExecutionEngine::execute_tool_with_langgraph_mechanics(tool, &ohc_builtin_agent_core::types::ToolCall{id: "dynamic".into(), name: tool_name.clone(), arguments: args}, 2, &crate::agent::AgentRunConfig::default()).await;

                        let result_str = match result {
                            Ok(res) => res,
                            Err(ohc_builtin_agent_core::types::ToolError::LlmRecoverable(msg)) => {
                                ohc_builtin_agent_core::types::format_llm_recoverable_error(
                                    &tool_name, &msg,
                                )
                            }
                            Err(ohc_builtin_agent_core::types::ToolError::UserFixable(msg)) => {
                                return Err(format!("USER_FIXABLE: {}", msg));
                            }
                            Err(ohc_builtin_agent_core::types::ToolError::Fatal(msg)) => {
                                return Err(format!("Fatal tool error: {}", msg));
                            }
                            Err(ohc_builtin_agent_core::types::ToolError::Unexpected(msg)) => {
                                return Err(format!("Unexpected tool error: {}", msg));
                            }
                            Err(e) => {
                                return Err(format!("Tool {} execution failed: {}", tool_name, e));
                            }
                        };

                        state.insert(node.id.clone(), result_str);
                    }
                    NodeType::Condition {
                        condition_expression,
                        true_target,
                        false_target,
                    } => {
                        let mut expr = condition_expression.clone();
                        for (k, v) in &state {
                            expr = expr.replace(&format!("{{{{{}}}}}", k), v);
                        }

                        let is_true = evaluate_condition(&expr);

                        current_node_id = if is_true {
                            true_target.clone()
                        } else {
                            false_target.clone()
                        };
                        continue;
                    }
                    NodeType::SubAgent {
                        agent_name,
                        task_template,
                    } => {
                        let mut task = task_template.clone();
                        for (k, v) in &state {
                            task = task.replace(&format!("{{{{{}}}}}", k), v);
                        }

                        let sub_agent = self
                            .sub_agents
                            .get(agent_name)
                            .ok_or_else(|| format!("SubAgent {} not found", agent_name))?;

                        let mut on_event = |_| {};
                        let result = sub_agent
                            .run(&self.config, &task, &mut on_event)
                            .await
                            .map_err(|e| format!("SubAgent node {} failed: {}", node.id, e))?;

                        state.insert(node.id.clone(), result);
                    }
                    NodeType::Merge {
                        state_keys,
                        output_key,
                    } => {
                        let mut merged_data = Vec::new();
                        for key in state_keys {
                            if let Some(val) = state.get(key) {
                                merged_data.push(val.clone());
                            }
                        }
                        merged_data.sort();
                        let merged_string = serde_json::to_string(&merged_data)
                            .unwrap_or_else(|_| "[]".to_string());
                        state.insert(output_key.clone(), merged_string);
                    }
                    NodeType::ParallelFork { targets } => {
                        let mut handles = Vec::new();
                        for target in targets {
                            let target_clone = target.clone();
                            let state_clone = state.clone();

                            let agent_clone = self.agent.clone();
                            let tools_clone = self.tools.clone();
                            let sub_agents_clone = self.sub_agents.clone();
                            let config_clone = self.config.clone();
                            let graph_clone = self.graph.clone();

                            let handle = tokio::spawn(async move {
                                let sub_executor = WorkflowExecutor::new(
                                    graph_clone,
                                    agent_clone,
                                    tools_clone,
                                    sub_agents_clone,
                                    config_clone,
                      None,
                                );
                                sub_executor
                                    .execute_from_node(target_clone, state_clone)
                                    .await
                            });
                            handles.push(handle);
                        }

                        let results = futures::future::join_all(handles).await;

                        let mut join_node_opt = None;

                        for res in results {
                            match res {
                                Ok(Ok((sub_state, sub_join_node))) => {
                                    for (k, v) in sub_state {
                                        state.insert(k, v);
                                    }
                                    if sub_join_node.is_some() {
                                        join_node_opt = sub_join_node;
                                    }
                                }
                                Ok(Err(e)) => {
                                    return Err(format!("Parallel execution failed: {}", e));
                                }
                                Err(e) => return Err(format!("Parallel task join failed: {}", e)),
                            }
                        }

                        if let Some(join_node) = join_node_opt {
                            current_node_id = join_node;
                            just_finished_fork = true;
                            continue;
                        } else {
                            return Ok((state, None));
                        }
                    }
                }

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

            Ok((state, None))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::llm::LlmClient;
    use crate::tools::{Tool, ToolExecutor};
    use crate::types::{ChatRequest, ChatResponse, Message, ToolError, Usage};

    struct MockVisualLlmClient;
    #[async_trait::async_trait]
    impl LlmClient for MockVisualLlmClient {
        async fn chat(
            &self,
            req: ChatRequest,
        ) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
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
    async fn test_visual_workflow_cycle_allowed_by_config() {
        let graph = WorkflowGraph {
            nodes: vec![
                Node {
                    id: "in".to_string(),
                    node_type: NodeType::Input {
                        name: "input_var".to_string(),
                    },
                },
                Node {
                    id: "llm1".to_string(),
                    node_type: NodeType::Llm {
                        prompt_template: "Loop: {{in}}".to_string(),
                    },
                },
                Node {
                    id: "cond".to_string(),
                    node_type: NodeType::Condition {
                        condition_expression: "trigger == trigger".to_string(),
                        true_target: "llm1".to_string(),
                        false_target: "out".to_string(),
                    },
                },
                Node {
                    id: "out".to_string(),
                    node_type: NodeType::Output,
                },
            ],
            edges: vec![
                Edge {
                    source: "in".to_string(),
                    target: "llm1".to_string(),
                },
                Edge {
                    source: "llm1".to_string(),
                    target: "cond".to_string(),
                },
            ],
        };

        let agent = Arc::new(Agent::new(Arc::new(MockVisualLlmClient), vec![]));
        let config = AgentRunConfig {
            max_workflow_cycles: Some(3),
            ..Default::default()
        };

        let executor = WorkflowExecutor::new(graph, agent, vec![], HashMap::new(), config, None);

        let mut inputs = HashMap::new();
        inputs.insert("in".to_string(), "trigger".to_string());

        let result = executor.execute(inputs).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Visual Orchestrator cycle detected");
    }

    #[tokio::test]
    async fn test_visual_workflow_missing_node() {
        let graph = WorkflowGraph {
            nodes: vec![Node {
                id: "in".to_string(),
                node_type: NodeType::Input {
                    name: "input_var".to_string(),
                },
            }],
            edges: vec![Edge {
                source: "in".to_string(),
                target: "missing".to_string(),
            }],
        };

        let agent = Arc::new(Agent::new(Arc::new(MockVisualLlmClient), vec![]));
        let config = AgentRunConfig::default();

        let executor = WorkflowExecutor::new(graph, agent, vec![], HashMap::new(), config, None);
        let mut inputs = HashMap::new();
        inputs.insert("in".to_string(), "trigger".to_string());

        let result = executor.execute(inputs).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Node not found: missing");
    }

    struct ErrorMockVisualLlmClient;
    #[async_trait::async_trait]
    impl LlmClient for ErrorMockVisualLlmClient {
        async fn chat(
            &self,
            _req: ChatRequest,
        ) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            Err("Simulated LLM failure".into())
        }
    }

    #[tokio::test]
    async fn test_visual_workflow_llm_failure() {
        let graph = WorkflowGraph {
            nodes: vec![
                Node {
                    id: "in".to_string(),
                    node_type: NodeType::Input {
                        name: "input_var".to_string(),
                    },
                },
                Node {
                    id: "llm1".to_string(),
                    node_type: NodeType::Llm {
                        prompt_template: "Prompt".to_string(),
                    },
                },
            ],
            edges: vec![Edge {
                source: "in".to_string(),
                target: "llm1".to_string(),
            }],
        };

        let agent = Arc::new(Agent::new(Arc::new(ErrorMockVisualLlmClient), vec![]));
        let config = AgentRunConfig::default();

        let executor = WorkflowExecutor::new(graph, agent, vec![], HashMap::new(), config, None);
        let mut inputs = HashMap::new();
        inputs.insert("in".to_string(), "trigger".to_string());

        let result = executor.execute(inputs).await;
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "LLM node llm1 failed: LLM error: Simulated LLM failure"
        );
    }

    #[tokio::test]
    async fn test_visual_workflow_tool_failure() {
        let graph = WorkflowGraph {
            nodes: vec![
                Node {
                    id: "in".to_string(),
                    node_type: NodeType::Input {
                        name: "input_var".to_string(),
                    },
                },
                Node {
                    id: "tool1".to_string(),
                    node_type: NodeType::Tool {
                        tool_name: "test_tool".to_string(),
                        args_template: "{invalid_json".to_string(),
                    },
                },
            ],
            edges: vec![Edge {
                source: "in".to_string(),
                target: "tool1".to_string(),
            }],
        };

        let agent = Arc::new(Agent::new(Arc::new(MockVisualLlmClient), vec![]));
        let config = AgentRunConfig::default();

        let executor = WorkflowExecutor::new(graph, agent, vec![], HashMap::new(), config, None);
        let mut inputs = HashMap::new();
        inputs.insert("in".to_string(), "trigger".to_string());

        let result = executor.execute(inputs).await;
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .contains("Tool node tool1 failed to parse args")
        );
    }

    #[tokio::test]
    async fn test_visual_workflow_linear() {
        let graph = WorkflowGraph {
            nodes: vec![
                Node {
                    id: "in".to_string(),
                    node_type: NodeType::Input {
                        name: "input_var".to_string(),
                    },
                },
                Node {
                    id: "llm1".to_string(),
                    node_type: NodeType::Llm {
                        prompt_template: "Please format: {{in}}".to_string(),
                    },
                },
                Node {
                    id: "tool1".to_string(),
                    node_type: NodeType::Tool {
                        tool_name: "echo".to_string(),
                        args_template: r#"{"val": "{{llm1}}"}"#.to_string(),
                    },
                },
                Node {
                    id: "out".to_string(),
                    node_type: NodeType::Output,
                },
            ],
            edges: vec![
                Edge {
                    source: "in".to_string(),
                    target: "llm1".to_string(),
                },
                Edge {
                    source: "llm1".to_string(),
                    target: "tool1".to_string(),
                },
                Edge {
                    source: "tool1".to_string(),
                    target: "out".to_string(),
                },
            ],
        };

        let tools = vec![Tool {
            name: "echo".to_string(),
            description: "".to_string(),
            is_read_only: true,
            parameters: serde_json::json!({}),
            execute: Arc::new(MockEchoTool),
        }];

        let agent = Arc::new(Agent::new(Arc::new(MockVisualLlmClient), vec![]));
        let config = AgentRunConfig::default();

        let executor = WorkflowExecutor::new(graph, agent, tools, HashMap::new(), config, None);

        let mut inputs = HashMap::new();
        inputs.insert("in".to_string(), "raw_data".to_string());

        let result = executor.execute(inputs).await.unwrap();
        assert_eq!(result, "Tool echo: Processed: Please format: raw_data");
    }

    #[tokio::test]
    async fn test_visual_workflow_condition() {
        let graph = WorkflowGraph {
            nodes: vec![
                Node {
                    id: "in".to_string(),
                    node_type: NodeType::Input {
                        name: "input_var".to_string(),
                    },
                },
                Node {
                    id: "cond1".to_string(),
                    node_type: NodeType::Condition {
                        condition_expression: "{{in}} == trigger".to_string(),
                        true_target: "out_true".to_string(),
                        false_target: "out_false".to_string(),
                    },
                },
                Node {
                    id: "out_true".to_string(),
                    node_type: NodeType::Llm {
                        prompt_template: "True branch".to_string(),
                    },
                },
                Node {
                    id: "out_false".to_string(),
                    node_type: NodeType::Llm {
                        prompt_template: "False branch".to_string(),
                    },
                },
                Node {
                    id: "out".to_string(),
                    node_type: NodeType::Output,
                },
            ],
            edges: vec![
                Edge {
                    source: "in".to_string(),
                    target: "cond1".to_string(),
                },
                Edge {
                    source: "out_true".to_string(),
                    target: "out".to_string(),
                },
            ],
        };

        let agent = Arc::new(Agent::new(Arc::new(MockVisualLlmClient), vec![]));
        let config = AgentRunConfig::default();

        let executor = WorkflowExecutor::new(
            graph.clone(),
            agent.clone(),
            vec![],
            HashMap::new(),
            config.clone(),
            None,
        );
        let mut inputs = HashMap::new();
        inputs.insert("in".to_string(), "trigger".to_string());

        let result = executor.execute(inputs).await.unwrap();
        assert_eq!(result, "Processed: True branch");
    }

    #[tokio::test]
    async fn test_visual_workflow_cycle() {
        let graph = WorkflowGraph {
            nodes: vec![
                Node {
                    id: "in".to_string(),
                    node_type: NodeType::Input {
                        name: "input_var".to_string(),
                    },
                },
                Node {
                    id: "llm1".to_string(),
                    node_type: NodeType::Llm {
                        prompt_template: "Loop: {{in}}".to_string(),
                    },
                },
                Node {
                    id: "llm2".to_string(),
                    node_type: NodeType::Llm {
                        prompt_template: "Loop2: {{llm1}}".to_string(),
                    },
                },
                Node {
                    id: "out".to_string(),
                    node_type: NodeType::Output,
                },
            ],
            edges: vec![
                Edge {
                    source: "in".to_string(),
                    target: "llm1".to_string(),
                },
                Edge {
                    source: "llm1".to_string(),
                    target: "llm2".to_string(),
                },
                Edge {
                    source: "llm2".to_string(),
                    target: "llm1".to_string(),
                }, // CYCLE
            ],
        };

        let agent = Arc::new(Agent::new(Arc::new(MockVisualLlmClient), vec![]));
        let config = AgentRunConfig::default();

        let executor = WorkflowExecutor::new(graph, agent, vec![], HashMap::new(), config, None);

        let mut inputs = HashMap::new();
        inputs.insert("in".to_string(), "trigger".to_string());

        let result = executor.execute(inputs).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Visual Orchestrator cycle detected");
    }

    #[tokio::test]
    async fn test_ml_resilience_visual_workflow_timeout() {
        let graph = WorkflowGraph {
            nodes: vec![
                Node {
                    id: "in".to_string(),
                    node_type: NodeType::Input {
                        name: "input_var".to_string(),
                    },
                },
                Node {
                    id: "llm1".to_string(),
                    node_type: NodeType::Llm {
                        prompt_template: "Should timeout: {{in}}".to_string(),
                    },
                },
                Node {
                    id: "out".to_string(),
                    node_type: NodeType::Output,
                },
            ],
            edges: vec![
                Edge {
                    source: "in".to_string(),
                    target: "llm1".to_string(),
                },
                Edge {
                    source: "llm1".to_string(),
                    target: "out".to_string(),
                },
            ],
        };

        struct TimeoutLlmClient;
        #[async_trait::async_trait]
        impl crate::llm::LlmClient for TimeoutLlmClient {
            async fn chat(
                &self,
                _req: crate::types::ChatRequest,
            ) -> Result<crate::types::ChatResponse, Box<dyn std::error::Error + Send + Sync>>
            {
                // Simulate a hang that exceeds a 60-second limit (or any other timeout logic).
                // We'll just return an error directly simulating a timeout failure from the runtime.
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                Err("LLM Request Timed Out".into())
            }
        }

        let agent = Arc::new(Agent::new(Arc::new(TimeoutLlmClient), vec![]));
        let mut config = AgentRunConfig::default();
        config.max_retries = 0; // Prevent retries so the test completes quickly

        let executor = WorkflowExecutor::new(graph, agent, vec![], HashMap::new(), config, None);

        let mut inputs = HashMap::new();
        inputs.insert("in".to_string(), "trigger".to_string());

        let result = tokio::time::timeout(
            std::time::Duration::from_millis(1500),
            executor.execute(inputs),
        )
        .await;

        // Verify that the executor gracefully bubbles up the LLM error instead of panicking
        assert!(
            result.is_ok(),
            "Workflow execution should not hang indefinitely"
        );
        let inner_result = result.unwrap();
        assert!(
            inner_result.is_err(),
            "Workflow execution must fail when LLM times out"
        );
        // The error message is formatted as "LLM node {} failed: {}"
        assert!(inner_result.unwrap_err().contains("LLM Request Timed Out"));
    }

    #[tokio::test]
    async fn test_visual_workflow_human_in_loop() {
        let graph = WorkflowGraph {
            nodes: vec![
                Node {
                    id: "in".to_string(),
                    node_type: NodeType::Input {
                        name: "input_var".to_string(),
                    },
                },
                Node {
                    id: "human".to_string(),
                    node_type: NodeType::HumanInLoop {
                        prompt_template: "Please approve this text: {{input_var}}".to_string(),
                    },
                },
            ],
            edges: vec![Edge {
                source: "in".to_string(),
                target: "human".to_string(),
            }],
        };

        let config = AgentRunConfig::default();

        let mut inputs = HashMap::new();
        inputs.insert("input_var".to_string(), "test".to_string());

        struct EmptyMockLlmClient;
        #[async_trait::async_trait]
        impl crate::llm::LlmClient for EmptyMockLlmClient {
            async fn chat(
                &self,
                _req: crate::types::ChatRequest,
            ) -> Result<crate::types::ChatResponse, Box<dyn std::error::Error + Send + Sync>>
            {
                Ok(crate::types::ChatResponse {
                    message: crate::types::Message::assistant("mock"),
                    usage: crate::types::Usage::default(),
                    stop_reason: "".to_string(),
                    response_id: Some("123".to_string()),
                })
            }
        }

        let agent = Arc::new(Agent::new(Arc::new(EmptyMockLlmClient), vec![]));
        let executor = WorkflowExecutor::new(graph, agent, vec![], HashMap::new(), config, None);

        let res = executor.execute(inputs).await;
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err(),
            "USER_FIXABLE: Human in loop required: Please approve this text: test"
        );
    }

    #[tokio::test]
    async fn test_visual_workflow_subagent() {
        let graph = WorkflowGraph {
            nodes: vec![
                Node {
                    id: "in".to_string(),
                    node_type: NodeType::Input {
                        name: "input_var".to_string(),
                    },
                },
                Node {
                    id: "sub1".to_string(),
                    node_type: NodeType::SubAgent {
                        agent_name: "test_sub".to_string(),
                        task_template: "Run task: {{in}}".to_string(),
                    },
                },
                Node {
                    id: "out".to_string(),
                    node_type: NodeType::Output,
                },
            ],
            edges: vec![
                Edge {
                    source: "in".to_string(),
                    target: "sub1".to_string(),
                },
                Edge {
                    source: "sub1".to_string(),
                    target: "out".to_string(),
                },
            ],
        };

        struct MockSubAgentLlmClient;
        #[async_trait::async_trait]
        impl LlmClient for MockSubAgentLlmClient {
            async fn chat(
                &self,
                req: ChatRequest,
            ) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
                let last_user = req.messages.last().unwrap().content.clone();
                Ok(ChatResponse {
                    message: Message::assistant(format!("SubAgent Output: {}", last_user)),
                    usage: Usage::default(),
                    stop_reason: "stop".to_string(),
                    response_id: Some("sub_id1".to_string()),
                })
            }
        }

        let main_agent = Arc::new(Agent::new(Arc::new(MockVisualLlmClient), vec![]));
        let sub_agent = Arc::new(Agent::new(Arc::new(MockSubAgentLlmClient), vec![]));

        let mut sub_agents = HashMap::new();
        sub_agents.insert("test_sub".to_string(), sub_agent);

        let config = AgentRunConfig::default();

        let executor = WorkflowExecutor::new(graph, main_agent, vec![], sub_agents, config, None);

        let mut inputs = HashMap::new();
        inputs.insert("in".to_string(), "my_task_data".to_string());

        let result = executor.execute(inputs).await.unwrap();
        assert_eq!(result, "SubAgent Output: Run task: my_task_data");
    }

    #[tokio::test]
    async fn test_visual_workflow_merge() {
        let graph = WorkflowGraph {
            nodes: vec![
                Node {
                    id: "in1".to_string(),
                    node_type: NodeType::Input {
                        name: "input_1".to_string(),
                    },
                },
                Node {
                    id: "merge1".to_string(),
                    node_type: NodeType::Merge {
                        state_keys: vec!["in1".to_string(), "in2".to_string()],
                        output_key: "merged".to_string(),
                    },
                },
                Node {
                    id: "llm1".to_string(),
                    node_type: NodeType::Llm {
                        prompt_template: "Merged is {{merged}}".to_string(),
                    },
                },
                Node {
                    id: "out".to_string(),
                    node_type: NodeType::Output,
                },
            ],
            edges: vec![
                Edge {
                    source: "in1".to_string(),
                    target: "merge1".to_string(),
                },
                Edge {
                    source: "merge1".to_string(),
                    target: "llm1".to_string(),
                },
                Edge {
                    source: "llm1".to_string(),
                    target: "out".to_string(),
                },
            ],
        };

        let main_agent = Arc::new(Agent::new(Arc::new(MockVisualLlmClient), vec![]));
        let config = AgentRunConfig::default();

        let executor = WorkflowExecutor::new(graph, main_agent, vec![], HashMap::new(), config, None);

        let mut inputs = HashMap::new();
        inputs.insert("in1".to_string(), "val1".to_string());
        inputs.insert("in2".to_string(), "val2".to_string());

        let result = executor.execute(inputs).await.unwrap();
        assert_eq!(result, "Processed: Merged is [\"val1\",\"val2\"]");
    }

    #[tokio::test]
    async fn test_visual_workflow_parallel_fork_join() {
        let graph = WorkflowGraph {
            nodes: vec![
                Node {
                    id: "in".to_string(),
                    node_type: NodeType::Input {
                        name: "input_var".to_string(),
                    },
                },
                Node {
                    id: "fork1".to_string(),
                    node_type: NodeType::ParallelFork {
                        targets: vec!["pathA".to_string(), "pathB".to_string()],
                    },
                },
                Node {
                    id: "pathA".to_string(),
                    node_type: NodeType::Llm {
                        prompt_template: "A Processed: {{in}}".to_string(),
                    },
                },
                Node {
                    id: "pathB".to_string(),
                    node_type: NodeType::Llm {
                        prompt_template: "B Processed: {{in}}".to_string(),
                    },
                },
                Node {
                    id: "join1".to_string(),
                    node_type: NodeType::ParallelJoin {
                        state_keys: vec!["pathA".to_string(), "pathB".to_string()],
                        output_key: "joined".to_string(),
                    },
                },
                Node {
                    id: "out_llm".to_string(),
                    node_type: NodeType::Llm {
                        prompt_template: "Final: {{joined}}".to_string(),
                    },
                },
                Node {
                    id: "out".to_string(),
                    node_type: NodeType::Output,
                },
            ],
            edges: vec![
                Edge {
                    source: "in".to_string(),
                    target: "fork1".to_string(),
                },
                Edge {
                    source: "pathA".to_string(),
                    target: "join1".to_string(),
                },
                Edge {
                    source: "pathB".to_string(),
                    target: "join1".to_string(),
                },
                Edge {
                    source: "join1".to_string(),
                    target: "out_llm".to_string(),
                },
                Edge {
                    source: "out_llm".to_string(),
                    target: "out".to_string(),
                },
            ],
        };

        let main_agent = Arc::new(Agent::new(Arc::new(MockVisualLlmClient), vec![]));
        let config = AgentRunConfig::default();

        let executor = WorkflowExecutor::new(graph, main_agent, vec![], HashMap::new(), config, None);

        let mut inputs = HashMap::new();
        inputs.insert("in".to_string(), "init_data".to_string());

        let result = executor.execute(inputs).await.unwrap();

        assert!(result.contains("Processed: Final: [\"Processed: A Processed: init_data\",\"Processed: B Processed: init_data\"]"),
                "Result was: {}", result);
    }

    #[tokio::test]
    async fn test_visual_workflow_nested_parallel_fork() {
        let graph = WorkflowGraph {
            nodes: vec![
                Node {
                    id: "in".to_string(),
                    node_type: NodeType::Input {
                        name: "input_var".to_string(),
                    },
                },
                Node {
                    id: "fork_outer".to_string(),
                    node_type: NodeType::ParallelFork {
                        targets: vec!["path_outer_a".to_string(), "path_outer_b".to_string()],
                    },
                },
                Node {
                    id: "path_outer_a".to_string(),
                    node_type: NodeType::Llm {
                        prompt_template: "Outer A: {{in}}".to_string(),
                    },
                },
                Node {
                    id: "path_outer_b".to_string(),
                    node_type: NodeType::ParallelFork {
                        targets: vec!["path_inner_b1".to_string(), "path_inner_b2".to_string()],
                    },
                },
                Node {
                    id: "path_inner_b1".to_string(),
                    node_type: NodeType::Llm {
                        prompt_template: "Inner B1: {{in}}".to_string(),
                    },
                },
                Node {
                    id: "path_inner_b2".to_string(),
                    node_type: NodeType::Llm {
                        prompt_template: "Inner B2: {{in}}".to_string(),
                    },
                },
                Node {
                    id: "join_inner_b".to_string(),
                    node_type: NodeType::ParallelJoin {
                        state_keys: vec!["path_inner_b1".to_string(), "path_inner_b2".to_string()],
                        output_key: "joined_inner".to_string(),
                    },
                },
                Node {
                    id: "path_outer_b_post_join".to_string(),
                    node_type: NodeType::Llm {
                        prompt_template: "Outer B post join: {{joined_inner}}".to_string(),
                    },
                },
                Node {
                    id: "join_outer".to_string(),
                    node_type: NodeType::ParallelJoin {
                        state_keys: vec![
                            "path_outer_a".to_string(),
                            "path_outer_b_post_join".to_string(),
                        ],
                        output_key: "joined_outer".to_string(),
                    },
                },
                Node {
                    id: "out_llm".to_string(),
                    node_type: NodeType::Llm {
                        prompt_template: "Final Nested: {{joined_outer}}".to_string(),
                    },
                },
                Node {
                    id: "out".to_string(),
                    node_type: NodeType::Output,
                },
            ],
            edges: vec![
                Edge {
                    source: "in".to_string(),
                    target: "fork_outer".to_string(),
                },
                Edge {
                    source: "path_outer_a".to_string(),
                    target: "join_outer".to_string(),
                },
                Edge {
                    source: "path_outer_b".to_string(),
                    target: "join_inner_b".to_string(),
                }, // fork edge implicitly starts targets, this edge just ensures topology connectivity if needed by other components, although WorkflowExecutor follows target arrays directly. We will explicitly route inner targets to inner join
                Edge {
                    source: "path_inner_b1".to_string(),
                    target: "join_inner_b".to_string(),
                },
                Edge {
                    source: "path_inner_b2".to_string(),
                    target: "join_inner_b".to_string(),
                },
                Edge {
                    source: "join_inner_b".to_string(),
                    target: "path_outer_b_post_join".to_string(),
                },
                Edge {
                    source: "path_outer_b_post_join".to_string(),
                    target: "join_outer".to_string(),
                },
                Edge {
                    source: "join_outer".to_string(),
                    target: "out_llm".to_string(),
                },
                Edge {
                    source: "out_llm".to_string(),
                    target: "out".to_string(),
                },
            ],
        };

        let main_agent = Arc::new(Agent::new(Arc::new(MockVisualLlmClient), vec![]));
        let config = AgentRunConfig::default();

        let executor = WorkflowExecutor::new(graph, main_agent, vec![], HashMap::new(), config, None);

        let mut inputs = HashMap::new();
        inputs.insert("in".to_string(), "root_data".to_string());

        let result = executor.execute(inputs).await.unwrap();

        assert!(
            result.contains("Processed: Final Nested: "),
            "Result missing final prefix: {}",
            result
        );
        assert!(
            result.contains("Outer A: root_data"),
            "Result missing Outer A: {}",
            result
        );
        assert!(
            result.contains("Inner B1: root_data"),
            "Result missing Inner B1: {}",
            result
        );
        assert!(
            result.contains("Inner B2: root_data"),
            "Result missing Inner B2: {}",
            result
        );
    }
}

#[cfg(test)]
mod additional_tests {
    use super::*;

    #[tokio::test]
    async fn test_visual_workflow_llm_node_cache() {
        let graph = WorkflowGraph {
            nodes: vec![
                Node {
                    id: "in".to_string(),
                    node_type: NodeType::Input {
                        name: "input_var".to_string(),
                    },
                },
                Node {
                    id: "llm1".to_string(),
                    node_type: NodeType::Llm {
                        prompt_template: "Cache test: {{in}}".to_string(),
                    },
                },
                Node {
                    id: "out".to_string(),
                    node_type: NodeType::Output,
                },
            ],
            edges: vec![
                Edge {
                    source: "in".to_string(),
                    target: "llm1".to_string(),
                },
                Edge {
                    source: "llm1".to_string(),
                    target: "out".to_string(),
                },
            ],
        };

        struct CallCountingLlmClient {
            call_count: Arc<tokio::sync::Mutex<usize>>,
        }

        #[async_trait::async_trait]
        impl crate::llm::LlmClient for CallCountingLlmClient {
            async fn chat(
                &self,
                _req: crate::types::ChatRequest,
            ) -> Result<crate::types::ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
                let mut count = self.call_count.lock().await;
                *count += 1;
                Ok(crate::types::ChatResponse {
                    message: crate::types::Message::assistant("From LLM"),
                    usage: crate::types::Usage::default(),
                    stop_reason: "stop".to_string(),
                    response_id: Some("id1".to_string()),
                })
            }
        }

        let call_count = Arc::new(tokio::sync::Mutex::new(0));
        let llm_client = Arc::new(CallCountingLlmClient { call_count: call_count.clone() });
        let agent = Arc::new(Agent::new(llm_client, vec![]));
        let config = AgentRunConfig::default();
        let cache = Arc::new(tokio::sync::Mutex::new(HashMap::new()));

        // Run 1: Should call LLM
        let executor1 = WorkflowExecutor::new(graph.clone(), agent.clone(), vec![], HashMap::new(), config.clone(), None)
            .with_cache(cache.clone());
        let mut inputs = HashMap::new();
        inputs.insert("in".to_string(), "data1".to_string());

        let res1 = executor1.execute(inputs.clone()).await.unwrap();
        assert_eq!(res1, "From LLM");
        let count1 = *call_count.lock().await;
        assert_eq!(count1, 1);

        // We inject a fake response into the cache for the exact same prompt to prove cache hit works
        {
            let mut cache_lock = cache.lock().await;
            cache_lock.insert("llm:Cache test: data1".to_string(), "From Cache".to_string());
        }

        // Run 2: Should hit cache, not the LLM
        let executor2 = WorkflowExecutor::new(graph.clone(), agent.clone(), vec![], HashMap::new(), config.clone(), None)
            .with_cache(cache.clone());

        let res2 = executor2.execute(inputs).await.unwrap();
        assert_eq!(res2, "From Cache");
        let count2 = *call_count.lock().await;
        // Call count should remain 1
        assert_eq!(count2, 1);
    }

    #[tokio::test]
    async fn test_visual_workflow_deep_nesting_and_json_parsing() {
        let graph = WorkflowGraph {
            nodes: vec![
                Node {
                    id: "in".to_string(),
                    node_type: NodeType::Input {
                        name: "input_json".to_string(),
                    },
                },
                Node {
                    id: "fork".to_string(),
                    node_type: NodeType::ParallelFork {
                        targets: vec!["pathA".to_string(), "pathB".to_string()],
                    },
                },
                Node {
                    id: "pathA".to_string(),
                    node_type: NodeType::Llm {
                        prompt_template: "Process A: {{in}}".to_string(),
                    },
                },
                Node {
                    id: "pathB".to_string(),
                    node_type: NodeType::Llm {
                        prompt_template: "Process B: {{in}}".to_string(),
                    },
                },
                Node {
                    id: "join".to_string(),
                    node_type: NodeType::ParallelJoin {
                        state_keys: vec!["pathA".to_string(), "pathB".to_string()],
                        output_key: "joined".to_string(),
                    },
                },
                Node {
                    id: "out".to_string(),
                    node_type: NodeType::Output,
                },
            ],
            edges: vec![
                Edge {
                    source: "in".to_string(),
                    target: "fork".to_string(),
                },
                Edge {
                    source: "join".to_string(),
                    target: "out".to_string(),
                },
            ],
        };

        let mut config = AgentRunConfig::default();
        config.developer_instructions = "test".to_string();

        let mut inputs = HashMap::new();
        inputs.insert("in".to_string(), r#"{"key": "value"}"#.to_string());

        assert_eq!(graph.nodes.len(), 6);
        assert_eq!(graph.edges.len(), 2);
    }

    #[test]
    fn test_visual_workflow_block_connect_ui_schema() {
        let graph = WorkflowGraph {
            nodes: vec![
                Node {
                    id: "in".to_string(),
                    node_type: NodeType::Input {
                        name: "input_json".to_string(),
                    },
                },
                Node {
                    id: "out".to_string(),
                    node_type: NodeType::Output,
                },
            ],
            edges: vec![
                Edge {
                    source: "in".to_string(),
                    target: "out".to_string(),
                },
            ],
        };

        let schema = graph.generate_ui_schema();
        assert!(schema.contains(r#""id": "in""#));
        assert!(schema.contains(r#""type": "Input""#));
        assert!(schema.contains(r#""id": "out""#));
        assert!(schema.contains(r#""type": "Output""#));
        assert!(schema.contains(r#""source": "in""#));
        assert!(schema.contains(r#""target": "out""#));
    }

    #[test]
    fn test_visual_workflow_block_connect_ui_validation() {
        let graph = WorkflowGraph { nodes: vec![], edges: vec![] };
        let in_node = Node { id: "in".to_string(), node_type: NodeType::Input { name: "input_json".to_string() } };
        let out_node = Node { id: "out".to_string(), node_type: NodeType::Output };

        let res1 = graph.validate_connection(&out_node, &in_node);
        assert!(res1.is_err());
        let res2 = graph.validate_connection(&in_node, &out_node);
        assert!(res2.is_ok());
    }
}
