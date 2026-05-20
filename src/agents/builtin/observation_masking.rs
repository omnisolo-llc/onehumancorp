use crate::types::{Message, Role};
use crate::agent::AgentRunConfig;

/// ObservationMasker implements the JetBrains Junie Observation Masking mechanic.
/// It hides the raw output of old tools from the prompt, but keeps the `tool_calls`
/// themselves visible so the model remembers what it did.
/// Upgraded to Recency-Aware Masking: Only mask if older than threshold and exceeds size limit.
pub struct ObservationMasker;

impl ObservationMasker {
    pub fn apply_masking(messages: &mut Vec<Message>, cfg: &AgentRunConfig) {
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
