#[cfg(test)]
mod tests {
    use crate::prompt_caching::PromptCache;
    use crate::budget::BudgetManager;
    use std::time::Duration;

    #[tokio::test]
    async fn test_cache_high_concurrency() {
        let cache = PromptCache::new(Duration::from_secs(60));
        let mut handles = vec![];
        for i in 0..100 {
            let cache_clone = cache.clone();
            handles.push(tokio::spawn(async move {
                let p = format!("prompt_{}", i);
                let m = format!("message_{}", i);
                cache_clone.set(&p, &m, "response").await;
            }));
        }
        for h in handles { h.await.unwrap(); }
    }

    #[test]
    fn test_budget_exhaustion_recovery() {
        let manager = BudgetManager::new(1.0);
        assert!(manager.record_spend(0.9).is_ok());
        assert!(manager.record_spend(0.2).is_err());
        // Use epsilon for float comparison
        assert!((manager.get_remaining() - 0.1).abs() < 1e-9);
    }
}
