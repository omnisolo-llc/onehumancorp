
use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, Message, Role, ToolCall, ToolResult, ToolDefinition};
use crate::types::ToolError;
use crate::agent::AgentEvent;
use crate::agent::{Agent, AgentRunConfig, AgentProgress};
use std::sync::Arc;
use tracing::{info_span, Instrument};


pub enum TaoPhase {
    InitializeIteration,
    AssemblePrompt,
    CallLlm,
    ParseOutput(ChatResponse),
    ExecuteTools(Vec<ToolCall>),
    FormatResults(Vec<ToolResult>),
    TerminalCondition(String),
}

pub struct TaoOrchestrator {
    pub agent: Arc<Agent>,
    pub max_iterations: i32,
}

impl TaoOrchestrator {
    pub fn new(agent: Arc<Agent>, max_iterations: i32) -> Self {
        Self { agent, max_iterations }
    }

    pub async fn run_tao_loop<F>(
        &self,
        final_cfg: &AgentRunConfig,
        initial_message: &str,
        session_tools: &[crate::tools::Tool],
        on_event: &mut F,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>>
    where
        F: FnMut(AgentEvent) + Send + Sync,
    {
        let mut turn_count = 0;
        let mut current_phase = TaoPhase::InitializeIteration;
        let mut messages = vec![Message::user(initial_message)];
        let mut last_assistant_content = String::new();
        let mut global_turn_tokens = 0i32;
        let mut loop_req = ChatRequest {
            model: final_cfg.model.clone(),
            system: final_cfg.server_system_message.clone(),
            messages: vec![],
            tools: vec![],
            max_tokens: final_cfg.max_tokens,
            temperature: final_cfg.temperature,
        };

        loop {
            match current_phase {
                TaoPhase::InitializeIteration => {
                    if turn_count >= self.max_iterations {
                        current_phase = TaoPhase::TerminalCondition("Max iterations reached".to_string());
                        continue;
                    }
                    on_event(AgentEvent::IterationStarted { iteration: turn_count, message_count: messages.len() });
                    turn_count += 1;
                    current_phase = TaoPhase::AssemblePrompt;
                }
                TaoPhase::AssemblePrompt => {
                    let mut req_tools = Vec::new();
                    for t in session_tools {
                        req_tools.push(ToolDefinition {
                            name: t.name.clone(),
                            description: t.description.clone(),
                            parameters: t.parameters.clone(),
                        });
                    }

                    loop_req.messages = messages.clone();
                    loop_req.tools = req_tools;
                    current_phase = TaoPhase::CallLlm;
                }
                TaoPhase::CallLlm => {
                    let span = info_span!("tao_call_llm");
                    let resp = match self.agent.llm.chat(loop_req.clone()).instrument(span).await {
                        Ok(r) => r,
                        Err(e) => return Err(e),
                    };
                    current_phase = TaoPhase::ParseOutput(resp);
                }
                TaoPhase::ParseOutput(resp) => {
                    global_turn_tokens += resp.usage.output_tokens as i32;
                    if global_turn_tokens >= final_cfg.max_task_tokens {
                        current_phase = TaoPhase::TerminalCondition("Token budget exhausted".to_string());
                        continue;
                    }
                    if resp.stop_reason == "safety" {
                        current_phase = TaoPhase::TerminalCondition("Safety refusal".to_string());
                        continue;
                    }

                    last_assistant_content = resp.message.content.clone();
                    messages.push(resp.message.clone());
                    on_event(AgentEvent::TextChunk { content: last_assistant_content.clone() });

                    let tc = resp.message.tool_calls.clone();
                    if tc.is_empty() {
                        current_phase = TaoPhase::TerminalCondition("No tool calls".to_string());
                        continue;
                    }
                    current_phase = TaoPhase::ExecuteTools(tc);
                }
                TaoPhase::ExecuteTools(tc) => {
                    let mut t_results = vec![ToolResult { tool_call_id: String::new(), content: String::new(), error: String::new() }; tc.len()];
                    for (i, call) in tc.iter().enumerate() {
                        let res = self.agent.execute_tool(call, session_tools, &messages).await;
                        match res {
                            Ok(c) => t_results[i] = ToolResult { tool_call_id: call.id.clone(), content: c, error: String::new() },
                            Err(e) => t_results[i] = ToolResult { tool_call_id: call.id.clone(), content: String::new(), error: format!("{:?}", e) },
                        }
                    }
                    current_phase = TaoPhase::FormatResults(t_results);
                }
                TaoPhase::FormatResults(tr) => {
                    messages.push(Message {
                        role: Role::Tool,
                        content: String::new(),
                        tool_calls: vec![],
                        tool_results: tr,
                        response_id: None,
                        previous_response_id: None,
                    });
                    current_phase = TaoPhase::InitializeIteration;
                }
                TaoPhase::TerminalCondition(msg) => {
                    tracing::info!("TAO Loop terminated: {}", msg);
                    on_event(AgentEvent::TaskComplete { content: last_assistant_content.clone() });
                    return Ok(last_assistant_content);
                }
            }
        }
    }
}
