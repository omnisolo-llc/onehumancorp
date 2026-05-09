use crate::types::{Message, Role};

/// Applies Recency-Aware Observation Masking (JetBrains Junie mechanic).
/// Hides the raw output of old tools from the prompt, but keeps the `tool_calls`
/// themselves visible so the model remembers what it did without consuming too much context window.
pub fn apply_observation_masking(messages: &mut [Message], enable: bool, threshold: usize, size_limit: usize) {
    if !enable {
        return;
    }

    let msg_count = messages.len();
    for i in 0..msg_count {
        if messages[i].role == Role::Tool {
            let age = msg_count - i;
            if age > threshold {
                for tr in &mut messages[i].tool_results {
                    if tr.error.is_empty() && !tr.content.starts_with("[Observation Masked") {
                        let bytes = tr.content.len();
                        if bytes > size_limit {
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
