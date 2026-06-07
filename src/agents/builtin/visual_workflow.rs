use crate::agent::{Agent, AgentRunConfig};

use std::collections::HashMap;
use std::sync::Arc;
use serde::{Deserialize, Serialize};

/// AutoGPT Unique Harness Innovations: Block-based visual workflow
/// No-code agent assembly via block-connect UI.
/// SOTA Harness Patterns (2025-2026): 3. Visual/low-code orchestration -> democratizing agent construction

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum NodeType {
    Llm { prompt_template: String, max_retries: Option<usize> },
    Tool { tool_name: String, args_template: String, max_retries: Option<usize> },
    Condition { condition_expression: String, true_target: String, false_target: String },
    Input { name: String },
    Output,
    SubAgent { agent_name: String, task_template: String, max_retries: Option<usize> },
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
pub struct WorkflowCheckpoint {
    pub state: HashMap<String, String>,
    pub current_node_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowGraph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

pub struct WorkflowExecutor {
    pub is_sub_executor: bool,
    pub graph: WorkflowGraph,
    pub agent: Arc<Agent>,
    pub tools: Vec<crate::tools::Tool>,
    pub sub_agents: HashMap<String, Arc<Agent>>,
    pub config: AgentRunConfig,
}

impl WorkflowExecutor {
    pub fn new(graph: WorkflowGraph, agent: Arc<Agent>, tools: Vec<crate::tools::Tool>, sub_agents: HashMap<String, Arc<Agent>>, config: AgentRunConfig) -> Self {
        Self { graph, agent, tools, sub_agents, config, is_sub_executor: false }
    }

    pub async fn execute(&self, input_vars: HashMap<String, String>) -> Result<String, String> {
        let current_node_id = self.graph.nodes.iter()
            .find(|n| matches!(n.node_type, NodeType::Input { .. }))
            .map(|n| n.id.clone())
            .ok_or_else(|| "No input node found in graph".to_string())?;

        match self.execute_from_node(current_node_id, input_vars).await {
            Ok((state, _)) => {
                let out_node = self.graph.nodes.iter().find(|n| matches!(n.node_type, NodeType::Output)).unwrap();
                let mut result = String::new();
                for edge in &self.graph.edges {
                    if edge.target == out_node.id {
                        if let Some(val) = state.get(&edge.source) {
                            result.push_str(val);
                        }
                    }
                }
                Ok(result)
            }
            Err(e) => Err(e),
        }
    }

    pub async fn save_checkpoint(&self, checkpoint_path: &str, state: &HashMap<String, String>, current_node_id: &str) -> Result<(), String> {
        let cp = WorkflowCheckpoint {
            state: state.clone(),
            current_node_id: current_node_id.to_string(),
        };
        let data = serde_json::to_string_pretty(&cp).map_err(|e| e.to_string())?;
        tokio::fs::write(checkpoint_path, data).await.map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn load_checkpoint(&self, checkpoint_path: &str) -> Result<WorkflowCheckpoint, String> {
        let data = tokio::fs::read_to_string(checkpoint_path).await.map_err(|e| e.to_string())?;
        let cp: WorkflowCheckpoint = serde_json::from_str(&data).map_err(|e| e.to_string())?;
        Ok(cp)
    }

    pub async fn resume_from_checkpoint(&self, checkpoint_path: &str) -> Result<String, String> {
        let cp = self.load_checkpoint(checkpoint_path).await?;
        match self.execute_from_node(cp.current_node_id, cp.state).await {
            Ok((state, _)) => {
                let out_node = self.graph.nodes.iter().find(|n| matches!(n.node_type, NodeType::Output)).unwrap();
                let mut result = String::new();
                for edge in &self.graph.edges {
                    if edge.target == out_node.id {
                        if let Some(val) = state.get(&edge.source) {
                            result.push_str(val);
                        }
                    }
                }
                Ok(result)
            }
            Err(e) => Err(e),
        }
    }

    pub fn execute_from_node<'a>(&'a self, start_node_id: String, initial_state: HashMap<String, String>) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(HashMap<String, String>, Option<String>), String>> + Send + 'a>> {
        Box::pin(async move {
            let mut state: HashMap<String, String> = initial_state;
            let mut current_node_id = start_node_id;

            let nodes_map: HashMap<String, Node> = self.graph.nodes.iter()
                .map(|n| (n.id.clone(), n.clone()))
                .collect();

            let mut outgoing_edges: HashMap<String, Vec<String>> = HashMap::new();
            for edge in &self.graph.edges {
                outgoing_edges.entry(edge.source.clone()).or_default().push(edge.target.clone());
            }

            let mut visited = std::collections::HashSet::new();

            loop {
                if !self.is_sub_executor {
                    if let Some(cp_path) = std::env::var("OHC_WORKFLOW_CHECKPOINT").ok() {
                        let _ = self.save_checkpoint(&cp_path, &state, &current_node_id).await;
                    }
                }

                let node = nodes_map.get(&current_node_id).ok_or_else(|| format!("Node not found: {}", current_node_id))?;

                if visited.contains(&current_node_id) {
                    return Err("Visual Orchestrator cycle detected".to_string());
                }
                visited.insert(current_node_id.clone());

                match &node.node_type {
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
                                let mut sub_executor = WorkflowExecutor::new(graph_clone, agent_clone, tools_clone, sub_agents_clone, config_clone);
                                sub_executor.is_sub_executor = true;
                                sub_executor.execute_from_node(target_clone, state_clone).await
                            });
                            handles.push(handle);
                        }

                        let results = futures::future::join_all(handles).await;

                        let mut join_node_opt = None;
                        for res in results {
                            match res {
                                Ok(Ok((final_state, stopped_at))) => {
                                    for (k, v) in final_state {
                                        state.insert(k, v);
                                    }
                                    if let Some(join) = stopped_at {
                                        join_node_opt = Some(join);
                                    }
                                }
                                Ok(Err(e)) => return Err(format!("Parallel branch failed: {}", e)),
                                Err(e) => return Err(format!("Task join failed: {}", e)),
                            }
                        }

                        if let Some(join_node) = join_node_opt {
                            if let Some(NodeType::ParallelJoin { state_keys, output_key }) = nodes_map.get(&join_node).map(|n| &n.node_type) {
                                let mut merged_data = Vec::new();
                                for key in state_keys {
                                    if let Some(val) = state.get(key) {
                                        merged_data.push(val.clone());
                                    }
                                }
                                let merged_string = serde_json::to_string(&merged_data).unwrap_or_else(|_| "[]".to_string());
                                state.insert(output_key.clone(), merged_string);
                            }

                            if let Some(targets) = outgoing_edges.get(&join_node) {
                                if let Some(next) = targets.first() {
                                    current_node_id = next.clone();
                                    continue;
                                } else {
                                    return Ok((state, None));
                                }
                            } else {
                                return Ok((state, None));
                            }
                        } else {
                            return Ok((state, None));
                        }
                    }
                    NodeType::ParallelJoin { state_keys: _, output_key: _ } => {
                        return Ok((state, Some(current_node_id.clone())));
                    }
                    NodeType::Input { name: _ } => {
                        // Start execution
                    }
                    NodeType::Llm { prompt_template, max_retries } => {
                        let mut prompt = prompt_template.clone();
                        for (k, v) in &state {
                            prompt = prompt.replace(&format!("{{{{{}}}}}", k), v);
                        }

                        let max_attempts = max_retries.unwrap_or(0) + 1;
                        let mut attempt = 0;
                        let mut _last_err = String::new();
                        let mut result = String::new();

                        while attempt < max_attempts {
                            let mut on_event = |_| {};
                            match self.agent.run(&self.config, &prompt, &mut on_event).await {
                                Ok(res) => {
                                    result = res;
                                    break;
                                }
                                Err(e) => {
                                    attempt += 1;
                                    _last_err = format!("LLM node {} failed (attempt {}/{}): {}", node.id, attempt, max_attempts, e);
                                    if attempt >= max_attempts {
                                        return Err(_last_err);
                                    }
                                    tokio::time::sleep(tokio::time::Duration::from_millis(100 * (1 << attempt))).await;
                                }
                            }
                        }

                        state.insert(current_node_id.clone(), result);
                    }
                    NodeType::Tool { tool_name, args_template, max_retries } => {
                        let mut args_str = args_template.clone();
                        for (k, v) in &state {
                            args_str = args_str.replace(&format!("{{{{{}}}}}", k), v);
                        }

                        let parsed_args: serde_json::Value = serde_json::from_str(&args_str)
                            .map_err(|e| format!("Invalid JSON args for tool {}: {}", tool_name, e))?;

                        let tool = self.tools.iter().find(|t| &t.name == tool_name)
                            .ok_or_else(|| format!("Tool {} not available", tool_name))?;

                        let max_attempts = max_retries.unwrap_or(0) + 1;
                        let mut attempt = 0;
                        let mut _last_err = String::new();
                        let mut result = String::new();

                        while attempt < max_attempts {
                            match crate::tool_executor_engine::ToolExecutionEngine::execute_tool_with_langgraph_mechanics(
                                &tool,
                                &ohc_builtin_agent_core::types::ToolCall { id: "wf_tool".to_string(), name: tool_name.clone(), arguments: parsed_args.clone() },
                                2
                            ).await {
                                Ok(res) => {
                                    result = res;
                                    break;
                                }
                                Err(e) => {
                                    attempt += 1;
                                    _last_err = format!("Tool node {} failed (attempt {}/{}): {}", node.id, attempt, max_attempts, e);
                                    if attempt >= max_attempts {
                                        return Err(_last_err);
                                    }
                                    tokio::time::sleep(tokio::time::Duration::from_millis(100 * (1 << attempt))).await;
                                }
                            }
                        }

                        state.insert(current_node_id.clone(), result);
                    }
                    NodeType::Condition { condition_expression, true_target, false_target } => {
                        let mut expr = condition_expression.clone();
                        for (k, v) in &state {
                            expr = expr.replace(&format!("{{{{{}}}}}", k), v);
                        }

                        let parts: Vec<&str> = expr.split("==").map(|s| s.trim()).collect();
                        let mut is_true = false;
                        if parts.len() == 2 {
                            is_true = parts[0] == parts[1];
                        }

                        if is_true {
                            current_node_id = true_target.clone();
                        } else {
                            current_node_id = false_target.clone();
                        }
                        continue;
                    }
                    NodeType::SubAgent { agent_name, task_template, max_retries } => {
                        let mut task = task_template.clone();
                        for (k, v) in &state {
                            task = task.replace(&format!("{{{{{}}}}}", k), v);
                        }

                        let sub_agent = self.sub_agents.get(agent_name)
                            .ok_or_else(|| format!("SubAgent {} not found", agent_name))?;

                        let max_attempts = max_retries.unwrap_or(0) + 1;
                        let mut attempt = 0;
                        let mut _last_err = String::new();
                        let mut result = String::new();

                        while attempt < max_attempts {
                            let mut on_event = |_| {};
                            match sub_agent.run(&self.config, &task, &mut on_event).await {
                                Ok(res) => {
                                    result = res;
                                    break;
                                }
                                Err(e) => {
                                    attempt += 1;
                                    _last_err = format!("SubAgent node {} failed (attempt {}/{}): {}", node.id, attempt, max_attempts, e);
                                    if attempt >= max_attempts {
                                        return Err(_last_err);
                                    }
                                    tokio::time::sleep(tokio::time::Duration::from_millis(100 * (1 << attempt))).await;
                                }
                            }
                        }

                        state.insert(current_node_id.clone(), result);
                    }
                    NodeType::Merge { state_keys, output_key } => {
                        let mut merged_data = Vec::new();
                        for key in state_keys {
                            if let Some(val) = state.get(key) {
                                merged_data.push(val.clone());
                            }
                        }
                        let merged_string = serde_json::to_string(&merged_data).unwrap_or_else(|_| "[]".to_string());
                        state.insert(output_key.clone(), merged_string);
                    }
                    NodeType::Output => {
                        return Ok((state, Some(current_node_id.clone())));
                    }
                }

                if let Some(targets) = outgoing_edges.get(&current_node_id) {
                    if let Some(next) = targets.first() {
                        current_node_id = next.clone();
                    } else {
                        return Ok((state, None));
                    }
                } else {
                    return Ok((state, None));
                }
            }
        })
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
                Node { id: "llm1".to_string(), node_type: NodeType::Llm { prompt_template: "Please format: {{in}}".to_string(), max_retries: None } },
                Node { id: "tool1".to_string(), node_type: NodeType::Tool { tool_name: "echo".to_string(), args_template: r#"{"val": "{{llm1}}"}"#.to_string(), max_retries: None } },
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

        let executor = WorkflowExecutor::new(graph, agent, tools, HashMap::new(), config);

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
                Node { id: "out_true".to_string(), node_type: NodeType::Llm { prompt_template: "True branch".to_string(), max_retries: None } },
                Node { id: "out_false".to_string(), node_type: NodeType::Llm { prompt_template: "False branch".to_string(), max_retries: None } },
                Node { id: "out".to_string(), node_type: NodeType::Output },
            ],
            edges: vec![
                Edge { source: "in".to_string(), target: "cond1".to_string() },
                Edge { source: "out_true".to_string(), target: "out".to_string() },
            ],
        };

        let agent = Arc::new(Agent::new(Arc::new(MockVisualLlmClient), vec![]));
        let config = AgentRunConfig::default();

        let executor = WorkflowExecutor::new(graph.clone(), agent.clone(), vec![], HashMap::new(), config.clone());
        let mut inputs = HashMap::new();
        inputs.insert("in".to_string(), "trigger".to_string());

        let result = executor.execute(inputs).await.unwrap();
        assert_eq!(result, "Processed: True branch");
    }

    #[tokio::test]
    async fn test_visual_workflow_cycle() {
        let graph = WorkflowGraph {
            nodes: vec![
                Node { id: "in".to_string(), node_type: NodeType::Input { name: "input_var".to_string() } },
                Node { id: "llm1".to_string(), node_type: NodeType::Llm { prompt_template: "Loop: {{in}}".to_string(), max_retries: None } },
                Node { id: "llm2".to_string(), node_type: NodeType::Llm { prompt_template: "Loop2: {{llm1}}".to_string(), max_retries: None } },
                Node { id: "out".to_string(), node_type: NodeType::Output },
            ],
            edges: vec![
                Edge { source: "in".to_string(), target: "llm1".to_string() },
                Edge { source: "llm1".to_string(), target: "llm2".to_string() },
                Edge { source: "llm2".to_string(), target: "llm1".to_string() }, // CYCLE
            ],
        };

        let agent = Arc::new(Agent::new(Arc::new(MockVisualLlmClient), vec![]));
        let config = AgentRunConfig::default();

        let executor = WorkflowExecutor::new(graph, agent, vec![], HashMap::new(), config);

        let mut inputs = HashMap::new();
        inputs.insert("in".to_string(), "trigger".to_string());

        let result = executor.execute(inputs).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Visual Orchestrator cycle detected");
    }


    #[tokio::test]
    async fn test_visual_workflow_subagent() {
        let graph = WorkflowGraph {
            nodes: vec![
                Node { id: "in".to_string(), node_type: NodeType::Input { name: "input_var".to_string() } },
                Node { id: "sub1".to_string(), node_type: NodeType::SubAgent { agent_name: "test_sub".to_string(), task_template: "Run task: {{in}}".to_string(), max_retries: None } },
                Node { id: "out".to_string(), node_type: NodeType::Output },
            ],
            edges: vec![
                Edge { source: "in".to_string(), target: "sub1".to_string() },
                Edge { source: "sub1".to_string(), target: "out".to_string() },
            ],
        };

        struct MockSubAgentLlmClient;
        #[async_trait::async_trait]
        impl LlmClient for MockSubAgentLlmClient {
            async fn chat(&self, req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
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

        let executor = WorkflowExecutor::new(graph, main_agent, vec![], sub_agents, config);

        let mut inputs = HashMap::new();
        inputs.insert("in".to_string(), "my_task_data".to_string());

        let result = executor.execute(inputs).await.unwrap();
        assert_eq!(result, "SubAgent Output: Run task: my_task_data");
    }

    #[tokio::test]
    async fn test_visual_workflow_merge() {
        let graph = WorkflowGraph {
            nodes: vec![
                Node { id: "in1".to_string(), node_type: NodeType::Input { name: "input_1".to_string() } },
                Node { id: "merge1".to_string(), node_type: NodeType::Merge { state_keys: vec!["in1".to_string(), "in2".to_string()], output_key: "merged".to_string() } },
                Node { id: "llm1".to_string(), node_type: NodeType::Llm { prompt_template: "Merged is {{merged}}".to_string(), max_retries: None } },
                Node { id: "out".to_string(), node_type: NodeType::Output },
            ],
            edges: vec![
                Edge { source: "in1".to_string(), target: "merge1".to_string() },
                Edge { source: "merge1".to_string(), target: "llm1".to_string() },
                Edge { source: "llm1".to_string(), target: "out".to_string() },
            ],
        };

        let main_agent = Arc::new(Agent::new(Arc::new(MockVisualLlmClient), vec![]));
        let config = AgentRunConfig::default();

        let executor = WorkflowExecutor::new(graph, main_agent, vec![], HashMap::new(), config);

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
                Node { id: "in".to_string(), node_type: NodeType::Input { name: "input_var".to_string() } },
                Node { id: "fork1".to_string(), node_type: NodeType::ParallelFork { targets: vec!["pathA".to_string(), "pathB".to_string()] } },
                Node { id: "pathA".to_string(), node_type: NodeType::Llm { prompt_template: "A Processed: {{in}}".to_string(), max_retries: None } },
                Node { id: "pathB".to_string(), node_type: NodeType::Llm { prompt_template: "B Processed: {{in}}".to_string(), max_retries: None } },
                Node { id: "join1".to_string(), node_type: NodeType::ParallelJoin { state_keys: vec!["pathA".to_string(), "pathB".to_string()], output_key: "joined".to_string() } },
                Node { id: "out_llm".to_string(), node_type: NodeType::Llm { prompt_template: "Final: {{joined}}".to_string(), max_retries: None } },
                Node { id: "out".to_string(), node_type: NodeType::Output },
            ],
            edges: vec![
                Edge { source: "in".to_string(), target: "fork1".to_string() },
                Edge { source: "pathA".to_string(), target: "join1".to_string() },
                Edge { source: "pathB".to_string(), target: "join1".to_string() },
                Edge { source: "join1".to_string(), target: "out_llm".to_string() },
                Edge { source: "out_llm".to_string(), target: "out".to_string() },
            ],
        };

        let main_agent = Arc::new(Agent::new(Arc::new(MockVisualLlmClient), vec![]));
        let config = AgentRunConfig::default();

        let executor = WorkflowExecutor::new(graph, main_agent, vec![], HashMap::new(), config);

        let mut inputs = HashMap::new();
        inputs.insert("in".to_string(), "init_data".to_string());

        let result = executor.execute(inputs).await.unwrap();

        assert!(result.contains("Processed: Final: [\"Processed: A Processed: init_data\",\"Processed: B Processed: init_data\"]") ||
                result.contains("Processed: Final: [\"Processed: B Processed: init_data\",\"Processed: A Processed: init_data\"]"),
                "Result was: {}", result);
    }
}
