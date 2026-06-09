use crate::agent::AgentRunConfig;
use crate::types::Message;
/// Master Catalog B.5. Prompt Construction
use std::fmt::Write;

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
                    if recent_errors.is_empty() {
                        String::new()
                    } else {
                        format!(" | Recent Tool Errors: {}", recent_errors)
                    }
                );

                if !developer_instructions.is_empty() {
                    reminder_text.push_str(&format!(
                        "\n\n[Developer Instructions Reminder: {}]",
                        developer_instructions
                    ));
                }

                final_messages.push(Message::user(reminder_text));
            } else if !developer_instructions.is_empty() {
                final_messages.push(Message::user(format!(
                    "[Developer Instructions Reminder: {}]",
                    developer_instructions
                )));
            }
        } else if !developer_instructions.is_empty() {
            final_messages.push(Message::user(format!(
                "[Developer Instructions Reminder: {}]",
                developer_instructions
            )));
        }
    }

    fn extract_core_objective(user_instructions: &str) -> String {
        let first_paragraph = user_instructions
            .split("\n\n")
            .next()
            .unwrap_or(user_instructions);
        if first_paragraph.len() > 1000 {
            let mut end_idx = 997;
            while end_idx > 0 && !first_paragraph.is_char_boundary(end_idx) {
                end_idx -= 1;
            }
            format!("{}...", &first_paragraph[..end_idx])
        } else {
            first_paragraph.to_string()
        }
    }

    fn summarize_recent_tool_errors(messages: &[Message]) -> String {
        let mut error_summary = String::new();
        // Look at the last 5 messages for tool errors
        for msg in messages.iter().rev().take(5) {
            if msg.role == crate::types::Role::User && msg.content.to_lowercase().contains("error")
            {
                if !error_summary.is_empty() {
                    error_summary.push_str(", ");
                }
                let err_msg = if msg.content.len() > 100 {
                    let mut end_idx = 97;
                    while end_idx > 0 && !msg.content.is_char_boundary(end_idx) {
                        end_idx -= 1;
                    }
                    format!("{}...", &msg.content[..end_idx])
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
            let mut contents = Vec::new();
            let mut current_dir = std::env::current_dir().ok();
            while let Some(dir) = current_dir {
                let agents_file = dir.join("AGENTS.md");
                if let Ok(content) = std::fs::read_to_string(&agents_file) {
                    contents.push(content);
                }
                current_dir = dir.parent().map(|p| p.to_path_buf());
            }

            let mut combined_agents_md = String::new();
            let limit = 32768;

            // Prioritize more deeply nested files.
            // `contents` has deepest first because we started at current_dir and went up.
            for content in contents {
                if combined_agents_md.is_empty() {
                    combined_agents_md.push_str(&content);
                } else {
                    let addition = format!("\n\n{}", content);
                    combined_agents_md.push_str(&addition);
                }

                if combined_agents_md.len() > limit {
                    let mut end_idx = limit;
                    while end_idx > 0 && !combined_agents_md.is_char_boundary(end_idx) {
                        end_idx -= 1;
                    }
                    combined_agents_md.truncate(end_idx);
                    combined_agents_md.push_str("\n... [AGENTS.md TRUNCATED TO 32KiB]");
                    break;
                }
            }

            if !combined_agents_md.is_empty() {
                source_name = "AGENTS.md";
            }
            combined_agents_md
        } else {
            source_name = "User Instructions";
            cfg.user_instructions.clone()
        };

        if source_name == "User Instructions" && user_instr.len() > 32768 {
            let mut end_idx = 32768;
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

    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_cascading_agents_md_truncation() {
        let dir = tempdir().unwrap();

        let root_dir = dir.path().join("root");
        let child_dir = root_dir.join("child");
        let grandchild_dir = child_dir.join("grandchild");

        fs::create_dir_all(&grandchild_dir).unwrap();

        // Root is very long, but lowest priority (read last)
        let root_content = "A".repeat(20000);
        fs::write(root_dir.join("AGENTS.md"), &root_content).unwrap();

        // Child is medium
        let child_content = "B".repeat(10000);
        fs::write(child_dir.join("AGENTS.md"), &child_content).unwrap();

        // Grandchild is highest priority (read first)
        let grandchild_content = "C".repeat(10000);
        fs::write(grandchild_dir.join("AGENTS.md"), &grandchild_content).unwrap();

        // Total is 40,000 bytes > 32,768 bytes.
        // It reads grandchild first, then child, then root.
        // So combined is: grandchild + child + root.
        // Thus, "C"s should all be there, "B"s should all be there, and "A"s will be truncated.

        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(&grandchild_dir).unwrap();

        let cfg = AgentRunConfig::default();
        let builder = HierarchicalPromptBuilder::new(&cfg, &[]);
        let built = builder.build();

        std::env::set_current_dir(original_dir).unwrap();

        assert!(
            built.contains(&grandchild_content),
            "Grandchild content (highest priority) should be fully present"
        );
        assert!(
            built.contains(&child_content),
            "Child content should be fully present"
        );

        // Root content should be partially present and truncated.
        assert!(built.contains("AAAA"), "Should contain some of root");
        assert!(
            built.contains("[AGENTS.md TRUNCATED TO 32KiB]"),
            "Should contain truncation warning"
        );

        // Total size of user instructions part should be around 32,768 + length of the truncation warning message.
        // Let's just check the length of the string `built`. It includes the headers "[User Instructions]\n".
        let user_instructions_section_len = built.len() - "[User Instructions]\n".len();
        assert!(
            user_instructions_section_len <= 33000,
            "Output should be bounded to around 32KiB + padding"
        );
    }

    #[test]
    fn test_cascading_agents_md_truncation_char_boundary() {
        let dir = tempdir().unwrap();
        let root_dir = dir.path().join("root");
        fs::create_dir_all(&root_dir).unwrap();

        // Exactly 32,766 bytes of "A", plus a 4-byte emoji "😊"
        // Total size = 32770 bytes. The 32768 limit falls right in the middle of the emoji.
        let mut content = "A".repeat(32766);
        content.push_str("😊");
        fs::write(root_dir.join("AGENTS.md"), &content).unwrap();

        let original_dir = std::env::current_dir().unwrap();
        std::env::set_current_dir(&root_dir).unwrap();

        let cfg = AgentRunConfig::default();
        let builder = HierarchicalPromptBuilder::new(&cfg, &[]);
        let built = builder.build();

        std::env::set_current_dir(original_dir).unwrap();

        assert!(built.contains("[AGENTS.md TRUNCATED TO 32KiB]"));
        // The emoji should be completely stripped (meaning we go back to 32766)
        assert!(
            !built.contains("😊"),
            "Emoji should be stripped to respect char boundary"
        );
        assert!(
            built.contains(&"A".repeat(32766)),
            "Preceding ASCII should remain intact"
        );
    }

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
        assert!(
            last_msg
                .content
                .contains("[Developer Instructions Reminder: Use Rust and be efficient.]")
        );
    }

    #[test]
    fn test_apply_lost_in_the_middle_prevention_with_errors() {
        let mut messages = vec![
            Message::user("Message 1"),
            Message::assistant("Message 2"),
            Message::user("There was an error: file not found"),
            Message::assistant("Message 4"),
        ];

        PromptBuilder::apply_lost_in_the_middle_prevention(&mut messages, true, "", "Objective");

        let last_msg = &messages[4];
        assert!(
            last_msg
                .content
                .contains("Recent Tool Errors: There was an error: file not found")
        );
    }

    #[test]
    fn test_apply_lost_in_the_middle_prevention_short_conversation() {
        let mut messages = vec![Message::user("Message 1")];

        PromptBuilder::apply_lost_in_the_middle_prevention(
            &mut messages,
            true,
            "Dev rules",
            "User rule",
        );

        assert_eq!(messages.len(), 2);
        let last_msg = &messages[1];
        assert_eq!(last_msg.role, Role::User);
        assert_eq!(
            last_msg.content,
            "[Developer Instructions Reminder: Dev rules]"
        );
    }

    #[test]
    fn test_extract_core_objective_char_boundary() {
        let user_instructions = "A".repeat(995) + "😊"; // '😊' is 4 bytes. 995 + 4 = 999 bytes. Total string is < 1000 so no truncation.
        let obj = PromptBuilder::extract_core_objective(&user_instructions);
        assert_eq!(obj, user_instructions);

        let long_user_instructions = "A".repeat(995) + "😊" + "BC"; // 1001 bytes.
        let obj2 = PromptBuilder::extract_core_objective(&long_user_instructions);

        // 997 is in the middle of '😊' (bytes 995..999).
        // `is_char_boundary` should walk back to 995.
        let expected = "A".repeat(995) + "...";
        assert_eq!(obj2, expected);
    }

    #[test]
    fn test_summarize_recent_tool_errors_char_boundary() {
        let error_msg = "error: ".to_string() + &"A".repeat(88) + "😊"; // 7 + 88 + 4 = 99 bytes
        let error_msg = error_msg + "BC"; // 101 bytes
        let messages = vec![Message::user(error_msg)];

        let summary = PromptBuilder::summarize_recent_tool_errors(&messages);

        // 97 is inside the '😊' (bytes 95..99). Walk back to 95.
        let expected = "error: ".to_string() + &"A".repeat(88) + "...";
        assert_eq!(summary, expected);
    }
}
