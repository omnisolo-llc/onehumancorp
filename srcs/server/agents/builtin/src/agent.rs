use anyhow::anyhow;
use tracing::{info, warn};

use crate::llm::{
    AssistantMessage, CompletionRequest, ContentPart, ConversationMessage, LLMClient,
    ToolDefinition,
};
use crate::tools::{all_tool_definitions, execute_tool, ToolExecutor};

pub const MAX_AGENT_TURNS: u32 = 50;
pub const DEFAULT_MAX_TOKENS: u32 = 8192;

pub const SYSTEM_PROMPT: &str = r#"You are an agent for Claude Code, Anthropic's official CLI for Claude.
Given the user's message, use the tools available to complete the task fully.
Complete the task — don't gold-plate, but don't leave it half-done.

Important guidelines:
- Work autonomously without asking for confirmation
- Use bash, file_read, file_write, file_edit, grep, glob, and other tools freely
- When writing code, ensure it is complete and correct
- Read existing code before modifying it
- Run tests if available to verify your changes
- Provide a clear summary when done"#;

pub struct AgentConfig {
    pub system_prompt: Option<String>,
    pub max_turns: u32,
    pub max_tokens: u32,
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            system_prompt: None,
            max_turns: MAX_AGENT_TURNS,
            max_tokens: DEFAULT_MAX_TOKENS,
        }
    }
}

pub async fn run_agent(
    task: &str,
    config: AgentConfig,
    llm: &dyn LLMClient,
    executor: &mut ToolExecutor,
) -> anyhow::Result<String> {
    let system = config
        .system_prompt
        .as_deref()
        .unwrap_or(SYSTEM_PROMPT)
        .to_string();

    let tools: Vec<ToolDefinition> = all_tool_definitions();
    let max_turns = config.max_turns;
    let max_tokens = config.max_tokens;

    let mut messages: Vec<ConversationMessage> = vec![ConversationMessage {
        role: "user".into(),
        content: vec![ContentPart::Text {
            text: task.to_string(),
        }],
    }];

    let mut final_text = String::new();

    for turn in 0..max_turns {
        info!(turn, "agent turn");

        let req = CompletionRequest {
            system: system.clone(),
            messages: messages.clone(),
            tools: tools.clone(),
            max_tokens,
        };

        let assistant_msg: AssistantMessage = llm
            .complete(req)
            .await
            .map_err(|e| anyhow!("LLM error on turn {}: {}", turn, e))?;

        info!(
            turn,
            stop_reason = %assistant_msg.stop_reason,
            tool_count = assistant_msg.tool_uses.len(),
            "LLM response"
        );

        if !assistant_msg.text.is_empty() {
            final_text = assistant_msg.text.clone();
        }

        // Build the assistant content parts
        let mut assistant_content: Vec<ContentPart> = Vec::new();
        if !assistant_msg.text.is_empty() {
            assistant_content.push(ContentPart::Text {
                text: assistant_msg.text.clone(),
            });
        }
        for tu in &assistant_msg.tool_uses {
            assistant_content.push(ContentPart::ToolUse {
                id: tu.id.clone(),
                name: tu.name.clone(),
                input: tu.input.clone(),
            });
        }

        messages.push(ConversationMessage {
            role: "assistant".into(),
            content: assistant_content,
        });

        // If no tool uses and end_turn, we're done
        if assistant_msg.tool_uses.is_empty()
            && (assistant_msg.stop_reason == "end_turn"
                || assistant_msg.stop_reason == "stop")
        {
            info!(turn, "agent complete");
            break;
        }

        if assistant_msg.tool_uses.is_empty() {
            // No tools and not end_turn (e.g. max_tokens) — stop anyway
            warn!(stop_reason = %assistant_msg.stop_reason, "stopping agent loop");
            break;
        }

        // Execute tools and collect results
        let mut tool_result_parts: Vec<ContentPart> = Vec::new();
        for tool_use in &assistant_msg.tool_uses {
            info!(tool = %tool_use.name, id = %tool_use.id, "executing tool");
            let result = execute_tool(&tool_use.name, tool_use.input.clone(), executor).await;
            match result {
                Ok(output) => {
                    info!(tool = %tool_use.name, "tool success");
                    tool_result_parts.push(ContentPart::ToolResult {
                        tool_use_id: tool_use.id.clone(),
                        content: vec![ContentPart::Text { text: output }],
                        is_error: false,
                    });
                }
                Err(e) => {
                    warn!(tool = %tool_use.name, error = %e, "tool error");
                    tool_result_parts.push(ContentPart::ToolResult {
                        tool_use_id: tool_use.id.clone(),
                        content: vec![ContentPart::Text {
                            text: format!("Error: {}", e),
                        }],
                        is_error: true,
                    });
                }
            }
        }

        messages.push(ConversationMessage {
            role: "user".into(),
            content: tool_result_parts,
        });
    }

    Ok(final_text)
}
