use std::path::{Path};
use std::fs;

/// A dedicated builder for the Hierarchical Priority Stack mechanic.
/// This fulfills the Master Catalog specification (Prompt Construction: OpenAI Codex Mechanic):
/// 1. Server-controlled System Message (Highest Priority)
/// 2. Tool Definitions
/// 3. Developer Instructions
/// 4. User Instructions (cascading `AGENTS.md` files, capped at 32 KiB)
/// 5. Conversation History
/// Important: Put high-signal context at the very beginning and very end (combatting "Lost in the Middle").
pub struct HierarchicalPromptStack {
    server_system_message: String,
    tool_definitions: String,
    developer_instructions: String,
    user_instructions: String,
    enable_lost_in_the_middle_prevention: bool,
}

impl HierarchicalPromptStack {
    pub fn new(
        server_system_message: String,
        developer_instructions: String,
        user_instructions: String,
        tools: &[crate::tools::Tool],
        enable_lost_in_the_middle_prevention: bool,
    ) -> Self {
        let mut tool_defs = String::new();
        if !tools.is_empty() {
            for tool in tools {
                tool_defs.push_str(&format!("Tool: {}\n", tool.name));
                tool_defs.push_str(&format!("Description: {}\n", tool.description));
                let params_str = serde_json::to_string_pretty(&tool.parameters).unwrap_or_default();
                tool_defs.push_str(&format!("Parameters: {}\n\n", params_str));
            }
        }

        Self {
            server_system_message,
            tool_definitions: tool_defs,
            developer_instructions,
            user_instructions,
            enable_lost_in_the_middle_prevention,
        }
    }

    /// Recursively scan upwards from the given directory to collect AGENTS.md files.
    /// Reverses the collection so that the most deeply-nested AGENTS.md files are evaluated last (highest precedence).
    fn collect_agents_md(start_dir: &Path) -> String {
        let mut agents_content = Vec::new();
        let mut current_dir = Some(start_dir);

        while let Some(dir) = current_dir {
            let agents_file = dir.join("AGENTS.md");
            if agents_file.exists() && agents_file.is_file() {
                if let Ok(content) = fs::read_to_string(&agents_file) {
                    agents_content.push(format!("--- AGENTS.md ({}) ---\n{}\n", agents_file.display(), content));
                }
            }
            current_dir = dir.parent();
        }

        agents_content.reverse();

        let mut combined = agents_content.join("\n");

        // Cap at 32 KiB
        let max_size = 32 * 1024;
        if combined.len() > max_size {
            combined.truncate(max_size);
            combined.push_str("\n... [AGENTS.md truncated to 32 KiB limit]");
        }

        combined
    }

    pub fn build(&self, workspace_path: Option<&Path>) -> String {
        let mut combined = String::new();

        // 1. Server-controlled System Message (Highest Priority - Start)
        if !self.server_system_message.is_empty() {
            combined.push_str(&self.server_system_message);
            combined.push_str("\n\n");
        }

        // 2. Tool Definitions
        if !self.tool_definitions.is_empty() {
            combined.push_str("--- Available Tools ---\n");
            combined.push_str(&self.tool_definitions);
            combined.push_str("\n");
        }

        // 3. Developer Instructions
        if !self.developer_instructions.is_empty() {
            combined.push_str("--- Developer Instructions ---\n");
            combined.push_str(&self.developer_instructions);
            combined.push_str("\n\n");
        }

        // 4. User Instructions & AGENTS.md
        combined.push_str("--- User Instructions ---\n");
        if !self.user_instructions.is_empty() {
            combined.push_str(&self.user_instructions);
            combined.push_str("\n\n");
        }

        if let Some(path) = workspace_path {
            let agents_md_content = Self::collect_agents_md(path);
            if !agents_md_content.is_empty() {
                combined.push_str("--- Workspace AGENTS.md Context ---\n");
                combined.push_str(&agents_md_content);
                combined.push_str("\n\n");
            }
        }

        // 5. Reinforce highest priority context at the very end to combat "Lost in the Middle"
        if self.enable_lost_in_the_middle_prevention && !self.server_system_message.is_empty() {
            combined.push_str("--- CRITICAL REMINDER (Server System Message) ---\n");
            combined.push_str(&self.server_system_message);
            combined.push_str("\n");
        }

        combined
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs;

    #[test]
    fn test_hierarchical_prompt_stack_no_agents_md() {
        let tools = vec![];
        let stack = HierarchicalPromptStack::new(
            "Server sys".to_string(),
            "Dev sys".to_string(),
            "User sys".to_string(),
            &tools,
            true,
        );

        let prompt = stack.build(None);
        assert!(prompt.contains("Server sys"));
        assert!(prompt.contains("Dev sys"));
        assert!(prompt.contains("User sys"));
        assert!(prompt.contains("CRITICAL REMINDER"));
        assert!(!prompt.contains("Workspace AGENTS.md Context"));
    }

    #[test]
    fn test_hierarchical_prompt_stack_with_agents_md() {
        let tools = vec![];
        let stack = HierarchicalPromptStack::new(
            "Server sys".to_string(),
            "Dev sys".to_string(),
            "User sys".to_string(),
            &tools,
            false,
        );

        let dir = tempdir().unwrap();
        let sub_dir = dir.path().join("sub");
        fs::create_dir(&sub_dir).unwrap();

        fs::write(dir.path().join("AGENTS.md"), "Root Agent Context").unwrap();
        fs::write(sub_dir.join("AGENTS.md"), "Sub Agent Context").unwrap();

        let prompt = stack.build(Some(&sub_dir));
        assert!(prompt.contains("Workspace AGENTS.md Context"));
        assert!(prompt.contains("Root Agent Context"));
        assert!(prompt.contains("Sub Agent Context"));

        // Ensure reverse collection order (sub agent is AFTER root agent)
        let root_idx = prompt.find("Root Agent Context").unwrap();
        let sub_idx = prompt.find("Sub Agent Context").unwrap();
        assert!(root_idx < sub_idx);
    }

    #[test]
    fn test_hierarchical_prompt_stack_truncates_agents_md() {
        let tools = vec![];
        let stack = HierarchicalPromptStack::new(
            "".to_string(),
            "".to_string(),
            "".to_string(),
            &tools,
            false,
        );

        let dir = tempdir().unwrap();
        // Create an AGENTS.md that is > 32 KiB
        let large_content = "A".repeat(40 * 1024);
        fs::write(dir.path().join("AGENTS.md"), large_content).unwrap();

        let prompt = stack.build(Some(dir.path()));
        assert!(prompt.contains("[AGENTS.md truncated to 32 KiB limit]"));

        // Length should be 32KiB + length of headers/truncation messages
        assert!(prompt.len() < 35 * 1024);
    }
}
