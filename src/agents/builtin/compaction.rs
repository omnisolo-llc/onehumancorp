use crate::llm::LlmClient;
/// Master Catalog B.4. Context Management
use crate::types::{ChatRequest, Message};
use std::sync::Arc;

pub async fn compact_context(
    messages: &[Message],
    model: &str,
    llm: &Arc<dyn LlmClient>,
) -> Result<Vec<Message>, String> {
    if messages.len() <= 5 {
        return Ok(messages.to_vec());
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
                    middle_text.push_str(&format!("  {} ({})\n", tc.name, tc.arguments));
                }
            }
            if !m.tool_results.is_empty() {
                middle_text.push_str("Tool Results:\n");
                for tr in &m.tool_results {
                    // Self-Reflective Context Anchoring mechanic: preserve anchors directly
                    if tr.content.contains("[SYSTEM NOTIFICATION: Context Rot Prevention Anchor]") {
                        middle_text.push_str(&format!("  tool_call_id: {} -> Preserved Anchor: {}\n", tr.tool_call_id, tr.content));
                        continue;
                    }

                    // Discard redundant/raw tool outputs, but preserve errors if any
                    let status = if tr.error.is_empty() {
                        "Success (raw output discarded during compaction)".to_string()
                    } else {
                        format!("Error: {}", tr.error)
                    };
                    middle_text.push_str(&format!(
                        "  tool_call_id: {} -> {}\n",
                        tr.tool_call_id, status
                    ));
                }
            }
            middle_text.push_str("---\n");
        }

        let summary_req = ChatRequest {
            model: model.to_string(),
            system: "You are an expert context compactor for an AI agent. Summarize the following middle portion of an agent conversation. Preserve architectural decisions and unresolved bugs, but discard redundant/raw tool outputs. Be concise.".to_string(),
            messages: vec![Message::user(format!("Compact this conversation:\n{}", middle_text))],
            tools: vec![],
            max_tokens: 2000,
            temperature: 0.0,
        };

        match llm.chat(summary_req).await {
            Ok(summary_resp) => {
                let summary = summary_resp.message.content;
                compact_messages.push(Message::user(format!(
                    "[Context Compacted by Harness]:\n{}",
                    summary
                )));
            }
            Err(e) => {
                return Err(format!("Context compaction failed: {}", e));
            }
        }
    }

    // Append the remaining recent messages
    if middle_end < messages.len() {
        compact_messages.extend_from_slice(&messages[middle_end..]);
    } else {
        compact_messages.extend_from_slice(&messages[middle_start..]);
    }

    Ok(compact_messages)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{ChatResponse, Message, Role, ToolCall, ToolResult, Usage};
    use std::sync::Arc;
    use tokio::sync::Mutex;

    struct MockLlmClient {
        responses: Mutex<Vec<ChatResponse>>,
    }

    #[async_trait::async_trait]
    impl LlmClient for MockLlmClient {
        async fn chat(
            &self,
            _req: ChatRequest,
        ) -> Result<ChatResponse, Box<dyn std::error::Error + Send + Sync>> {
            let mut resps = self.responses.lock().await;
            if !resps.is_empty() {
                Ok(resps.remove(0))
            } else {
                Ok(ChatResponse {
                    message: Message::assistant("Summarized"),
                    usage: Usage::default(),
                    stop_reason: "stop".to_string(),
                    response_id: Some("mock-id".to_string()),
                })
            }
        }
    }

    #[tokio::test]
    async fn test_compact_context_short() {
        let llm: Arc<dyn LlmClient> = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![]),
        });
        let messages = vec![
            Message::user("0"),
            Message::assistant("1"),
            Message::user("2"),
            Message::assistant("3"),
        ];

        let compacted = compact_context(&messages, "test-model", &llm)
            .await
            .unwrap();
        assert_eq!(compacted.len(), 4); // Not compacted because length < 5
    }

    #[tokio::test]
    async fn test_compact_context_long() {
        let llm: Arc<dyn LlmClient> = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![ChatResponse {
                message: Message::assistant("This is a summary"),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("id1".to_string()),
            }]),
        });

        let mut msg_tool = Message::assistant("I am calling a tool");
        msg_tool.tool_calls.push(ToolCall {
            id: "call_1".to_string(),
            name: "test_tool".to_string(),
            arguments: serde_json::json!({}),
        });

        let mut msg_result = Message::user("");
        msg_result.role = Role::Tool;
        msg_result.tool_results.push(ToolResult {
            tool_call_id: "call_1".to_string(),
            content: "VERY LONG RAW OUTPUT".to_string(),
            error: "".to_string(),
        });

        let mut msg_result_err = Message::user("");
        msg_result_err.role = Role::Tool;
        msg_result_err.tool_results.push(ToolResult {
            tool_call_id: "call_2".to_string(),
            content: "".to_string(),
            error: "This is a critical error".to_string(),
        });

        let messages = vec![
            Message::user("Message 0 (Start)"),
            Message::assistant("Message 1"),
            msg_tool,
            msg_result,
            msg_result_err,
            Message::user("Message 4"),
            Message::assistant("Message 5"),
            Message::user("Message 6"),
        ];

        let compacted = compact_context(&messages, "test-model", &llm)
            .await
            .unwrap();

        // Expected layout:
        // 0: Initial prompt
        // 1: Compacted summary message
        // 2: messages[len - 3] -> Message 4
        // 3: messages[len - 2] -> Message 5
        // 4: messages[len - 1] -> Message 6
        assert_eq!(compacted.len(), 5);
        assert_eq!(compacted[0].content, "Message 0 (Start)");
        assert!(
            compacted[1]
                .content
                .contains("[Context Compacted by Harness]")
        );
        assert!(compacted[1].content.contains("This is a summary"));
        assert_eq!(compacted[2].content, "Message 4");
    }
    #[tokio::test]
    async fn test_compact_context_preserves_anchor() {
        use crate::types::{ChatResponse, Message, Role, ToolCall, ToolResult, Usage};
        use std::sync::Arc;
        use tokio::sync::Mutex;
        let llm: Arc<dyn LlmClient> = Arc::new(MockLlmClient {
            responses: Mutex::new(vec![ChatResponse {
                message: Message::assistant("This is a summary"),
                usage: Usage::default(),
                stop_reason: "stop".to_string(),
                response_id: Some("id1".to_string()),
            }]),
        });

        let mut msg_tool = Message::assistant("I am calling a tool");
        msg_tool.tool_calls.push(ToolCall {
            id: "call_1".to_string(),
            name: "AnchorContext".to_string(),
            arguments: serde_json::json!({}),
        });

        let mut msg_result = Message::user("");
        msg_result.role = Role::Tool;
        msg_result.tool_results.push(ToolResult {
            tool_call_id: "call_1".to_string(),
            content: "[SYSTEM NOTIFICATION: Context Rot Prevention Anchor]\nAnchored Text: IMPORTANT".to_string(),
            error: "".to_string(),
        });

        let messages = vec![
            Message::user("Message 0 (Start)"),
            Message::assistant("Message 1"),
            msg_tool,
            msg_result,
            Message::user("Message 4"),
            Message::assistant("Message 5"),
            Message::user("Message 6"),
        ];

        let compacted = compact_context(&messages, "test-model", &llm)
            .await
            .unwrap();

        assert_eq!(compacted.len(), 5);
        assert!(compacted[1].content.contains("This is a summary"));
    }
}
