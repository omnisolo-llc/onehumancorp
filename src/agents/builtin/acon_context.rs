use crate::types::{Message, Role};

/// Master Catalog C.3: Context Window Strategy: ACON Research Metric: Prioritizing reasoning traces over raw tool outputs yields 26-54% token reduction while preserving 95%+ accuracy.
/// Configuration for the ACON Context Window Strategy.
#[derive(Debug, Clone)]
pub struct AconConfig {
    /// Number of recent messages to preserve completely.
    pub preserve_recent_messages_count: usize,
}

impl Default for AconConfig {
    fn default() -> Self {
        Self {
            preserve_recent_messages_count: 2,
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
                            && !tr.content.contains("[SYSTEM NOTIFICATION: Context Rot Prevention Anchor]")
                            && !tr.content.is_empty()
                        {
                            tr.content =
                                "[ACON: Tool output omitted to prioritize reasoning traces.]"
                                    .to_string();
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
                    content: "Massive log output".to_string(),
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
                    content: "Another tool result".to_string(),
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
        };
        apply_acon_strategy(&mut messages, &config);

        // First tool message should be masked (it is outside the preserved count)
        assert_eq!(
            messages[0].tool_results[0].content,
            "[ACON: Tool output omitted to prioritize reasoning traces.]"
        );

        // Assistant message reasoning is preserved
        assert_eq!(
            messages[1].content,
            "I'm thinking about the massive log output..."
        );
        assert_eq!(messages[2].tool_results[0].content, "Another tool result");
    }
}

    #[test]
    fn test_apply_acon_strategy_preserves_anchor() {
        use crate::types::ToolResult;
        let mut messages = vec![
            Message {
                role: Role::Tool,
                content: String::new(),
                tool_calls: vec![],
                tool_results: vec![ToolResult {
                    tool_call_id: "call_1".to_string(),
                    content: "[SYSTEM NOTIFICATION: Context Rot Prevention Anchor]\nAnchored!".to_string(),
                    error: String::new(),
                }],
                response_id: None,
                previous_response_id: None,
            },
            Message {
                role: Role::Assistant,
                content: "I'm thinking...".to_string(),
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
                    content: "Another tool result".to_string(),
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
        };
        apply_acon_strategy(&mut messages, &config);

        // First tool message should NOT be masked because it's an anchor
        assert_eq!(
            messages[0].tool_results[0].content,
            "[SYSTEM NOTIFICATION: Context Rot Prevention Anchor]\nAnchored!"
        );
    }
