#[cfg(test)]
mod tests {
    use crate::rate_limit::PlanTier;

    #[test]
    fn test_plan_tier_limits_exhaustive() {
        let tiers = vec![PlanTier::Free, PlanTier::Starter, PlanTier::Pro, PlanTier::Business];

        for tier in tiers {
            match tier {
                PlanTier::Free => {
                    assert_eq!(tier.monthly_action_limit(), Some(100));
                    assert_eq!(tier.storage_limit_mb(), Some(500));
                },
                PlanTier::Starter => {
                    assert_eq!(tier.monthly_action_limit(), Some(1000));
                    assert_eq!(tier.storage_limit_mb(), Some(5000));
                },
                PlanTier::Pro => {
                    assert_eq!(tier.monthly_action_limit(), None);
                    assert_eq!(tier.storage_limit_mb(), Some(50000));
                },
                PlanTier::Business => {
                    assert_eq!(tier.monthly_action_limit(), None);
                    assert_eq!(tier.storage_limit_mb(), Some(512000));
                }
            }
        }
    }

    #[test]
    fn test_agent_limits() {
        assert_eq!(PlanTier::Free.max_agents(), Some(1));
        assert_eq!(PlanTier::Starter.max_agents(), Some(3));
        assert_eq!(PlanTier::Pro.max_agents(), Some(10));
        assert_eq!(PlanTier::Business.max_agents(), None);
    }
}
