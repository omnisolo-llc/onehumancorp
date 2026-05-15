use ohc_builtin_agent_core::types::{ChatRequest, Message, Role, ToolCall, ToolDefinition, ToolResult, Usage, ChatResponse, ToolError};
use std::sync::Arc;
use std::sync::atomic::{AtomicI64, AtomicU64, Ordering};
use opentelemetry::{global, KeyValue};
use crate::budget::{check_token_budget, BudgetAction, BudgetTracker};
use crate::guardrails::GuardrailConfig;
use crate::llm::LlmClient;
use crate::tools::Tool;
use super::events::AgentEvent;
use super::config::{AgentRunConfig, AgentProgress};
use super::core::{Agent, build_hierarchical_system_prompt};


    pub async fn run_anthropic_dumb_loop<F>(
        agent: &Agent,
        cfg: &AgentRunConfig,
        initial_message: &str,
        session_tools: &[ohc_builtin_agent_tools::Tool],
        on_event: &mut F,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>>
    where
        F: FnMut(AgentEvent) + Send + Sync,
    {
        on_event(AgentEvent::RunStarted { iteration: 0 });

        let mut messages = vec![crate::types::Message::user(initial_message)];
        let phases = ["Gather", "Act", "Verify"];

        for (i, phase) in phases.iter().enumerate() {
            on_event(AgentEvent::IterationStarted { iteration: i as i32, message_count: messages.len() });

            let phase_prompt = match *phase {
                "Gather" => "Phase: Gather context. Use read-only tools like read, head, grep to search files and read code.",
                "Act" => "Phase: Take action. Use mutating tools like write, edit, bash to edit files and run commands based on gathered context.",
                "Verify" => "Phase: Verify results. Use bash to run tests or check output to verify your actions.",
                _ => unreachable!(),
            };

            let req = crate::types::ChatRequest {
                model: cfg.model.clone(),
                system: format!("{}\n\nYou are in the {} phase.", cfg.server_system_message, phase_prompt),
                messages: messages.clone(),
                tools: session_tools.iter().map(|t| crate::types::ToolDefinition {
                    name: t.name.clone(),
                    description: t.description.clone(),
                    parameters: t.parameters.clone(),
                }).collect(),
                max_tokens: cfg.max_tokens,
                temperature: cfg.temperature,
            };

            let resp = agent.llm.chat(req).await?;
            let msg = resp.message;
            messages.push(msg.clone());

            if msg.tool_calls.is_empty() {
                if *phase == "Verify" {
                    return Ok(msg.content);
                } else {
                    continue;
                }
            }

            // Component: Tools (Read-only concurrent, mutating serial)
            let mut read_only_calls = vec![];
            let mut mutating_calls = vec![];

            for tc in &msg.tool_calls {
                if let Some(tool) = session_tools.iter().find(|t| t.name == tc.name) {
                    if tool.is_read_only {
                        read_only_calls.push(tc.clone());
                    } else {
                        mutating_calls.push(tc.clone());
                    }
                } else {
                    // Default to mutating if not found
                    mutating_calls.push(tc.clone());
                }
            }

            let mut tool_results = vec![crate::types::ToolResult { tool_call_id: String::new(), content: String::new(), error: String::new() }; msg.tool_calls.len()];

            let mut read_only_futures = Vec::new();
            for tc in &read_only_calls {
                let tc_clone = tc.clone();
                let session_tools_clone = session_tools.to_vec();
                let messages_clone = messages.clone();
                read_only_futures.push(async move {
                    let r = match agent.execute_tool(&tc_clone, &session_tools_clone, &messages_clone).await {
                        Ok(res) => res,
                        Err(e) => format!("Error: {:?}", e),
                    };
                    (tc_clone, r)
                });
            }
            let ro_results = futures::future::join_all(read_only_futures).await;
            for (tc, r) in ro_results {
                let idx = msg.tool_calls.iter().position(|t| t.id == tc.id).unwrap();

                on_event(AgentEvent::ToolCall {
                    name: tc.name.clone(),
                    args_json: tc.arguments.to_string(),
                    result: r.clone(),
                    iteration: i as i32,
                });

                tool_results[idx] = crate::types::ToolResult {
                    tool_call_id: tc.id.clone(),
                    content: r,
                    error: String::new(),
                };
            }

            for tc in &mutating_calls {
                let r = match agent.execute_tool(tc, session_tools, &messages).await {
                    Ok(res) => res,
                    Err(e) => format!("Error: {:?}", e),
                };

                let idx = msg.tool_calls.iter().position(|t| t.id == tc.id).unwrap();

                on_event(AgentEvent::ToolCall {
                    name: tc.name.clone(),
                    args_json: tc.arguments.to_string(),
                    result: r.clone(),
                    iteration: i as i32,
                });

                tool_results[idx] = crate::types::ToolResult {
                    tool_call_id: tc.id.clone(),
                    content: r,
                    error: String::new(),
                };
            }

            messages.push(crate::types::Message {
                role: crate::types::Role::Tool,
                content: String::new(),
                tool_calls: vec![],
                tool_results,
                response_id: None,
                previous_response_id: None,
            });
        }

        // Final fallback if Verify phase didn't exit
        let req = crate::types::ChatRequest {
            model: cfg.model.clone(),
            system: "Summarize the final result of the Gather-Act-Verify cycle.".to_string(),
            messages: messages.clone(),
            tools: vec![],
            max_tokens: cfg.max_tokens,
            temperature: cfg.temperature,
        };
        let resp = agent.llm.chat(req).await?;
        Ok(resp.message.content)
    }
