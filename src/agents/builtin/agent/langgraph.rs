use super::*;
use crate::budget::{check_token_budget, BudgetAction};
use std::sync::atomic::Ordering;
use opentelemetry::{global, KeyValue};

impl Agent {
    pub async fn run_langgraph<F>(
        &self,
        cfg: &AgentRunConfig,
        initial_message: &str,
        session_tools: Vec<crate::tools::Tool>,
        initial_messages: &mut Vec<Message>,
        on_event: &mut F,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>>
    where
        F: FnMut(AgentEvent) + Send + Sync,
    {
        // Architectural Decision 1: Single-agent vs Multi-agent: Maximize single-agent first.
        // Mechanic: Split into multi-agent ONLY when overlapping tools exceed ~10.
        if cfg.enable_single_agent_maximization && session_tools.len() > 10 {
            let err_msg =
                "Task requires multi-agent split: >10 overlapping tools provided".to_string();

            // Workaround to call the generic closure since on_event is a generic F.
            // Wait, we can just return the error directly.
            return Err(Box::new(crate::types::ToolError::HandoffRequested(err_msg)));
        }

        // Add initial message if needed
        if !initial_message.is_empty() {
            initial_messages.push(Message::user(initial_message));
        }

        let mut graph = crate::langgraph::StateGraph::new(std::sync::Arc::new(
            crate::langgraph::DefaultReducer,
        ));

        let llm = self.llm.clone();
        let tools_def: Vec<_> = session_tools
            .iter()
            .map(|t| crate::types::ToolDefinition {
                name: t.name.clone(),
                description: t.description.clone(),
                parameters: t.parameters.clone(),
            })
            .collect();

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
                previous_response_id: None,
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
                        let total_tokens_this_turn = resp.usage.input_tokens + resp.usage.output_tokens;
                        let mut current_total = state.get("total_tokens").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
                        current_total += total_tokens_this_turn;

                        let mut final_content = resp.message.content.clone();
                        let mut has_tool_calls = !resp.message.tool_calls.is_empty();

                        if llm_cfg_c.max_task_tokens > 0 && current_total > llm_cfg_c.max_task_tokens {
                            final_content = "I've reached my token budget for this task. Please upgrade your plan to unlock longer interactions!".to_string();
                            has_tool_calls = false; // Prevent further tool calls
                        }

                        let final_tool_calls = if has_tool_calls {
                            resp.message.tool_calls.iter().map(|tc| serde_json::json!({
                                "id": tc.id,
                                "name": tc.name,
                                "arguments": tc.arguments,
                            })).collect::<Vec<_>>()
                        } else {
                            vec![]
                        };

                        let mut update = serde_json::json!({
                            "has_tool_calls": has_tool_calls,
                            "total_tokens": current_total,
                            "last_message": {
                                "role": "assistant",
                                "content": final_content,
                                "tool_calls": final_tool_calls
                            }
                        });
                        // Also append to messages array using the reducer
                        update.as_object_mut().unwrap().insert("messages".to_string(), serde_json::json!([{
                                "role": "assistant",
                                "content": final_content,
                                "tool_calls": final_tool_calls
                        }]));
                        Ok(update)
                    }
                    Err(e) => Err(format!("LLM Error: {}", e)),
                }
            })
        });

        // --- NODE 2: Tool Execution ---
        let tool_tools = session_tools_arc.clone();
        let cfg_max_retries = cfg.max_retries;
        graph.add_node("tool_node", move |state| {
            let tt = tool_tools.clone();
            Box::pin(async move {
                let last_msg = state.get("last_message").unwrap();
                let tool_calls = last_msg.get("tool_calls").unwrap().as_array().unwrap();

                let mut error_counts = state.get("error_counts").unwrap().as_object().unwrap().clone();
                let mut read_only_calls = Vec::new();
                let mut mutating_calls = Vec::new();

                for tc_val in tool_calls {
                    let name = tc_val["name"].as_str().unwrap();
                    let is_read_only = tt.iter().find(|t| t.name == name).map(|t| t.is_read_only).unwrap_or(false);
                    if is_read_only {
                        read_only_calls.push(tc_val.clone());
                    } else {
                        mutating_calls.push(tc_val.clone());
                    }
                }

                let mut tool_results_json = vec![serde_json::json!(null); tool_calls.len()];

                // Execute read-only calls concurrently
                let mut read_only_futures = Vec::new();
                for tc_val in read_only_calls {
                    let tt_clone = tt.clone();
                    read_only_futures.push(async move {
                        let name = tc_val["name"].as_str().unwrap();
                        let args = tc_val["arguments"].clone();
                        let id = tc_val["id"].as_str().unwrap().to_string();

                        if let Some(tool) = tt_clone.iter().find(|t| t.name == name) {
                            let mut retry_count = 0;
                            let max_retries = cfg_max_retries; // Error Handling (Compounding Error Prevention): Stripe limits retries to exactly 2.
                            let final_res;

                            loop {
                                match tool.execute.execute(args.clone()).await {
                                    Ok(res) => {
                                        final_res = Ok(res);
                                        break;
                                    }
                                    Err(crate::types::ToolError::Transient(msg)) => {
                                        if retry_count < max_retries {
                                            retry_count += 1;
                                            let backoff = std::time::Duration::from_millis(50 * (1 << retry_count));
                                            tokio::time::sleep(backoff).await;
                                            continue;
                                        } else {
                                            final_res = Err(crate::types::ToolError::Transient(msg));
                                            break;
                                        }
                                    }
                                    Err(e) => {
                                        final_res = Err(e);
                                        break;
                                    }
                                }
                            }
                            (id, final_res)
                        } else {
                            // Unreachable if tool not found goes to mutating calls
                            unreachable!()
                        }
                    });
                }

                let ro_results = futures::future::join_all(read_only_futures).await;

                for (id, final_res) in ro_results {
                    let idx = tool_calls.iter().position(|tc| tc["id"].as_str().unwrap() == id).unwrap();
                    match final_res {
                        Ok(res) => {
                            let tool_name = tool_calls.iter().find(|tc| tc["id"].as_str().unwrap() == id).unwrap()["name"].as_str().unwrap().to_string();
                            error_counts.insert(tool_name, serde_json::json!(0));
                            tool_results_json[idx] = serde_json::json!({
                                "tool_call_id": id,
                                "content": res,
                                "error": ""
                            });
                        }
                        Err(crate::types::ToolError::LlmRecoverable(msg)) => {
                            let tool_name = tool_calls.iter().find(|tc| tc["id"].as_str().unwrap() == id).unwrap()["name"].as_str().unwrap().to_string();
                            let count = error_counts.entry(tool_name.clone()).or_insert(serde_json::json!(0)).as_u64().unwrap() + 1;
                            error_counts.insert(tool_name.clone(), serde_json::json!(count));
                            if count > cfg_max_retries as u64 {
                                return Err(format!("Fatal tool error: Tool '{}' failed consecutively beyond max_retries limit with recoverable errors. Escalating to Fatal to prevent compounding error loops. Last error: {}", tool_name, msg));
                            }
                            tool_results_json[idx] = serde_json::json!({
                                "tool_call_id": id,
                                "content": "",
                                "error": msg
                            });
                        }
                        Err(crate::types::ToolError::Transient(msg)) => {
                            return Err(format!("Unexpected tool error: Transient error after retries: {}", msg));
                        }
                        Err(crate::types::ToolError::UserFixable(msg)) => {
                            return Err(format!("USER_FIXABLE:{}", msg));
                        }
                        Err(crate::types::ToolError::Fatal(msg)) => {
                            return Err(format!("Fatal tool error: {}", msg));
                        }
                        Err(crate::types::ToolError::Unexpected(msg)) => {
                            return Err(format!("Unexpected tool error: {}", msg));
                        }
                        Err(crate::types::ToolError::HandoffRequested(target)) => {
                            return Err(format!("Handoff requested to {}", target));
                        }
                    }
                }

                // Execute mutating calls sequentially
                for tc_val in mutating_calls {
                    let name = tc_val["name"].as_str().unwrap();
                    let args = tc_val["arguments"].clone();
                    let id = tc_val["id"].as_str().unwrap();
                    let idx = tool_calls.iter().position(|tc| tc["id"].as_str().unwrap() == id).unwrap();

                    if let Some(tool) = tt.iter().find(|t| t.name == name) {
                        let mut retry_count = 0;
                        let max_retries = cfg_max_retries; // Error Handling (Compounding Error Prevention): Stripe limits retries to exactly 2.
                        let final_res;

                        loop {
                            match tool.execute.execute(args.clone()).await {
                                Ok(res) => {
                                    final_res = Ok(res);
                                    break;
                                }
                                Err(crate::types::ToolError::Transient(msg)) => {
                                    if retry_count < max_retries {
                                        retry_count += 1;
                                        let backoff = std::time::Duration::from_millis(50 * (1 << retry_count));
                                        tokio::time::sleep(backoff).await;
                                        continue;
                                    } else {
                                        final_res = Err(crate::types::ToolError::Transient(msg));
                                        break;
                                    }
                                }
                                Err(e) => {
                                    final_res = Err(e);
                                    break;
                                }
                            }
                        }

                        match final_res {
                            Ok(res) => {
                                error_counts.insert(name.to_string(), serde_json::json!(0));
                                tool_results_json[idx] = serde_json::json!({
                                    "tool_call_id": id,
                                    "content": res,
                                    "error": ""
                                });
                            }
                            Err(crate::types::ToolError::LlmRecoverable(msg)) => {
                                let count = error_counts.entry(name.to_string()).or_insert(serde_json::json!(0)).as_u64().unwrap() + 1;
                                error_counts.insert(name.to_string(), serde_json::json!(count));
                                if count > cfg_max_retries as u64 {
                                    return Err(format!("Fatal tool error: Tool '{}' failed consecutively beyond max_retries limit with recoverable errors. Escalating to Fatal to prevent compounding error loops. Last error: {}", name, msg));
                                }
                                tool_results_json[idx] = serde_json::json!({
                                    "tool_call_id": id,
                                    "content": "",
                                    "error": msg
                                });
                            }
                            Err(crate::types::ToolError::Transient(msg)) => {
                                return Err(format!("Unexpected tool error: Transient error after retries: {}", msg));
                            }
                            Err(crate::types::ToolError::UserFixable(msg)) => {
                                return Err(format!("USER_FIXABLE:{}", msg));
                            }
                            Err(crate::types::ToolError::Fatal(msg)) => {
                                return Err(format!("Fatal tool error: {}", msg));
                            }
                            Err(crate::types::ToolError::Unexpected(msg)) => {
                                return Err(format!("Unexpected tool error: {}", msg));
                            }
                            Err(crate::types::ToolError::HandoffRequested(target)) => {
                                return Err(format!("Handoff requested to {}", target));
                            }
                        }
                    } else {
                        tool_results_json[idx] = serde_json::json!({
                            "tool_call_id": id,
                            "content": "",
                            "error": format!("Tool {} not found", name)
                        });
                    }
                }

                Ok(serde_json::json!({
                    "has_tool_calls": false, // Clear flag
                    "error_counts": error_counts,
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
            if state
                .get("has_tool_calls")
                .and_then(|v| v.as_bool())
                .unwrap_or(false)
            {
                "tool_node".to_string()
            } else {
                crate::langgraph::END.to_string()
            }
        });

        graph.set_entry_point("llm_call");

        // Convert initial messages to json state
        let msgs_json: Vec<_> = initial_messages
            .iter()
            .map(|m| {
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
            })
            .collect();

        let initial_state = serde_json::json!({
            "messages": msgs_json,
            "has_tool_calls": false,
            "total_tokens": 0,
            "error_counts": {}
        });

        match graph.run(initial_state).await {
            Ok(final_state) => {
                let final_msgs = final_state.get("messages").unwrap().as_array().unwrap();
                let last_msg = final_msgs.last().unwrap();
                let content = last_msg
                    .get("content")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                on_event(AgentEvent::TaskComplete {
                    content: content.clone(),
                });

                // Cross-Department Memory Consolidation for LangGraph
                if !content.is_empty() {
                    if let Some(store) = &self.memory_store {
                        let content_to_store = content.clone();
                        let store_clone = store.clone();
                        tokio::spawn(async move {
                            if let Err(e) = store_clone
                                .store(
                                    &content_to_store,
                                    vec!["AUTO_CONSOLIDATED_LANGGRAPH".to_string()],
                                )
                                .await
                            {
                                tracing::error!(
                                    "Failed to auto-consolidate LangGraph memory: {}",
                                    e
                                );
                            } else {
                                tracing::debug!("Successfully auto-consolidated LangGraph memory.");
                            }
                        });
                    }
                }

                Ok(content)
            }
            Err(e) => {
                if let Some(msg) = e.strip_prefix("USER_FIXABLE:") {
                    let err_msg = format!("User intervention required: {}", msg);
                    on_event(AgentEvent::UserInterventionRequired {
                        error: err_msg.clone(),
                    });
                    return Err(err_msg.into());
                }
                let err_msg = format!("LangGraph Error: {}", e);
                on_event(AgentEvent::TaskError {
                    error: err_msg.clone(),
                });
                Err(err_msg.into())
            }
        }
    }

}
