use ohc_builtin_agent_core::types::{Message, Role};

/// Applies JetBrains Observation Masking.
/// Hides the raw output of old tools from the prompt,
/// but keeps the `tool_calls` themselves visible so the model remembers what it did.
/// Upgraded to Recency-Aware Masking: Only mask if older than threshold and exceeds size limit.
pub fn apply_observation_masking(
    messages: &mut Vec<Message>,
    threshold: usize,
    size_limit: usize,
) {
    let msg_count = messages.len();
    for i in 0..msg_count {
        if messages[i].role == Role::Tool {
            let age = msg_count - i;
            if age > threshold {
                for tr in &mut messages[i].tool_results {
                    if tr.error.is_empty() && !tr.content.starts_with("[Observation Masked") {
                        let bytes = tr.content.len();
                        if bytes > size_limit {
                            let preview_chars = 100;
                            let char_count = tr.content.chars().count();
                            if char_count > preview_chars * 2 {
                                let start_preview: String = tr.content.chars().take(preview_chars).collect();
                                let end_preview: String = tr.content.chars().skip(char_count - preview_chars).collect();
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

#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent_core::types::ToolResult;

    #[test]
    fn test_apply_observation_masking() {
        let mut messages = vec![
            Message {
                role: Role::Tool,
                content: String::new(),
                tool_calls: vec![],
                tool_results: vec![
                    ToolResult {
                        tool_call_id: "call_1".to_string(),
                        content: "A".repeat(500),
                        error: String::new(),
                    },
                ],
                response_id: None,
                previous_response_id: None,
                refusal: None,
            },
            Message {
                role: Role::Assistant,
                content: "Hmm".to_string(),
                tool_calls: vec![],
                tool_results: vec![],
                response_id: None,
                previous_response_id: None,
                refusal: None,
            },
            Message {
                role: Role::Tool,
                content: String::new(),
                tool_calls: vec![],
                tool_results: vec![
                    ToolResult {
                        tool_call_id: "call_2".to_string(),
                        content: "B".repeat(500),
                        error: String::new(),
                    },
                ],
                response_id: None,
                previous_response_id: None,
                refusal: None,
            },
        ];

        // Mask messages older than 1 message from the end. Only 'call_1' is older.
        // Size limit 100 bytes.
        apply_observation_masking(&mut messages, 1, 100);

        // First message should be masked
        assert!(messages[0].tool_results[0].content.contains("[Observation Masked"));
        assert!(messages[0].tool_results[0].content.contains("call_1"));

        // Last message should NOT be masked
        assert!(!messages[2].tool_results[0].content.contains("[Observation Masked"));
        assert_eq!(messages[2].tool_results[0].content, "B".repeat(500));
    }
}
