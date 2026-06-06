use std::sync::Arc;
use tokio::sync::mpsc;
use ohc_builtin_agent::agent::{Agent, AgentEvent, AgentRunConfig};
use ohc_builtin_agent::service::{AgentConfig, InterventionDispatcher};
use ohc_builtin_agent_llm::LlmClient;
use ohc_builtin_agent_core::types::{ChatRequest, ChatResponse, Message, ToolCall, Usage, ToolError};
use ohc_builtin_agent::tools::Tool;
use serde_json::json;

struct MockInterventionLlm {
    step: tokio::sync::Mutex<usize>,
}

#[async_trait::async_trait]
impl LlmClient for MockInterventionLlm {
    async fn chat(&self, _req: ChatRequest) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
        let mut step = self.step.lock().await;
        *step += 1;
        if *step == 1 {
            Ok(ChatResponse {
                message: Message {
                    role: ohc_builtin_agent_core::types::Role::Assistant,
                    content: "I need to call a tool that requires your input.".to_string(),
                    tool_calls: vec![ToolCall {
                        id: "call_123".to_string(),
                        name: "needs_input_tool".to_string(),
                        arguments: json!({}),
                    }],
                    tool_results: vec![],
                    response_id: Some("resp_1".to_string()),
                    previous_response_id: None,
                },
                usage: Usage::default(),
                stop_reason: "tool_calls".to_string(),
                response_id: Some("resp_1".to_string()),
            })
        } else {
            Ok(ChatResponse {
                message: Message::assistant("Thank you for the input. Task complete."),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("resp_2".to_string()),
            })
        }
    }
}

struct NeedsInputTool;

#[async_trait::async_trait]
impl ohc_builtin_agent::tools::ToolExecutor for NeedsInputTool {
    async fn execute(&self, _args: serde_json::Value) -> Result<String, ToolError> {
        Err(ToolError::UserFixable("Authentication required for this tool. Please provide your token.".to_string()))
    }
}

#[tokio::test]
async fn test_intervention_suspension_and_resume_flow() {
    let llm = Arc::new(MockInterventionLlm { step: tokio::sync::Mutex::new(0) });

    let tool = Tool {
        name: "needs_input_tool".to_string(),
        description: "A tool that always requires intervention".to_string(),
        is_read_only: false,
        parameters: json!({}),
        execute: Arc::new(NeedsInputTool),
    };

    let agent = Agent::new(llm.clone(), vec![tool]);
    let active_interventions = Arc::new(dashmap::DashMap::new());
    let handler = Arc::new(InterventionDispatcher {
        active_interventions: active_interventions.clone(),
    });

    let cfg = AgentRunConfig {
        task_id: Some("task_456".to_string()),
        intervention_handler: Some(handler.clone()),
        ..AgentRunConfig::default()
    };

    let (tx, mut rx) = mpsc::channel(10);
    // Move tx into the closure correctly
    let tx_clone = tx.clone();

    let agent_handle = tokio::spawn(async move {
        let mut on_event = move |evt| {
            let _ = tx_clone.blocking_send(evt);
        };
        agent.run(&cfg, "Start the task", &mut on_event).await
    });

    // 1. Wait for intervention required event
    let mut intervention_required = false;
    while let Some(evt) = rx.recv().await {
        if let AgentEvent::UserInterventionRequired { error } = evt {
            assert!(error.contains("Authentication required"));
            intervention_required = true;
            break;
        }
    }
    assert!(intervention_required);

    // 2. Resolve the intervention
    let key = "task_456:call_123";
    // Simulate what resolve_intervention RPC does
    if let Some((_, tx_res)) = active_interventions.remove(key) {
        tx_res.send(Ok("secret-token-123".to_string())).unwrap();
    } else {
        panic!("Intervention not found in map");
    }

    // 3. Wait for resolution event and completion
    let mut resolved = false;
    let mut completed = false;
    while let Some(evt) = rx.recv().await {
        match evt {
            AgentEvent::InterventionResolved { tool_call_id, input } => {
                assert_eq!(tool_call_id, "call_123");
                assert_eq!(input, "secret-token-123");
                resolved = true;
            }
            AgentEvent::TaskComplete { content } => {
                assert!(content.contains("Task complete"));
                completed = true;
                break;
            }
            _ => {}
        }
    }

    assert!(resolved);
    assert!(completed);

    let final_res = agent_handle.await.unwrap();
    assert!(final_res.is_ok());
}
