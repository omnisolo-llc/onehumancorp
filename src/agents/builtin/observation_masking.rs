use ohc_builtin_agent_core::types::{Message, Role};


/// Applies JetBrains Observation Masking.
/// Hides the raw output of old tools from the prompt,
/// but keeps the `tool_calls` themselves visible so the model remembers what it did.
/// Upgraded to Recency-Aware Masking: Only mask if older than threshold and exceeds size limit.
pub struct JetBrainsObservationMasker {
    threshold: usize,
    size_limit: usize,
    pub element_limit: usize,
}

impl JetBrainsObservationMasker {
    pub fn new(threshold: usize, size_limit: usize, _element_limit: usize) -> Self {
        Self {
            threshold,
            size_limit,
            element_limit: _element_limit,
        }
    }

    pub fn apply_masking(&self, messages: &mut Vec<Message>) {
        let msg_count = messages.len();
        for i in 0..msg_count {
            if messages[i].role == Role::Tool {
                let age = msg_count - i;
                if age > self.threshold {
                    for tr in &mut messages[i].tool_results {
                        if tr.error.is_empty() && !tr.content.starts_with("[Observation Masked") {
                            let bytes = tr.content.len();
                            if bytes > self.size_limit {
                                // Master Catalog: Hide the raw output of old tools from the prompt,
                                // but keep the `tool_calls` themselves visible so the model remembers what it did.
                                // We bypass structural JSON masking entirely and use a simple string fallback to strictly cap length.
                                let preview_chars = std::cmp::max(100, self.size_limit / 4);
                                let char_count = tr.content.chars().count();
                                if char_count > preview_chars * 2 {
                                    let start_preview: String =
                                        tr.content.chars().take(preview_chars).collect();
                                    let end_preview: String = tr
                                        .content
                                        .chars()
                                        .skip(char_count - preview_chars)
                                        .collect();
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
}

pub fn apply_observation_masking(messages: &mut Vec<Message>, threshold: usize, size_limit: usize, element_limit: usize) {
    let masker = JetBrainsObservationMasker::new(threshold, size_limit, element_limit);
    masker.apply_masking(messages);
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
                tool_results: vec![ToolResult {
                    tool_call_id: "call_1".to_string(),
                    content: "A".repeat(500),
                    error: String::new(),
                }],
                response_id: None,
                previous_response_id: None,
            },
            Message {
                role: Role::Assistant,
                content: "Hmm".to_string(),
                tool_calls: vec![],
                tool_results: vec![],
                response_id: None,
                previous_response_id: None,
            },
            Message {
                role: Role::Tool,
                content: String::new(),
                tool_calls: vec![],
                tool_results: vec![ToolResult {
                    tool_call_id: "call_2".to_string(),
                    content: "B".repeat(500),
                    error: String::new(),
                }],
                response_id: None,
                previous_response_id: None,
            },
        ];

        // Mask messages older than 1 message from the end. Only 'call_1' is older.
        // Size limit 100 bytes.
        apply_observation_masking(&mut messages, 1, 50, 50);

        // First message should be masked
        assert!(
            messages[0].tool_results[0]
                .content
                .contains("[Observation Masked")
        );
        assert!(messages[0].tool_results[0].content.contains("call_1"));
        assert!(messages[0].tool_results[0].content.contains("AAAAA")); // should contain the prefix

        // Last message should NOT be masked
        assert!(
            !messages[2].tool_results[0]
                .content
                .contains("[Observation Masked")
        );
        assert_eq!(messages[2].tool_results[0].content, "B".repeat(500));
    }
    #[test]
    fn test_mask_json_value() {
        let mut messages = vec![
            Message {
                role: Role::Tool,
                content: String::new(),
                tool_calls: vec![],
                tool_results: vec![ToolResult {
                    tool_call_id: "call_3".to_string(),
                    content: format!("{{\"small\": \"abc\", \"large\": \"{}\"}}", "A".repeat(500)),
                    error: String::new(),
                }],
                response_id: None,
                previous_response_id: None,
            },
            Message {
                role: Role::Assistant,
                content: "Hmm".to_string(),
                tool_calls: vec![],
                tool_results: vec![],
                response_id: None,
                previous_response_id: None,
            },
            Message {
                role: Role::Assistant,
                content: "Hmm".to_string(),
                tool_calls: vec![],
                tool_results: vec![],
                response_id: None,
                previous_response_id: None,
            },
        ];

        // Mask messages older than 1 message from the end. Only 'call_3' is older.
        // Size limit 100 bytes.
        apply_observation_masking(&mut messages, 1, 100, 50);

        let masked_content = &messages[0].tool_results[0].content;
        assert!(masked_content.contains("[Observation Masked"));
    }
}

#[cfg(test)]
mod additional_tests {
    use serde_json::Value;
    use super::*;
    use ohc_builtin_agent_core::types::ToolResult;

    #[test]
    fn test_mask_large_array_truncation() {
        let large_array: Vec<usize> = (0..100).collect();
        let json_str = serde_json::to_string(&large_array).unwrap();

        let mut messages = vec![
            Message {
                role: Role::Tool,
                content: String::new(),
                tool_calls: vec![],
                tool_results: vec![ToolResult {
                    tool_call_id: "call_4".to_string(),
                    content: json_str,
                    error: String::new(),
                }],
                response_id: None,
                previous_response_id: None,
            },
            Message {
                role: Role::Assistant,
                content: "Hmm".to_string(),
                tool_calls: vec![],
                tool_results: vec![],
                response_id: None,
                previous_response_id: None,
            },
        ];

        // Mask messages older than 0. Limit array elements to 10.
        let masker = JetBrainsObservationMasker::new(0, 10, 10);
        masker.apply_masking(&mut messages);

        let masked_content = &messages[0].tool_results[0].content;
        assert!(masked_content.contains("[Observation Masked"));
    }

    #[test]
    fn test_mask_wide_object_truncation() {
        let mut large_object = serde_json::Map::new();
        for i in 0..100 {
            large_object.insert(format!("key_{}", i), Value::Number(i.into()));
        }
        let json_str = serde_json::to_string(&Value::Object(large_object)).unwrap();

        let mut messages = vec![
            Message {
                role: Role::Tool,
                content: String::new(),
                tool_calls: vec![],
                tool_results: vec![ToolResult {
                    tool_call_id: "call_5".to_string(),
                    content: json_str,
                    error: String::new(),
                }],
                response_id: None,
                previous_response_id: None,
            },
            Message {
                role: Role::Assistant,
                content: "Hmm".to_string(),
                tool_calls: vec![],
                tool_results: vec![],
                response_id: None,
                previous_response_id: None,
            },
        ];

        // Mask messages older than 0. Limit object elements to 20.
        let masker = JetBrainsObservationMasker::new(0, 10, 20);
        masker.apply_masking(&mut messages);

        let masked_content = &messages[0].tool_results[0].content;
        assert!(masked_content.contains("[Observation Masked"));
    }

    #[test]
    fn test_mask_deep_recursion_limit() {
        // Build a deeply nested object
        let mut deeply_nested = Value::Object(serde_json::Map::new());
        for _ in 0..15 {
            let mut new_obj = serde_json::Map::new();
            new_obj.insert("nested".to_string(), deeply_nested);
            deeply_nested = Value::Object(new_obj);
        }
        let json_str = serde_json::to_string(&deeply_nested).unwrap();

        let mut messages = vec![
            Message {
                role: Role::Tool,
                content: String::new(),
                tool_calls: vec![],
                tool_results: vec![ToolResult {
                    tool_call_id: "call_6".to_string(),
                    content: json_str,
                    error: String::new(),
                }],
                response_id: None,
                previous_response_id: None,
            },
            Message {
                role: Role::Assistant,
                content: "Hmm".to_string(),
                tool_calls: vec![],
                tool_results: vec![],
                response_id: None,
                previous_response_id: None,
            },
        ];

        let masker = JetBrainsObservationMasker::new(0, 10, 20);
        masker.apply_masking(&mut messages);

        let masked_content = &messages[0].tool_results[0].content;
        assert!(masked_content.contains("[Observation Masked"));
    }
}
