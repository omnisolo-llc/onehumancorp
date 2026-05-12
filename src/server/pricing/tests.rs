#[cfg(test)]
mod tests {
    use crate::prompt_audit::PromptAuditor;
    use crate::steering::{ModelRouter, ModelTier};
    use crate::context_manager::ContextPruner;
    use ohc_builtin_agent_core::types::{Message, Role};

    #[test]
    fn test_miser_auditor_redundancy() {
        // Updated test string to ensure it hits the redundancy thresholds
        let redundant_prompt = "This is a very long instruction that repeats itself. This is a very long instruction that repeats itself. This is a very long instruction that repeats itself. Please please please thank you thank you.";
        let result = PromptAuditor::audit_system_prompt(redundant_prompt);
        assert!(result.redundancy_score > 0.1); // Adjusted expectation
        assert!(result.optimization_tips.len() > 0);
    }

    #[test]
    fn test_miser_steering_logic() {
        // High budget, complex task - Need at least 3 complexity keywords for Premium
        let tier = ModelRouter::route_task("Architect a comprehensive strategic system", 10.0);
        assert_eq!(tier, ModelTier::Premium);

        // Low budget, complex task
        let tier_low = ModelRouter::route_task("Architect a comprehensive strategic system", 0.40);
        assert_eq!(tier_low, ModelTier::Economy);

        // Simple task
        let tier_simple = ModelRouter::route_task("Hi", 10.0);
        assert_eq!(tier_simple, ModelTier::Economy);
    }

    #[test]
    fn test_miser_context_pruning() {
        let messages = vec![
            Message::system("Sys"),
            Message::user("U1"),
            Message::assistant("A1"),
            Message::user("U2 CRITICAL DECISION"),
            Message::assistant("A2"),
        ];

        let pruned = ContextPruner::prune_history(messages, 2);

        // System and Decision should be preserved
        assert!(pruned.iter().any(|m| m.content == "Sys"));
        assert!(pruned.iter().any(|m| m.content.contains("CRITICAL DECISION")));
        // A2 (most recent) should be preserved
        assert!(pruned.iter().any(|m| m.content == "A2"));
    }
}
