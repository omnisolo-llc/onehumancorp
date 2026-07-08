#![allow(clippy::collapsible_if)]
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
                let momentum = Self::summarize_recent_momentum(final_messages);

                let mut reminder_text = format!(
                    "[SYSTEM NOTIFICATION: Context Rot Prevention Anchor]\n\
                     Remember your core objective: {}\n",
                    core_objective
                );

                if !momentum.is_empty() {
                    reminder_text.push_str(&format!("| {}\n", momentum));
                }

                if !recent_errors.is_empty() {
                    reminder_text.push_str(&format!("| Recent Tool Errors: {}\n", recent_errors));
                }

                if !developer_instructions.is_empty() {
                    reminder_text.push_str(&format!(
                        "\n[Developer Instructions Reminder: {}]",
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

    pub(crate) fn extract_core_objective(user_instructions: &str) -> String {
        let first_paragraph = user_instructions
            .split("\n\n")
            .next()
            .unwrap_or(user_instructions);

        // Use character counting rather than byte lengths for robustness
        let char_count = first_paragraph.chars().count();
        if char_count > 1000 {
            let truncated: String = first_paragraph.chars().take(997).collect();
            format!("{}...", truncated)
        } else {
            first_paragraph.to_string()
        }
    }

    fn summarize_recent_momentum(messages: &[Message]) -> String {
        let mut successes = Vec::new();
        let mut successful_ids = std::collections::HashSet::new();

        // Step 1: Collect successful tool call IDs from Tool messages
        for msg in messages.iter().rev().take(10) {
            if msg.role == crate::types::Role::Tool {
                for tr in &msg.tool_results {
                    if tr.error.is_empty()
                        && !tr.content.is_empty()
                        && !tr.content.starts_with("[Observation Masked")
                    {
                        successful_ids.insert(&tr.tool_call_id);
                    }
                }
            }
        }

        // Step 2: Find tool names for those IDs from Assistant messages
        for msg in messages.iter().rev().take(15) {
            if msg.role == crate::types::Role::Assistant {
                for tc in msg.tool_calls.iter().rev() {
                    if successful_ids.contains(&tc.id) && !successes.contains(&tc.name) {
                        successes.push(tc.name.clone());
                    }
                    if successes.len() >= 3 {
                        break;
                    }
                }
            }
            if successes.len() >= 3 {
                break;
            }
        }

        if successes.is_empty() {
            String::new()
        } else {
            successes.reverse();
            format!("Recent Momentum: {}", successes.join(" -> "))
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
                let char_count = msg.content.chars().count();
                let err_msg = if char_count > 100 {
                    let truncated: String = msg.content.chars().take(97).collect();
                    format!("{}...", truncated)
                } else {
                    msg.content.clone()
                };
                error_summary.push_str(&err_msg);
            }
        }
        error_summary
    }
}

/// Industry Standard: Cascading AGENTS.md loader.
/// More deeply-nested files take precedence (are loaded first).
/// Capped at 32 KiB to prevent context explosion.
pub async fn load_cascading_instructions(start_dir: Option<&std::path::Path>) -> String {
    let mut contents = Vec::new();
    let mut current_dir = start_dir
        .map(|p| p.to_path_buf())
        .or_else(|| std::env::current_dir().ok());

    let mut max_depth = 50;
    while let Some(dir) = current_dir {
        let agents_file = dir.join("AGENTS.md");
        if let Ok(content) = tokio::fs::read_to_string(&agents_file).await {
            contents.push(content);
        }
        current_dir = dir.parent().map(|p| p.to_path_buf());
        max_depth -= 1;
        if max_depth == 0 {
            break;
        }
    }

    let mut combined = String::with_capacity(32768 + 1024);
    let limit = 32768; // char limit

    for content in contents {
        if combined.is_empty() {
            combined.push_str(&content);
        } else {
            combined.push_str("\n\n---\n\n");
            combined.push_str(&content);
        }

        if combined.chars().count() > limit {
            let truncated: String = combined.chars().take(limit).collect();
            combined = format!(
                "{}\n\n[System: AGENTS.md content truncated to 32KiB limit.]",
                truncated
            );
            break;
        }
    }

    combined
}

/// 4. User Instructions (capped at 32 KiB)
// Prompt Construction: OpenAI Codex Hierarchy
// This builder implements a strict hierarchical priority stack for prompt components.
pub struct StrictHierarchicalPromptBuilder {
    server_system_message: String,
    tool_definitions: String,
    developer_instructions: String,
    user_instructions: String,
    lightweight_memory_index: Vec<String>,
}

impl StrictHierarchicalPromptBuilder {
    pub fn new(
        cfg: &AgentRunConfig,
        tools: &[crate::tools::Tool],
        cascading_agents_md: Option<String>,
        lightweight_memory_index: Option<Vec<String>>,
    ) -> Self {
        let mut tool_defs = String::new();
        if !tools.is_empty() {
            for tool in tools {
                let _ = writeln!(tool_defs, "Tool: {}", tool.name);
                let _ = writeln!(tool_defs, "Description: {}", tool.description);
                let _ = writeln!(tool_defs, "Parameters: {}", tool.parameters);
            }
            tool_defs.pop(); // Remove trailing newline
        }

        let mut user_instr = cfg.user_instructions.clone();

        // Inject cascading AGENTS.md instructions
        if let Some(agents_md) = cascading_agents_md {
            if !agents_md.is_empty() {
                if !user_instr.is_empty() {
                    user_instr.push_str("\n\n---\n\n");
                }
                user_instr.push_str("[Cascading AGENTS.md Instructions]:\n");
                user_instr.push_str(&agents_md);
            }
        }

        // OpenHands MicroAgents Injection
        if let Ok(repo_dir) = std::env::current_dir() {
            let microagents_dir = repo_dir.join(".openhands").join("microagents");
            if microagents_dir.exists() {
                let mut registry = crate::microagent::MicroAgentRegistry::new();
                let _ = registry.load_from_dir(&microagents_dir);
                let active_microagents = registry.get_active_instructions(&user_instr);
                if !active_microagents.is_empty() {
                    user_instr.push_str(&format!(
                        "\n\n[MicroAgent Instructions]:\n{}!",
                        active_microagents
                    ));
                }
            }
        }
        let limit = 32768;

        if user_instr.chars().count() > limit {
            let truncated: String = user_instr.chars().take(limit).collect();
            user_instr = format!("{}\n... [User Instructions TRUNCATED TO 32KiB]", truncated);
        }

        let mut processed_memory_index = Vec::new();
        if let Some(mut index) = lightweight_memory_index {
            for entry in index.drain(..) {
                let char_count = entry.chars().count();
                if char_count > 150 {
                    let truncated: String = entry.chars().take(147).collect();
                    processed_memory_index.push(format!("{}...", truncated));
                } else {
                    processed_memory_index.push(entry);
                }
            }
        }

        Self {
            server_system_message: cfg.server_system_message.clone(),
            tool_definitions: tool_defs,
            developer_instructions: cfg.developer_instructions.clone(),
            user_instructions: user_instr,
            lightweight_memory_index: processed_memory_index,
        }
    }

    pub fn build(&self) -> String {
        // Omni-Context Injection
        let mut grounding_injection = String::new();
        if let Ok(cwd) = std::env::current_dir() {
            let router = crate::omni_context::OmniContextRouter::new(cwd);
            if let Some(grounding) = router.get_system_grounding() {
                grounding_injection = format!("<omni_context>\n{}\n</omni_context>\n\n", grounding);
            }
        }

        // Pre-allocate capacity to avoid reallocation
        let estimated_capacity = grounding_injection.len()
            + self.server_system_message.len()
            + self.tool_definitions.len()
            + self.developer_instructions.len()
            + self.user_instructions.len()
            + 1024; // buffer for tags and formatting
        let mut combined_system = String::with_capacity(estimated_capacity);

        // 1. Server-controlled System Message (Highest Priority)
        if !self.server_system_message.is_empty() {
            combined_system.push_str(&grounding_injection);
            combined_system.push_str("<server_system_message>\n");
            combined_system.push_str(&self.server_system_message);
            combined_system.push_str("\n</server_system_message>");
        }

        // 2. Tool Definitions
        if !self.tool_definitions.is_empty() {
            if !combined_system.is_empty() {
                combined_system.push_str("\n\n");
            }
            combined_system.push_str("<tool_definitions>\n");
            combined_system.push_str(&self.tool_definitions);
            combined_system.push_str("\n</tool_definitions>");
        }

        // 3. Developer Instructions
        if !self.developer_instructions.is_empty() {
            if !combined_system.is_empty() {
                combined_system.push_str("\n\n");
            }
            combined_system.push_str("<developer_instructions>\n");
            combined_system.push_str(&self.developer_instructions);
            combined_system.push_str("\n</developer_instructions>");
        }

        // 4. User Instructions
        if !self.user_instructions.is_empty() {
            if !combined_system.is_empty() {
                combined_system.push_str("\n\n");
            }
            combined_system.push_str("<user_instructions>\n");
            combined_system.push_str(&self.user_instructions);
            combined_system.push_str("\n</user_instructions>");
        }

        // 4.5. System Memory Index (Anthropic Lightweight Index Mechanic)
        if !self.lightweight_memory_index.is_empty() {
            if !combined_system.is_empty() {
                combined_system.push_str("\n\n");
            }
            combined_system.push_str("<system_memory_index>\n");
            for entry in &self.lightweight_memory_index {
                combined_system.push_str("- ");
                combined_system.push_str(entry);
                combined_system.push('\n');
            }
            combined_system.push_str("</system_memory_index>");
        }

        // High-Signal Re-injection (System Anchor)
        // If the system prompt is long, re-inject critical instructions at the end.
        if combined_system.chars().count() > 4000 {
            let core_objective = PromptBuilder::extract_core_objective(&self.user_instructions);
            combined_system.push_str("\n\n<system_anchor_high_signal_context_reinjection>\n");
            combined_system.push_str("To maintain focus in this large context, remember your core objective and constraints:\n");
            combined_system.push_str(&format!("Core Objective: {}\n", core_objective));
            if !self.developer_instructions.is_empty() {
                combined_system.push_str(&format!(
                    "Developer Instructions: {}\n",
                    self.developer_instructions
                ));
            }
            combined_system.push_str("</system_anchor_high_signal_context_reinjection>");
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

        let built = tokio::runtime::Runtime::new().unwrap().block_on(async {
            let agents_md = load_cascading_instructions(Some(&grandchild_dir)).await;
            let cfg = AgentRunConfig::default();
            let builder = StrictHierarchicalPromptBuilder::new(&cfg, &[], Some(agents_md), None);
            builder.build()
        });

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
            built.contains("[System: AGENTS.md content truncated to 32KiB limit.]")
                || built.contains("[User Instructions TRUNCATED TO 32KiB]"),
            "Should contain truncation warning"
        );

        // Total size of user instructions part should be bounded.
        assert!(
            built.len() <= 35000,
            "Output should be bounded. Current length: {}",
            built.len()
        );
    }

    #[test]
    fn test_cascading_agents_md_truncation_char_boundary() {
        let dir = tempdir().unwrap();
        let root_dir = dir.path().join("root");
        fs::create_dir_all(&root_dir).unwrap();

        // Let's use exactly 32,768 logical characters to reach the boundary, then push an emoji.
        let mut content = "A".repeat(32768);
        content.push('😊');
        fs::write(root_dir.join("AGENTS.md"), &content).unwrap();

        let built = tokio::runtime::Runtime::new().unwrap().block_on(async {
            let agents_md = load_cascading_instructions(Some(&root_dir)).await;
            let cfg = AgentRunConfig::default();
            let builder = StrictHierarchicalPromptBuilder::new(&cfg, &[], Some(agents_md), None);
            builder.build()
        });

        assert!(
            built.contains("[System: AGENTS.md content truncated to 32KiB limit.]")
                || built.contains("[User Instructions TRUNCATED TO 32KiB]")
        );
        // The emoji should be stripped because it's past the 32768 limit
        assert!(
            !built.contains("😊"),
            "Emoji should be stripped since it's character 32769"
        );
        assert!(
            built.contains(&"A".repeat(32700)),
            "Preceding characters up to limit should remain intact"
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
        assert!(
            last_msg
                .content
                .contains("[SYSTEM NOTIFICATION: Context Rot Prevention Anchor]")
        );
        assert!(
            last_msg
                .content
                .contains("Remember your core objective: Build a web server.")
        );
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
                .contains("| Recent Tool Errors: There was an error: file not found")
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
        let user_instructions = "A".repeat(995) + "😊";
        let obj = PromptBuilder::extract_core_objective(&user_instructions);
        assert_eq!(obj, user_instructions);

        let long_user_instructions = "A".repeat(995) + "😊" + "BCDEF";
        let obj2 = PromptBuilder::extract_core_objective(&long_user_instructions);

        // With new chars() logic, 997 logical characters are maintained.
        // The first 995 are 'A', 996 is '😊', 997 is 'B'.
        let expected = "A".repeat(995) + "😊" + "B...";
        assert_eq!(obj2, expected);
    }

    #[test]
    fn test_summarize_recent_tool_errors_char_boundary() {
        let mut error_msg = "error: ".to_string() + &"A".repeat(88) + "😊";
        error_msg += "BCDEF";
        let messages = vec![Message::user(error_msg)];

        let summary = PromptBuilder::summarize_recent_tool_errors(&messages);

        // 97 logical chars: 'error: ' (7 chars) + 88 'A' + '😊' (1 char) + 'B' (1 char)
        let expected = "error: ".to_string() + &"A".repeat(88) + "😊B...";
        assert_eq!(summary, expected);
    }

    #[test]
    fn test_high_signal_reinjection_trigger() {
        let mut cfg = AgentRunConfig {
            user_instructions: "Objective: Build a skyscraper.".repeat(200),
            ..Default::default()
        }; // ~6000 chars
        cfg.developer_instructions = "Use steel beams.".repeat(50); // ~800 chars
        // Total will be > 4000 chars

        let tools = vec![];
        let builder = StrictHierarchicalPromptBuilder::new(&cfg, &tools, None, None);
        let built = builder.build();

        assert!(built.contains("<system_anchor_high_signal_context_reinjection>"));
        assert!(built.contains("Core Objective: Objective: Build a skyscraper."));
        assert!(built.contains("Developer Instructions: Use steel beams."));
    }

    #[test]
    fn test_summarize_recent_momentum() {
        use crate::types::{ToolCall, ToolResult};

        let messages = vec![
            Message {
                role: Role::Assistant,
                content: "Calling tools".to_string(),
                tool_calls: vec![
                    ToolCall {
                        id: "c1".to_string(),
                        name: "read_file".to_string(),
                        arguments: serde_json::json!({}),
                    },
                    ToolCall {
                        id: "c2".to_string(),
                        name: "ls".to_string(),
                        arguments: serde_json::json!({}),
                    },
                ],
                tool_results: vec![],
                response_id: None,
                previous_response_id: None,
            },
            Message {
                role: Role::Tool,
                content: "".to_string(),
                tool_calls: vec![],
                tool_results: vec![
                    ToolResult {
                        tool_call_id: "c1".to_string(),
                        content: "file content".to_string(),
                        error: "".to_string(),
                    },
                    ToolResult {
                        tool_call_id: "c2".to_string(),
                        content: "dir list".to_string(),
                        error: "".to_string(),
                    },
                ],
                response_id: None,
                previous_response_id: None,
            },
        ];

        let momentum = PromptBuilder::summarize_recent_momentum(&messages);
        assert_eq!(momentum, "Recent Momentum: read_file -> ls");
    }

    #[test]
    fn test_lightweight_memory_index_injection_and_truncation() {
        let cfg = AgentRunConfig::default();
        let long_entry = "A".repeat(200);
        let normal_entry = "Just a regular memory entry".to_string();
        let index = vec![long_entry, normal_entry.clone()];

        let builder = StrictHierarchicalPromptBuilder::new(&cfg, &[], None, Some(index));
        let built = builder.build();

        assert!(built.contains("<system_memory_index>"));
        assert!(built.contains("</system_memory_index>"));

        // Normal entry should be fully present
        assert!(built.contains(&format!("- {}", normal_entry)));

        // Long entry should be truncated to 147 chars + "..." = 150 chars
        let truncated_long = format!("- {}...", "A".repeat(147));
        assert!(built.contains(&truncated_long));
        assert!(!built.contains(&"A".repeat(151)));
    }

    #[tokio::test]
    async fn test_load_cascading_instructions_precedence() {
        let dir = tempdir().unwrap();
        let root_dir = dir.path().join("root");
        let child_dir = root_dir.join("child");
        fs::create_dir_all(&child_dir).unwrap();

        fs::write(root_dir.join("AGENTS.md"), "Root Rule").unwrap();
        fs::write(child_dir.join("AGENTS.md"), "Child Rule").unwrap();

        let combined = load_cascading_instructions(Some(&child_dir)).await;

        // More deeply nested (Child) should come first in the vec, thus first in combined string
        assert!(combined.starts_with("Child Rule"));
        assert!(combined.contains("Root Rule"));
    }
}
