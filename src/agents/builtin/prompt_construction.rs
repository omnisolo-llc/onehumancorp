use std::fmt::Write;
use crate::agent::AgentRunConfig;
use crate::types::Message;

pub struct PromptBuilder;

impl PromptBuilder {
    pub fn apply_lost_in_the_middle_prevention(
        final_messages: &mut Vec<Message>,
        enable: bool,
        developer_instructions: &str,
        user_instructions: &str,
    ) {
        if enable {
            // If it's a long conversation (>3 messages), re-inject the core user instructions
            // and developer instructions at the very end to prevent context rot.
            if final_messages.len() > 3 {
                // Truncate user instructions safely to a reasonable length (e.g. 1000 bytes or first paragraph)
                let core_objective = Self::extract_core_objective(user_instructions);

                // Summarize recent tool errors if any exist near the end of the context
                let recent_errors = Self::summarize_recent_tool_errors(final_messages);

                let mut reminder_text = format!(
                    "[System Reminder to combat 'Lost in the Middle' effect: Remember your core objective: {}]{}",
                    core_objective,
                    if recent_errors.is_empty() { String::new() } else { format!(" | Recent Tool Errors: {}", recent_errors) }
                );

                if !developer_instructions.is_empty() {
                    reminder_text.push_str(&format!("\n\n[Developer Instructions Reminder: {}]", developer_instructions));
                }

                final_messages.push(Message::user(reminder_text));
            } else if !developer_instructions.is_empty() {
                final_messages.push(Message::user(format!("[Developer Instructions Reminder: {}]", developer_instructions)));
            }
        } else if !developer_instructions.is_empty() {
            final_messages.push(Message::user(format!("[Developer Instructions Reminder: {}]", developer_instructions)));
        }
    }

    fn extract_core_objective(user_instructions: &str) -> String {
        let first_paragraph = user_instructions.split("\n\n").next().unwrap_or(user_instructions);
        if first_paragraph.len() > 1000 {
            format!("{}...", &first_paragraph[..997])
        } else {
            first_paragraph.to_string()
        }
    }

    fn summarize_recent_tool_errors(messages: &[Message]) -> String {
        let mut error_summary = String::new();
        // Look at the last 5 messages for tool errors
        for msg in messages.iter().rev().take(5) {
            if msg.role == crate::types::Role::User && msg.content.to_lowercase().contains("error") {
                if !error_summary.is_empty() {
                    error_summary.push_str(", ");
                }
                let err_msg = if msg.content.len() > 100 {
                    format!("{}...", &msg.content[..97])
                } else {
                    msg.content.clone()
                };
                error_summary.push_str(&err_msg);
            }
        }
        error_summary
    }
}


/// 4. User Instructions (capped at 32 KiB)
pub struct HierarchicalPromptBuilder {
    server_system_message: String,
    tool_definitions: String,
    developer_instructions: String,
    user_instructions: String,
}

impl HierarchicalPromptBuilder {
    pub fn new(cfg: &AgentRunConfig, tools: &[crate::tools::Tool]) -> Self {
        let mut tool_defs = String::new();
        if !tools.is_empty() {
            for tool in tools {
                let _ = write!(tool_defs, "Tool: {}\n", tool.name);
                let _ = write!(tool_defs, "Description: {}\n", tool.description);
                let _ = write!(tool_defs, "Parameters: {}\n", tool.parameters);
            }
            tool_defs.pop(); // Remove trailing newline
        }

        let mut source_name = "User Instructions";
        let mut user_instr = if cfg.user_instructions.is_empty() {
            let mut combined_agents_md = String::new();
            let mut current_dir = std::env::current_dir().ok();
            while let Some(dir) = current_dir {
                let agents_file = dir.join("AGENTS.md");
                if let Ok(content) = std::fs::read_to_string(&agents_file) {
                    if !combined_agents_md.is_empty() {
                        combined_agents_md.insert_str(0, "\n\n");
                    }
                    combined_agents_md.insert_str(0, &content);
                }
                current_dir = dir.parent().map(|p| p.to_path_buf());
            }
            if !combined_agents_md.is_empty() {
                source_name = "AGENTS.md";
            }
            combined_agents_md
        } else {
            source_name = "User Instructions";
            cfg.user_instructions.clone()
        };

        let mut end_idx = 32768;
        if user_instr.len() > 32768 {
            while end_idx > 0 && !user_instr.is_char_boundary(end_idx) {
                end_idx -= 1;
            }
            let truncated = &user_instr[..end_idx];
            user_instr = format!("{}\n... [{} TRUNCATED TO 32KiB]", truncated, source_name);
        }

        Self {
            server_system_message: cfg.server_system_message.clone(),
            tool_definitions: tool_defs,
            developer_instructions: cfg.developer_instructions.clone(),
            user_instructions: user_instr,
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

        // Lost in the Middle prevention is handled by `apply_lost_in_the_middle_prevention`
        // which appends the reminder to the end of the context window.

        combined_system
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Message, Role};

    #[test]
    fn test_apply_lost_in_the_middle_prevention_long_conversation() {
        let mut messages = vec![
            Message::user("Message 1"),
            Message::assistant("Message 2"),
            Message::user("Message 3"),
            Message::assistant("Message 4"),
        ];

        let developer_instructions = "Use Rust and be efficient.";
        let user_instructions = "Build a web server.\n\nMake sure it has logging.";

        PromptBuilder::apply_lost_in_the_middle_prevention(
            &mut messages,
            true,
            developer_instructions,
            user_instructions,
        );

        assert_eq!(messages.len(), 5);
        let last_msg = &messages[4];
        assert_eq!(last_msg.role, Role::User);
        assert!(last_msg.content.contains("[System Reminder to combat 'Lost in the Middle' effect: Remember your core objective: Build a web server.]"));
        assert!(last_msg.content.contains("[Developer Instructions Reminder: Use Rust and be efficient.]"));
    }

    #[test]
    fn test_apply_lost_in_the_middle_prevention_with_errors() {
        let mut messages = vec![
            Message::user("Message 1"),
            Message::assistant("Message 2"),
            Message::user("There was an error: file not found"),
            Message::assistant("Message 4"),
        ];

        PromptBuilder::apply_lost_in_the_middle_prevention(
            &mut messages,
            true,
            "",
            "Objective",
        );

        let last_msg = &messages[4];
        assert!(last_msg.content.contains("Recent Tool Errors: There was an error: file not found"));
    }

    #[test]
    fn test_apply_lost_in_the_middle_prevention_short_conversation() {
        let mut messages = vec![
            Message::user("Message 1"),
        ];

        PromptBuilder::apply_lost_in_the_middle_prevention(
            &mut messages,
            true,
            "Dev rules",
            "User rule",
        );

        assert_eq!(messages.len(), 2);
        let last_msg = &messages[1];
        assert_eq!(last_msg.role, Role::User);
        assert_eq!(last_msg.content, "[Developer Instructions Reminder: Dev rules]");
    }
}
