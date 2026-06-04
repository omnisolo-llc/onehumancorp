use ohc_builtin_agent_core::types::{Message, Role};
use regex::Regex;
use std::sync::OnceLock;

static EMAIL_REGEX: OnceLock<Regex> = OnceLock::new();
static PHONE_REGEX: OnceLock<Regex> = OnceLock::new();

pub fn redact_sensitive_content(text: &str) -> String {
    let email_regex = EMAIL_REGEX.get_or_init(|| Regex::new(r"(?i)\b[A-Z0-9._%+-]+@[A-Z0-9.-]+\.[A-Z]{2,}\b").unwrap());
    // Simple phone number regex matching standard US formats
    let phone_regex = PHONE_REGEX.get_or_init(|| Regex::new(r"\b(\+\d{1,2}\s?)?\(?\d{3}\)?[\s.-]?\d{3}[\s.-]?\d{4}\b").unwrap());

    let mut redacted = text.to_string();
    redacted = email_regex.replace_all(&redacted, "[REDACTED EMAIL]").to_string();
    redacted = phone_regex.replace_all(&redacted, "[REDACTED PHONE]").to_string();
    redacted
}

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
                                let preview_chars = 100;
                                let char_count = tr.content.chars().count();
                                if char_count > preview_chars * 2 {
                                    // Use line-based truncation heuristic if possible
                                    let lines: Vec<&str> = tr.content.lines().collect();
                                    if lines.len() > 4 {
                                        let start_preview = redact_sensitive_content(&lines[0..2].join("\n"));
                                        let end_preview = redact_sensitive_content(&lines[lines.len() - 2..].join("\n"));
                                        tr.content = format!(
                                            "[Observation Masked to save context. Output was {} bytes. Preview:\n{}\n...\n{}\n The tool call itself remains visible. Use 'RecallObservation' with ID '{}' if you need the full output again.]",
                                            bytes, start_preview, end_preview, tr.tool_call_id
                                        );
                                    } else {
                                        let start_preview: String = redact_sensitive_content(&tr.content.chars().take(preview_chars).collect::<String>());
                                        let end_preview: String = redact_sensitive_content(&tr.content.chars().skip(char_count - preview_chars).collect::<String>());
                                        tr.content = format!(
                                            "[Observation Masked to save context. Output was {} bytes. Preview: {}...{} The tool call itself remains visible. Use 'RecallObservation' with ID '{}' if you need the full output again.]",
                                            bytes, start_preview, end_preview, tr.tool_call_id
                                        );
                                    }
                                } else {
                                    tr.content = format!(
                                        "[Observation Masked to save context. Output was {} bytes. The tool call itself remains visible. Use 'RecallObservation' with ID '{}' if you need the full output again.]",
                                        bytes, tr.tool_call_id
                                    );
                                }
                            } else {
                                // Apply PII redaction if we're not truncating, since the full output will be visible
                                tr.content = redact_sensitive_content(&tr.content);
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
    fn test_redact_sensitive_content() {
        let input = "Contact us at support@example.com or call 1-800-555-1234. Another email: user.name+tag@domain.co.uk. Another phone: (555) 123-4567.";
        let redacted = redact_sensitive_content(input);
        assert!(!redacted.contains("support@example.com"));
        assert!(!redacted.contains("user.name+tag@domain.co.uk"));
        assert!(!redacted.contains("1-800-555-1234"));
        assert!(!redacted.contains("(555) 123-4567"));

        assert!(redacted.contains("[REDACTED EMAIL]"));
        assert!(redacted.contains("[REDACTED PHONE]"));
    }

    #[test]
    fn test_line_based_truncation_heuristic() {
        let mut messages = vec![
            Message {
                role: Role::Tool,
                content: String::new(),
                tool_calls: vec![],
                tool_results: vec![
                    ToolResult {
                        tool_call_id: "call_lines".to_string(),
                        content: "Line 1\nLine 2\nLine 3\nLine 4\nLine 5\nLine 6\nLine 7\nLine 8\nLine 9\nLine 10".to_string(),
                        error: String::new(),
                    },
                ],
                response_id: None,
                previous_response_id: None,
            },
            Message {
                role: Role::Assistant,
                content: "Padding".to_string(),
                tool_calls: vec![],
                tool_results: vec![],
                response_id: None,
                previous_response_id: None,
            },
        ];

        // Mask messages older than 0 from the end. Size limit 10 bytes.
        apply_observation_masking(&mut messages, 0, 10);

        let masked_content = &messages[0].tool_results[0].content;
        assert!(masked_content.contains("[Observation Masked"));
        // Check if it used line-based truncation
        assert!(masked_content.contains("Line 1\nLine 2"));
        assert!(masked_content.contains("Line 9\nLine 10"));
        assert!(!masked_content.contains("Line 5"));
    }
}
