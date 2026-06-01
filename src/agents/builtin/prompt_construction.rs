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
    pub async fn load_cascading_agents_md(start_dir: &Path) -> String {
        let mut current_dir = start_dir.to_path_buf();
        let mut contents = Vec::new();
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
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_load_cascading_agents_md() {
        let dir = tempdir().unwrap();
        let root = dir.path().join("root");
        let sub1 = root.join("sub1");
        let sub2 = sub1.join("sub2");

        tokio::fs::create_dir_all(&sub2).await.unwrap();

        tokio::fs::write(root.join("AGENTS.md"), "Root Agent").await.unwrap();
        tokio::fs::write(sub2.join("AGENTS.md"), "Sub2 Agent").await.unwrap();

        let result = PromptBuilder::load_cascading_agents_md(&sub2).await;

        // Deeply nested files take precedence, meaning they are appended last or first?
        // Let's check the code: it collects from deepest up to root.
        // Then it joins them starting from deepest (index 0) to root.
        // Wait, the original code:
        // `contents` has sub2 at index 0, and root at index 1.
        // `for content in contents.iter().enumerate()` appends them in order.
        // So deepest is first.
        assert_eq!(result, "Sub2 Agent\n\n---\n\nRoot Agent");
    }

    #[tokio::test]
    async fn test_load_cascading_agents_md_truncation() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("AGENTS.md");

        let large_content = "A".repeat(40 * 1024); // 40 KB
        tokio::fs::write(&file_path, large_content).await.unwrap();

        let result = PromptBuilder::load_cascading_agents_md(dir.path()).await;

        assert!(result.ends_with("[System: AGENTS.md content truncated to 32KiB limit.]"));
        assert!(result.len() <= 32 * 1024 + 100);
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
