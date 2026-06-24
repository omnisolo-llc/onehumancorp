use crate::types::{Message, Role};

/// Configuration for the ACON Context Window Strategy.
#[derive(Debug, Clone)]
pub struct AconConfig {
    /// Number of recent messages to preserve completely.
    pub preserve_recent_messages_count: usize,
    /// Do not omit outputs shorter than this length (in chars) as it wastes tokens.
    pub preserve_short_outputs: Option<usize>,
}

impl Default for AconConfig {
    fn default() -> Self {
        Self {
            preserve_recent_messages_count: 2,
            preserve_short_outputs: Some(100),
        }
    }
}

/// The ACON Context Window Strategy implementation.
/// Prioritizes reasoning traces over raw tool outputs, yielding significant token reduction (26-54%)
/// while preserving 95%+ accuracy.
pub struct AconStrategy {
    config: AconConfig,
}

impl AconStrategy {
    pub fn new(config: AconConfig) -> Self {
        Self { config }
    }

    /// Applies the ACON strategy to a mutable list of messages.
    pub fn apply(&self, messages: &mut [Message]) {
        let msg_count = messages.len();
        if msg_count > self.config.preserve_recent_messages_count + 1 {
            let threshold = msg_count - self.config.preserve_recent_messages_count;
            for msg in messages.iter_mut().take(threshold) {
                if msg.role == Role::Tool {
                    for tr in &mut msg.tool_results {
                        if tr.error.is_empty()
                            && !tr.content.starts_with("[ACON:")
                            && !tr.content.is_empty()
                        {
                            let content_len = tr.content.len();
                            // Do not mask short outputs to avoid wasting tokens on the omission message
                            if let Some(short_threshold) = self.config.preserve_short_outputs {
                                if content_len <= short_threshold {
                                    continue;
                                }
                            }
                            tr.content = format!(
                                "[ACON: Tool output omitted to prioritize reasoning traces. Original length: {} chars.]",
                                content_len
                            );
                        }
                    }
                }
            }
        }
    }
}

pub fn apply_acon_strategy(messages: &mut [Message], config: &AconConfig) {
    let strategy = AconStrategy::new(config.clone());
    strategy.apply(messages);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::ToolResult;

    #[test]
    fn test_apply_acon_strategy() {
        let mut messages = vec![
            Message {
                role: Role::Tool,
                content: String::new(),
                tool_calls: vec![],
                tool_results: vec![ToolResult {
                    tool_call_id: "call_1".to_string(),
                    content: "Massive log output that exceeds the one hundred character limit to ensure that it gets properly truncated by the ACON strategy. We need to be absolutely sure this is long enough.".to_string(),
                    error: String::new(),
                }],
                response_id: None,
                previous_response_id: None,
            },
            Message {
                role: Role::Assistant,
                content: "I'm thinking about the massive log output...".to_string(),
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
                    content: "Another tool result that is also very long, much longer than a hundred characters, because if it's too short, it will simply be preserved completely due to the short output threshold!".to_string(),
                    error: String::new(),
                }],
                response_id: None,
                previous_response_id: None,
            },
            Message {
                role: Role::Assistant,
                content: "Final reasoning".to_string(),
                tool_calls: vec![],
                tool_results: vec![],
                response_id: None,
                previous_response_id: None,
            },
        ];

        let config = AconConfig {
            preserve_recent_messages_count: 2,
            preserve_short_outputs: Some(100),
        };
        apply_acon_strategy(&mut messages, &config);

        // First tool message should be masked (it is outside the preserved count)
        assert!(
            messages[0].tool_results[0].content.contains("[ACON: Tool output omitted to prioritize reasoning traces.")
        );
        assert!(messages[0].tool_results[0].content.contains("Original length:"));

        // Assistant message reasoning is preserved
        assert_eq!(
            messages[1].content,
            "I'm thinking about the massive log output..."
        );

        // Second tool message is within the recent preserved count
        assert_eq!(messages[2].tool_results[0].content, "Another tool result that is also very long, much longer than a hundred characters, because if it's too short, it will simply be preserved completely due to the short output threshold!");
    }

    #[test]
    fn test_apply_acon_strategy_preserves_short_outputs() {
        let mut messages = vec![
            Message {
                role: Role::Tool,
                content: String::new(),
                tool_calls: vec![],
                tool_results: vec![ToolResult {
                    tool_call_id: "call_1".to_string(),
                    content: "Short output".to_string(),
                    error: String::new(),
                }],
                response_id: None,
                previous_response_id: None,
            },
            Message {
                role: Role::Assistant,
                content: "Final reasoning".to_string(),
                tool_calls: vec![],
                tool_results: vec![],
                response_id: None,
                previous_response_id: None,
            },
        ];

        let config = AconConfig {
            preserve_recent_messages_count: 0,
            preserve_short_outputs: Some(100),
        };
        apply_acon_strategy(&mut messages, &config);

        // Short tool message should NOT be masked, because it's under 100 chars
        assert_eq!(messages[0].tool_results[0].content, "Short output");
    }
}
