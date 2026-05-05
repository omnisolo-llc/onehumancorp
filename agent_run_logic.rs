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
                for m in &mut messages {
                    if m.role == Role::Tool {
                        for tr in &mut m.tool_results {
                            if tr.error.is_empty() && !tr.content.starts_with("[Observation Masked to save context.") {
                                let bytes = tr.content.len();
                                if bytes > 150 {
                                    tr.content = format!(
                                        "[Observation Masked to save context. Output was {} bytes. The tool call itself remains visible so you remember this action.]",
                                        bytes
                                    );
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
