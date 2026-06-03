use crate::types::{Message, Role};

/// Applies JetBrains Observation Masking.
/// Hides the raw output of old tools from the prompt,
/// but keeps the `tool_calls` themselves visible so the model remembers what it did.
/// Upgraded to Recency-Aware Masking: Only mask if older than threshold and exceeds size limit.
pub struct JetBrainsObservationMasker {
    threshold: usize,
    size_limit: usize,
}

impl JetBrainsObservationMasker {
    pub fn new(threshold: usize, size_limit: usize) -> Self {
        Self { threshold, size_limit }
    }

    pub fn apply_masking(&self, messages: &mut [Message]) {
        let msg_count = messages.len();
        for (i, msg) in messages.iter_mut().enumerate() {
            if msg.role == Role::Tool {
                let age = msg_count - i;
                if age > self.threshold {
                    for tr in &mut msg.tool_results {
                        if tr.error.is_empty() && !tr.content.starts_with("[Observation Masked") {
                            let bytes = tr.content.len();
                            if bytes > self.size_limit {
                                let preview_chars = 100;

                                if bytes > preview_chars * 2 {
                                    // Optimize preview generation using string slicing if possible
                                    if tr.content.is_char_boundary(preview_chars) && tr.content.is_char_boundary(bytes - preview_chars) {
                                        let start_preview = &tr.content[..preview_chars];
                                        let end_preview = &tr.content[bytes - preview_chars..];
                                        tr.content = format!(
                                            "[Observation Masked to save context. Output was {} bytes. Preview: {}...{} The tool call itself remains visible. Use 'RecallObservation' with ID '{}' if you need the full output again.]",
                                            bytes, start_preview, end_preview, tr.tool_call_id
                                        );
                                    } else {
                                        // Fallback to char iteration if string slicing is not on a boundary
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

pub fn apply_observation_masking(messages: &mut [Message], threshold: usize, size_limit: usize) {
    let masker = JetBrainsObservationMasker::new(threshold, size_limit);
    masker.apply_masking(messages);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::ToolResult;

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
        assert!(messages[0].tool_results[0].content.contains("Preview: "));

        // Last message should NOT be masked
        assert!(!messages[2].tool_results[0].content.contains("[Observation Masked"));
        assert_eq!(messages[2].tool_results[0].content, "B".repeat(500));
    }

    #[test]
    fn test_apply_observation_masking_unicode_boundaries() {
        let mut messages = vec![
            Message {
                role: Role::Tool,
                content: String::new(),
                tool_calls: vec![],
                tool_results: vec![
                    ToolResult {
                        tool_call_id: "call_unicode".to_string(),
                        // Emojis and unicode characters
                        content: "🌍🚀".repeat(100) + "Hello World!",
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

        apply_observation_masking(&mut messages, 0, 100);

        assert!(messages[0].tool_results[0].content.contains("[Observation Masked"));
        assert!(messages[0].tool_results[0].content.contains("call_unicode"));
    }

    #[test]
    fn test_apply_observation_masking_short_content() {
        let mut messages = vec![
            Message {
                role: Role::Tool,
                content: String::new(),
                tool_calls: vec![],
                tool_results: vec![
                    ToolResult {
                        tool_call_id: "call_short".to_string(),
                        content: "Short content".to_string(),
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

        // Size limit is large, shouldn't mask
        apply_observation_masking(&mut messages, 0, 1000);

        assert_eq!(messages[0].tool_results[0].content, "Short content");
    }

    #[test]
    fn test_apply_observation_masking_underflow() {
        let mut messages = vec![
            Message {
                role: Role::Tool,
                content: String::new(),
                tool_calls: vec![],
                tool_results: vec![
                    ToolResult {
                        tool_call_id: "call_underflow".to_string(),
                        content: "A".repeat(80),
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

        apply_observation_masking(&mut messages, 0, 50);

        assert!(messages[0].tool_results[0].content.contains("[Observation Masked"));
        assert!(!messages[0].tool_results[0].content.contains("Preview: "));
    }
}
