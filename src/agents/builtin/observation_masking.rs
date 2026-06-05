use ohc_builtin_agent_core::types::{Message, Role};
use serde_json::Value;

/// Applies JetBrains Observation Masking.
/// Hides the raw output of old tools from the prompt,
/// but keeps the `tool_calls` themselves visible so the model remembers what it did.
/// Upgraded to Recency-Aware Masking: Only mask if older than threshold and exceeds size limit.
pub struct JetBrainsObservationMasker {
    threshold: usize,
    size_limit: usize,
    max_depth: usize,
}

impl JetBrainsObservationMasker {
    pub fn new(threshold: usize, size_limit: usize) -> Self {
        Self { threshold, size_limit, max_depth: 20 } // default depth limit 20
    }

    pub fn with_max_depth(mut self, max_depth: usize) -> Self {
        self.max_depth = max_depth;
        self
    }

    fn mask_json_value(val: &mut Value, size_limit: usize, current_depth: usize, max_depth: usize) -> bool {
        if current_depth >= max_depth {
            // Reached maximum depth, replace entire subtree to avoid stack overflow
            *val = Value::String("[Masked string: depth limit exceeded]".to_string());
            return true;
        }

        let mut modified = false;
        match val {
            Value::String(s) => {
                let bytes = s.len();
                if bytes > size_limit {
                    let preview_chars = 50;
                    let char_count = s.chars().count();
                    if char_count > preview_chars * 2 {
                        let start_preview: String = s.chars().take(preview_chars).collect();
                        let end_preview: String = s.chars().skip(char_count - preview_chars).collect();
                        *s = format!("[Masked string: {} bytes. Preview: {}...{}]", bytes, start_preview, end_preview);
                    } else {
                        *s = format!("[Masked string: {} bytes]", bytes);
                    }
                    modified = true;
                }
            }
            Value::Array(arr) => {
                for item in arr {
                    if Self::mask_json_value(item, size_limit, current_depth + 1, max_depth) {
                        modified = true;
                    }
                }
            }
            Value::Object(obj) => {
                for (_, value) in obj.iter_mut() {
                    if Self::mask_json_value(value, size_limit, current_depth + 1, max_depth) {
                        modified = true;
                    }
                }
            }
            _ => {}
        }
        modified
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
                                // Try structural JSON masking first
                                if let Ok(mut json_val) = serde_json::from_str::<Value>(&tr.content) {
                                    if Self::mask_json_value(&mut json_val, self.size_limit, 0, self.max_depth) {
                                        tr.content = serde_json::to_string(&json_val).unwrap_or_else(|_| tr.content.clone());
                                        continue; // Successfully masked as JSON
                                    }
                                }

                                // Fallback to raw string masking
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
}

pub fn apply_observation_masking(messages: &mut Vec<Message>, threshold: usize, size_limit: usize) {
    let masker = JetBrainsObservationMasker::new(threshold, size_limit);
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
                tool_results: vec![
                    ToolResult {
                        tool_call_id: "call_1".to_string(),
                        content: "A".repeat(500),
                        error: String::new(),
                    },
                ],
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
                tool_results: vec![
                    ToolResult {
                        tool_call_id: "call_2".to_string(),
                        content: "B".repeat(500),
                        error: String::new(),
                    },
                ],
                response_id: None,
                previous_response_id: None,
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

    #[test]
    fn test_mask_json_value() {
        let mut messages = vec![
            Message {
                role: Role::Tool,
                content: String::new(),
                tool_calls: vec![],
                tool_results: vec![
                    ToolResult {
                        tool_call_id: "call_3".to_string(),
                        content: format!("{{\"small\": \"abc\", \"large\": \"{}\"}}", "A".repeat(500)),
                        error: String::new(),
                    },
                ],
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

        apply_observation_masking(&mut messages, 1, 100);

        let masked_content = &messages[0].tool_results[0].content;
        assert!(masked_content.contains("\"small\":\"abc\""));
        assert!(masked_content.contains("[Masked string: 500 bytes"));
        // Ensure it is still valid JSON
        let parsed: Value = serde_json::from_str(masked_content).expect("Should be valid JSON");
        assert_eq!(parsed["small"].as_str().unwrap(), "abc");
        assert!(parsed["large"].as_str().unwrap().contains("Masked string"));
    }

    #[test]
    fn test_max_depth_masking() {
        let mut messages = vec![
            Message {
                role: Role::Tool,
                content: String::new(),
                tool_calls: vec![],
                tool_results: vec![
                    ToolResult {
                        tool_call_id: "call_4".to_string(),
                        content: r#"{"level1":{"level2":{"level3":"too deep"}}}"#.to_string(),
                        error: String::new(),
                    },
                ],
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

        let masker = JetBrainsObservationMasker::new(0, 10).with_max_depth(2);
        masker.apply_masking(&mut messages);

        let masked_content = &messages[0].tool_results[0].content;
        let parsed: Value = serde_json::from_str(masked_content).expect("Should be valid JSON");

        // Depth 0: root object
        // Depth 1: level1 object
        // Depth 2: level2 object (should be replaced because max_depth=2)
        assert_eq!(parsed["level1"]["level2"].as_str().unwrap(), "[Masked string: depth limit exceeded]");
    }
}
