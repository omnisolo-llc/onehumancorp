use crate::agent::AgentRunConfig;

/// A dedicated builder for the Hierarchical Priority Stack mechanic.
/// This fulfills the Master Catalog specification:
/// 1. Server-controlled System Message (Highest Priority)
/// 2. Tool Definitions
/// 3. Developer Instructions
/// 4. User Instructions (capped at 32 KiB)
pub(crate) struct HierarchicalPromptBuilder {
    server_system_message: String,
    tool_definitions: String,
    developer_instructions: String,
    user_instructions: String,
    enable_lost_in_the_middle_prevention: bool,
}

impl HierarchicalPromptBuilder {
    pub fn new(cfg: &AgentRunConfig, tools: &[crate::tools::Tool]) -> Self {
        let mut tool_defs = String::new();
        if !tools.is_empty() {
            for tool in tools {
                tool_defs.push_str(&format!("Tool: {}\n", tool.name));
                tool_defs.push_str(&format!("Description: {}\n", tool.description));
                tool_defs.push_str(&format!("Parameters: {}\n", tool.parameters));
            }
            tool_defs.pop(); // Remove trailing newline
        }

        let mut end_idx = 32768;
        if cfg.user_instructions.len() > 32768 {
            while end_idx > 0 && !cfg.user_instructions.is_char_boundary(end_idx) {
                end_idx -= 1;
            }
        } else {
            end_idx = cfg.user_instructions.len();
        }
        let user_instr = cfg.user_instructions[..end_idx].to_string();

        Self {
            server_system_message: cfg.server_system_message.clone(),
            tool_definitions: tool_defs,
            developer_instructions: cfg.developer_instructions.clone(),
            user_instructions: user_instr,
            enable_lost_in_the_middle_prevention: cfg.enable_lost_in_the_middle_prevention,
        }
    }

    pub fn build(&self) -> String {
        let mut combined_system = String::new();

        // 1. Server-controlled System Message (Highest Priority)
        if !self.server_system_message.is_empty() {
            combined_system.push_str("[Server System Message]\n");
            combined_system.push_str(&self.server_system_message);
        }

        // 2. Tool Definitions
        if !self.tool_definitions.is_empty() {
            if !combined_system.is_empty() {
                combined_system.push_str("\n\n");
            }
            combined_system.push_str("[Tool Definitions]\n");
            combined_system.push_str(&self.tool_definitions);
        }

        // 3. Developer Instructions
        if !self.developer_instructions.is_empty() {
            if !combined_system.is_empty() {
                combined_system.push_str("\n\n");
            }
            combined_system.push_str("[Developer Instructions]\n");
            combined_system.push_str(&self.developer_instructions);
        }

        // 4. User Instructions
        if !self.user_instructions.is_empty() {
            if !combined_system.is_empty() {
                combined_system.push_str("\n\n");
            }
            combined_system.push_str("[User Instructions]\n");
            combined_system.push_str(&self.user_instructions);
        }

        // 5. Conversation History (happens at run loop outside this builder)

        // Lost in the Middle prevention: High-signal context at the very beginning and very end
        if self.enable_lost_in_the_middle_prevention {
            if !self.server_system_message.is_empty() {
                if !combined_system.is_empty() {
                    combined_system.push_str("\n\n");
                }
                combined_system.push_str("[CRITICAL REMINDER: High-Signal Context Repeated to prevent 'Lost in the Middle']\n");
                combined_system.push_str(&self.server_system_message);
            }
        }

        combined_system
    }
}

pub(crate) fn build_hierarchical_system_prompt(cfg: &AgentRunConfig, tools: &[crate::tools::Tool]) -> String {
    HierarchicalPromptBuilder::new(cfg, tools).build()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::agent::AgentRunConfig;
    use ohc_builtin_agent_tools::Tool;
    use serde_json::json;
    use std::sync::Arc;

    // Helper dummy tool executor
    struct DummyToolExecutor;
    #[async_trait::async_trait]
    impl ohc_builtin_agent_tools::ToolExecutor for DummyToolExecutor {
        async fn execute(&self, _args: serde_json::Value) -> Result<String, ohc_builtin_agent_core::types::ToolError> {
            Ok("".to_string())
        }
    }

    #[test]
    fn test_hierarchical_prompt_builder_basic() {
        let mut cfg = AgentRunConfig::default();
        cfg.server_system_message = "Server message".to_string();
        cfg.developer_instructions = "Dev instructions".to_string();
        cfg.user_instructions = "User instructions".to_string();
        cfg.enable_lost_in_the_middle_prevention = false;

        let tool = Tool {
            name: "test_tool".to_string(),
            description: "A test tool".to_string(),
            parameters: json!({"type": "object"}),
            is_read_only: true,
            execute: Arc::new(DummyToolExecutor),
        };

        let builder = HierarchicalPromptBuilder::new(&cfg, &[tool]);
        let prompt = builder.build();

        assert!(prompt.contains("[Server System Message]\nServer message"));
        assert!(prompt.contains("[Tool Definitions]\nTool: test_tool\nDescription: A test tool"));
        assert!(prompt.contains("[Developer Instructions]\nDev instructions"));
        assert!(prompt.contains("[User Instructions]\nUser instructions"));
        assert!(!prompt.contains("[CRITICAL REMINDER"));
    }

    #[test]
    fn test_lost_in_the_middle_prevention() {
        let mut cfg = AgentRunConfig::default();
        cfg.server_system_message = "Critical info".to_string();
        cfg.enable_lost_in_the_middle_prevention = true;

        let builder = HierarchicalPromptBuilder::new(&cfg, &[]);
        let prompt = builder.build();

        assert!(prompt.starts_with("[Server System Message]\nCritical info"));
        assert!(prompt.ends_with("[CRITICAL REMINDER: High-Signal Context Repeated to prevent 'Lost in the Middle']\nCritical info"));
    }

    #[test]
    fn test_user_instructions_truncation() {
        let mut cfg = AgentRunConfig::default();

        // Create a string that is exactly 32768 bytes
        let mut long_instr = "a".repeat(32768);
        // Add more bytes to exceed the limit
        long_instr.push_str("bbbb");
        cfg.user_instructions = long_instr;

        let builder = HierarchicalPromptBuilder::new(&cfg, &[]);
        let prompt = builder.build();

        // Check it contains the user instructions block
        assert!(prompt.contains("[User Instructions]\n"));
        // The builder should extract exactly up to 32768 bytes.
        let lines: Vec<&str> = prompt.split("[User Instructions]\n").collect();
        assert_eq!(lines[1].len(), 32768);
        assert!(lines[1].ends_with('a'));
        assert!(!lines[1].contains("bbbb"));
    }
}
