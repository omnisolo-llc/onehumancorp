#[cfg(test)]
mod tests {
    use crate::rate_limit::PlanTier;
    use crate::budget::BudgetManager;

    #[test]
    fn test_tier_limits() {
        let free_tier = PlanTier::Free;
        assert_eq!(free_tier.monthly_action_limit(), Some(100));
        assert_eq!(free_tier.storage_limit_mb(), Some(500));
    }

    #[test]
    fn test_budget_logic() {
        let manager = BudgetManager::new(10.0);
        assert_eq!(manager.get_remaining(), 10.0);
        manager.record_spend(2.5).unwrap();
        assert_eq!(manager.get_remaining(), 7.5);
    }
}
