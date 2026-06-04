use std::sync::Mutex;

pub struct BudgetManager {
    pub total_limit: f64,
    current: Mutex<f64>,
    pub telemetry_store: Option<std::sync::Arc<::server_harness::telemetry::ViolationStore>>,
    tenant_id: Option<String>,
}

impl BudgetManager {
    pub fn new(limit: f64) -> Self {
        BudgetManager {
            total_limit: limit,
            current: Mutex::new(0.0),
            telemetry_store: None,
            tenant_id: None,
        }
    }

    pub fn with_telemetry(mut self, tenant_id: String, store: std::sync::Arc<::server_harness::telemetry::ViolationStore>) -> Self {
        self.tenant_id = Some(tenant_id);
        self.telemetry_store = Some(store);
        self
    }

    pub fn record_spend(&self, amount: f64) -> Result<bool, String> {
        let mut current = self.current.lock().unwrap();

        if amount < 0.0 {
            return Err("spend amount cannot be negative".to_string());
        }

        *current += amount;

        if let (Some(store), Some(tid)) = (&self.telemetry_store, &self.tenant_id) {
            let cents = (amount * 100.0).round() as u64;
            if cents > 0 {
                store.llm_cost_counter.add(
                    cents,
                    &[opentelemetry::KeyValue::new("tenant_id", tid.to_string())],
                );
            }
        }

        if *current > self.total_limit {
            Ok(false)
        } else {
            Ok(true)
        }
    }

    pub fn get_remaining(&self) -> f64 {
        let current = self.current.lock().unwrap();
        self.total_limit - *current
    }

    pub fn get_remaining_cents(&self) -> i64 {
        let current = self.current.lock().unwrap();
        ((self.total_limit - *current) * 100.0).round() as i64
    }

    pub fn record_spend_cents(&self, amount_cents: i64) -> Result<bool, String> {
        self.record_spend((amount_cents as f64) / 100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_budget_manager() {
        let manager = BudgetManager::new(100.0);
        
        assert_eq!(manager.get_remaining(), 100.0);
        
        assert_eq!(manager.record_spend(50.0).unwrap(), true);
        assert_eq!(manager.get_remaining(), 50.0);
        
        // Exceed budget, it's a soft limit so it returns false but updates current
        assert_eq!(manager.record_spend(60.0).unwrap(), false);
        assert_eq!(manager.get_remaining(), -10.0);
        
        let err = manager.record_spend(-10.0).unwrap_err();
        assert_eq!(err, "spend amount cannot be negative");

        // test cents (soft limit still applies)
        assert_eq!(manager.record_spend_cents(1000).unwrap(), false); // spend $10
        assert_eq!(manager.get_remaining(), -20.0);
        assert_eq!(manager.get_remaining_cents(), -2000);
    }

    #[test]
    fn test_budget_manager_exact_limit() {
        let manager = BudgetManager::new(100.0);
        assert_eq!(manager.get_remaining(), 100.0);

        // Spend exactly the limit
        assert_eq!(manager.record_spend(100.0).unwrap(), true);
        assert_eq!(manager.get_remaining(), 0.0);
        assert_eq!(manager.get_remaining_cents(), 0);

        // Even an epsilon more should be over the limit (soft limit handled as false)
        assert_eq!(manager.record_spend(0.01).unwrap(), false);
        let rem = manager.get_remaining();
        assert!(rem < -0.009 && rem > -0.011);
    }

    #[test]
    fn test_budget_manager_with_telemetry() {
        let store = std::sync::Arc::new(::server_harness::telemetry::ViolationStore::new(None));

        let manager = BudgetManager::new(50.0).with_telemetry("tenant-123".to_string(), store);

        // Ensure struct states updated correctly
        assert_eq!(manager.tenant_id, Some("tenant-123".to_string()));
        assert!(manager.telemetry_store.is_some());

        // Spend money to hit telemetry path without panic
        assert_eq!(manager.record_spend(10.0).unwrap(), true);
        assert_eq!(manager.get_remaining(), 40.0);
    }

    #[test]
    fn test_record_spend_cents_zero() {
        let manager = BudgetManager::new(100.0);
        assert_eq!(manager.record_spend_cents(0).unwrap(), true);
        assert_eq!(manager.get_remaining_cents(), 10000);
    }
}
