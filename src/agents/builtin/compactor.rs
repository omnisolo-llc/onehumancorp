use crate::agent::AgentRunConfig;
use crate::types::{ChatRequest, Message, Role};
use crate::llm::LlmClient;
use std::sync::Arc;

pub struct ContextCompactor;

impl ContextCompactor {
    pub async fn compact(
        llm: &Arc<dyn LlmClient>,
        messages: &mut Vec<Message>,
        final_cfg: &AgentRunConfig,
    ) -> Result<(), String> {
        if messages.len() <= 5 {
            return Ok(());
        }

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
                        // Discard redundant/raw tool outputs, but preserve errors if any
                        let status = if tr.error.is_empty() {
                            "Success (raw output discarded during compaction)"
                        } else {
                            &tr.error
                        };
                        middle_text.push_str(&format!("  tool_call_id: {} -> {}\n", tr.tool_call_id, status));
                    }
                }
                middle_text.push_str("---\n");
            }

            let summary_req = ChatRequest {
                model: final_cfg.model.clone(),
                system: "You are an expert context compactor for an AI agent. Summarize the following middle portion of an agent conversation. Preserve architectural decisions and unresolved bugs, but discard redundant/raw tool outputs. Be concise.".to_string(),
                messages: vec![Message::user(format!("Compact this conversation:\n{}", middle_text))],
                tools: vec![],
                max_tokens: 2000,
                temperature: 0.0,
            };

            match llm.chat(summary_req).await {
                Ok(summary_resp) => {
                    let summary = summary_resp.message.content;
                    compact_messages.push(Message::user(format!("[Context Compacted by Harness]:\n{}", summary)));
                    // Append the remaining recent messages
                    compact_messages.extend_from_slice(&messages[middle_end..]);
                    *messages = compact_messages;
                    Ok(())
                }
                Err(e) => Err(format!("Context compaction failed: {}", e)),
            }
        } else {
            Ok(())
        }
    }
}
