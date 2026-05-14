#[cfg(test)]
mod tests {
    use crate::prompt_caching::PromptCache;
    use crate::miser::get_active_recommendations;
    use crate::steering::{steer_request, SteeringConfig, ModelTier};
    use crate::context_manager::ContextWindow;
    use crate::budget::BudgetManager;

    #[test]
    fn test_miser_integration_flow() {
        let recs = get_active_recommendations();
        assert!(!recs.is_empty());
        assert_eq!(recs[0].id, "ach_optimization");
    }

    #[test]
    fn test_full_agent_cycle_simulation() {
        let budget = BudgetManager::new(10.0);
        let config = SteeringConfig::default();
        let mut window = ContextWindow::new(100);

        // 1. New turn
        let prompt = "Analyze my sales data.";
        let tier = steer_request(prompt, budget.get_remaining_cents(), &config);
        assert_eq!(tier, ModelTier::Standard);

        // 2. Record spend
        budget.record_spend(0.05).unwrap();

        // 3. Update context
        window.add_message("user".to_string(), prompt.to_string());
        window.add_message("assistant".to_string(), "Done.".to_string());

        assert_eq!(budget.get_remaining(), 9.95);
        assert_eq!(window.messages.len(), 2);
    }
}
