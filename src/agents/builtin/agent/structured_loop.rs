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


    pub async fn run_structured<T: serde::de::DeserializeOwned + Send + Sync + 'static, F>(
        agent: &Agent,
        cfg: &AgentRunConfig,
        initial_message: &str,
        output_schema: serde_json::Value,
        on_event: &mut F,
    ) -> Result<T, Box<dyn std::error::Error + Send + Sync>>
    where
        F: FnMut(AgentEvent) + Send + Sync,
    {
        let mut final_cfg = cfg.clone();

        // Append instruction to force the use of the structured output tool
        final_cfg.server_system_message = format!(
            "{}\n\nCRITICAL INSTRUCTION: You MUST call the `return_structured_output` tool to return your final structured answer. Do NOT return raw text as the final answer.",
            final_cfg.server_system_message
        );

        let mut structured_tools = agent.tools.clone();

        // We define a dummy executor because the tool is intercepted before execution
        struct DummyExecutor;
        #[async_trait::async_trait]
        impl crate::tools::ToolExecutor for DummyExecutor {
            async fn execute(&self, _args: serde_json::Value) -> Result<String, crate::types::ToolError> {
                Ok("Dummy".to_string())
            }
        }

        structured_tools.push(crate::tools::Tool {
            name: "return_structured_output".to_string(),
            description: "Returns the final output matching the required JSON schema.".to_string(),
            is_read_only: false,
            parameters: output_schema,
            execute: std::sync::Arc::new(DummyExecutor),
        });

        let temp_agent = Agent {
            llm: agent.llm.clone(),
            tools: structured_tools,
            progress: agent.progress.clone(),
            memory_store: agent.memory_store.clone(),
            checkpointer: agent.checkpointer.clone(),
            observation_store: agent.observation_store.clone(),
        };

        // Run the agent. The run loop will intercept `return_structured_output` and return `tc.arguments` as JSON string.
        let raw_json_str = temp_agent.run(&final_cfg, initial_message, on_event).await?;

        let parsed: T = serde_json::from_str(&raw_json_str)
            .map_err(|e| format!("Failed to parse JSON into struct: {}. Raw: {}", e, raw_json_str))?;
        Ok(parsed)
    }
