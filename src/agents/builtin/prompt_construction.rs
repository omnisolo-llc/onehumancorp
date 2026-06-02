use crate::types::Message;
use std::path::Path;
use tokio::fs;

/// Prompt Construction: OpenAI Codex Mechanic
/// 1. Server-controlled System Message (Highest Priority)
/// 2. Tool Definitions
/// 3. Developer Instructions
/// 4. User Instructions (cascading AGENTS.md files, capped at 32 KiB)
/// 5. Conversation History (happens at run loop)

pub struct PromptBuilder;

impl PromptBuilder {
    /// Assembles the complete prompt stack following the strict OpenAI Codex hierarchical priority stack.
    /// Returns the assembled System Message string and the final conversation history.
    pub fn build_prompt_stack(
        server_system_message: &str,
        tool_definitions: &str,
        developer_instructions: &str,
        user_instructions: &str,
        mut conversation_history: Vec<Message>,
        enable_litm_prevention: bool,
    ) -> (String, Vec<Message>) {
        let mut final_system_message = String::new();

        // 1. Server-controlled System Message (Highest Priority)
        if !server_system_message.is_empty() {
            final_system_message.push_str(server_system_message);
            final_system_message.push_str("\n\n");
        }

        // 2. Tool Definitions
        if !tool_definitions.is_empty() {
            final_system_message.push_str("=== Tool Definitions ===\n");
            final_system_message.push_str(tool_definitions);
            final_system_message.push_str("\n========================\n\n");
        }

        // 3. Developer Instructions
        if !developer_instructions.is_empty() {
            final_system_message.push_str("=== Developer Instructions ===\n");
            final_system_message.push_str(developer_instructions);
            final_system_message.push_str("\n==============================\n\n");
        }

        // 4. User Instructions (cascading AGENTS.md files, capped at 32 KiB)
        if !user_instructions.is_empty() {
            final_system_message.push_str("=== User Instructions (AGENTS.md) ===\n");
            final_system_message.push_str(user_instructions);
            final_system_message.push_str("\n=====================================\n\n");
        }

        let final_system_message = final_system_message.trim_end().to_string();

        // 5. Conversation History (Lost in the Middle prevention)
        Self::apply_lost_in_the_middle_prevention(
            &mut conversation_history,
            enable_litm_prevention,
            developer_instructions,
            user_instructions,
        );

        (final_system_message, conversation_history)
    }

    pub async fn load_cascading_agents_md(start_dir: &Path) -> String {
        let mut current_dir = start_dir.to_path_buf();
        let mut contents: Vec<String> = Vec::new();
        let mut max_depth = 50;

        loop {
            let agent_file = current_dir.join("AGENTS.md");
            if agent_file.exists() && agent_file.is_file() {
                if let Ok(content) = fs::read_to_string(&agent_file).await {
                    contents.push(content);
                }
            }

            if !current_dir.pop() || max_depth == 0 {
                break;
            }
            max_depth -= 1;
        }

        // Order: more deeply-nested files take precedence
        let mut combined = String::new();
        for (i, content) in contents.iter().enumerate() {
            if i > 0 {
                combined.push_str("\n\n---\n\n");
            }
            combined.push_str(content);
        }

        let max_bytes = 32 * 1024;
        if combined.len() > max_bytes {
            let mut end_idx = max_bytes;
            while end_idx > 0 && !combined.is_char_boundary(end_idx) {
                end_idx -= 1;
            }
            combined.truncate(end_idx);
            combined.push_str("\n\n[System: AGENTS.md content truncated to 32KiB limit.]");
        }

        combined
    }

    /// Prompt Construction Mechanic: "Lost in the Middle" Prevention
    /// High-signal context at the very beginning and very end.
    pub fn apply_lost_in_the_middle_prevention(
        final_messages: &mut Vec<Message>,
        enable_prevention: bool,
        developer_instructions: &str,
        user_instructions: &str,
    ) {
        if enable_prevention {
            let mut reminder_text = String::new();
            if !developer_instructions.is_empty() {
                reminder_text.push_str(&format!("[System Reminder: {}]\n\n", developer_instructions));
            }
            if !user_instructions.is_empty() && final_messages.len() > 3 {
                // Truncate user instructions if it's too long, just to remind the core objective
                let mut end_idx = 1000;
                if user_instructions.len() > 1000 {
                    while end_idx > 0 && !user_instructions.is_char_boundary(end_idx) {
                        end_idx -= 1;
                    }
                } else {
                    end_idx = user_instructions.len();
                }
                let summary = &user_instructions[..end_idx];
                reminder_text.push_str(&format!("[System Reminder to combat 'Lost in the Middle' effect: Remember your core objective: {}...]", summary));
            }

            if !reminder_text.is_empty() {
                final_messages.push(Message::user(reminder_text.trim()));
            }
        } else if !developer_instructions.is_empty() {
            final_messages.push(Message::user(format!("[System Reminder: {}]", developer_instructions)));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_build_prompt_stack_hierarchy() {
        let server_sys = "You are an AI assistant.";
        let tools = "Tool A: prints a.\nTool B: prints b.";
        let dev_inst = "Never use markdown.";
        let user_inst = "Build a webpage.";
        let conv_history = vec![Message::user("Hello")];

        let (final_sys, final_conv) = PromptBuilder::build_prompt_stack(
            server_sys,
            tools,
            dev_inst,
            user_inst,
            conv_history,
            false,
        );

        // Verify order and presence
        assert!(final_sys.starts_with(server_sys));
        assert!(final_sys.contains("=== Tool Definitions ==="));
        assert!(final_sys.contains(tools));
        assert!(final_sys.contains("=== Developer Instructions ==="));
        assert!(final_sys.contains(dev_inst));
        assert!(final_sys.contains("=== User Instructions (AGENTS.md) ==="));
        assert!(final_sys.contains(user_inst));

        // Ensure server_sys is before tools
        let pos_sys = final_sys.find(server_sys).unwrap();
        let pos_tools = final_sys.find("=== Tool Definitions ===").unwrap();
        let pos_dev = final_sys.find("=== Developer Instructions ===").unwrap();
        let pos_user = final_sys.find("=== User Instructions (AGENTS.md) ===").unwrap();

        assert!(pos_sys < pos_tools);
        assert!(pos_tools < pos_dev);
        assert!(pos_dev < pos_user);

        // LitM disabled, but dev instructions should be appended as a system reminder
        assert_eq!(final_conv.len(), 2);
        assert_eq!(final_conv[0].content, "Hello");
        assert!(final_conv[1].content.contains("[System Reminder: Never use markdown.]"));
    }

    #[test]
    fn test_apply_lost_in_the_middle_prevention() {
        let mut messages = vec![
            Message::user("Message 1"),
            Message::user("Message 2"),
            Message::user("Message 3"),
            Message::user("Message 4"),
        ];

        PromptBuilder::apply_lost_in_the_middle_prevention(
            &mut messages,
            true,
            "Dev rules",
            "User instructions that are short",
        );

        assert_eq!(messages.len(), 5);
        assert!(messages.last().unwrap().content.contains("Dev rules"));
        assert!(messages.last().unwrap().content.contains("combat 'Lost in the Middle' effect"));
        assert!(messages.last().unwrap().content.contains("User instructions that are short"));
    }

    #[test]
    fn test_apply_lost_in_the_middle_prevention_disabled_but_dev_rules() {
        let mut messages = vec![
            Message::user("Message 1"),
        ];

        PromptBuilder::apply_lost_in_the_middle_prevention(
            &mut messages,
            false,
            "Dev rules only",
            "User instructions",
        );

        assert_eq!(messages.len(), 2);
        assert!(messages.last().unwrap().content.contains("[System Reminder: Dev rules only]"));
        assert!(!messages.last().unwrap().content.contains("combat"));
    }
}
