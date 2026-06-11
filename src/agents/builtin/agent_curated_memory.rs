use crate::types::Message;

/// Hermes Agent Unique Harness Innovations: Agent-curated memory
/// Periodic nudges, autonomous skill creation after complex tasks.
pub struct MemoryCurator {
    pub nudge_threshold: usize,
}

impl Default for MemoryCurator {
    fn default() -> Self {
        Self {
            nudge_threshold: 5,
        }
    }
}

impl MemoryCurator {
    pub fn new(nudge_threshold: usize) -> Self {
        Self { nudge_threshold }
    }

    /// Evaluates if a given transcript represents a "complex task".
    /// We consider a task complex if it spans multiple tool calls/turns.
    pub fn is_complex_task(&self, messages: &[Message]) -> bool {
        let tool_call_count = messages.iter().filter(|m| !m.tool_calls.is_empty()).count();
        tool_call_count >= self.nudge_threshold
    }

    /// Analyzes the conversation transcript and suggests an autonomous skill to create.
    /// This represents the "autonomous skill creation" part of Agent-curated memory.
    pub fn autonomously_suggest_skill(&self, messages: &[Message]) -> Option<String> {
        if !self.is_complex_task(messages) {
            return None;
        }

        // Extremely simplified heuristic for skill suggestion based on tools used
        let mut used_tools = std::collections::HashSet::new();
        for msg in messages {
            for tc in &msg.tool_calls {
                used_tools.insert(tc.name.clone());
            }
        }

        if used_tools.contains("bash") && used_tools.contains("edit_file") {
            Some("BashEditorSkill: A reusable skill for safely running bash commands and modifying files.".to_string())
        } else if used_tools.contains("read_file") && used_tools.contains("grep") {
            Some("LogAnalyzerSkill: A reusable skill for parsing and analyzing log files.".to_string())
        } else if !used_tools.is_empty() {
            let tools: Vec<_> = used_tools.into_iter().collect();
            Some(format!("CustomSkill: Reusable skill combining {}.", tools.join(", ")))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Role, ToolCall};

    fn create_mock_message_with_tool(tool_name: &str) -> Message {
        Message {
            role: Role::Assistant,
            content: "".to_string(),
            tool_calls: vec![ToolCall {
                id: "test".to_string(),
                name: tool_name.to_string(),
                arguments: serde_json::json!({}),
            }],
            tool_results: vec![],
            response_id: None,
            previous_response_id: None,
        }
    }

    #[test]
    fn test_is_complex_task() {
        let curator = MemoryCurator::new(2);

        let mut msgs = vec![create_mock_message_with_tool("bash")];
        assert!(!curator.is_complex_task(&msgs));

        msgs.push(create_mock_message_with_tool("edit_file"));
        assert!(curator.is_complex_task(&msgs));
    }

    #[test]
    fn test_autonomously_suggest_skill() {
        let curator = MemoryCurator::new(2);
        let msgs = vec![
            create_mock_message_with_tool("bash"),
            create_mock_message_with_tool("edit_file"),
        ];

        let suggestion = curator.autonomously_suggest_skill(&msgs);
        assert!(suggestion.is_some());
        assert!(suggestion.unwrap().contains("BashEditorSkill"));
    }
}
