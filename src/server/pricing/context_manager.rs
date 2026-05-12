use ohc_builtin_agent_core::types::{Message, Role};

pub struct ContextPruner;

impl ContextPruner {
    /// Prunes conversation history based on "importance" and age.
    /// Keep: last N messages, system messages, and high-importance task markers.
    pub fn prune_history(messages: Vec<Message>, max_messages: usize) -> Vec<Message> {
        if messages.len() <= max_messages {
            return messages;
        }

        let mut final_messages = Vec::new();
        let system_messages: Vec<Message> = messages.iter().filter(|m| m.role == Role::System).cloned().collect();

        // Always keep system messages
        final_messages.extend(system_messages);

        let other_messages: Vec<Message> = messages.into_iter().filter(|m| m.role != Role::System).collect();

        if other_messages.len() > max_messages {
            // Keep the most recent ones
            let start_idx = other_messages.len() - max_messages;
            let recent_messages = &other_messages[start_idx..];

            // Search for high-importance markers in the part being pruned
            let pruned_part = &other_messages[..start_idx];
            for m in pruned_part {
                if m.content.contains("DECISION") || m.content.contains("ARCHITECTURAL") || m.content.contains("CRITICAL") {
                    final_messages.push(m.clone());
                }
            }

            final_messages.extend_from_slice(recent_messages);
        } else {
            final_messages.extend(other_messages);
        }

        final_messages
    }

    /// Semantic TTL: Determine if a memory record should be archived.
    pub fn should_archive_memory(reference_count: i32, age_days: i64) -> bool {
        if reference_count > 10 {
            return false; // High utility
        }

        if age_days > 30 && reference_count < 2 {
            return true; // Old and low utility
        }

        false
    }
}
