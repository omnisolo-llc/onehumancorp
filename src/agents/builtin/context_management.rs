use ohc_builtin_agent_core::types::{Message, Role};
use crate::agent::AgentRunConfig;

/// Implements the Context Management mechanic from the Master Catalog:
/// Observation Masking (JetBrains' Junie): Hide the raw output of old tools from the prompt,
/// but keep the `tool_calls` themselves visible so the model remembers what it did.
pub struct ObservationMasker;

impl ObservationMasker {
    pub fn mask_observations(messages: &mut Vec<Message>, cfg: &AgentRunConfig) {
        if !cfg.enable_observation_masking {
            return;
        }

        let msg_count = messages.len();
        for i in 0..msg_count {
            if messages[i].role == Role::Tool {
                let age = msg_count - i;
                if age > cfg.observation_masking_threshold {
                    for tr in &mut messages[i].tool_results {
                        if tr.error.is_empty() && !tr.content.starts_with("[Observation Masked") {
                            let bytes = tr.content.len();
                            if bytes > cfg.observation_masking_size_limit {
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

#[cfg(test)]
mod tests {
    use super::*;
    use ohc_builtin_agent_core::types::{Message, Role, ToolResult};

    #[test]
    fn test_masking_applied_above_threshold() {
        let mut cfg = AgentRunConfig::default();
        cfg.enable_observation_masking = true;
        cfg.observation_masking_threshold = 0; // aggressively mask everything older than 0 (which is anything not the very last message)
        cfg.observation_masking_size_limit = 10; // small size limit

        let mut messages = vec![
            Message {
                role: Role::Tool,
                content: "".to_string(),
                tool_calls: vec![],
                tool_results: vec![
                    ToolResult {
                        tool_call_id: "call_1".to_string(),
                        content: "This is a very long observation output that should be masked because it exceeds the size limit and threshold.".to_string(),
                        error: "".to_string(),
                    }
                ],
                response_id: None,
                previous_response_id: None,
            },
            Message {
                role: Role::User,
                content: "Next user message".to_string(),
                tool_calls: vec![],
                tool_results: vec![],
                response_id: None,
                previous_response_id: None,
            }
        ];

        ObservationMasker::mask_observations(&mut messages, &cfg);

        // the first message has age = 2 (messages.len() - 0), which is > 0 threshold
        let tr = &messages[0].tool_results[0];
        assert!(tr.content.starts_with("[Observation Masked"));
        assert!(tr.content.contains("call_1"));
    }

    #[test]
    fn test_masking_ignored_below_threshold() {
        let mut cfg = AgentRunConfig::default();
        cfg.enable_observation_masking = true;
        cfg.observation_masking_threshold = 2; // only mask if age > 2
        cfg.observation_masking_size_limit = 10;

        let mut messages = vec![
            Message {
                role: Role::Tool,
                content: "".to_string(),
                tool_calls: vec![],
                tool_results: vec![
                    ToolResult {
                        tool_call_id: "call_1".to_string(),
                        content: "This is a very long observation output that should NOT be masked because it is below the threshold.".to_string(),
                        error: "".to_string(),
                    }
                ],
                response_id: None,
                previous_response_id: None,
            },
            Message {
                role: Role::User,
                content: "Next user message".to_string(),
                tool_calls: vec![],
                tool_results: vec![],
                response_id: None,
                previous_response_id: None,
            }
        ];

        ObservationMasker::mask_observations(&mut messages, &cfg);

        // age = 2. 2 is not > 2, so it should NOT be masked
        let tr = &messages[0].tool_results[0];
        assert!(!tr.content.starts_with("[Observation Masked"));
        assert!(tr.content.contains("long observation output"));
    }
}
