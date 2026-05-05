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
