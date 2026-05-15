use super::*;
use crate::budget::{check_token_budget, BudgetAction, BudgetTracker};
use std::sync::atomic::Ordering;
use opentelemetry::{global, KeyValue};

impl Agent {
    pub async fn run<F>(
        &self,
        cfg: &AgentRunConfig,
        initial_message: &str,
        on_event: &mut F,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>>
    where
        F: FnMut(AgentEvent) + Send + Sync,
    {
        let mut self_with_memory = self;
        let owned_agent;
        if let Some(ltm) = &cfg.long_term_memory {
            owned_agent = Agent {
                llm: self.llm.clone(),
                tools: self.tools.clone(),
                progress: self.progress.clone(),
                memory_store: Some(ltm.clone()),
                checkpointer: self.checkpointer.clone(),
                observation_store: self.observation_store.clone(),
            };
            self_with_memory = &owned_agent;
        }

        let session_tools = self_with_memory.tools.clone();

        let mut final_cfg = cfg.clone();

        // 4. User Instructions (cascading AGENTS.md files, capped at 32 KiB)
        if let Some(ref wp) = final_cfg.workspace_path {
            let start_dir = std::path::Path::new(wp);
            let cascading_md = load_cascading_agents_md(start_dir).await;
            if !cascading_md.is_empty() {
                if !final_cfg.user_instructions.is_empty() {
                    final_cfg.user_instructions =
                        format!("{}\n\n{}", cascading_md, final_cfg.user_instructions);
                } else {
                    final_cfg.user_instructions = cascading_md;
                }
            }
        }

        let mut end_idx = 32768;
        if final_cfg.user_instructions.len() > 32768 {
            while end_idx > 0 && !final_cfg.user_instructions.is_char_boundary(end_idx) {
                end_idx -= 1;
            }
            final_cfg.user_instructions.truncate(end_idx);
        }

        if final_cfg.enable_harness_thickness_optimization {
            let model_lower = final_cfg.model.to_lowercase();
            // Harness Thickness Mechanic: Delete harness planning steps as the LLM internalizes them.
            if model_lower.contains("gpt-4o")
                || model_lower.contains("claude-3-5-sonnet")
                || model_lower.contains("o1")
            {
                final_cfg.enable_llmcompiler_plan_and_execute = false;
                final_cfg.server_system_message = final_cfg
                    .server_system_message
                    .replace("You must think step by step and make a detailed plan.", "");
                final_cfg.server_system_message = final_cfg
                    .server_system_message
                    .replace("Make a plan before executing.", "");
            }
        }
        if final_cfg.enable_llmcompiler_plan_and_execute {
            return self
                .run_plan_and_execute(&final_cfg, initial_message, &session_tools, on_event)
                .await;
        }
        let mut session_tools = self.tools.clone();
        let active_tools =
            std::sync::Arc::new(tokio::sync::RwLock::new(std::collections::HashSet::new()));

        // Tool Scoping: *Vercel Metric:* Removed 80% of tools from v0 for better results.
        if final_cfg.enable_vercel_tool_scoping_metric && session_tools.len() > 5 {
            let keep_count = (session_tools.len() as f64 * 0.2).max(1.0) as usize;
            session_tools.truncate(keep_count);
        }

        if final_cfg.enable_lazy_tool_loading {
            let active_tools_clone = active_tools.clone();
            session_tools.push(crate::tools::lazy_load::lazy_load_tool(active_tools_clone));
            // Tool Scoping (Claude Lazy-loading): Achieves 95% context reduction via lazy-loading.
        }

        // Architectural Decision 1: Single-agent vs Multi-agent: Maximize single-agent first.
        // Mechanic: Split into multi-agent ONLY when overlapping tools exceed ~10.
        if cfg.enable_single_agent_maximization && session_tools.len() > 10 {
            let err_msg =
                "Task requires multi-agent split: >10 overlapping tools provided".to_string();
            on_event(AgentEvent::TaskError {
                error: err_msg.clone(),
            });
            return Err(Box::new(crate::types::ToolError::HandoffRequested(err_msg)));
        }

        // OpenAI Mechanic: Input Guardrails
        if let Some(guard_cfg) = &final_cfg.guardrails {
            if let Err(e) = crate::guardrails::check_input(initial_message, guard_cfg) {
                on_event(AgentEvent::TaskError { error: e.clone() });
                return Err(e.into());
            }
        }

        on_event(AgentEvent::RunStarted { iteration: 0 });

        let meter = global::meter("ohc_agent");
        let token_counter = meter.u64_counter("ohc_agent_token_usage_total").build();
        let cost_counter = meter.f64_counter("ohc_agent_cost_estimate_usd").build();

        let mut tool_error_counts: std::collections::HashMap<String, usize> =
            std::collections::HashMap::new();
        let mut malformed_retries = 0;
        let max_malformed_retries = 3;

        let mut messages: Vec<Message> = final_cfg.injected_context.clone().unwrap_or_default();
        let mut last_checkpoint_id: Option<String> = None;

        if final_cfg.enable_langgraph_mechanic {
            return self_with_memory
                .run_langgraph(
                    &final_cfg,
                    initial_message,
                    session_tools,
                    &mut messages,
                    on_event,
                )
                .await;
        }

        if let (Some(checkpointer), Some(thread_id)) = (&self.checkpointer, &final_cfg.thread_id) {
            if let Some(resume_id) = &final_cfg.resume_from_checkpoint_id {
                let cp = checkpointer
                    .get_checkpoint(thread_id, resume_id)
                    .await
                    .map_err(|e| {
                        format!("Failed to fetch requested checkpoint {}: {}", resume_id, e)
                    })?
                    .ok_or_else(|| format!("Requested checkpoint {} not found", resume_id))?;

                messages = serde_json::from_value::<Vec<Message>>(cp.data.clone())
                    .map_err(|e| format!("Failed to deserialize requested checkpoint: {}", e))?;
                last_checkpoint_id = Some(cp.checkpoint_id.clone());
                checkpointer
                    .restore_checkpoint(resume_id)
                    .await
                    .map_err(|e| format!("Failed to restore workspace: {}", e))?;
            } else {
                if let Ok(checkpoints) = checkpointer.list_checkpoints(thread_id).await {
                    if let Some(cp) = checkpoints.first() {
                        if let Ok(saved_msgs) =
                            serde_json::from_value::<Vec<Message>>(cp.data.clone())
                        {
                            messages = saved_msgs;
                            last_checkpoint_id = Some(cp.checkpoint_id.clone());
                        }
                    }
                }
            }
        }

        let generated_uuid_path = format!(".agent_checkpoint_{}.json", uuid::Uuid::new_v4());
        let scratchpad_path = final_cfg
            .state_scratchpad_path
            .clone()
            .unwrap_or(generated_uuid_path);

        if messages.is_empty() && final_cfg.enable_state_checkpointing {
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
        let mut last_response_id: Option<String> = None;
        let mut last_assistant_content = String::new();

        let max_iterations = if final_cfg.max_iterations <= 0 {
            100
        } else {
            final_cfg.max_iterations
        };

        let mut combined_system = build_hierarchical_system_prompt(&final_cfg, &session_tools);

        // Long-Term Memory Retrieval
        let mut checkpoint_history: Vec<String> = Vec::new();
        if let Some(id) = &last_checkpoint_id {
            checkpoint_history.push(id.clone());
        }
        let mut rewind_attempts_remaining = final_cfg.max_rewind_attempts;

        if let Some(store) = &self_with_memory.memory_store {
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

        let mut turn_count = 0;
        while turn_count < max_iterations {
            let iteration = turn_count;
            turn_count += 1;

            on_event(AgentEvent::IterationStarted {
                iteration,
                message_count: messages.len(),
            });

            let mut final_messages = messages.clone();

            // Context Window Strategy: Prioritize reasoning traces over raw tool outputs (ACON Research)
            if final_cfg.enable_acon_context_strategy {
                let msg_count = final_messages.len();
                if msg_count > 3 {
                    // We preserve the last 2 messages (usually assistant + tool results)
                    // For older Tool role messages, we strip the raw tool output but keep reasoning
                    let threshold = msg_count - 2;
                    for i in 0..threshold {
                        if final_messages[i].role == Role::Tool {
                            for tr in &mut final_messages[i].tool_results {
                                if tr.error.is_empty()
                                    && !tr.content.starts_with("[ACON:")
                                    && !tr.content.is_empty()
                                {
                                    tr.content = "[ACON: Tool output omitted to prioritize reasoning traces.]".to_string();
                                }
                            }
                        }
                    }
                }
            }

            // Prompt Construction Mechanic: "Lost in the Middle" Prevention
            // High-signal context at the very beginning and very end.
            if final_cfg.enable_lost_in_the_middle_prevention {
                let mut reminder_text = String::new();
                if !final_cfg.developer_instructions.is_empty() {
                    reminder_text.push_str(&format!(
                        "[System Reminder: {}]\n\n",
                        final_cfg.developer_instructions
                    ));
                }
                if !final_cfg.user_instructions.is_empty() && final_messages.len() > 3 {
                    // Truncate user instructions if it's too long, just to remind the core objective
                    let mut end_idx = 1000;
                    if final_cfg.user_instructions.len() > 1000 {
                        while end_idx > 0 && !final_cfg.user_instructions.is_char_boundary(end_idx)
                        {
                            end_idx -= 1;
                        }
                    } else {
                        end_idx = final_cfg.user_instructions.len();
                    }
                    let summary = &final_cfg.user_instructions[..end_idx];
                    reminder_text.push_str(&format!("[System Reminder to combat 'Lost in the Middle' effect: Remember your core objective: {}...]", summary));
                }

                if !reminder_text.is_empty() {
                    final_messages.push(Message::user(reminder_text.trim()));
                }
            } else if !final_cfg.developer_instructions.is_empty() {
                final_messages.push(Message::user(format!(
                    "[System Reminder: {}]",
                    final_cfg.developer_instructions
                )));
            }

            let mut req_tools = Vec::new();
            for t in &session_tools {
                if !final_cfg.enable_lazy_tool_loading
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
                model: final_cfg.model.clone(),
                system: combined_system.clone(),
                messages: final_messages,
                tools: req_tools,
                max_tokens: final_cfg.max_tokens,
                temperature: final_cfg.temperature,
            };

            // Intelligent Context Truncation to save tokens
            let req = ohc_builtin_agent_llm::truncate_chat_request(req, 10000); // Limit history to ~10k words

            let resp = match self.llm.chat(req).await {
                Ok(r) => r,
                Err(e) => {
                    let err = format!("LLM error: {}", e);
                    if err.to_lowercase().contains("timeout")
                        || err.to_lowercase().contains("rate limit")
                        || err.to_lowercase().contains("unavailable")
                        || err.to_lowercase().contains("resource exhausted")
                    {
                        let err_msg = "LLM API is currently unavailable or rate-limited. Agent transitioning to PAUSED state. Please try again later.".to_string();
                        on_event(AgentEvent::TaskError {
                            error: err_msg.clone(),
                        });
                        return Err(err_msg.into());
                    } else if err.to_lowercase().contains("malformed")
                        || err.to_lowercase().contains("invalid json")
                    {
                        malformed_retries += 1;
                        if malformed_retries >= max_malformed_retries {
                            let err_msg = format!("Terminal condition reached: Malformed LLM response retries exhausted ({}).", max_malformed_retries);
                            on_event(AgentEvent::TaskError {
                                error: err_msg.clone(),
                            });
                            return Err(err_msg.into());
                        }
                        let err_msg = format!("Malformed LLM response: {}. Agent retrying...", e);
                        on_event(AgentEvent::TaskError {
                            error: err_msg.clone(),
                        });
                        messages.push(Message::user("Your previous response was malformed or invalid JSON. Please ensure your tool calls are properly formatted."));
                        continue;
                    } else {
                        on_event(AgentEvent::TaskError { error: err.clone() });
                        return Err(err.into());
                    }
                }
            };

            if let Some(rid) = &resp.response_id {
                last_response_id = Some(rid.clone());
            }

            let turn_input_tokens = resp.usage.input_tokens;
            let output_tokens = resp.usage.output_tokens;
            let total_tokens = (turn_input_tokens + output_tokens) as i64;
            self.progress.add_tokens(total_tokens);
            global_turn_tokens += output_tokens;

            // Telemetry: Record token usage
            let model_label = KeyValue::new("model", final_cfg.model.clone());
            let agent_label = KeyValue::new("agent_id", final_cfg.agent_id.clone());
            token_counter.add(
                turn_input_tokens as u64,
                &[
                    model_label.clone(),
                    agent_label.clone(),
                    KeyValue::new("type", "input"),
                ],
            );
            token_counter.add(
                output_tokens as u64,
                &[
                    model_label.clone(),
                    agent_label.clone(),
                    KeyValue::new("type", "output"),
                ],
            );

            // Enforce Server-side token budget strictly every turn
            if global_turn_tokens >= final_cfg.max_task_tokens {
                let msg = "I've reached my token budget for this task. Please upgrade your plan to unlock longer interactions!".to_string();
                on_event(AgentEvent::TextChunk {
                    content: msg.clone(),
                });
                on_event(AgentEvent::TaskComplete {
                    content: msg.clone(),
                });
                return Ok(msg);
            }

            // Unified Cost Calculation Mechanic
            // Note: We use the local pricing calculator logic to avoid a direct
            // dependency on server_lib which would cause a circular dependency.
            let input_cost_per_m = match final_cfg.model.to_lowercase().as_str() {
                m if m.contains("gpt-4o") && !m.contains("mini") => 5.0,
                m if m.contains("gpt-4-turbo") => 10.0,
                m if m.contains("gpt-3.5") || m.contains("gpt-4o-mini") => 0.15,
                m if m.contains("gemini-1.5-pro") => 3.5,
                m if m.contains("gemini-1.5-flash") => 0.075,
                m if m.contains("claude-3-5-sonnet") => 3.0,
                m if m.contains("claude-3-haiku") => 0.25,
                _ => 3.0,
            };
            let output_cost_per_m = match final_cfg.model.to_lowercase().as_str() {
                m if m.contains("gpt-4o") && !m.contains("mini") => 15.0,
                m if m.contains("gpt-4-turbo") => 30.0,
                m if m.contains("gpt-3.5") || m.contains("gpt-4o-mini") => 0.60,
                m if m.contains("gemini-1.5-pro") => 10.5,
                m if m.contains("gemini-1.5-flash") => 0.30,
                m if m.contains("claude-3-5-sonnet") => 15.0,
                m if m.contains("claude-3-haiku") => 1.25,
                _ => 15.0,
            };

            let turn_cost = (turn_input_tokens as f64 * input_cost_per_m / 1_000_000.0)
                + (output_tokens as f64 * output_cost_per_m / 1_000_000.0);

            if turn_cost > 0.0 {
                cost_counter.add(turn_cost, &[model_label, agent_label]);
            }

            let stop_reason = resp.stop_reason.as_str();

            // Layered Termination Condition: Safety Refusal
            if stop_reason == "content_filter" || stop_reason == "safety" {
                let err_msg = "Terminal condition reached: Safety refusal. The model halted execution due to content safety policy.".to_string();
                on_event(AgentEvent::TaskError {
                    error: err_msg.clone(),
                });
                return Err(err_msg.into());
            }

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
                    final_cfg.max_task_tokens,
                    global_turn_tokens,
                );

                if decision.action == BudgetAction::Stop {
                    let msg = "I've reached my token budget for this task. Please upgrade your plan to unlock longer interactions!".to_string();
                    on_event(AgentEvent::TextChunk {
                        content: msg.clone(),
                    });
                    on_event(AgentEvent::TaskComplete {
                        content: msg.clone(),
                    });
                    return Ok(msg);
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
                tool_call_counter.add(
                    1,
                    &[
                        KeyValue::new("agent_id", final_cfg.agent_id.clone()),
                        KeyValue::new("tool_name", tc.name.clone()),
                    ],
                );
            }

            // Terminal condition: no tool calls.
            if tool_calls.is_empty() {
                // Computational/Guides (feedforward verification)
                if final_cfg.enable_computational_guides
                    && !final_cfg.computational_guide_command.is_empty()
                {
                    let wd = final_cfg
                        .workspace_path
                        .clone()
                        .unwrap_or_else(|| ".".to_string());
                    let mut cmd = std::process::Command::new("bash");
                    cmd.arg("-c")
                        .arg(&final_cfg.computational_guide_command)
                        .current_dir(wd);

                    match cmd.output() {
                        Ok(output) => {
                            if !output.status.success() {
                                let stdout = String::from_utf8_lossy(&output.stdout);
                                let stderr = String::from_utf8_lossy(&output.stderr);
                                let err_msg = format!(
                                    "Computational guide verification failed (command: {}).\nStdout: {}\nStderr: {}\nPlease correct your work and use tools to fix the issue before providing the final answer.",
                                    final_cfg.computational_guide_command, stdout, stderr
                                );
                                messages.push(Message::user(err_msg));
                                continue;
                            }
                        }
                        Err(e) => {
                            let err_msg = format!(
                                "Failed to execute computational guide command '{}': {}",
                                final_cfg.computational_guide_command, e
                            );
                            messages.push(Message::user(err_msg));
                            continue;
                        }
                    }
                }

                // Visual Verification (screenshots via Playwright or Slint)
                if final_cfg.enable_visual_verification
                    && !final_cfg.visual_verification_command.is_empty()
                {
                    let wd = final_cfg
                        .workspace_path
                        .clone()
                        .unwrap_or_else(|| ".".to_string());
                    let mut cmd = std::process::Command::new("bash");
                    cmd.arg("-c")
                        .arg(&final_cfg.visual_verification_command)
                        .current_dir(wd);

                    match cmd.output() {
                        Ok(output) => {
                            if !output.status.success() {
                                let stdout = String::from_utf8_lossy(&output.stdout);
                                let stderr = String::from_utf8_lossy(&output.stderr);
                                let err_msg = format!(
                                    "Visual verification failed (command: {}).\nStdout: {}\nStderr: {}\nPlease correct your work based on the visual feedback and use tools to fix the issue.",
                                    final_cfg.visual_verification_command, stdout, stderr
                                );
                                messages.push(Message::user(err_msg));
                                continue;
                            } else {
                                let stdout = String::from_utf8_lossy(&output.stdout);
                                if stdout.contains("REJECT") {
                                    let err_msg = format!("Visual verification rejected the output. Reason: {}\nPlease correct your work and use tools to fix the issue.", stdout.trim());
                                    messages.push(Message::user(err_msg));
                                    continue;
                                }
                            }
                        }
                        Err(e) => {
                            let err_msg = format!(
                                "Failed to execute visual verification command '{}': {}",
                                final_cfg.visual_verification_command, e
                            );
                            messages.push(Message::user(err_msg));
                            continue;
                        }
                    }
                }

                // Inferential/Sensors (LLM-as-judge subagent)
                if final_cfg.enable_llm_judge {
                    let judge_req = ChatRequest {
                        model: final_cfg.model.clone(),
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
                                let reason = judge_text
                                    .strip_prefix("REJECT:")
                                    .unwrap_or(judge_text)
                                    .trim();
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
                if let Some(guard_cfg) = &final_cfg.guardrails {
                    if let Err(e) =
                        crate::guardrails::check_output(&last_assistant_content, guard_cfg)
                    {
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
                let is_read_only = self
                    .tools
                    .iter()
                    .find(|t| t.name == tc.name)
                    .map(|t| t.is_read_only)
                    .unwrap_or(false);
                if is_read_only {
                    read_only_calls.push(tc.clone());
                } else {
                    mutating_calls.push(tc.clone());
                }
            }

            // We need a helper to execute a single tool call with retries and guardrails.
            // We use a macro or inline logic to avoid borrowing issues with `on_event`.
            let mut tool_results: Vec<ToolResult> = vec![
                ToolResult {
                    tool_call_id: String::new(),
                    content: String::new(),
                    error: String::new()
                };
                tool_calls.len()
            ];

            // Note: Since `on_event` is `&mut F`, we can't easily share it across concurrent tasks.
            // For now, we will collect events and results from the concurrent execution, then emit them sequentially.
            // We will execute the read-only calls concurrently using `futures::future::join_all`.

            // Output Parsing mechanic: Schema-Constrained Responses
            // Intercept special output formatting tool natively
            if let Some(tc) = mutating_calls
                .iter()
                .chain(read_only_calls.iter())
                .find(|t| t.name == "return_structured_output")
            {
                on_event(AgentEvent::ToolCall {
                    name: tc.name.clone(),
                    args_json: tc.arguments.to_string(),
                    result: "Returning structured output".to_string(),
                    iteration,
                });

                // When the model calls the structured output tool,
                // we terminate the orchestrator immediately with the raw JSON arguments as the task completion.
                return Ok(tc.arguments.to_string());
            }

            let mut read_only_futures = Vec::new();
            for tc in &read_only_calls {
                // OpenAI Mechanic: Tool Guardrails
                if let Some(guard_cfg) = &final_cfg.guardrails {
                    if let Err(e) = crate::guardrails::check_tool(tc, guard_cfg) {
                        on_event(AgentEvent::TaskError { error: e.clone() });
                        return Err(e.into()); // Tripwire: halt the loop immediately
                    }
                }
                let gating_res = Self::check_tool_gating(tc, true, &final_cfg);
                let tc_clone = tc.clone();
                let session_tools_clone = session_tools.clone();
                let messages_clone = messages.clone();
                let cfg_max_retries = final_cfg.max_retries;
                read_only_futures.push(async move {
                    if let Err(e) = gating_res {
                        return (tc_clone, Err(e));
                    }
                    let mut retry_count = 0;
                    let max_retries = cfg_max_retries; // Error Handling (Compounding Error Prevention): Stripe limits retries to exactly 2.
                    loop {
                        match self
                            .execute_tool(&tc_clone, &session_tools_clone, &messages_clone)
                            .await
                        {
                            Ok(r) => {
                                return (tc_clone, Ok(r));
                            }
                            Err(ToolError::Transient(msg)) => {
                                if retry_count < max_retries {
                                    retry_count += 1;
                                    let backoff =
                                        std::time::Duration::from_millis(500 * (1 << retry_count));
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
                        if *count > final_cfg.max_retries {
                            if final_cfg.enable_time_travel_rewind
                                && rewind_attempts_remaining > 0
                                && checkpoint_history.len() > 1
                            {
                                rewind_attempts_remaining -= 1;
                                let _ = checkpoint_history.pop();
                                if let Some(prev_id) = checkpoint_history.last().cloned() {
                                    let mut restored_msgs = None;
                                    if let Some(checkpointer) = &self.checkpointer {
                                        if let Ok(Some(cp)) = checkpointer
                                            .get_checkpoint(
                                                final_cfg.thread_id.as_ref().unwrap(),
                                                &prev_id,
                                            )
                                            .await
                                        {
                                            if let Ok(msgs) =
                                                serde_json::from_value::<Vec<Message>>(cp.data)
                                            {
                                                let _ =
                                                    checkpointer.restore_checkpoint(&prev_id).await;
                                                restored_msgs = Some(msgs);
                                            }
                                        }
                                    }

                                    // State Management: OpenAI uses lightweight previous_response_id chaining.
                                    // Fallback to lightweight chaining if checkpointer is absent or fails.
                                    if restored_msgs.is_none() {
                                        let mut new_messages = Vec::new();
                                        let mut found = false;
                                        for m in messages.iter() {
                                            new_messages.push(m.clone());
                                            if let Some(rid) = &m.response_id {
                                                if rid == &prev_id {
                                                    found = true;
                                                    break;
                                                }
                                            }
                                        }
                                        if found {
                                            restored_msgs = Some(new_messages);
                                        } else if !new_messages.is_empty() {
                                            new_messages.truncate(1);
                                            restored_msgs = Some(new_messages);
                                        }
                                    }

                                    if let Some(msgs) = restored_msgs {
                                        messages = msgs;
                                        messages.push(Message::system(format!(
                                            "TIME-TRAVEL REWIND: Tool '{}' failed consecutively beyond max_retries limit. I have rewound your state to checkpoint '{}'. Please try a different approach to solve the task.",
                                            tc.name, prev_id
                                        )));
                                        on_event(AgentEvent::RewindOccurred {
                                            iteration,
                                            checkpoint_id: prev_id,
                                            reason: format!("Tool '{}' failed 3 times", tc.name),
                                        });
                                        tool_error_counts.remove(&tc.name);
                                        continue;
                                    }
                                }
                            }
                            let fatal_msg = format!("Tool '{}' failed consecutively beyond max_retries limit with recoverable errors. Escalating to Fatal to prevent compounding error loops. Last error: {}", tc.name, msg);
                            on_event(AgentEvent::TaskError {
                                error: fatal_msg.clone(),
                            });
                            return Err(fatal_msg.into());
                        }

                        // Error Handling (Compounding Error Prevention): LLM-recoverable (return the raw error as a ToolMessage directly to the model so it can self-correct)
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
                        let err = format!("USER_FIXABLE: {}", msg);
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
                        on_event(AgentEvent::Handoff {
                            target_agent: target.clone(),
                        });
                        return Ok(format!("Handoff requested to {}", target));
                    }
                }
            }

            // Execute mutating calls sequentially to prevent race conditions
            for tc in &mutating_calls {
                // OpenAI Mechanic: Tool Guardrails
                if let Some(guard_cfg) = &final_cfg.guardrails {
                    if let Err(e) = crate::guardrails::check_tool(&tc, guard_cfg) {
                        on_event(AgentEvent::TaskError { error: e.clone() });
                        return Err(e.into()); // Tripwire: halt the loop immediately
                    }
                }

                // Anthropic Mechanic: 3-Stage Tool Gating
                if let Err(e) = Self::check_tool_gating(&tc, false, &final_cfg) {
                    match e {
                        ToolError::UserFixable(msg) => {
                            let err = format!("USER_FIXABLE: {}", msg);
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
                            on_event(AgentEvent::Handoff {
                                target_agent: target.clone(),
                            });
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
                let max_retries = final_cfg.max_retries; // Error Handling (Compounding Error Prevention): Stripe limits retries to exactly 2.
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
                                let backoff =
                                    std::time::Duration::from_millis(500 * (1 << retry_count));
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
                            if *count > final_cfg.max_retries {
                                if final_cfg.enable_time_travel_rewind
                                    && rewind_attempts_remaining > 0
                                    && checkpoint_history.len() > 1
                                {
                                    rewind_attempts_remaining -= 1;
                                    let _ = checkpoint_history.pop();
                                    if let Some(prev_id) = checkpoint_history.last().cloned() {
                                        let mut restored_msgs = None;
                                        if let Some(checkpointer) = &self.checkpointer {
                                            if let Ok(Some(cp)) = checkpointer
                                                .get_checkpoint(
                                                    final_cfg.thread_id.as_ref().unwrap(),
                                                    &prev_id,
                                                )
                                                .await
                                            {
                                                if let Ok(msgs) =
                                                    serde_json::from_value::<Vec<Message>>(cp.data)
                                                {
                                                    let _ = checkpointer
                                                        .restore_checkpoint(&prev_id)
                                                        .await;
                                                    restored_msgs = Some(msgs);
                                                }
                                            }
                                        }

                                        // State Management: OpenAI uses lightweight previous_response_id chaining.
                                        // Fallback to lightweight chaining if checkpointer is absent or fails.
                                        if restored_msgs.is_none() {
                                            let mut new_messages = Vec::new();
                                            let mut found = false;
                                            for m in messages.iter() {
                                                new_messages.push(m.clone());
                                                if let Some(rid) = &m.response_id {
                                                    if rid == &prev_id {
                                                        found = true;
                                                        break;
                                                    }
                                                }
                                            }
                                            if found {
                                                restored_msgs = Some(new_messages);
                                            } else if !new_messages.is_empty() {
                                                new_messages.truncate(1);
                                                restored_msgs = Some(new_messages);
                                            }
                                        }

                                        if let Some(msgs) = restored_msgs {
                                            messages = msgs;
                                            messages.push(Message::system(format!(
                                                "TIME-TRAVEL REWIND: Tool '{}' failed consecutively beyond max_retries limit. I have rewound your state to checkpoint '{}'. Please try a different approach to solve the task.",
                                                tc.name, prev_id
                                            )));
                                            on_event(AgentEvent::RewindOccurred {
                                                iteration,
                                                checkpoint_id: prev_id,
                                                reason: format!(
                                                    "Tool '{}' failed 3 times",
                                                    tc.name
                                                ),
                                            });
                                            tool_error_counts.remove(&tc.name);
                                            continue;
                                        }
                                    }
                                }
                                let fatal_msg = format!("Tool '{}' failed consecutively beyond max_retries limit with recoverable errors. Escalating to Fatal to prevent compounding error loops. Last error: {}", tc.name, msg);
                                on_event(AgentEvent::TaskError {
                                    error: fatal_msg.clone(),
                                });
                                return Err(fatal_msg.into());
                            }

                            // Error Handling (Compounding Error Prevention): LLM-recoverable (return the raw error as a ToolMessage directly to the model so it can self-correct)
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
                            let err = format!("USER_FIXABLE: {}", msg);
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
                            on_event(AgentEvent::Handoff {
                                target_agent: target.clone(),
                            });
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

            if final_cfg.enable_observation_masking {
                // JetBrains Observation Masking: Hide the raw output of old tools from the prompt,
                // but keep the `tool_calls` themselves visible so the model remembers what it did.
                // Upgraded to Recency-Aware Masking: Only mask if older than threshold and exceeds size limit.
                let msg_count = messages.len();
                for i in 0..msg_count {
                    if messages[i].role == Role::Tool {
                        let age = msg_count - i;
                        if age > final_cfg.observation_masking_threshold {
                            for tr in &mut messages[i].tool_results {
                                if tr.error.is_empty()
                                    && !tr.content.starts_with("[Observation Masked")
                                {
                                    let bytes = tr.content.len();
                                    if bytes > final_cfg.observation_masking_size_limit {
                                        let preview_chars = 100;
                                        let char_count = tr.content.chars().count();
                                        if char_count > preview_chars * 2 {
                                            let start_preview: String =
                                                tr.content.chars().take(preview_chars).collect();
                                            let end_preview: String = tr
                                                .content
                                                .chars()
                                                .skip(char_count - preview_chars)
                                                .collect();
                                            tr.content = format!(
                                                "[Observation Masked to save context. Output was {} bytes. Preview: {}...{} The tool call itself remains visible. Use 'RecallObservation' with ID '{}' if you need the full output again.]",
                                                bytes, start_preview, end_preview, tr.tool_call_id
                                            );
                                        } else {
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
            }

            // Append tool results as a user turn.
            messages.push(Message {
                role: Role::Tool,
                content: String::new(),
                tool_calls: vec![],
                tool_results,
                response_id: None,
                previous_response_id: last_response_id.clone(),
            });

            // State Management Checkpointing Mechanic
            // 1. Configured Checkpointer (Database or Git)
            if let (Some(checkpointer), Some(thread_id)) =
                (&self.checkpointer, &final_cfg.thread_id)
            {
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
                    checkpoint_history.push(checkpoint_id.clone());
                    on_event(AgentEvent::CheckpointSaved {
                        iteration,
                        path: format!("db:{}", checkpoint_id),
                    });
                }
            }

            // 2. Local File Scratchpad (Claude Code)
            if final_cfg.enable_state_checkpointing && !mutating_calls.is_empty() {
                if let Ok(json_state) = serde_json::to_string_pretty(&messages) {
                    if tokio::fs::write(&scratchpad_path, json_state).await.is_ok() {
                        on_event(AgentEvent::CheckpointSaved {
                            iteration,
                            path: scratchpad_path.clone(),
                        });
                    }
                }
            }

            // 3. Git Commit Checkpointing (Claude Code Mechanic)
            if cfg.enable_git_checkpointing && !mutating_calls.is_empty() {
                let wd = cfg
                    .workspace_path
                    .clone()
                    .unwrap_or_else(|| ".".to_string());

                // 1. Progress File (Claude Code structured scratchpad)
                let thread_id_val = final_cfg
                    .thread_id
                    .clone()
                    .unwrap_or_else(|| "default".to_string());
                let progress_file_path = std::path::Path::new(&wd)
                    .join(format!(".agent_progress_{}.json", thread_id_val));

                let checkpoint_id = uuid::Uuid::new_v4().to_string();
                let cp = crate::checkpointer::Checkpoint {
                    thread_id: thread_id_val,
                    checkpoint_id: checkpoint_id.clone(),
                    parent_id: last_checkpoint_id.clone(),
                    data: serde_json::to_value(&messages).unwrap_or(serde_json::Value::Null),
                    metadata: serde_json::json!({
                        "iteration": iteration,
                        "agent_id": final_cfg.agent_id,
                    }),
                    created_at: chrono::Utc::now(),
                };

                if let Ok(json_data) = serde_json::to_string_pretty(&cp) {
                    let _ = std::fs::write(&progress_file_path, json_data);
                }

                // 2. Git commit (Claude Code)
                let commit_msg = format!("Checkpoint: {}", checkpoint_id);
                let _ = std::process::Command::new("git")
                    .arg("add")
                    .arg(".")
                    .current_dir(&wd)
                    .output();
                let _ = std::process::Command::new("git")
                    .arg("commit")
                    .arg("--allow-empty")
                    .arg("-m")
                    .arg(&commit_msg)
                    .current_dir(&wd)
                    .output();
                let _ = std::process::Command::new("git")
                    .arg("tag")
                    .arg("-f")
                    .arg(&checkpoint_id)
                    .current_dir(&wd)
                    .output();

                last_checkpoint_id = Some(checkpoint_id.clone());
                checkpoint_history.push(checkpoint_id.clone());

                on_event(AgentEvent::CheckpointSaved {
                    iteration,
                    path: format!("git:{}", checkpoint_id),
                });
            }

            // Cross-Department Memory Consolidation: Auto-store task result if successful
            if iteration == max_iterations - 1 || tool_calls.is_empty() {
                // This is the last iteration or no more tool calls (terminal)
                // We'll store the final thought in long-term memory if configured
                if !last_assistant_content.is_empty() {
                    if let Some(store) = &self_with_memory.memory_store {
                        let content_to_store = last_assistant_content.clone();
                        let store_clone = store.clone();
                        tokio::spawn(async move {
                            if let Err(e) = store_clone
                                .store(&content_to_store, vec!["AUTO_CONSOLIDATED".to_string()])
                                .await
                            {
                                tracing::error!("Failed to auto-consolidate memory: {}", e);
                            } else {
                                tracing::debug!("Successfully auto-consolidated memory.");
                            }
                        });
                    }
                }
            }

            // Context Compaction Mechanic
            // Use the input_tokens from the last request to determine the current context window size.

            if final_cfg.enable_context_compaction
                && turn_input_tokens > final_cfg.compaction_threshold_tokens
            {
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
                                    middle_text.push_str(&format!(
                                        "  {} ({})\n",
                                        tc.name,
                                        tc.arguments.to_string()
                                    ));
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
                                    middle_text.push_str(&format!(
                                        "  {} (error: {})\n",
                                        preview, tr.error
                                    ));
                                }
                            }
                            middle_text.push_str("---\n");
                        }

                        let summary_req = ChatRequest {
                            model: final_cfg.model.clone(),
                            system: "You are an expert context compactor for an AI agent. Summarize the following middle portion of an agent conversation. Preserve all architectural decisions, unresolved bugs, and the exact state of progress. Discard redundant or raw tool outputs. Be concise.".to_string(),
                            messages: vec![Message::user(format!("Compact this conversation:\n{}", middle_text))],
                            tools: vec![],
                            max_tokens: 2000,
                            temperature: 0.0,
                        };

                        match self.llm.chat(summary_req).await {
                            Ok(summary_resp) => {
                                let summary = summary_resp.message.content;
                                compact_messages.push(Message::user(format!(
                                    "[Context Compacted by Harness]:\n{}",
                                    summary
                                )));
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
        let err_msg = format!(
            "Terminal condition reached: max turn limit exceeded ({} iterations).",
            max_iterations
        );
        on_event(AgentEvent::TaskError {
            error: err_msg.clone(),
        });
        return Err(err_msg.into());
    }

}
