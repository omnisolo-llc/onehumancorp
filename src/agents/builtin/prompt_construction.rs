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

        // Prompt Construction: OpenAI Codex Mechanic
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

        // 4. User Instructions (cascading AGENTS.md files, capped at 32 KiB)
        if !self.user_instructions.is_empty() {
            if !combined_system.is_empty() {
                combined_system.push_str("\n\n");
            }
            combined_system.push_str("[User Instructions]\n");
            combined_system.push_str(&self.user_instructions);
        }

        if self.enable_lost_in_the_middle_prevention && !self.server_system_message.is_empty() {
            if !combined_system.is_empty() {
                combined_system.push_str("\n\n");
            }
            combined_system.push_str("[CRITICAL REMINDER: High-Signal Context Repeated to prevent 'Lost in the Middle']\n");
            combined_system.push_str(&self.server_system_message);
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

    #[test]
    fn test_hierarchical_prompt_builder() {
        let mut cfg = AgentRunConfig::default();
        cfg.server_system_message = "CRITICAL: Never delete the database.".to_string();
        cfg.developer_instructions = "Use standard libraries.".to_string();
        cfg.user_instructions = "Please calculate 2+2".to_string();
        cfg.enable_lost_in_the_middle_prevention = true;

        let tools = vec![];
        let builder = HierarchicalPromptBuilder::new(&cfg, &tools);
        let prompt = builder.build();

        assert!(prompt.starts_with("[Server System Message]
CRITICAL: Never delete the database."));
        assert!(prompt.contains("[CRITICAL REMINDER: High-Signal Context Repeated to prevent 'Lost in the Middle']
CRITICAL: Never delete the database."));
        assert!(prompt.ends_with("CRITICAL: Never delete the database."));
    }

    #[test]
    fn test_lost_in_the_middle_prevention_disabled() {
        let mut cfg = AgentRunConfig::default();
        cfg.server_system_message = "CRITICAL: Never delete the database.".to_string();
        cfg.developer_instructions = "Use standard libraries.".to_string();
        cfg.user_instructions = "Please calculate 2+2".to_string();
        cfg.enable_lost_in_the_middle_prevention = false;

        let tools = vec![];
        let builder = HierarchicalPromptBuilder::new(&cfg, &tools);
        let prompt = builder.build();

        assert!(prompt.starts_with("[Server System Message]
CRITICAL: Never delete the database."));
        assert!(!prompt.contains("[CRITICAL REMINDER: High-Signal Context Repeated to prevent 'Lost in the Middle']"));
    }
}
