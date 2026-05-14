#[cfg(test)]
mod tests {
    use crate::miser_engine::MiserEngine;
    use crate::budget::BudgetManager;
    use crate::steering::ModelTier;
    use crate::context_manager::ContextWindow;

    #[test]
    fn test_miser_full_simulation_starter_tier() {
        let engine = MiserEngine::new();
        let budget = BudgetManager::new(50.0); // 0 budget
        let mut window = ContextWindow::new(500);

        // Turn 1: Complex analysis
        let prompt = "Analyze the ROI of my current Instagram ad campaign and suggest optimizations.";
        let tier = engine.select_model(prompt, &budget);
        assert_eq!(tier, ModelTier::Standard);

        budget.record_spend(0.15).unwrap(); // -bash.15 spend
        window.add_message("user".to_string(), prompt.to_string());
        window.add_message("assistant".to_string(), "Analysis complete...".to_string());

        // Turn 2: Follow up
        let prompt2 = "Rewrite the copy for the second ad.";
        let tier2 = engine.select_model(prompt2, &budget);
        assert_eq!(tier2, ModelTier::Standard);

        budget.record_spend(0.05).unwrap();
        window.add_message("user".to_string(), prompt2.to_string());

        assert_eq!(budget.get_remaining(), 49.80);
        assert_eq!(window.messages.len(), 3);
    }

    #[test]
    fn test_miser_budget_exhaustion_steering() {
        let engine = MiserEngine::new();
        let budget = BudgetManager::new(0.60); // Low budget

        // Complex prompt but budget is too low for Premium/Standard
        let prompt = "Perform extremely complex multi-step reasoning task.";

        // Spend 0.20
        budget.record_spend(0.20).unwrap();
        assert_eq!(budget.get_remaining_cents(), 40);

        // Remaining 40 cents is below the 50 cent threshold for Standard/Premium in my steering.rs
        let tier = engine.select_model(prompt, &budget);
        assert_eq!(tier, ModelTier::Economy);
    }

    #[test]
    fn test_recommendation_filtering() {
        let engine = MiserEngine::new();
        let budget = BudgetManager::new(5.0);

        let recs = engine.get_recommendations("tenant_1", &budget);
        // Should include the standard ones plus maybe a low budget warning if it was <
        assert!(recs.len() >= 2);

        let budget_low = BudgetManager::new(0.5);
        let recs_low = engine.get_recommendations("tenant_2", &budget_low);
        assert!(recs_low.iter().any(|r| r.id == "low_budget_warning"));
    }
}
