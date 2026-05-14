
pub mod token_budgeting {
    use std::sync::atomic::{AtomicU64, Ordering};

    pub struct TokenBudget {
        pub total_tokens_used: AtomicU64,
        pub budget_limit: u64,
    }

    impl TokenBudget {
        pub fn new(budget_limit: u64) -> Self {
            Self {
                total_tokens_used: AtomicU64::new(0),
                budget_limit,
            }
        }

        pub fn consume_tokens(&self, count: u64) -> Result<(), String> {
            let current = self.total_tokens_used.load(Ordering::Relaxed);
            if current + count > self.budget_limit {
                return Err("Token budget exceeded".to_string());
            }
            self.total_tokens_used.fetch_add(count, Ordering::Relaxed);
            Ok(())
        }

        pub fn reset_budget(&self) {
            self.total_tokens_used.store(0, Ordering::Relaxed);
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_token_budget_consumption() {
            let budget = TokenBudget::new(100);
            assert!(budget.consume_tokens(50).is_ok());
            assert!(budget.consume_tokens(60).is_err());
            assert!(budget.consume_tokens(40).is_ok());
            assert_eq!(budget.total_tokens_used.load(Ordering::Relaxed), 90);

            budget.reset_budget();
            assert_eq!(budget.total_tokens_used.load(Ordering::Relaxed), 0);
            assert!(budget.consume_tokens(100).is_ok());
        }
    }
}

pub mod quota_manager {
    use std::collections::HashMap;
    use std::sync::RwLock;

    pub struct QuotaManager {
        quotas: RwLock<HashMap<String, u32>>,
    }

    impl QuotaManager {
        pub fn new() -> Self {
            Self {
                quotas: RwLock::new(HashMap::new()),
            }
        }

        pub fn set_quota(&self, tenant_id: &str, limit: u32) {
            let mut q = self.quotas.write().unwrap();
            q.insert(tenant_id.to_string(), limit);
        }

        pub fn get_quota(&self, tenant_id: &str) -> Option<u32> {
            let q = self.quotas.read().unwrap();
            q.get(tenant_id).cloned()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_quota_manager() {
            let qm = QuotaManager::new();
            qm.set_quota("t1", 100);
            assert_eq!(qm.get_quota("t1"), Some(100));
            assert_eq!(qm.get_quota("t2"), None);
            qm.set_quota("t1", 200);
            assert_eq!(qm.get_quota("t1"), Some(200));
        }
    }
}
