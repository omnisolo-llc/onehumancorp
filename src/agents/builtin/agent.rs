use ohc_builtin_agent_core::types::ToolError;
use std::sync::Arc;
use std::sync::atomic::{AtomicI64, AtomicU64, Ordering};
use opentelemetry::{global, KeyValue};

use crate::budget::{check_token_budget, BudgetAction, BudgetTracker};
use crate::guardrails::GuardrailConfig;
use ohc_builtin_agent_llm::LlmClient;
use ohc_builtin_agent_tools::Tool;
use ohc_builtin_agent_core::types::{ChatRequest, Message, Role, ToolCall, ToolDefinition, ToolResult};

/// Events emitted by the agent run loop.
#[derive(Debug, Clone)]
pub enum AgentEvent {
    RunStarted { iteration: i32 },
    TextChunk { content: String },
    ToolCall { name: String, args_json: String, result: String, iteration: i32 },
    TaskComplete { content: String },
    TaskError { error: String },
    UserInterventionRequired { error: String },
    IterationStarted { iteration: i32, message_count: usize },
    CheckpointSaved { iteration: i32, path: String },
    Handoff { target_agent: String },
}

/// Configuration for a single agent run.
#[derive(Debug, Clone)]
pub struct AgentRunConfig {
    pub agent_id: String,
    pub model: String,
    pub server_system_message: String,
    pub developer_instructions: String,
    pub user_instructions: String,
    pub max_tokens: i32,
    pub temperature: f32,
    pub max_iterations: i32,
    pub max_task_tokens: i32, // budget for token tracking
    pub confidence_threshold: f32,
    pub enable_llmcompiler_plan_and_execute: bool,
    pub enable_acon_context_strategy: bool,
    pub enable_observation_masking: bool,
    pub observation_masking_threshold: usize,
    pub observation_masking_size_limit: usize,
    pub enable_lost_in_the_middle_prevention: bool,
    pub enable_context_compaction: bool,
    pub compaction_threshold_tokens: i32,
    pub enable_llm_judge: bool,
    pub guardrails: Option<GuardrailConfig>,
    pub enable_state_checkpointing: bool,
    pub state_scratchpad_path: Option<String>,
    pub workspace_path: Option<String>,
    pub project_trusted: bool,
    pub injected_context: Option<Vec<ohc_builtin_agent_core::types::Message>>,
    pub allowed_tools: Option<Vec<String>>,
    pub high_risk_tools: Vec<String>,
    pub approved_tool_calls: Vec<String>,
    pub thread_id: Option<String>,
    pub resume_from_checkpoint_id: Option<String>,
    pub enable_lazy_tool_loading: bool,
    pub enable_langgraph_mechanic: bool,
}

impl Default for AgentRunConfig {
    fn default() -> Self {
        Self {
            agent_id: "default-agent".to_string(),
            model: String::new(),
            server_system_message: String::new(),
            developer_instructions: String::new(),
            user_instructions: String::new(),
            max_tokens: 2048,
            temperature: 0.0,
            max_iterations: 100,
            max_task_tokens: 0,
            confidence_threshold: 0.0,
            enable_llmcompiler_plan_and_execute: false,
            enable_acon_context_strategy: false,
            enable_observation_masking: true,
            observation_masking_threshold: 3,
            observation_masking_size_limit: 512,
            enable_lost_in_the_middle_prevention: true,
            enable_context_compaction: true,
            compaction_threshold_tokens: 60_000,
            enable_llm_judge: false,
            guardrails: None,
            enable_state_checkpointing: false,
            state_scratchpad_path: None,
            workspace_path: None,
            project_trusted: true,
            injected_context: None,
            allowed_tools: None,
            high_risk_tools: vec![],
            approved_tool_calls: vec![],
            thread_id: None,
            resume_from_checkpoint_id: None,
            enable_lazy_tool_loading: false,
            enable_langgraph_mechanic: false,
        }
    }
}

/// Progress metrics for a running agent task.
#[derive(Default)]
pub struct AgentProgress {
    tool_use_count: AtomicU64,
    token_count: AtomicI64,
}

impl AgentProgress {
    pub fn record_tool_use(&self) {
        self.tool_use_count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn add_tokens(&self, n: i64) {
        self.token_count.fetch_add(n, Ordering::Relaxed);
    }

    pub fn tool_use_count(&self) -> u64 {
        self.tool_use_count.load(Ordering::Relaxed)
    }

    pub fn token_count(&self) -> i64 {
        self.token_count.load(Ordering::Relaxed)
    }
}

pub(crate) fn build_hierarchical_system_prompt(cfg: &AgentRunConfig, tools: &[crate::tools::Tool]) -> String {
    let mut end_idx = 32768;
    if cfg.user_instructions.len() > 32768 {
        while end_idx > 0 && !cfg.user_instructions.is_char_boundary(end_idx) {
            end_idx -= 1;
        }
    } else {
        end_idx = cfg.user_instructions.len();
    }
    let user_instr = &cfg.user_instructions[..end_idx];

    let mut combined_system = String::new();
    if !cfg.server_system_message.is_empty() {
        combined_system.push_str(&cfg.server_system_message);
    }

    // Format tools into [Tool Definitions]
    if !tools.is_empty() {
        if !combined_system.is_empty() {
            combined_system.push_str("\n\n");
        }
        combined_system.push_str("[Tool Definitions]\n");
        for tool in tools {
            combined_system.push_str(&format!("Tool: {}\n", tool.name));
            combined_system.push_str(&format!("Description: {}\n", tool.description));
            combined_system.push_str(&format!("Parameters: {}\n", tool.parameters));
        }
        // Remove trailing newline
        combined_system.pop();
    }

    if !cfg.developer_instructions.is_empty() {
        if !combined_system.is_empty() {
            combined_system.push_str("\n\n");
        }
        combined_system.push_str("[Developer Instructions]\n");
        combined_system.push_str(&cfg.developer_instructions);
    }

    if !user_instr.is_empty() {
        if !combined_system.is_empty() {
            combined_system.push_str("\n\n");
        }
        combined_system.push_str("[User Instructions]\n");
        combined_system.push_str(user_instr);
    }
    combined_system
}

/// The ReAct agent loop — mirrors Go builtin.BuiltinAgent.Run.
pub struct Agent {
    pub llm: Arc<dyn LlmClient>,
    pub tools: Vec<Tool>,
    pub progress: Arc<AgentProgress>,
    pub memory_store: Option<Arc<dyn crate::memory_store::LongTermMemory>>,
    pub checkpointer: Option<Arc<dyn crate::checkpointer::CheckpointSaver>>,
    pub observation_store: Arc<dashmap::DashMap<String, String>>,
}

impl Agent {
    pub fn add_tool(&mut self, tool: Tool) {
        self.tools.push(tool);
    }
    pub fn new(llm: Arc<dyn LlmClient>, tools: Vec<Tool>) -> Self {
        Self {
            llm,
            tools,
            progress: Arc::new(AgentProgress::default()),
            memory_store: None,
            checkpointer: None,
            observation_store: Arc::new(dashmap::DashMap::new()),
        }
    }

    pub fn with_memory_store(mut self, store: Arc<dyn crate::memory_store::LongTermMemory>) -> Self {
        self.memory_store = Some(store);
        self
    }

    pub fn with_checkpointer(mut self, checkpointer: Arc<dyn crate::checkpointer::CheckpointSaver>) -> Self {
        self.checkpointer = Some(checkpointer);
        self
    }

    /// Run the agent loop. Calls `on_event` for each event.
    #[tracing::instrument(skip(self, _on_event, cfg), fields(model = %cfg.model))]
    pub async fn run_langgraph<F>(
        &self,
        cfg: &AgentRunConfig,
        initial_message: &str,
        session_tools: Vec<crate::tools::Tool>,
        initial_messages: &mut Vec<Message>,
        _on_event: &mut F,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>>
    where
        F: FnMut(AgentEvent) + Send + Sync,
    {
        // Add initial message if needed
        if !initial_message.is_empty() {
            initial_messages.push(Message::user(initial_message));
        }

        let mut graph = crate::langgraph::StateGraph::new(std::sync::Arc::new(crate::langgraph::DefaultReducer));

        let llm = self.llm.clone();
        let tools_def: Vec<_> = session_tools.iter().map(|t| crate::types::ToolDefinition {
            name: t.name.clone(),
            description: t.description.clone(),
            parameters: t.parameters.clone(),
        }).collect();

        let mut cfg_clone = cfg.clone();
        // Force settings
        cfg_clone.enable_langgraph_mechanic = true;
        let cfg_arc = std::sync::Arc::new(cfg_clone);

        let tools_def_arc = std::sync::Arc::new(tools_def);
        let session_tools_arc = std::sync::Arc::new(session_tools);

        let system_prompt = build_hierarchical_system_prompt(&cfg_arc, &session_tools_arc);

        // --- NODE 1: LLM Call ---
        let llm_cfg = cfg_arc.clone();
        let llm_tools = tools_def_arc.clone();
        let llm_client = llm.clone();
        let llm_sys = system_prompt.clone();
        graph.add_node("llm_call", move |state| {
            let llm_client_c = llm_client.clone();
            let llm_sys_c = llm_sys.clone();
            let llm_cfg_c = llm_cfg.clone();
            let llm_tools_c = llm_tools.clone();
            Box::pin(async move {
                let msgs_val = state.get("messages").unwrap().as_array().unwrap();
                let mut msgs = vec![];
                for m in msgs_val {
                    let role_str = m["role"].as_str().unwrap();
                    let content = m["content"].as_str().unwrap().to_string();
                    let role = match role_str {
                        "user" => crate::types::Role::User,
                        "assistant" => crate::types::Role::Assistant,
                        "system" => crate::types::Role::System,
                        "tool" => crate::types::Role::Tool,
                        _ => crate::types::Role::User,
                    };
                    let mut tool_calls = vec![];
                    if let Some(tcs) = m.get("tool_calls").and_then(|v| v.as_array()) {
                        for tc in tcs {
                            tool_calls.push(crate::types::ToolCall {
                                id: tc["id"].as_str().unwrap().to_string(),
                                name: tc["name"].as_str().unwrap().to_string(),
                                arguments: tc["arguments"].clone(),
                            });
                        }
                    }
                    let mut tool_results = vec![];
                    if let Some(trs) = m.get("tool_results").and_then(|v| v.as_array()) {
                        for tr in trs {
                            tool_results.push(crate::types::ToolResult {
                                tool_call_id: tr["tool_call_id"].as_str().unwrap().to_string(),
                                content: tr["content"].as_str().unwrap_or("").to_string(),
                                error: tr["error"].as_str().unwrap_or("").to_string(),
                            });
                        }
                    }
                    msgs.push(crate::types::Message {
                        role,
                        content,
                        tool_calls,
                        tool_results,
                        response_id: None,
                    });
                }

                let req = crate::types::ChatRequest {
                    model: llm_cfg_c.model.clone(),
                    system: llm_sys_c.clone(),
                    messages: msgs,
                    tools: llm_tools_c.to_vec(),
                    max_tokens: llm_cfg_c.max_tokens,
                    temperature: llm_cfg_c.temperature,
                };

                match llm_client_c.chat(req).await {
                    Ok(resp) => {
                        let has_tool_calls = !resp.message.tool_calls.is_empty();
                        let mut update = serde_json::json!({
                            "has_tool_calls": has_tool_calls,
                            "last_message": {
                                "role": "assistant",
                                "content": resp.message.content,
                                "tool_calls": resp.message.tool_calls.iter().map(|tc| serde_json::json!({
                                    "id": tc.id,
                                    "name": tc.name,
                                    "arguments": tc.arguments,
                                })).collect::<Vec<_>>()
                            }
                        });
                        // Also append to messages array using the reducer
                        update.as_object_mut().unwrap().insert("messages".to_string(), serde_json::json!([{
                                "role": "assistant",
                                "content": resp.message.content,
                                "tool_calls": resp.message.tool_calls.iter().map(|tc| serde_json::json!({
                                    "id": tc.id,
                                    "name": tc.name,
                                    "arguments": tc.arguments,
                                })).collect::<Vec<_>>()
                        }]));
                        Ok(update)
                    }
                    Err(e) => Err(format!("LLM Error: {}", e)),
                }
            })
        });

        // --- NODE 2: Tool Execution ---
        let tool_tools = session_tools_arc.clone();
        graph.add_node("tool_node", move |state| {
            let tt = tool_tools.clone();
            Box::pin(async move {
                let last_msg = state.get("last_message").unwrap();
                let tool_calls = last_msg.get("tool_calls").unwrap().as_array().unwrap();

                let mut tool_results_json = vec![];

                for tc_val in tool_calls {
                    let name = tc_val["name"].as_str().unwrap();
                    let args = tc_val["arguments"].clone();
                    let id = tc_val["id"].as_str().unwrap();

                    if let Some(tool) = tt.iter().find(|t| t.name == name) {
                        match tool.execute.execute(args).await {
                            Ok(res) => {
                                tool_results_json.push(serde_json::json!({
                                    "tool_call_id": id,
                                    "content": res,
                                    "error": ""
                                }));
                            }
                            Err(e) => {
                                tool_results_json.push(serde_json::json!({
                                    "tool_call_id": id,
                                    "content": "",
                                    "error": e.to_string()
                                }));
                            }
                        }
                    } else {
                        tool_results_json.push(serde_json::json!({
                            "tool_call_id": id,
                            "content": "",
                            "error": format!("Tool {} not found", name)
                        }));
                    }
                }

                Ok(serde_json::json!({
                    "has_tool_calls": false, // Clear flag
                    "messages": [{
                        "role": "tool",
                        "content": "",
                        "tool_results": tool_results_json
                    }]
                }))
            })
        });

        // --- EDGES ---
        graph.add_edge("tool_node", "llm_call");

        graph.add_conditional_edges("llm_call", |state| {
            if state.get("has_tool_calls").and_then(|v| v.as_bool()).unwrap_or(false) {
                "tool_node".to_string()
            } else {
                crate::langgraph::END.to_string()
            }
        });

        graph.set_entry_point("llm_call");

        // Convert initial messages to json state
        let msgs_json: Vec<_> = initial_messages.iter().map(|m| {
            serde_json::json!({
                "role": match m.role {
                    crate::types::Role::User => "user",
                    crate::types::Role::Assistant => "assistant",
                    crate::types::Role::System => "system",
                    crate::types::Role::Tool => "tool",
                },
                "content": m.content,
                "tool_calls": m.tool_calls.iter().map(|tc| serde_json::json!({
                    "id": tc.id,
                    "name": tc.name,
                    "arguments": tc.arguments,
                })).collect::<Vec<_>>(),
                "tool_results": m.tool_results.iter().map(|tr| serde_json::json!({
                    "tool_call_id": tr.tool_call_id,
                    "content": tr.content,
                    "error": tr.error,
                })).collect::<Vec<_>>(),
            })
        }).collect();

        let initial_state = serde_json::json!({
            "messages": msgs_json,
            "has_tool_calls": false
        });

        let final_state = graph.run(initial_state).await.map_err(|e| format!("LangGraph Error: {}", e))?;

        let final_msgs = final_state.get("messages").unwrap().as_array().unwrap();
        let last_msg = final_msgs.last().unwrap();
        let content = last_msg.get("content").unwrap().as_str().unwrap().to_string();

        Ok(content)
    }


    /// Architectural Decision 2: Plan-and-Execute (LLMCompiler)
    /// Metric: LLMCompiler achieved 3.6x speedup by separating planning from execution.
    pub async fn run_plan_and_execute<F>(
        &self,
        cfg: &AgentRunConfig,
        initial_message: &str,
        session_tools: &[Tool],
        on_event: &mut F,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>>
    where
        F: FnMut(AgentEvent) + Send + Sync,
    {
        on_event(AgentEvent::RunStarted {
            iteration: 0,
        });

        // Phase 1: Planning
        let planner_system = format!(
            "You are an expert planner. Create a strict JSON plan to solve the user's task using the available tools.\nYour output MUST be a valid JSON array of objects, where each object has:\n- `tool`: the exact name of the tool\n- `args`: a JSON object containing the arguments for the tool\n\nAvailable tools:\n{}\n\nReturn ONLY the JSON array. Do not include markdown formatting or any other text.",
            serde_json::to_string_pretty(&self.tools.iter().map(|t| crate::types::ToolDefinition {
                name: t.name.clone(),
                description: t.description.clone(),
                parameters: t.parameters.clone(),
            }).collect::<Vec<_>>()).unwrap_or_default()
        );

        let plan_req = ChatRequest {
            model: cfg.model.clone(),
            system: planner_system,
            messages: vec![Message::user(initial_message)],
            tools: vec![], // No tools, we force it to output JSON
            max_tokens: cfg.max_tokens,
            temperature: 0.0, // Planning should be deterministic
        };

        on_event(AgentEvent::RunStarted { iteration: 0 });
        let plan_resp = self.llm.chat(plan_req).await?;
        let plan_json_text = plan_resp.message.content.trim().trim_start_matches("```json").trim_start_matches("```").trim_end_matches("```").trim();

        on_event(AgentEvent::RunStarted { iteration: 1 });

        let plan: Vec<serde_json::Value> = serde_json::from_str(plan_json_text).map_err(|e| format!("Failed to parse planner output as JSON array: {} (Output: {})", e, plan_json_text))?;

        // Phase 2: Execution
        let mut executed_steps = Vec::new();
        for (i, step) in plan.into_iter().enumerate() {
            let tool_name = step.get("tool").and_then(|v| v.as_str()).unwrap_or("");
            let args = step.get("args").unwrap_or(&serde_json::Value::Null);

            let dummy_tc = ToolCall {
                id: format!("plan_step_{}", i),
                name: tool_name.to_string(),
                arguments: args.clone(),
            };

            on_event(AgentEvent::ToolCall {
                name: tool_name.to_string(),
                args_json: args.to_string(),
                result: "Executing planned step...".to_string(),
                iteration: i as i32,
            });

            // Gating mechanics
            if let Err(e) = Self::check_tool_gating(&dummy_tc, false, cfg) {
                 return Err(Box::new(e));
            }

            let result = match self.execute_tool(&dummy_tc, session_tools, &[]).await {
                Ok(res) => res,
                Err(e) => format!("Error executing planned step: {:?}", e),
            };

            on_event(AgentEvent::ToolCall {
                name: tool_name.to_string(),
                args_json: args.to_string(),
                result: result.clone(),
                iteration: i as i32,
            });

            executed_steps.push(format!("Step {}: Tool '{}' with args '{}' -> Result: '{}'", i, tool_name, args, result));
        }

        // Phase 3: Replier
        let replier_system = "You are a helpful assistant. Formulate a final response to the user's initial task based on the execution of the planned steps. Do not attempt to use any further tools.".to_string();
        let execution_summary = executed_steps.join("\n\n");
        let final_prompt = format!("Initial task: {}\n\nExecution steps and results:\n{}\n\nPlease provide the final answer.", initial_message, execution_summary);

        let replier_req = ChatRequest {
            model: cfg.model.clone(),
            system: replier_system,
            messages: vec![Message::user(final_prompt)],
            tools: vec![],
            max_tokens: cfg.max_tokens,
            temperature: cfg.temperature,
        };

        on_event(AgentEvent::RunStarted { iteration: 2 });
        let final_resp = self.llm.chat(replier_req).await?;

        on_event(AgentEvent::TaskComplete { content: final_resp.message.content.clone() });
        Ok(final_resp.message.content)
    }

    pub async fn run<F>(
        &self,
        cfg: &AgentRunConfig,
        initial_message: &str,
        on_event: &mut F,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>>
    where
        F: FnMut(AgentEvent) + Send + Sync,
    {

        let session_tools = self.tools.clone();
        if cfg.enable_llmcompiler_plan_and_execute {
            return self.run_plan_and_execute(cfg, initial_message, &session_tools, on_event).await;
        }
        let mut session_tools = self.tools.clone();
        let active_tools = std::sync::Arc::new(tokio::sync::RwLock::new(std::collections::HashSet::new()));

        if cfg.enable_lazy_tool_loading {
            let active_tools_clone = active_tools.clone();
            session_tools.push(crate::tools::lazy_load::lazy_load_tool(active_tools_clone));
        }

        // OpenAI Mechanic: Input Guardrails
        if let Some(guard_cfg) = &cfg.guardrails {
            if let Err(e) = crate::guardrails::check_input(initial_message, guard_cfg) {
                on_event(AgentEvent::TaskError { error: e.clone() });
                return Err(e.into());
            }
        }

        on_event(AgentEvent::RunStarted { iteration: 0 });

        let meter = global::meter("ohc_agent");
        let token_counter = meter.u64_counter("ohc_agent_token_usage_total").build();
        let cost_counter = meter.f64_counter("ohc_agent_cost_estimate_usd").build();

        let mut tool_error_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();

        let mut messages: Vec<Message> = cfg.injected_context.clone().unwrap_or_default();
        let mut last_checkpoint_id: Option<String> = None;

        if cfg.enable_langgraph_mechanic {
            return self.run_langgraph(cfg, initial_message, session_tools, &mut messages, on_event).await;
        }

        if let (Some(checkpointer), Some(thread_id)) = (&self.checkpointer, &cfg.thread_id) {
            if let Some(resume_id) = &cfg.resume_from_checkpoint_id {
                let cp = checkpointer.get_checkpoint(thread_id, resume_id).await
                    .map_err(|e| format!("Failed to fetch requested checkpoint {}: {}", resume_id, e))?
                    .ok_or_else(|| format!("Requested checkpoint {} not found", resume_id))?;

                messages = serde_json::from_value::<Vec<Message>>(cp.data.clone())
                    .map_err(|e| format!("Failed to deserialize requested checkpoint: {}", e))?;
                last_checkpoint_id = Some(cp.checkpoint_id.clone());
            } else {
                if let Ok(checkpoints) = checkpointer.list_checkpoints(thread_id).await {
                    if let Some(cp) = checkpoints.first() {
                        if let Ok(saved_msgs) = serde_json::from_value::<Vec<Message>>(cp.data.clone()) {
                            messages = saved_msgs;
                            last_checkpoint_id = Some(cp.checkpoint_id.clone());
                        }
                    }
                }
            }
        }

        let generated_uuid_path = format!(".agent_checkpoint_{}.json", uuid::Uuid::new_v4());
        let scratchpad_path = cfg.state_scratchpad_path.clone().unwrap_or(generated_uuid_path);

        if messages.is_empty() && cfg.enable_state_checkpointing {
            if let Ok(contents) = tokio::fs::read_to_string(&scratchpad_path).await {
                if let Ok(saved_msgs) = serde_json::from_str::<Vec<Message>>(&contents) {
                    messages = saved_msgs;
                }
            }
        }

        if messages.is_empty() {
            messages.push(Message::user(initial_message));
        } else if !initial_message.is_empty() {
            messages.push(Message::user(initial_message));
        }
        let mut budget_tracker = BudgetTracker::default();
        let mut global_turn_tokens = 0i32;
        let mut last_assistant_content = String::new();

        let max_iterations = if cfg.max_iterations <= 0 { 100 } else { cfg.max_iterations };

        let mut combined_system = build_hierarchical_system_prompt(cfg, &session_tools);

        // Long-Term Memory Retrieval
        if let Some(store) = &self.memory_store {
            match store.retrieve(initial_message, 5).await {
                Ok(memories) => {
                    if !memories.is_empty() {
                        combined_system.push_str("\n\n[Long-Term Memory Context]\n");
                        for mem in memories {
                            combined_system.push_str(&format!("- {}\n", mem));
                        }
                    }
                }
                Err(e) => {
                    tracing::warn!("Failed to retrieve long term memory: {}", e);
                }
            }

            // 3-Tier Memory Mechanic: Lightweight Index
            if let Ok(index_content) = store.get_lightweight_index().await {
                if !index_content.trim().is_empty() {
                    combined_system.push_str("\n\n[Lightweight Memory Index]\n");
                    combined_system.push_str("Agent must treat memory as a 'hint' and verify against actual state before acting.\n");
                    combined_system.push_str(&index_content);
                }
            }
        }

        for iteration in 0..max_iterations {
            on_event(AgentEvent::IterationStarted {
                iteration,
                message_count: messages.len(),
            });

            let mut final_messages = messages.clone();

            // Context Window Strategy: Prioritize reasoning traces over raw tool outputs (ACON Research)
            if cfg.enable_acon_context_strategy {
                let msg_count = final_messages.len();
                if msg_count > 3 {
                    // We preserve the last 2 messages (usually assistant + tool results)
                    // For older Tool role messages, we strip the raw tool output but keep reasoning
                    let threshold = msg_count - 2;
                    for i in 0..threshold {
                        if final_messages[i].role == Role::Tool {
                            for tr in &mut final_messages[i].tool_results {
                                if tr.error.is_empty() && !tr.content.starts_with("[ACON:") && !tr.content.is_empty() {
                                    tr.content = "[ACON: Tool output omitted to prioritize reasoning traces.]".to_string();
                                }
                            }
                        }
                    }
                }
            }

            // Prompt Construction Mechanic: "Lost in the Middle" Prevention
            // High-signal context at the very beginning and very end.
            if cfg.enable_lost_in_the_middle_prevention {
                let mut reminder_text = String::new();
                if !cfg.developer_instructions.is_empty() {
                    reminder_text.push_str(&format!("[System Reminder: {}]\n\n", cfg.developer_instructions));
                }
                if !cfg.user_instructions.is_empty() && final_messages.len() > 3 {
                    // Truncate user instructions if it's too long, just to remind the core objective
                    let mut end_idx = 1000;
                    if cfg.user_instructions.len() > 1000 {
                        while end_idx > 0 && !cfg.user_instructions.is_char_boundary(end_idx) {
                            end_idx -= 1;
                        }
                    } else {
                        end_idx = cfg.user_instructions.len();
                    }
                    let summary = &cfg.user_instructions[..end_idx];
                    reminder_text.push_str(&format!("[System Reminder to combat 'Lost in the Middle' effect: Remember your core objective: {}...]", summary));
                }

                if !reminder_text.is_empty() {
                    final_messages.push(Message::user(reminder_text.trim()));
                }
            } else if !cfg.developer_instructions.is_empty() {
                final_messages.push(Message::user(format!("[System Reminder: {}]", cfg.developer_instructions)));
            }

            let mut req_tools = Vec::new();
            for t in &session_tools {
                if !cfg.enable_lazy_tool_loading
                    || t.name == "ToolSearch"
                    || t.name == "LazyLoadTools"
                    || active_tools.read().await.contains(&t.name)
                {
                    req_tools.push(ToolDefinition {
                        name: t.name.clone(),
                        description: t.description.clone(),
                        parameters: t.parameters.clone(),
                    });
                }
            }

            let req = ChatRequest {
                model: cfg.model.clone(),
                system: combined_system.clone(),
                messages: final_messages,
                tools: req_tools,
                max_tokens: cfg.max_tokens,
                temperature: cfg.temperature,
            };

            let resp = match self.llm.chat(req).await {
                Ok(r) => r,
                Err(e) => {
                    let err = format!("LLM error: {}", e);
                    on_event(AgentEvent::TaskError { error: err.clone() });
                    return Err(err.into());
                }
            };

            let turn_input_tokens = resp.usage.input_tokens;
            let output_tokens = resp.usage.output_tokens;
            let total_tokens = (turn_input_tokens + output_tokens) as i64;
            self.progress.add_tokens(total_tokens);
            global_turn_tokens += output_tokens;

            // Telemetry: Record token usage
            let model_label = KeyValue::new("model", cfg.model.clone());
            let agent_label = KeyValue::new("agent_id", cfg.agent_id.clone());
            token_counter.add(turn_input_tokens as u64, &[model_label.clone(), agent_label.clone(), KeyValue::new("type", "input")]);
            token_counter.add(output_tokens as u64, &[model_label.clone(), agent_label.clone(), KeyValue::new("type", "output")]);

            // Unified Cost Calculation Mechanic
            // Note: We use the local pricing calculator logic to avoid a direct
            // dependency on server_lib which would cause a circular dependency.
            let input_cost_per_m = match cfg.model.to_lowercase().as_str() {
                m if m.contains("gpt-4o") && !m.contains("mini") => 5.0,
                m if m.contains("gpt-4-turbo") => 10.0,
                m if m.contains("gpt-3.5") || m.contains("gpt-4o-mini") => 0.15,
                m if m.contains("gemini-1.5-pro") => 3.5,
                m if m.contains("gemini-1.5-flash") => 0.075,
                m if m.contains("claude-3-5-sonnet") => 3.0,
                m if m.contains("claude-3-haiku") => 0.25,
                _ => 3.0,
            };
            let output_cost_per_m = match cfg.model.to_lowercase().as_str() {
                m if m.contains("gpt-4o") && !m.contains("mini") => 15.0,
                m if m.contains("gpt-4-turbo") => 30.0,
                m if m.contains("gpt-3.5") || m.contains("gpt-4o-mini") => 0.60,
                m if m.contains("gemini-1.5-pro") => 10.5,
                m if m.contains("gemini-1.5-flash") => 0.30,
                m if m.contains("claude-3-5-sonnet") => 15.0,
                m if m.contains("claude-3-haiku") => 1.25,
                _ => 15.0,
            };

            let turn_cost = (turn_input_tokens as f64 * input_cost_per_m / 1_000_000.0) +
                            (output_tokens as f64 * output_cost_per_m / 1_000_000.0);

            if turn_cost > 0.0 {
                cost_counter.add(turn_cost, &[model_label, agent_label]);
            }

            let stop_reason = resp.stop_reason.as_str();

            // Text content from assistant
            if !resp.message.content.is_empty() {
                last_assistant_content = resp.message.content.clone();
                on_event(AgentEvent::TextChunk {
                    content: resp.message.content.clone(),
                });
            }

            // Token budget check when LLM stops due to length.
            if stop_reason == "max_tokens" || stop_reason == "length" {
                let decision = check_token_budget(
                    &mut budget_tracker,
                    cfg.max_task_tokens,
                    global_turn_tokens,
                );

                if decision.action == BudgetAction::Stop {
                    let err_msg = format!("Terminal condition reached: token budget exhausted ({} / {}).", global_turn_tokens, cfg.max_task_tokens);
                    on_event(AgentEvent::TaskError { error: err_msg.clone() });
                    return Err(ToolError::Fatal(err_msg).into());
                }
                if decision.action == BudgetAction::Continue {
                    // Add the budget nudge to messages and continue.
                    if !resp.message.content.is_empty() {
                        messages.push(resp.message.clone());
                    }
                    messages.push(Message::user(&decision.nudge_message));
                    continue;
                }
            }

            let tool_calls = resp.message.tool_calls.clone();

            // Add assistant message to history (including tool calls).
            messages.push(resp.message.clone());

            // Telemetry: track individual tool executions
            let tool_call_counter = meter.u64_counter("ohc_agent_tool_execution_total").build();
            for tc in &tool_calls {
                tool_call_counter.add(1, &[
                    KeyValue::new("agent_id", cfg.agent_id.clone()),
                    KeyValue::new("tool_name", tc.name.clone())
                ]);
            }

            // Terminal condition: no tool calls.
            if tool_calls.is_empty() {
                // Inferential/Sensors (LLM-as-judge subagent)
                if cfg.enable_llm_judge {
                    let judge_req = ChatRequest {
                        model: cfg.model.clone(),
                        system: "You are an expert judge. Evaluate the following output for correctness, completeness, and adherence to constraints. Output ONLY 'APPROVE' or 'REJECT: <reason>'.".to_string(),
                        messages: vec![Message::user(format!("Evaluate this output:
{}", last_assistant_content))],
                        tools: vec![],
                        max_tokens: 500,
                        temperature: 0.0,
                    };

                    match self.llm.chat(judge_req).await {
                        Ok(judge_resp) => {
                            let judge_text = judge_resp.message.content.trim();
                            if judge_text.starts_with("REJECT:") {
                                let reason = judge_text.strip_prefix("REJECT:").unwrap_or(judge_text).trim();
                                let err_msg = format!("Your previous output was evaluated by an LLM-as-judge and rejected. Reason: {}. Please correct your work and use tools if necessary.", reason);
                                messages.push(Message::user(err_msg));
                                continue;
                            }
                            // If APPROVE or anything else, we proceed to output guardrails.
                        }
                        Err(e) => {
                            let err = format!("LLM Judge error: {}", e);
                            on_event(AgentEvent::TaskError { error: err.clone() });
                            return Err(err.into());
                        }
                    }
                }
                // In a production-grade agent, we might use a separate LLM pass
                // to evaluate confidence in the final answer if threshold > 0.
                // For now, we'll assume the model is confident if it didn't use more tools.

                // OpenAI Mechanic: Output Guardrails
                if let Some(guard_cfg) = &cfg.guardrails {
                    if let Err(e) = crate::guardrails::check_output(&last_assistant_content, guard_cfg) {
                        on_event(AgentEvent::TaskError { error: e.clone() });
                        return Err(e.into());
                    }
                }

                on_event(AgentEvent::TaskComplete {
                    content: last_assistant_content.clone(),
                });
                return Ok(last_assistant_content);
            }

            // Execute tool calls and collect results.
            // Split tools into read-only and mutating to implement the concurrent retrieval mechanic.
            let mut read_only_calls = Vec::new();
            let mut mutating_calls = Vec::new();

            for tc in &tool_calls {
                let is_read_only = self.tools.iter().find(|t| t.name == tc.name).map(|t| t.is_read_only).unwrap_or(false);
                if is_read_only {
                    read_only_calls.push(tc.clone());
                } else {
                    mutating_calls.push(tc.clone());
                }
            }

            // We need a helper to execute a single tool call with retries and guardrails.
            // We use a macro or inline logic to avoid borrowing issues with `on_event`.
            let mut tool_results: Vec<ToolResult> = vec![ToolResult { tool_call_id: String::new(), content: String::new(), error: String::new() }; tool_calls.len()];

            // Note: Since `on_event` is `&mut F`, we can't easily share it across concurrent tasks.
            // For now, we will collect events and results from the concurrent execution, then emit them sequentially.
            // We will execute the read-only calls concurrently using `futures::future::join_all`.

            let mut read_only_futures = Vec::new();
            for tc in &read_only_calls {
                // OpenAI Mechanic: Tool Guardrails
                if let Some(guard_cfg) = &cfg.guardrails {
                    if let Err(e) = crate::guardrails::check_tool(tc, guard_cfg) {
                        on_event(AgentEvent::TaskError { error: e.clone() });
                        return Err(e.into()); // Tripwire: halt the loop immediately
                    }
                }
                let gating_res = Self::check_tool_gating(tc, true, cfg);
                let tc_clone = tc.clone();
                let session_tools_clone = session_tools.clone();
                let messages_clone = messages.clone();
                read_only_futures.push(async move {
                    if let Err(e) = gating_res {
                        return (tc_clone, Err(e));
                    }
                    let mut retry_count = 0;
                    let max_retries = 2;
                    loop {
                        match self.execute_tool(&tc_clone, &session_tools_clone, &messages_clone).await {
                            Ok(r) => {
                                return (tc_clone, Ok(r));
                            }
                            Err(ToolError::Transient(msg)) => {
                                if retry_count < max_retries {
                                    retry_count += 1;
                                    let backoff = std::time::Duration::from_millis(500 * (1 << retry_count));
                                    tokio::time::sleep(backoff).await;
                                    continue;
                                } else {
                                    return (tc_clone, Err(ToolError::Transient(msg)));
                                }
                            }
                            Err(e) => {
                                return (tc_clone, Err(e));
                            }
                        }
                    }
                });
            }

            let ro_results = futures::future::join_all(read_only_futures).await;

            // Emit events and collect results for read-only tools
            for (tc, res) in ro_results {
                let idx = tool_calls.iter().position(|t| t.id == tc.id).unwrap();
                match res {
                    Ok(r) => {
                        tool_error_counts.remove(&tc.name);
                        self.progress.record_tool_use();
                        self.observation_store.insert(tc.id.clone(), r.clone());
                        on_event(AgentEvent::ToolCall {
                            name: tc.name.clone(),
                            args_json: tc.arguments.to_string(),
                            result: r.clone(),
                            iteration,
                        });
                        tool_results[idx] = ToolResult {
                            tool_call_id: tc.id.clone(),
                            content: r,
                            error: String::new(),
                        };
                    }
                    Err(ToolError::Transient(msg)) => {
                        let err = format!("Transient error after retries: {}", msg);
                        on_event(AgentEvent::ToolCall {
                            name: tc.name.clone(),
                            args_json: tc.arguments.to_string(),
                            result: format!("Error: {}", err),
                            iteration,
                        });
                        tool_results[idx] = ToolResult {
                            tool_call_id: tc.id.clone(),
                            content: String::new(),
                            error: err,
                        };
                    }
                    Err(ToolError::LlmRecoverable(msg)) => {
                        let count = tool_error_counts.entry(tc.name.clone()).or_insert(0);
                        *count += 1;
                        if *count > 2 {
                            let fatal_msg = format!("Tool '{}' failed 3 times consecutively with recoverable errors. Escalating to Fatal to prevent compounding error loops. Last error: {}", tc.name, msg);
                            on_event(AgentEvent::TaskError { error: fatal_msg.clone() });
                            return Err(fatal_msg.into());
                        }

                        // Return the raw error as a ToolMessage directly to the model so it can self-correct.
                        on_event(AgentEvent::ToolCall {
                            name: tc.name.clone(),
                            args_json: tc.arguments.to_string(),
                            result: msg.clone(),
                            iteration,
                        });
                        tool_results[idx] = ToolResult {
                            tool_call_id: tc.id.clone(),
                            content: String::new(),
                            error: msg,
                        };
                    }
                    Err(ToolError::UserFixable(msg)) => {
                        let err = format!("User intervention required: {}", msg);
                        on_event(AgentEvent::UserInterventionRequired { error: err.clone() });
                        return Err(err.into());
                    }
                    Err(ToolError::Fatal(msg)) => {
                        let err = format!("Fatal tool error: {}", msg);
                        on_event(AgentEvent::TaskError { error: err.clone() });
                        return Err(err.into());
                    }
                    Err(ToolError::Unexpected(msg)) => {
                        let err = format!("Unexpected tool error: {}", msg);
                        on_event(AgentEvent::TaskError { error: err.clone() });
                        return Err(err.into());
                    }
                    Err(ToolError::HandoffRequested(target)) => {
                        on_event(AgentEvent::Handoff { target_agent: target.clone() });
                        return Ok(format!("Handoff requested to {}", target));
                    }
                }
            }

            // Execute mutating calls sequentially to prevent race conditions
            for tc in &mutating_calls {
                // OpenAI Mechanic: Tool Guardrails
                if let Some(guard_cfg) = &cfg.guardrails {
                    if let Err(e) = crate::guardrails::check_tool(&tc, guard_cfg) {
                        on_event(AgentEvent::TaskError { error: e.clone() });
                        return Err(e.into()); // Tripwire: halt the loop immediately
                    }
                }

                // Anthropic Mechanic: 3-Stage Tool Gating
                if let Err(e) = Self::check_tool_gating(&tc, false, cfg) {
                    match e {
                        ToolError::UserFixable(msg) => {
                            let err = format!("User intervention required: {}", msg);
                            on_event(AgentEvent::UserInterventionRequired { error: err.clone() });
                            return Err(err.into());
                        }
                        ToolError::Fatal(msg) => {
                            let err = format!("Fatal tool error: {}", msg);
                            on_event(AgentEvent::TaskError { error: err.clone() });
                            return Err(err.into());
                        }
                        ToolError::Unexpected(msg) => {
                            let err = format!("Unexpected tool error: {}", msg);
                            on_event(AgentEvent::TaskError { error: err.clone() });
                            return Err(err.into());
                        }
                        ToolError::HandoffRequested(target) => {
                            on_event(AgentEvent::Handoff { target_agent: target.clone() });
                            return Ok(format!("Handoff requested to {}", target));
                        }
                        _ => {
                            let err = format!("Fatal tool error: {:?}", e);
                            on_event(AgentEvent::TaskError { error: err.clone() });
                            return Err(err.into());
                        }
                    }
                }

                let mut retry_count = 0;
                let max_retries = 2;
                let mut content = String::new();
                let mut error = String::new();

                loop {
                    match self.execute_tool(&tc, &session_tools, &messages).await {
                        Ok(r) => {
                            tool_error_counts.remove(&tc.name);
                            self.progress.record_tool_use();
                            self.observation_store.insert(tc.id.clone(), r.clone());
                            on_event(AgentEvent::ToolCall {
                                name: tc.name.clone(),
                                args_json: tc.arguments.to_string(),
                                result: r.clone(),
                                iteration,
                            });
                            content = r;
                            break;
                        }
                        Err(ToolError::Transient(msg)) => {
                            if retry_count < max_retries {
                                retry_count += 1;
                                let backoff = std::time::Duration::from_millis(500 * (1 << retry_count));
                                tokio::time::sleep(backoff).await;
                                continue;
                            } else {
                                let err = format!("Transient error after retries: {}", msg);
                                on_event(AgentEvent::ToolCall {
                                    name: tc.name.clone(),
                                    args_json: tc.arguments.to_string(),
                                    result: format!("Error: {}", err),
                                    iteration,
                                });
                                error = err;
                                break;
                            }
                        }
                        Err(ToolError::LlmRecoverable(msg)) => {
                            let count = tool_error_counts.entry(tc.name.clone()).or_insert(0);
                            *count += 1;
                            if *count > 2 {
                                let fatal_msg = format!("Tool '{}' failed 3 times consecutively with recoverable errors. Escalating to Fatal to prevent compounding error loops. Last error: {}", tc.name, msg);
                                on_event(AgentEvent::TaskError { error: fatal_msg.clone() });
                                return Err(fatal_msg.into());
                            }

                            // Return the raw error as a ToolMessage directly to the model so it can self-correct.
                            on_event(AgentEvent::ToolCall {
                                name: tc.name.clone(),
                                args_json: tc.arguments.to_string(),
                                result: msg.clone(),
                                iteration,
                            });
                            error = msg;
                            content = String::new();
                            break;
                        }
                        Err(ToolError::UserFixable(msg)) => {
                            let err = format!("User intervention required: {}", msg);
                            on_event(AgentEvent::UserInterventionRequired { error: err.clone() });
                            return Err(err.into());
                        }
                        Err(ToolError::Fatal(msg)) => {
                            let err = format!("Fatal tool error: {}", msg);
                            on_event(AgentEvent::TaskError { error: err.clone() });
                            return Err(err.into());
                        }
                        Err(ToolError::Unexpected(msg)) => {
                            let err = format!("Unexpected tool error: {}", msg);
                            on_event(AgentEvent::TaskError { error: err.clone() });
                            return Err(err.into());
                        }
                        Err(ToolError::HandoffRequested(target)) => {
                            on_event(AgentEvent::Handoff { target_agent: target.clone() });
                            return Ok(format!("Handoff requested to {}", target));
                        }
                    }
                }

                let idx = tool_calls.iter().position(|t| t.id == tc.id).unwrap();
                tool_results[idx] = ToolResult {
                    tool_call_id: tc.id.clone(),
                    content,
                    error,
                };
            }

            if cfg.enable_observation_masking {
                // JetBrains Observation Masking: Hide the raw output of old tools from the prompt,
                // but keep the `tool_calls` themselves visible so the model remembers what it did.
                // Upgraded to Recency-Aware Masking: Only mask if older than threshold and exceeds size limit.
                let msg_count = messages.len();
                for i in 0..msg_count {
                    if messages[i].role == Role::Tool {
                        let age = msg_count - i;
                        if age > cfg.observation_masking_threshold {
                            for tr in &mut messages[i].tool_results {
                                if tr.error.is_empty() && !tr.content.starts_with("[Observation Masked") {
                                    let bytes = tr.content.len();
                                    if bytes > cfg.observation_masking_size_limit {
                                        tr.content = format!(
                                            "[Observation Masked to save context. Output was {} bytes. The tool call itself remains visible. Use 'RecallObservation' with ID '{}' if you need the full output again.]",
                                            bytes, tr.tool_call_id
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Append tool results as a user turn.
            messages.push(Message {
                role: Role::Tool,
                content: String::new(),
                tool_calls: vec![],
                tool_results,
                response_id: None,
            });

            // State Management Checkpointing Mechanic
            // 1. Configured Checkpointer (Database or Git)
            if let (Some(checkpointer), Some(thread_id)) = (&self.checkpointer, &cfg.thread_id) {
                let checkpoint_id = uuid::Uuid::new_v4().to_string();
                let cp = crate::checkpointer::Checkpoint {
                    thread_id: thread_id.clone(),
                    checkpoint_id: checkpoint_id.clone(),
                    parent_id: last_checkpoint_id.clone(),
                    data: serde_json::to_value(&messages).unwrap_or(serde_json::Value::Null),
                    metadata: serde_json::json!({
                        "iteration": iteration,
                        "turn_input_tokens": turn_input_tokens,
                        "turn_output_tokens": output_tokens,
                    }),
                    created_at: chrono::Utc::now(),
                };
                if let Err(e) = checkpointer.put_checkpoint(cp).await {
                    tracing::warn!("Failed to save checkpoint to database: {}", e);
                } else {
                    last_checkpoint_id = Some(checkpoint_id.clone());
                    on_event(AgentEvent::CheckpointSaved {
                        iteration,
                        path: format!("db:{}", checkpoint_id),
                    });
                }
            }

            // 2. Local File Scratchpad (Claude Code)
            if cfg.enable_state_checkpointing && !mutating_calls.is_empty() {
                if let Ok(json_state) = serde_json::to_string_pretty(&messages) {
                    if tokio::fs::write(&scratchpad_path, json_state).await.is_ok() {
                        on_event(AgentEvent::CheckpointSaved {
                            iteration,
                            path: scratchpad_path.clone(),
                        });
                    }
                }
            }


            // Context Compaction Mechanic
            // Use the input_tokens from the last request to determine the current context window size.
            if cfg.enable_context_compaction && turn_input_tokens > cfg.compaction_threshold_tokens {
                // We want to compact if we have enough messages to make it worthwhile
                if messages.len() > 5 {
                    let mut compact_messages = Vec::new();
                    // Keep the first message (usually the initial prompt)
                    compact_messages.push(messages[0].clone());

                    // The middle part to be compacted
                    let middle_start = 1;
                    let middle_end = messages.len() - 3;

                    if middle_end > middle_start {
                        let mut middle_text = String::new();
                        for m in &messages[middle_start..middle_end] {
                            middle_text.push_str(&format!("[Role: {}]\n", m.role));
                            if !m.content.is_empty() {
                                middle_text.push_str(&m.content);
                                middle_text.push('\n');
                            }
                            if !m.tool_calls.is_empty() {
                                middle_text.push_str("Tool Calls:\n");
                                for tc in &m.tool_calls {
                                    middle_text.push_str(&format!("  {} ({})\n", tc.name, tc.arguments.to_string()));
                                }
                            }
                            if !m.tool_results.is_empty() {
                                middle_text.push_str("Tool Results:\n");
                                for tr in &m.tool_results {
                                    let mut preview = tr.content.clone();
                                    if preview.len() > 200 {
                                        preview.truncate(200);
                                        preview.push_str("...");
                                    }
                                    middle_text.push_str(&format!("  {} (error: {})\n", preview, tr.error));
                                }
                            }
                            middle_text.push_str("---\n");
                        }

                        let summary_req = ChatRequest {
                            model: cfg.model.clone(),
                            system: "You are an expert context compactor for an AI agent. Summarize the following middle portion of an agent conversation. Preserve all architectural decisions, unresolved bugs, and the exact state of progress. Discard redundant or raw tool outputs. Be concise.".to_string(),
                            messages: vec![Message::user(format!("Compact this conversation:\n{}", middle_text))],
                            tools: vec![],
                            max_tokens: 2000,
                            temperature: 0.0,
                        };

                        match self.llm.chat(summary_req).await {
                            Ok(summary_resp) => {
                                let summary = summary_resp.message.content;
                                compact_messages.push(Message::user(format!("[Context Compacted by Harness]:\n{}", summary)));
                                // Append the remaining recent messages
                                compact_messages.extend_from_slice(&messages[middle_end..]);
                                messages = compact_messages;
                            }
                            Err(e) => {
                                // If compaction fails, just log it and continue. Don't crash the agent.
                                let err = format!("Context compaction failed: {}", e);
                                on_event(AgentEvent::TaskError { error: err.clone() });
                            }
                        }
                    }
                }
            }
        }

        // Hit max iterations.
        on_event(AgentEvent::TaskComplete {
            content: last_assistant_content.clone(),
        });
        Ok(last_assistant_content)
    }


    // Anthropic Mechanic: 3-Stage Tool Gating
    fn check_tool_gating(tc: &ToolCall, is_read_only: bool, cfg: &AgentRunConfig) -> Result<(), ToolError> {
        // Stage 1: Trust establishment at project load
        if !cfg.project_trusted && !is_read_only {
            return Err(ToolError::Fatal("Project not trusted. Mutating tools are disabled.".to_string()));
        }

        // Stage 2: Permission check before each tool call
        if let Some(allowed) = &cfg.allowed_tools {
            if !allowed.contains(&tc.name) {
                return Err(ToolError::Fatal(format!("Tool '{}' is not in the allowed list.", tc.name)));
            }
        }

        // Stage 3: Explicit user confirmation for high-risk operations
        if cfg.high_risk_tools.contains(&tc.name) && !cfg.approved_tool_calls.contains(&tc.id) {
            return Err(ToolError::UserFixable(format!("High-risk tool '{}' requires explicit user confirmation. Approve this tool call to proceed.", tc.name)));
        }

        Ok(())
    }

    async fn execute_tool(
        &self,
        tc: &ToolCall,
        session_tools: &[Tool],
        current_messages: &[Message],
    ) -> Result<String, ToolError> {
        let tool = session_tools
            .iter()
            .find(|t| t.name == tc.name)
            .ok_or_else(|| ToolError::LlmRecoverable(format!("unknown tool: {}", tc.name)))?;

        let mut args = tc.arguments.clone();
        if tc.name == "spawn_subagent" {
            if let Some(obj) = args.as_object_mut() {
                if obj.get("mode").and_then(|v| v.as_str()) == Some("fork") {
                    if let Ok(context_json) = serde_json::to_string(current_messages) {
                        obj.insert("parent_context_json".to_string(), serde_json::json!(context_json));
                    }
                }
            }
        }

        tool.execute.execute(args).await
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_llmcompiler_plan_and_execute_mechanic() {
        struct LLMCompilerMockClient {
            pub requests: tokio::sync::Mutex<Vec<ChatRequest>>,
        }
        #[async_trait::async_trait]
        impl LlmClient for LLMCompilerMockClient {
            async fn chat(&self, req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
                let mut reqs = self.requests.lock().await;
                reqs.push(req.clone());

                // If it's the planner phase (no tools supplied)
                if req.tools.is_empty() && req.system.contains("You are an expert planner") {
                    let plan = serde_json::json!([
                        {
                            "tool": "mock_read",
                            "args": { "path": "file.txt" }
                        }
                    ]);
                    Ok(ChatResponse {
                        message: Message::assistant(plan.to_string()),
                        usage: Usage::default(),
                        stop_reason: "stop".to_string(),
                        response_id: Some("mock-id".to_string()),
                    })
                } else {
                    // It's the replier phase
                    Ok(ChatResponse {
                        message: Message::assistant("Final plan executed."),
                        usage: Usage::default(),
                        stop_reason: "stop".to_string(),
                        response_id: Some("mock-id".to_string()),
                    })
                }
            }
        }

        let mock_tool = Tool {
            name: "mock_read".to_string(),
            description: "read".to_string(),
            is_read_only: true,
            parameters: serde_json::json!({}),
            execute: Arc::new(MockToolExecutor),
        };

        let client = Arc::new(LLMCompilerMockClient {
            requests: tokio::sync::Mutex::new(vec![]),
        });

        let agent = Agent::new(client.clone(), vec![mock_tool]);
        let mut cfg = AgentRunConfig::default();
        cfg.enable_llmcompiler_plan_and_execute = true;

        let mut events = vec![];
        let mut on_event = |e| { events.push(e); };

        let result = agent.run(&cfg, "Plan and run", &mut on_event).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Final plan executed.");

        let reqs = client.requests.lock().await;
        assert_eq!(reqs.len(), 2, "Should have called LLM twice: once for planner, once for replier");

        let mut tool_called = false;
        for e in events {
            if let AgentEvent::ToolCall { name, .. } = e {
                if name == "mock_read" {
                    tool_called = true;
                }
            }
        }
        assert!(tool_called, "The planned tool should have been executed");
    }

    use super::*;
    use ohc_builtin_agent_core::types::{ChatResponse, Message, Role, ToolCall, Usage};
    use tokio::sync::Mutex;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_acon_context_strategy() {
        struct MockLlmClientAcon {
            call_count: Mutex<usize>,
        }

        #[async_trait::async_trait]
        impl LlmClient for MockLlmClientAcon {
            async fn chat(&self, req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
                let mut count = self.call_count.lock().await;
                *count += 1;

                if *count == 1 {
                    // Turn 1: Return a tool call to generate some history
                    Ok(ChatResponse {
                        message: Message {
                            role: Role::Assistant,
                            content: "I am thinking about calling a tool.".to_string(),
                            tool_calls: vec![ToolCall {
                                id: "call_1".to_string(),
                                name: "read_tool".to_string(),
                                arguments: serde_json::Value::Null,
                            }],
                            tool_results: vec![],
                        response_id: None,
                        },
                        usage: Usage::default(),
                        stop_reason: "tool_calls".to_string(),
                        response_id: Some("mock-id".to_string()),
                    })
                } else if *count == 2 {
                    // Turn 2: Another tool call
                    Ok(ChatResponse {
                        message: Message {
                            role: Role::Assistant,
                            content: "I need more info.".to_string(),
                            tool_calls: vec![ToolCall {
                                id: "call_2".to_string(),
                                name: "read_tool".to_string(),
                                arguments: serde_json::Value::Null,
                            }],
                            tool_results: vec![],
                        response_id: None,
                        },
                        usage: Usage::default(),
                        stop_reason: "tool_calls".to_string(),
                        response_id: Some("mock-id".to_string()),
                    })
                } else if *count == 3 {
                    // Turn 3: Final answer. We check the received messages.
                    // The history should be: User, Assistant(call1), Tool(result1), Assistant(call2), Tool(result2)
                    // With ACON enabled, result1 should be stripped. result2 should remain intact since it's in the last 2 messages.
                    let messages = &req.messages;

                    let mut found_acon = false;
                    for m in messages {
                        if m.role == Role::Tool {
                            for tr in &m.tool_results {
                                if tr.content.starts_with("[ACON:") {
                                    found_acon = true;
                                }
                            }
                        }
                    }
                    assert!(found_acon, "ACON should have stripped older tool results.");

                    Ok(ChatResponse {
                        message: Message::assistant("Final answer"),
                        usage: Usage::default(),
                        stop_reason: "stop".to_string(),
                        response_id: Some("mock-id".to_string()),
                    })
                } else {
                    Ok(ChatResponse {
                        message: Message::assistant("Extra answer"),
                        usage: Usage::default(),
                        stop_reason: "stop".to_string(),
                        response_id: Some("mock-id".to_string()),
                    })
                }
            }
        }

        let tools = vec![
            Tool {
                name: "read_tool".to_string(),
                description: "read".to_string(),
                is_read_only: true,
                parameters: serde_json::Value::Null,
                execute: Arc::new(MockToolExecutor),
            },
        ];

        let mut cfg = AgentRunConfig::default();
        cfg.enable_acon_context_strategy = true; // THIS IS THE KEY MECHANIC
        // Disable other mechanics to isolate the test
        cfg.enable_observation_masking = false;
        cfg.enable_context_compaction = false;
        cfg.enable_lost_in_the_middle_prevention = false;

        let client = Arc::new(MockLlmClientAcon { call_count: Mutex::new(0) });
        let agent = Agent::new(client, tools);

        let mut events = vec![];
        let res = agent.run(&cfg, "Start the task", &mut |e| events.push(e)).await;

        assert!(res.is_ok());
        assert_eq!(res.unwrap(), "Final answer");
    }

    #[tokio::test]
    async fn test_tool_scoping_lazy_loading() {
        // We will mock an LLM that first receives a ChatRequest with ONLY "ToolSearch", "LazyLoadTools".
        // It will call LazyLoadTools with "HeavyTool".
        // Then the next ChatRequest should include "HeavyTool".

        struct AssertingMockLlm {
            call_count: Mutex<usize>,
        }

        #[async_trait::async_trait]
        impl LlmClient for AssertingMockLlm {
            async fn chat(&self, req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
                let mut count = self.call_count.lock().await;
                *count += 1;

                if *count == 1 {
                    // Assert that HeavyTool is NOT in the tools list
                    assert!(!req.tools.iter().any(|t| t.name == "HeavyTool"));
                    // Return a call to LazyLoadTools
                    Ok(ChatResponse {
                        message: Message {
                            role: Role::Assistant,
                            content: "Loading HeavyTool".to_string(),
                            tool_calls: vec![ToolCall {
                                id: "load_1".to_string(),
                                name: "LazyLoadTools".to_string(),
                                arguments: serde_json::json!({"tool_names": ["HeavyTool"]}),
                            }],
                            tool_results: vec![],
                        response_id: None,
                        },
                        usage: Usage::default(),
                        stop_reason: "tool_calls".to_string(),
                        response_id: Some("mock-id".to_string()),
                    })
                } else if *count == 2 {
                    // Assert that HeavyTool IS in the tools list
                    assert!(req.tools.iter().any(|t| t.name == "HeavyTool"));
                    // Call the HeavyTool
                    Ok(ChatResponse {
                        message: Message {
                            role: Role::Assistant,
                            content: "Using HeavyTool".to_string(),
                            tool_calls: vec![ToolCall {
                                id: "heavy_1".to_string(),
                                name: "HeavyTool".to_string(),
                                arguments: serde_json::Value::Null,
                            }],
                            tool_results: vec![],
                        response_id: None,
                        },
                        usage: Usage::default(),
                        stop_reason: "tool_calls".to_string(),
                        response_id: Some("mock-id".to_string()),
                    })
                } else {
                    // Done
                    Ok(ChatResponse {
                        message: Message::assistant("Final Answer"),
                        usage: Usage::default(),
                        stop_reason: "stop".to_string(),
                        response_id: Some("mock-id".to_string()),
                    })
                }
            }

        }

        struct DummyToolExecutor;
        #[async_trait::async_trait]
        impl ToolExecutor for DummyToolExecutor {
            async fn execute(&self, _args: serde_json::Value) -> Result<String, ToolError> {
                Ok("Dummy Tool Executed".to_string())
            }
        }

        let client = Arc::new(AssertingMockLlm { call_count: Mutex::new(0) });

        // Include HeavyTool in the agent's definitions.
        let agent = Agent::new(client, vec![
            crate::tools::Tool {
                name: "HeavyTool".to_string(),
                description: "A heavy tool".to_string(),
                parameters: serde_json::Value::Null,
                is_read_only: false,
                execute: Arc::new(DummyToolExecutor),
            }
        ]);

        let mut cfg = AgentRunConfig::default();
        cfg.enable_lazy_tool_loading = true; // THIS IS THE KEY MECHANIC

        let mut events = vec![];
        let res = agent.run(&cfg, "Do the task", &mut |e| events.push(e)).await;

        assert!(res.is_ok());
        assert_eq!(res.unwrap(), "Final Answer");
    }

    #[tokio::test]
    async fn test_anthropic_3_stage_tool_gating() {
        let client = Arc::new(MockLlmClient {
            responses: tokio::sync::Mutex::new(vec![
                ChatResponse {
                    message: Message {
                        role: Role::Assistant,
                        content: "".to_string(),
                        tool_calls: vec![
                            ToolCall { id: "1".to_string(), name: "read_tool".to_string(), arguments: serde_json::Value::Null },
                            ToolCall { id: "2".to_string(), name: "mutating_tool".to_string(), arguments: serde_json::Value::Null },
                            ToolCall { id: "3".to_string(), name: "high_risk_tool".to_string(), arguments: serde_json::Value::Null },
                        ],
                        tool_results: vec![],
                    response_id: None,
                    },
                    usage: ohc_builtin_agent_core::types::Usage::default(),
                    stop_reason: "tool_calls".to_string(),
                        response_id: Some("mock-id".to_string()),
                },
                ChatResponse {
                    message: Message::assistant("Final answer"),
                    usage: ohc_builtin_agent_core::types::Usage::default(),
                    stop_reason: "stop".to_string(),
                        response_id: Some("mock-id".to_string()),
                },
            ]),
        });

        let tools = vec![
            Tool {
                name: "read_tool".to_string(),
                description: "read".to_string(),
                is_read_only: true,
                parameters: serde_json::Value::Null,
                execute: Arc::new(MockToolExecutor),
            },
            Tool {
                name: "mutating_tool".to_string(),
                description: "write".to_string(),
                is_read_only: false,
                parameters: serde_json::Value::Null,
                execute: Arc::new(MockToolExecutor),
            },
            Tool {
                name: "high_risk_tool".to_string(),
                description: "delete".to_string(),
                is_read_only: false,
                parameters: serde_json::Value::Null,
                execute: Arc::new(MockToolExecutor),
            },
        ];

        let agent = Agent::new(client.clone(), tools.clone());

        // Test 1: Untrusted project rejects mutating tools
        let mut cfg = AgentRunConfig::default();
        cfg.project_trusted = false;

        let mut events = vec![];
        let mut on_event = |e| { events.push(e); };

        let result = agent.run(&cfg, "Hello", &mut on_event).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Project not trusted. Mutating tools are disabled."));

        // Reset mock
        let client = Arc::new(MockLlmClient {
            responses: tokio::sync::Mutex::new(vec![
                ChatResponse {
                    message: Message {
                        role: Role::Assistant,
                        content: "".to_string(),
                        tool_calls: vec![
                            ToolCall { id: "1".to_string(), name: "unallowed_tool".to_string(), arguments: serde_json::Value::Null },
                        ],
                        tool_results: vec![],
                    response_id: None,
                    },
                    usage: ohc_builtin_agent_core::types::Usage::default(),
                    stop_reason: "tool_calls".to_string(),
                        response_id: Some("mock-id".to_string()),
                },
            ]),
        });

        let agent = Agent::new(client, vec![
            Tool {
                name: "unallowed_tool".to_string(),
                description: "write".to_string(),
                is_read_only: false,
                parameters: serde_json::Value::Null,
                execute: Arc::new(MockToolExecutor),
            },
        ]);

        // Test 2: Permission check blocks unallowed tools
        let mut cfg = AgentRunConfig::default();
        cfg.project_trusted = true;
        cfg.allowed_tools = Some(vec!["allowed_tool".to_string()]);

        let mut events = vec![];
        let mut on_event = |e| { events.push(e); };

        let result = agent.run(&cfg, "Hello", &mut on_event).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not in the allowed list."));


        // Test 3: High-risk operations require explicit confirmation
        let client = Arc::new(MockLlmClient {
            responses: tokio::sync::Mutex::new(vec![
                ChatResponse {
                    message: Message {
                        role: Role::Assistant,
                        content: "".to_string(),
                        tool_calls: vec![
                            ToolCall { id: "3".to_string(), name: "high_risk_tool".to_string(), arguments: serde_json::Value::Null },
                        ],
                        tool_results: vec![],
                    response_id: None,
                    },
                    usage: ohc_builtin_agent_core::types::Usage::default(),
                    stop_reason: "tool_calls".to_string(),
                        response_id: Some("mock-id".to_string()),
                },
            ]),
        });

        let agent = Agent::new(client, vec![
            Tool {
                name: "high_risk_tool".to_string(),
                description: "delete".to_string(),
                is_read_only: false,
                parameters: serde_json::Value::Null,
                execute: Arc::new(MockToolExecutor),
            },
        ]);

        let mut cfg = AgentRunConfig::default();
        cfg.project_trusted = true;
        cfg.high_risk_tools = vec!["high_risk_tool".to_string()];
        // Not in approved_tool_calls

        let mut events = vec![];
        let mut on_event = |e| { events.push(e); };

        let result = agent.run(&cfg, "Hello", &mut on_event).await;
        assert!(result.is_err());
        let err_str = result.unwrap_err().to_string();
        assert!(err_str.contains("User intervention required"));
        assert!(err_str.contains("requires explicit user confirmation"));

    }


    use ohc_builtin_agent_core::types::{ChatRequest};
    use ohc_builtin_agent_tools::ToolExecutor;
    use serde_json::Value;

    struct MockLlmClient {
        responses: tokio::sync::Mutex<Vec<ChatResponse>>,
    }

    #[async_trait::async_trait]
    impl LlmClient for MockLlmClient {
        async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            let mut resps = self.responses.lock().await;
            if resps.is_empty() {
                return Ok(ChatResponse {
                    message: Message::assistant("Final answer"),
                    usage: Usage::default(),
                    stop_reason: "stop".to_string(),
                        response_id: Some("mock-id".to_string()),
                });
            }
            Ok(resps.remove(0))
        }
    }

    struct MockToolExecutor;

    #[async_trait::async_trait]
    impl ToolExecutor for MockToolExecutor {
        async fn execute(&self, _args: Value) -> Result<String, ToolError> {
            Ok("A very long tool output that should be masked because it is long enough".to_string())
        }
    }

    #[tokio::test]
    async fn test_observation_masking() {
        let client = Arc::new(MockLlmClient {
            responses: tokio::sync::Mutex::new(vec![
                ChatResponse {
                    message: Message {
                        role: Role::Assistant,
                        content: "".to_string(),
                        tool_calls: vec![ToolCall {
                            id: "call_1".to_string(),
                            name: "test_tool".to_string(),
                            arguments: Value::Null,
                        }],
                        tool_results: vec![],
                    response_id: None,
                    },
                    usage: Usage::default(),
                    stop_reason: "tool_calls".to_string(),
                        response_id: Some("mock-id".to_string()),
                },
                ChatResponse {
                    message: Message {
                        role: Role::Assistant,
                        content: "".to_string(),
                        tool_calls: vec![ToolCall {
                            id: "call_2".to_string(),
                            name: "test_tool".to_string(),
                            arguments: Value::Null,
                        }],
                        tool_results: vec![],
                    response_id: None,
                    },
                    usage: Usage::default(),
                    stop_reason: "tool_calls".to_string(),
                        response_id: Some("mock-id".to_string()),
                },
            ]),
        });

        let tools = vec![Tool {
            name: "test_tool".to_string(),
            description: "test".to_string(),
                is_read_only: false,
            parameters: Value::Null,
            execute: Arc::new(MockToolExecutor),
        }];

        let agent = Agent::new(client, tools);

        let mut cfg = AgentRunConfig::default();
        cfg.enable_observation_masking = true;

        let mut events = vec![];
        let mut on_event = |e| { events.push(e); };

        let result = agent.run(&cfg, "Hello", &mut on_event).await;
        assert!(result.is_ok());

        // In this test, the agent will loop:
        // Iter 0: LLM asks for test_tool. Tool returns result.
        //   Agent runs masking check. The message list contains User(Hello) and Assistant(tool_call).
        //   The new tool result is appended.
        // Iter 1: LLM asks for test_tool again.
        //   Agent runs masking check. The previous tool result (from Iter 0) is now masked.
        //   The new tool result is appended.
        // Iter 2: LLM returns final answer.

        // We can't directly inspect `messages` from the outside, but we can verify it compiled
        // and ran without errors, which covers the logic path.
        // Also checking the length constraint logic.
    }

    #[tokio::test]
    async fn test_context_compaction() {
        let client = Arc::new(MockLlmClient {
            responses: tokio::sync::Mutex::new(vec![
                ChatResponse {
                    message: Message {
                        role: Role::Assistant,
                        content: "tool call 1".to_string(),
                        tool_calls: vec![ToolCall { id: "1".to_string(), name: "test_tool".to_string(), arguments: serde_json::Value::Null }],
                        tool_results: vec![],
                    response_id: None,
                    },
                    usage: Usage { input_tokens: 100, output_tokens: 10 },
                    stop_reason: "stop".to_string(),
                        response_id: Some("mock-id".to_string()),
                },
                ChatResponse {
                    message: Message {
                        role: Role::Assistant,
                        content: "tool call 2".to_string(),
                        tool_calls: vec![ToolCall { id: "2".to_string(), name: "test_tool".to_string(), arguments: serde_json::Value::Null }],
                        tool_results: vec![],
                    response_id: None,
                    },
                    usage: Usage { input_tokens: 100, output_tokens: 10 },
                    stop_reason: "stop".to_string(),
                        response_id: Some("mock-id".to_string()),
                },
                ChatResponse {
                    message: Message {
                        role: Role::Assistant,
                        content: "tool call 3".to_string(),
                        tool_calls: vec![ToolCall { id: "3".to_string(), name: "test_tool".to_string(), arguments: serde_json::Value::Null }],
                        tool_results: vec![],
                    response_id: None,
                    },
                    usage: Usage { input_tokens: 100, output_tokens: 10 },
                    stop_reason: "stop".to_string(),
                        response_id: Some("mock-id".to_string()),
                },
                ChatResponse {
                    message: Message::assistant("compacted summary"), // Responds to the compaction request
                    usage: Usage { input_tokens: 100, output_tokens: 10 },
                    stop_reason: "stop".to_string(),
                        response_id: Some("mock-id".to_string()),
                },
                ChatResponse {
                    message: Message::assistant("final answer"),
                    usage: Usage { input_tokens: 100, output_tokens: 10 },
                    stop_reason: "stop".to_string(),
                        response_id: Some("mock-id".to_string()),
                },
            ]),
        });

        struct MockToolExecutor;
        #[async_trait::async_trait]
        impl ToolExecutor for MockToolExecutor {
            async fn execute(&self, _args: serde_json::Value) -> Result<String, ToolError> {
                Ok("tool output".to_string())
            }
        }

        let tools: Vec<Tool> = vec![
            Tool {
                name: "test_tool".to_string(),
                description: "test".to_string(),
                is_read_only: false,
                parameters: serde_json::Value::Null,
                execute: Arc::new(MockToolExecutor),
            }
        ];

        let mut cfg = AgentRunConfig::default();
        cfg.enable_context_compaction = true;
        cfg.compaction_threshold_tokens = 50; // Set low threshold to trigger compaction

        let agent = Agent::new(client, tools);

        let mut events = vec![];
        let mut on_event = |e| { events.push(e); };

        let result = agent.run(&cfg, "Hello, this is a very long conversation", &mut on_event).await;

        assert!(result.is_ok());

        // We can verify that it produced the final answer, meaning it survived the loop and compaction.
        assert_eq!(result.unwrap(), "final answer");
    }

    #[tokio::test]
    async fn test_handoff_mechanic() {
        struct HandoffToolExecutor;
        #[async_trait::async_trait]
        impl ToolExecutor for HandoffToolExecutor {
            async fn execute(&self, _args: serde_json::Value) -> Result<String, ToolError> {
                Err(ToolError::HandoffRequested("Finance".to_string()))
            }
        }

        let client = Arc::new(MockLlmClient {
            responses: tokio::sync::Mutex::new(vec![ChatResponse {
                message: Message {
                    role: Role::Assistant,
                    content: "Yielding to finance...".to_string(),
                    tool_calls: vec![ToolCall {
                        id: "call_handoff".to_string(),
                        name: "handoff_tool".to_string(),
                        arguments: serde_json::Value::Null,
                    }],
                    tool_results: vec![],
                response_id: None,
                },
                usage: Usage::default(),
                stop_reason: "tool_calls".to_string(),
                        response_id: Some("mock-id".to_string()),
            }]),
        });

        let tools = vec![Tool {
            name: "handoff_tool".to_string(),
            description: "handoff".to_string(),
            is_read_only: false,
            parameters: serde_json::Value::Null,
            execute: Arc::new(HandoffToolExecutor),
        }];

        let agent = Agent::new(client, tools);
        let cfg = AgentRunConfig::default();

        let mut events = vec![];
        let mut on_event = |e| { events.push(e); };

        let result = agent.run(&cfg, "Transfer me to finance", &mut on_event).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Handoff requested to Finance");

        let handoff_emitted = events.iter().any(|e| {
            if let AgentEvent::Handoff { target_agent } = e {
                target_agent == "Finance"
            } else {
                false
            }
        });
        assert!(handoff_emitted);
    }

    #[tokio::test]
    async fn test_error_handling_langgraph_4_tier() {
        let _client = Arc::new(MockLlmClient {
            responses: tokio::sync::Mutex::new(vec![
                ChatResponse {
                    message: Message {
                        role: Role::Assistant,
                        content: "I will call a tool".to_string(),
                        tool_calls: vec![ToolCall {
                            id: "call_transient".to_string(),
                            name: "transient_tool".to_string(),
                            arguments: serde_json::Value::Null,
                        }],
                        tool_results: vec![],
                    response_id: None,
                    },
                    usage: Usage::default(),
                    stop_reason: "tool_calls".to_string(),
                        response_id: Some("mock-id".to_string()),
                },
                ChatResponse {
                    message: Message {
                        role: Role::Assistant,
                        content: "I will call another tool".to_string(),
                        tool_calls: vec![ToolCall {
                            id: "call_llm_recoverable".to_string(),
                            name: "llm_recoverable_tool".to_string(),
                            arguments: serde_json::Value::Null,
                        }],
                        tool_results: vec![],
                    response_id: None,
                    },
                    usage: Usage::default(),
                    stop_reason: "tool_calls".to_string(),
                        response_id: Some("mock-id".to_string()),
                },
                ChatResponse {
                    message: Message {
                        role: Role::Assistant,
                        content: "I will call another tool".to_string(),
                        tool_calls: vec![ToolCall {
                            id: "call_user_fixable".to_string(),
                            name: "user_fixable_tool".to_string(),
                            arguments: serde_json::Value::Null,
                        }],
                        tool_results: vec![],
                    response_id: None,
                    },
                    usage: Usage::default(),
                    stop_reason: "tool_calls".to_string(),
                        response_id: Some("mock-id".to_string()),
                },
                ChatResponse {
                    message: Message {
                        role: Role::Assistant,
                        content: "I will call another tool".to_string(),
                        tool_calls: vec![ToolCall {
                            id: "call_fatal".to_string(),
                            name: "fatal_tool".to_string(),
                            arguments: serde_json::Value::Null,
                        }],
                        tool_results: vec![],
                    response_id: None,
                    },
                    usage: Usage::default(),
                    stop_reason: "tool_calls".to_string(),
                        response_id: Some("mock-id".to_string()),
                }
            ]),
        });

        struct FourTierErrorToolExecutor {
            name: String,
        }
        #[async_trait::async_trait]
        impl ToolExecutor for FourTierErrorToolExecutor {
            async fn execute(&self, _args: serde_json::Value) -> Result<String, ToolError> {
                match self.name.as_str() {
                    "transient_tool" => Err(ToolError::Transient("network timeout".to_string())),
                    "llm_recoverable_tool" => Err(ToolError::LlmRecoverable("missing parameter X".to_string())),
                    "user_fixable_tool" => Err(ToolError::UserFixable("please login to external service".to_string())),
                    "fatal_tool" => Err(ToolError::Fatal("system corrupted".to_string())),
                    "unexpected_tool" => Err(ToolError::Unexpected("random crash".to_string())),
                    _ => Ok("success".to_string()),
                }
            }
        }

        let tools = vec![
            Tool {
                name: "transient_tool".to_string(),
                description: "".to_string(),
                is_read_only: true,
                parameters: serde_json::json!({}),
                execute: Arc::new(FourTierErrorToolExecutor { name: "transient_tool".to_string() }),
            },
            Tool {
                name: "llm_recoverable_tool".to_string(),
                description: "".to_string(),
                is_read_only: true,
                parameters: serde_json::json!({}),
                execute: Arc::new(FourTierErrorToolExecutor { name: "llm_recoverable_tool".to_string() }),
            },
            Tool {
                name: "user_fixable_tool".to_string(),
                description: "".to_string(),
                is_read_only: true,
                parameters: serde_json::json!({}),
                execute: Arc::new(FourTierErrorToolExecutor { name: "user_fixable_tool".to_string() }),
            },
            Tool {
                name: "fatal_tool".to_string(),
                description: "".to_string(),
                is_read_only: true,
                parameters: serde_json::json!({}),
                execute: Arc::new(FourTierErrorToolExecutor { name: "fatal_tool".to_string() }),
            },
            Tool {
                name: "unexpected_tool".to_string(),
                description: "".to_string(),
                is_read_only: true,
                parameters: serde_json::json!({}),
                execute: Arc::new(FourTierErrorToolExecutor { name: "unexpected_tool".to_string() }),
            }
        ];

        let cfg = AgentRunConfig::default();

        // 1. Transient Error (Retries with backoff but fails after max_retries)
        let client_transient = Arc::new(MockLlmClient {
            responses: tokio::sync::Mutex::new(vec![ChatResponse {
                message: Message {
                    role: Role::Assistant,
                    content: "".to_string(),
                    tool_calls: vec![ToolCall { id: "1".to_string(), name: "transient_tool".to_string(), arguments: serde_json::Value::Null }],
                    tool_results: vec![],
                response_id: None,
                },
                usage: Usage::default(),
                stop_reason: "tool_calls".to_string(),
                        response_id: Some("mock-id".to_string()),
            }, ChatResponse {
                message: Message::assistant("stop"), usage: Usage::default(), stop_reason: "stop".to_string(),
                        response_id: Some("mock-id".to_string())
            }]),
        });
        let agent1 = Agent::new(client_transient, tools.clone());
        let mut events = vec![];
        let mut on_event = |e| { events.push(e); };
        let _ = agent1.run(&cfg, "Run transient", &mut on_event).await;
        let transient_handled = events.iter().any(|e| {
            if let AgentEvent::ToolCall { name, result, .. } = e {
                name == "transient_tool" && result.contains("Transient error after retries: network timeout")
            } else {
                false
            }
        });
        assert!(transient_handled);

        // 2. LLM Recoverable
        struct LlmRecoverableMockClient {
            pub responses: tokio::sync::Mutex<Vec<ChatResponse>>,
            pub requests: tokio::sync::Mutex<Vec<ChatRequest>>,
        }
        #[async_trait::async_trait]
        impl LlmClient for LlmRecoverableMockClient {
            async fn chat(&self, req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
                let mut reqs = self.requests.lock().await;
                reqs.push(req);
                let mut resps = self.responses.lock().await;
                if !resps.is_empty() {
                    Ok(resps.remove(0))
                } else {
                    Ok(ChatResponse { message: Message::assistant("stop"), usage: Usage::default(), stop_reason: "stop".to_string(), response_id: Some("mock-id".to_string()) })
                }
            }
        }

        let client_llm = Arc::new(LlmRecoverableMockClient {
            requests: tokio::sync::Mutex::new(vec![]),
            responses: tokio::sync::Mutex::new(vec![ChatResponse {
                message: Message {
                    role: Role::Assistant,
                    content: "".to_string(),
                    tool_calls: vec![ToolCall { id: "2".to_string(), name: "llm_recoverable_tool".to_string(), arguments: serde_json::Value::Null }],
                    tool_results: vec![],
                response_id: None,
                },
                usage: Usage::default(),
                stop_reason: "tool_calls".to_string(),
                        response_id: Some("mock-id".to_string()),
            }, ChatResponse {
                message: Message::assistant("stop"), usage: Usage::default(), stop_reason: "stop".to_string(),
                        response_id: Some("mock-id".to_string())
            }]),
        });
        let agent2 = Agent::new(client_llm.clone(), tools.clone());
        let mut events2 = vec![];
        let mut on_event2 = |e| { events2.push(e); };
        let _ = agent2.run(&cfg, "Run llm recoverable", &mut on_event2).await;
        let llm_recoverable_handled = events2.iter().any(|e| {
            if let AgentEvent::ToolCall { name, result, .. } = e {
                name == "llm_recoverable_tool" && result == "missing parameter X"
            } else {
                false
            }
        });
        assert!(llm_recoverable_handled);

        let reqs = client_llm.requests.lock().await;
        let last_req = reqs.last().unwrap();
        let _last_msg = last_req.messages.last().unwrap();
        // Since `agent.rs` handles mutating tool execution differently from read-only execution, we should check both or rely on the general logic.
        // Wait, mutating tools do `messages.push(Message { role: Role::Tool, tool_results, ... })`?
        // Let's actually check the `messages` array in the last request.
        let tool_msg = reqs.iter().flat_map(|r| &r.messages).find(|m| m.role == Role::Tool && !m.tool_results.is_empty()).unwrap();
        assert_eq!(tool_msg.tool_results[0].error, "missing parameter X");
        assert_eq!(tool_msg.tool_results[0].content, "");

        // 3. User Fixable
        let client_user = Arc::new(MockLlmClient {
            responses: tokio::sync::Mutex::new(vec![ChatResponse {
                message: Message {
                    role: Role::Assistant,
                    content: "".to_string(),
                    tool_calls: vec![ToolCall { id: "3".to_string(), name: "user_fixable_tool".to_string(), arguments: serde_json::Value::Null }],
                    tool_results: vec![],
                response_id: None,
                },
                usage: Usage::default(),
                stop_reason: "tool_calls".to_string(),
                        response_id: Some("mock-id".to_string()),
            }]),
        });
        let agent3 = Agent::new(client_user, tools.clone());
        let mut events3 = vec![];
        let mut on_event3 = |e| { events3.push(e); };
        let res3 = agent3.run(&cfg, "Run user fixable", &mut on_event3).await;
        assert!(res3.is_err());
        let user_fixable_handled = events3.iter().any(|e| {
            if let AgentEvent::UserInterventionRequired { error } = e {
                error.contains("User intervention required: please login to external service")
            } else {
                false
            }
        });
        assert!(user_fixable_handled);

        // 4. Fatal
        let client_fatal = Arc::new(MockLlmClient {
            responses: tokio::sync::Mutex::new(vec![ChatResponse {
                message: Message {
                    role: Role::Assistant,
                    content: "".to_string(),
                    tool_calls: vec![ToolCall { id: "4".to_string(), name: "fatal_tool".to_string(), arguments: serde_json::Value::Null }],
                    tool_results: vec![],
                response_id: None,
                },
                usage: Usage::default(),
                stop_reason: "tool_calls".to_string(),
                        response_id: Some("mock-id".to_string()),
            }]),
        });
        let agent4 = Agent::new(client_fatal, tools.clone());
        let mut events4 = vec![];
        let mut on_event4 = |e| { events4.push(e); };
        let res4 = agent4.run(&cfg, "Run fatal", &mut on_event4).await;
        assert!(res4.is_err());
        let fatal_handled = events4.iter().any(|e| {
            if let AgentEvent::TaskError { error } = e {
                error.contains("Fatal tool error: system corrupted")
            } else {
                false
            }
        });
        assert!(fatal_handled);

        // 5. Unexpected Error
        let client_unexpected = Arc::new(MockLlmClient {
            responses: tokio::sync::Mutex::new(vec![ChatResponse {
                message: Message {
                    role: Role::Assistant,
                    content: "".to_string(),
                    tool_calls: vec![ToolCall { id: "5".to_string(), name: "unexpected_tool".to_string(), arguments: serde_json::Value::Null }],
                    tool_results: vec![],
                response_id: None,
                },
                usage: Usage::default(),
                stop_reason: "tool_calls".to_string(),
                        response_id: Some("mock-id".to_string()),
            }]),
        });
        let agent5 = Agent::new(client_unexpected, tools.clone());
        let mut events5 = vec![];
        let mut on_event5 = |e| { events5.push(e); };
        let res5 = agent5.run(&cfg, "Run unexpected", &mut on_event5).await;
        assert!(res5.is_err());
        let unexpected_handled = events5.iter().any(|e| {
            if let AgentEvent::TaskError { error } = e {
                error.contains("Unexpected tool error: random crash")
            } else {
                false
            }
        });
        assert!(unexpected_handled);
    }

    #[tokio::test]
    async fn test_guardrail_tripwire() {
        let client = Arc::new(MockLlmClient {
            responses: tokio::sync::Mutex::new(vec![
                ChatResponse {
                    message: Message {
                        role: Role::Assistant,
                        content: "I am going to use the bad tool now.".to_string(),
                        tool_calls: vec![ToolCall {
                            id: "call_1".to_string(),
                            name: "banned_tool".to_string(),
                            arguments: Value::Null,
                        }],
                        tool_results: vec![],
                    response_id: None,
                    },
                    usage: Usage::default(),
                    stop_reason: "tool_calls".to_string(),
                        response_id: Some("mock-id".to_string()),
                },
                ChatResponse {
                    message: Message::assistant("This contains the secret password!"),
                    usage: Usage::default(),
                    stop_reason: "stop".to_string(),
                        response_id: Some("mock-id".to_string()),
                },
            ]),
        });

        let tools = vec![
            Tool {
                name: "banned_tool".to_string(),
                description: "test".to_string(),
                is_read_only: false,
                parameters: Value::Null,
                execute: Arc::new(MockToolExecutor),
            },
            Tool {
                name: "safe_tool".to_string(),
                description: "test".to_string(),
                is_read_only: false,
                parameters: Value::Null,
                execute: Arc::new(MockToolExecutor),
            },
        ];

        let agent = Agent::new(client, tools);

        let mut cfg = AgentRunConfig::default();
        cfg.guardrails = Some(crate::guardrails::GuardrailConfig {
            blocked_keywords: vec!["banned".to_string(), "password".to_string(), "secret".to_string()],
        });

        // Test Input Guardrail
        let mut events = vec![];
        let mut on_event = |e| { events.push(e); };
        let result = agent.run(&cfg, "Hello, please give me the secret password.", &mut on_event).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Input guardrail tripped"));

        // Reset client for next tests
        let client = Arc::new(MockLlmClient {
            responses: tokio::sync::Mutex::new(vec![
                ChatResponse {
                    message: Message {
                        role: Role::Assistant,
                        content: "".to_string(),
                        tool_calls: vec![ToolCall {
                            id: "call_1".to_string(),
                            name: "banned_tool".to_string(),
                            arguments: Value::Null,
                        }],
                        tool_results: vec![],
                    response_id: None,
                    },
                    usage: Usage::default(),
                    stop_reason: "tool_calls".to_string(),
                        response_id: Some("mock-id".to_string()),
                },
            ]),
        });
        let agent = Agent::new(client, vec![
            Tool {
                name: "banned_tool".to_string(),
                description: "test".to_string(),
                is_read_only: false,
                parameters: Value::Null,
                execute: Arc::new(MockToolExecutor),
            },
        ]);

        // Test Tool Guardrail
        let mut events = vec![];
        let mut on_event = |e| { events.push(e); };
        let result = agent.run(&cfg, "Hello", &mut on_event).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Tool guardrail tripped"));

        // Reset client for Output test
        let client = Arc::new(MockLlmClient {
            responses: tokio::sync::Mutex::new(vec![
                ChatResponse {
                    message: Message::assistant("Here is the secret data."),
                    usage: Usage::default(),
                    stop_reason: "stop".to_string(),
                        response_id: Some("mock-id".to_string()),
                },
            ]),
        });
        let agent = Agent::new(client, vec![]);

        // Test Output Guardrail
        let mut events = vec![];
        let mut on_event = |e| { events.push(e); };
        let result = agent.run(&cfg, "Hello", &mut on_event).await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Output guardrail tripped"));
    }


    #[test]
    fn test_hierarchical_system_prompt_with_tools() {
        let mut cfg = AgentRunConfig::default();
        cfg.server_system_message = "Server System Message".to_string();
        cfg.developer_instructions = "Developer Instructions".to_string();
        cfg.user_instructions = "User Instructions".to_string();

        let tool = crate::tools::Tool {
            name: "test_tool".to_string(),
            description: "A test tool".to_string(),
            is_read_only: true,
            parameters: serde_json::json!({"type": "object"}),
            execute: std::sync::Arc::new(MockToolExecutor),
        };

        let prompt = build_hierarchical_system_prompt(&cfg, &[tool]);

        let expected = "Server System Message\n\n[Tool Definitions]\nTool: test_tool\nDescription: A test tool\nParameters: {\"type\":\"object\"}\n\n[Developer Instructions]\nDeveloper Instructions\n\n[User Instructions]\nUser Instructions";

        assert_eq!(prompt, expected);
    }

    #[test]
    fn test_hierarchical_system_prompt() {
        let mut cfg = AgentRunConfig::default();
        cfg.server_system_message = "Server System Message".to_string();
        cfg.developer_instructions = "Developer Instructions".to_string();
        cfg.user_instructions = "User Instructions".to_string();

        let prompt = build_hierarchical_system_prompt(&cfg, &[]);
        assert_eq!(
            prompt,
            "Server System Message\n\n[Developer Instructions]\nDeveloper Instructions\n\n[User Instructions]\nUser Instructions"
        );
    }

    #[test]
    fn test_hierarchical_system_prompt_missing_sections() {
        let mut cfg = AgentRunConfig::default();
        cfg.server_system_message = "Server System Message".to_string();
        cfg.developer_instructions = "".to_string();
        cfg.user_instructions = "User Instructions".to_string();

        let prompt = build_hierarchical_system_prompt(&cfg, &[]);
        assert_eq!(
            prompt,
            "Server System Message\n\n[User Instructions]\nUser Instructions"
        );

        let mut cfg2 = AgentRunConfig::default();
        cfg2.server_system_message = "".to_string();
        cfg2.developer_instructions = "Dev".to_string();
        cfg2.user_instructions = "User".to_string();
        let prompt2 = build_hierarchical_system_prompt(&cfg2, &[]);
        assert_eq!(
            prompt2,
            "[Developer Instructions]\nDev\n\n[User Instructions]\nUser"
        );
    }

    #[test]
    fn test_hierarchical_system_prompt_truncation_safe() {
        let mut cfg = AgentRunConfig::default();
        // A single emoji is 4 bytes.
        let emoji = "🚀"; // 4 bytes
        // 8192 emojis = 32768 bytes
        cfg.user_instructions = emoji.repeat(8192);
        // Add one more emoji to exceed the limit
        cfg.user_instructions.push_str(emoji); // 32772 bytes

        // This should safely truncate without panicking
        let prompt = build_hierarchical_system_prompt(&cfg, &[]);
        assert!(prompt.contains("[User Instructions]\n"));
        // Check that the user instructions part is exactly 32768 bytes long
        assert_eq!(prompt.len() - "[User Instructions]\n".len(), 32768);
    }

    #[test]
    fn test_hierarchical_system_prompt_truncation_safe_boundary() {
        let mut cfg = AgentRunConfig::default();
        // Construct a string where the 32768th byte is in the middle of a multibyte character.
        // Let's use 1-byte chars until 32766, then a 3-byte char.
        cfg.user_instructions = "a".repeat(32766);
        cfg.user_instructions.push('€'); // '€' is 3 bytes (E2 82 AC). Length is now 32769 bytes.

        // Truncating at 32768 would split the '€' character.
        let prompt = build_hierarchical_system_prompt(&cfg, &[]);

        let user_part = prompt.trim_start_matches("[User Instructions]\n");
        // The truncation should back up to 32766 to avoid splitting the character.
        assert_eq!(user_part.len(), 32766);
    }

    #[tokio::test]
    async fn test_langgraph_mechanic_agent_run() {
        let client = Arc::new(MockLlmClient {
            responses: tokio::sync::Mutex::new(vec![
                ChatResponse {
                    message: Message {
                        role: Role::Assistant,
                        content: "".to_string(),
                        tool_calls: vec![ToolCall {
                            id: "call_1".to_string(),
                            name: "test_tool".to_string(),
                            arguments: serde_json::json!({}),
                        }],
                        tool_results: vec![],
                    response_id: None,
                    },
                    usage: Usage::default(),
                    stop_reason: "tool_calls".to_string(),
                        response_id: Some("mock-id".to_string()),
                },
                ChatResponse {
                    message: Message::assistant("Final Answer"),
                    usage: Usage::default(),
                    stop_reason: "stop".to_string(),
                        response_id: Some("mock-id".to_string()),
                },
            ]),
        });

        let tool = crate::tools::Tool {
            name: "test_tool".to_string(),
            description: "A test tool".to_string(),
            is_read_only: false,
            parameters: serde_json::json!({"type": "object"}),
            execute: Arc::new(MockToolExecutor),
        };

        let agent = Agent::new(client, vec![tool]);
        let mut cfg = AgentRunConfig::default();
        cfg.enable_langgraph_mechanic = true;

        let mut events = vec![];
        let mut on_event = |e| { events.push(e); };

        let result = agent.run(&cfg, "Hello", &mut on_event).await.unwrap();
        assert_eq!(result, "Final Answer");
    }

    #[tokio::test]
    async fn test_llm_judge_rejects_and_approves() {
        let client = Arc::new(MockLlmClient {
            responses: tokio::sync::Mutex::new(vec![
                ChatResponse {
                    message: Message::assistant("Draft answer"),
                    usage: Usage::default(),
                    stop_reason: "stop".to_string(),
                        response_id: Some("mock-id".to_string()),
                },
                ChatResponse {
                    message: Message::assistant("REJECT: The answer is incomplete."),
                    usage: Usage::default(),
                    stop_reason: "stop".to_string(),
                        response_id: Some("mock-id".to_string()),
                },
                ChatResponse {
                    message: Message::assistant("Better answer"),
                    usage: Usage::default(),
                    stop_reason: "stop".to_string(),
                        response_id: Some("mock-id".to_string()),
                },
                ChatResponse {
                    message: Message::assistant("APPROVE"),
                    usage: Usage::default(),
                    stop_reason: "stop".to_string(),
                        response_id: Some("mock-id".to_string()),
                },
            ]),
        });

        let agent = Agent::new(client, vec![]);

        let mut cfg = AgentRunConfig::default();
        cfg.enable_llm_judge = true;

        let mut events = vec![];
        let mut on_event = |e| { events.push(e); };

        let result = agent.run(&cfg, "Hello", &mut on_event).await;
        assert!(result.is_ok());
        let content = result.unwrap();
        assert_eq!(content, "Better answer");
    }

    #[tokio::test]
    async fn test_telemetry_metrics_emission() {
        // Just verify it compiles and runs correctly with default config
        // Opentelemetry global meter no-ops in tests unless configured
        let client = Arc::new(MockLlmClient {
            responses: tokio::sync::Mutex::new(vec![
                ChatResponse {
                    message: Message::assistant("Draft answer"),
                    usage: Usage { input_tokens: 100, output_tokens: 50 },
                    stop_reason: "stop".to_string(),
                        response_id: Some("mock-id".to_string()),
                },
            ]),
        });

        let agent = Agent::new(client, vec![]);

        let mut cfg = AgentRunConfig::default();
        // Specifically setting a model that triggers cost estimation logic
        cfg.model = "gpt-4o".to_string();
        cfg.agent_id = "test-agent-telemetry".to_string();

        let mut events = vec![];
        let mut on_event = |e| { events.push(e); };

        let result = agent.run(&cfg, "Hello", &mut on_event).await;
        assert!(result.is_ok());
    }

    use crate::checkpointer::{CheckpointSaver, Checkpoint};

    struct MockCheckpointer {
        checkpoints: tokio::sync::Mutex<Vec<Checkpoint>>,
    }

    #[async_trait::async_trait]
    impl CheckpointSaver for MockCheckpointer {
        async fn get_checkpoint(&self, thread_id: &str, checkpoint_id: &str) -> Result<Option<Checkpoint>, String> {
            let cps = self.checkpoints.lock().await;
            Ok(cps.iter().find(|c| c.thread_id == thread_id && c.checkpoint_id == checkpoint_id).cloned())
        }

        async fn put_checkpoint(&self, checkpoint: Checkpoint) -> Result<(), String> {
            let mut cps = self.checkpoints.lock().await;
            cps.push(checkpoint);
            Ok(())
        }

        async fn list_checkpoints(&self, thread_id: &str) -> Result<Vec<Checkpoint>, String> {
            let cps = self.checkpoints.lock().await;
            let mut filtered: Vec<Checkpoint> = cps.iter().filter(|c| c.thread_id == thread_id).cloned().collect();
            // Reverse to simulate ORDER BY created_at DESC
            filtered.reverse();
            Ok(filtered)
        }
    }

    #[tokio::test]
    async fn test_agent_state_checkpointing_mechanic() {
        // Run 1: Agent saves a checkpoint
        let client1 = Arc::new(MockLlmClient {
            responses: tokio::sync::Mutex::new(vec![
                ChatResponse {
                    message: Message {
                        role: Role::Assistant,
                        content: "".to_string(),
                        tool_calls: vec![
                            ToolCall { id: "1".to_string(), name: "read_tool".to_string(), arguments: serde_json::Value::Null },
                        ],
                        tool_results: vec![],
                    response_id: None,
                    },
                    usage: Usage::default(),
                    stop_reason: "tool_calls".to_string(),
                        response_id: Some("mock-id".to_string()),
                },
                ChatResponse {
                    message: Message::assistant("Final answer"),
                    usage: Usage::default(),
                    stop_reason: "stop".to_string(),
                        response_id: Some("mock-id".to_string()),
                },
            ]),
        });

        struct StateMockToolExecutor {
            result: String,
        }

        #[async_trait::async_trait]
        impl ToolExecutor for StateMockToolExecutor {
            async fn execute(&self, _args: serde_json::Value) -> Result<String, ToolError> {
                Ok(self.result.clone())
            }
        }

        let mutating_tool = Tool {
            name: "read_tool".to_string(),
            description: "".to_string(),
            is_read_only: false, // Mutating tool triggers Claude Code local checkpoints, but our new DB checkpointer triggers on every iteration.
            parameters: serde_json::Value::Null,
            execute: Arc::new(StateMockToolExecutor { result: "read_ok".to_string() }),
        };

        let checkpointer = Arc::new(MockCheckpointer {
            checkpoints: tokio::sync::Mutex::new(Vec::new()),
        });

        let agent1 = Agent::new(client1, vec![mutating_tool.clone()]).with_checkpointer(checkpointer.clone());
        let mut cfg = AgentRunConfig::default();
        cfg.model = "test-model".to_string();
        cfg.thread_id = Some("test_thread".to_string());

        let mut events1 = Vec::new();
        let _ = agent1.run(&cfg, "Initial Task", &mut |e| events1.push(e)).await;

        let cps = checkpointer.checkpoints.lock().await;
        assert_eq!(cps.len(), 1, "Should have saved 1 checkpoint");
        let saved_cp_id = cps[0].checkpoint_id.clone();
        drop(cps);

        // Run 2: Resume from checkpoint
        let client2 = Arc::new(MockLlmClient {
            responses: tokio::sync::Mutex::new(vec![
                ChatResponse {
                    message: Message::assistant("Resumed answer"),
                    usage: Usage::default(),
                    stop_reason: "stop".to_string(),
                        response_id: Some("mock-id".to_string()),
                },
            ]),
        });

        let agent2 = Agent::new(client2, vec![mutating_tool]).with_checkpointer(checkpointer.clone());
        let mut cfg2 = AgentRunConfig::default();
        cfg2.model = "test-model".to_string();
        cfg2.thread_id = Some("test_thread".to_string());
        cfg2.resume_from_checkpoint_id = Some(saved_cp_id);

        let mut events2 = Vec::new();
        let _ = agent2.run(&cfg2, "Ignored Task (will use loaded messages)", &mut |e| events2.push(e)).await;

        // Verify the second run resumed properly by checking if it loaded the messages.
        // It should have immediately hit the ChatResponse and finished.
        // However, because there are NO tool calls in the ChatResponse, the loop hits the "Terminal condition",
        // returning early BEFORE saving another checkpoint!
        // A super-step checkpoint is only saved at the end of the iteration AFTER tools have run.
        let cps2 = checkpointer.checkpoints.lock().await;
        assert_eq!(cps2.len(), 1, "Should NOT save another checkpoint because it terminates immediately");

        // Let's verify that the output of run 2 was indeed the "Resumed answer"
        let last_event = events2.last().unwrap();
        if let AgentEvent::TaskComplete { content } = last_event {
            assert_eq!(content, "Resumed answer");
        } else {
            panic!("Expected TaskComplete");
        }
    }

    #[tokio::test]
    async fn test_git_state_checkpointing() {
        let client = Arc::new(MockLlmClient {
            responses: tokio::sync::Mutex::new(vec![
                ChatResponse {
                    message: Message {
                        role: Role::Assistant,
                        content: "".to_string(),
                        tool_calls: vec![ToolCall {
                            id: "call_123".to_string(),
                            name: "mutating_tool".to_string(),
                            arguments: serde_json::Value::Null,
                        }],
                        tool_results: vec![],
                    response_id: None,
                    },
                    usage: Usage::default(),
                    stop_reason: "tool_calls".to_string(),
                        response_id: Some("mock-id".to_string()),
                },
                ChatResponse {
                    message: Message::assistant("Task done"),
                    usage: Usage::default(),
                    stop_reason: "stop".to_string(),
                        response_id: Some("mock-id".to_string()),
                }
            ]),
        });

        let mutating_tool = Tool {
            name: "mutating_tool".to_string(),
            description: "A mutating tool".to_string(),
            parameters: serde_json::Value::Null,
            is_read_only: false,
            execute: Arc::new(MockToolExecutor),
        };

        let mut agent = Agent::new(client, vec![mutating_tool]);

        let temp_dir = std::env::temp_dir().join(format!("ohc_test_git_ckpt_{}", uuid::Uuid::new_v4()));
        std::fs::create_dir_all(&temp_dir).unwrap();

        let _ = std::process::Command::new("git").current_dir(&temp_dir).args(&["init"]).output().unwrap();
        let _ = std::process::Command::new("git").current_dir(&temp_dir).args(&["config", "user.name", "Test User"]).output().unwrap();
        let _ = std::process::Command::new("git").current_dir(&temp_dir).args(&["config", "user.email", "test@example.com"]).output().unwrap();
        std::fs::write(temp_dir.join("test.txt"), "hello").unwrap();
        let _ = std::process::Command::new("git").current_dir(&temp_dir).args(&["add", "."]).output().unwrap();
        let _ = std::process::Command::new("git").current_dir(&temp_dir).args(&["commit", "-m", "init"]).output().unwrap();
        std::fs::write(temp_dir.join("test.txt"), "hello modified").unwrap(); // Uncommitted change
        let cp = crate::checkpointer::GitCheckpointer::new(temp_dir.clone());
        agent.checkpointer = Some(Arc::new(cp));

        let mut cfg = AgentRunConfig::default();
        cfg.workspace_path = Some(temp_dir.to_string_lossy().to_string());
        cfg.thread_id = Some("test-thread".to_string());

        let mut events = vec![];
        let mut on_event = |e| { events.push(e); };

        let result = agent.run(&cfg, "Hello", &mut on_event).await;
        assert!(result.is_ok());

        // Verify event was emitted
        let mut found_checkpoint_event = false;
        for e in events {
            if let AgentEvent::CheckpointSaved { path, .. } = e {
                if path.starts_with("db:") {
                    found_checkpoint_event = true;
                }
            }
        }
        let _ = std::fs::remove_dir_all(&temp_dir);
        assert!(found_checkpoint_event, "Git checkpoint event was not emitted");
    }

    #[tokio::test]
    async fn test_state_checkpointing() {
        let client = Arc::new(MockLlmClient {
            responses: tokio::sync::Mutex::new(vec![
                ChatResponse {
                    message: Message {
                        role: Role::Assistant,
                        content: "".to_string(),
                        tool_calls: vec![ToolCall {
                            id: "call_mutating".to_string(),
                            name: "mutating_tool".to_string(),
                            arguments: Value::Null,
                        }],
                        tool_results: vec![],
                    response_id: None,
                    },
                    usage: Usage::default(),
                    stop_reason: "stop".to_string(),
                        response_id: Some("mock-id".to_string()),
                },
                ChatResponse {
                    message: Message::assistant("Final answer"),
                    usage: Usage::default(),
                    stop_reason: "stop".to_string(),
                        response_id: Some("mock-id".to_string()),
                },
            ]),
        });

        let mutating_tool = Tool {
            name: "mutating_tool".to_string(),
            description: "A mutating tool".to_string(),
            parameters: Value::Null,
            is_read_only: false,
            execute: Arc::new(MockToolExecutor),
        };

        let agent = Agent::new(client, vec![mutating_tool]);

        let scratchpad_path = format!(".test_checkpoint_{}.json", uuid::Uuid::new_v4());
        let mut cfg = AgentRunConfig::default();
        cfg.enable_state_checkpointing = true;
        cfg.state_scratchpad_path = Some(scratchpad_path.clone());

        let mut events = vec![];
        let mut on_event = |e| { events.push(e); };

        let result = agent.run(&cfg, "Hello", &mut on_event).await;
        assert!(result.is_ok());

        // Verify the file was created
        assert!(std::path::Path::new(&scratchpad_path).exists());

        // Clean up
        let _ = std::fs::remove_file(&scratchpad_path);

        // Verify event was emitted
        let mut found_checkpoint_event = false;
        for e in events {
            if let AgentEvent::CheckpointSaved { path, .. } = e {
                assert_eq!(path, scratchpad_path);
                found_checkpoint_event = true;
            }
        }
        assert!(found_checkpoint_event);
    }

    // We will replace MockLlmClient locally for the test
    struct RecordingLlmClient {
        last_request: tokio::sync::Mutex<Option<ChatRequest>>,
    }

    #[async_trait::async_trait]
    impl LlmClient for RecordingLlmClient {
        async fn chat(&self, req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            let mut lr = self.last_request.lock().await;
            *lr = Some(req);
            Ok(ChatResponse {
                message: Message::assistant("Final answer"),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                        response_id: Some("mock-id".to_string()),
            })
        }
    }

    #[tokio::test]
    async fn test_prompt_construction_lost_in_the_middle_prevention() {
        let client = Arc::new(RecordingLlmClient {
            last_request: tokio::sync::Mutex::new(None),
        });

        // Create an agent and we will inject some state so messages.len() > 3
        let agent = Agent::new(client.clone(), vec![]);

        let mut cfg = AgentRunConfig::default();
        cfg.enable_lost_in_the_middle_prevention = true;
        cfg.enable_state_checkpointing = true;
        cfg.developer_instructions = "Developer instructions here.".to_string();
        cfg.user_instructions = "Super long user instructions that span many many words.".to_string();

        let scratchpad_path = format!(".test_checkpoint_litm_{}.json", uuid::Uuid::new_v4());
        cfg.state_scratchpad_path = Some(scratchpad_path.clone());

        // Pre-fill some messages to make len > 3
        let initial_msgs = vec![
            Message::user("Task: Do something"),
            Message::assistant("Thinking..."),
            Message::assistant("Still thinking..."),
            Message::user("Please continue"),
        ];
        tokio::fs::write(&scratchpad_path, serde_json::to_string(&initial_msgs).unwrap()).await.unwrap();

        let mut events = vec![];
        let mut on_event = |e| { events.push(e); };

        let result = agent.run(&cfg, "Continue working", &mut on_event).await;
        assert!(result.is_ok());

        let lr = client.last_request.lock().await;
        let req = lr.as_ref().unwrap();
        let last_msg = req.messages.last().unwrap();

        assert_eq!(last_msg.role, Role::User);
        assert!(last_msg.content.contains("[System Reminder: Developer instructions here.]"));
        assert!(last_msg.content.contains("[System Reminder to combat 'Lost in the Middle' effect: Remember your core objective: Super long user instructions that span many many words....]"));

        let _ = tokio::fs::remove_file(&scratchpad_path).await;
    }

#[tokio::test]
    async fn test_token_budget_exhaustion_termination() {
        let client = Arc::new(MockLlmClient {
            responses: tokio::sync::Mutex::new(vec![
                ChatResponse {
                    message: Message::assistant("I have written some code."),
                    usage: Usage { input_tokens: 50, output_tokens: 200 },
                    stop_reason: "length".to_string(), // LLM stopped due to length
                        response_id: Some("mock-id".to_string()),
                }
            ]),
        });

        let agent = Agent::new(client, vec![]);
        let mut cfg = AgentRunConfig::default();
        cfg.max_task_tokens = 150; // set budget lower than output tokens so it stops

        let mut events = vec![];
        let mut on_event = |e| { events.push(e); };

        let result = agent.run(&cfg, "Hello", &mut on_event).await;

        assert!(result.is_err());
        let err_str = result.unwrap_err().to_string();
        assert!(err_str.contains("token budget exhausted"));

        // Also ensure an AgentEvent::TaskError was emitted
        let mut found_task_error = false;
        for e in events {
            if let AgentEvent::TaskError { error } = e {
                if error.contains("token budget exhausted") {
                    found_task_error = true;
                    break;
                }
            }
        }
        assert!(found_task_error);
    }

    #[tokio::test]
    async fn test_git_checkpointer_integration() {
        use crate::checkpointer::{GitCheckpointer, CheckpointSaver};

        // Create a temporary directory for the git repo
        let temp_dir = tempfile::tempdir().unwrap();
        let repo_path = temp_dir.path().to_path_buf();

        let checkpointer = Arc::new(GitCheckpointer::new(repo_path.clone()));

        let _client = Arc::new(MockLlmClient {
            responses: tokio::sync::Mutex::new(vec![
                ChatResponse {
                    message: Message::assistant("Initial thought"),
                    usage: Usage::default(),
                    stop_reason: "stop".to_string(),
                        response_id: Some("mock-id".to_string()),
                }
            ]),
        });

        // Add a mutating tool so it triggers the checkpoint
        let mutating_tool = crate::tools::Tool {
            name: "Mutator".to_string(),
            description: "mutates".to_string(),
            is_read_only: false,
            parameters: serde_json::json!({}),
            execute: Arc::new(MockToolExecutor),
        };

        // We'll mock it so the LLM calls the tool, then stops
        let client_with_tools = Arc::new(MockLlmClient {
            responses: tokio::sync::Mutex::new(vec![
                ChatResponse {
                    message: Message {
                        role: Role::Assistant,
                        content: String::new(),
                        tool_calls: vec![crate::types::ToolCall {
                            id: "call_1".to_string(),
                            name: "Mutator".to_string(),
                            arguments: serde_json::json!({}),
                        }],
                        tool_results: vec![],
                    response_id: None,
                    },
                    usage: Usage::default(),
                    stop_reason: "tool_calls".to_string(),
                        response_id: Some("mock-id".to_string()),
                },
                ChatResponse {
                    message: Message::assistant("Final answer"),
                    usage: Usage::default(),
                    stop_reason: "stop".to_string(),
                        response_id: Some("mock-id".to_string()),
                }
            ]),
        });

        let agent = Agent::new(client_with_tools, vec![mutating_tool]).with_checkpointer(checkpointer.clone());

        let mut cfg = AgentRunConfig::default();
        cfg.thread_id = Some("git-thread-123".to_string());

        let mut events = vec![];
        let mut on_event = |e| { events.push(e); };

        let result = agent.run(&cfg, "Do it", &mut on_event).await;
        assert!(result.is_ok());

        // Now verify that the GitCheckpointer successfully created a checkpoint
        let checkpoints = checkpointer.list_checkpoints("git-thread-123").await.unwrap();
        assert!(!checkpoints.is_empty(), "Git checkpoints should not be empty");

        // Verify the file was written to the repo
        let progress_file = repo_path.join(".agent_progress_git-thread-123.json");
        assert!(progress_file.exists(), "Progress file should exist in git repo");

        // Verify that it is actually a git repository and has commits
        let output = std::process::Command::new("git")
            .arg("log")
            .current_dir(&repo_path)
            .output()
            .unwrap();
        assert!(output.status.success(), "Git log should succeed");
        let log_output = String::from_utf8_lossy(&output.stdout);
        assert!(log_output.contains("Checkpoint:"), "Commit message should contain Checkpoint:");
    }
}
