use std::sync::atomic::{AtomicU64, Ordering};

pub struct BudgetManager {
    pub total_limit: f64,
    current: AtomicU64,
    pub telemetry_store: Option<std::sync::Arc<::server_harness::telemetry::ViolationStore>>,
    tenant_id: Option<String>,
    pub alert_threshold_percent: f64,
}

impl BudgetManager {
    pub fn new(limit: f64) -> Self {
        BudgetManager {
            total_limit: limit,
            current: AtomicU64::new(0),
            telemetry_store: None,
            tenant_id: None,
            alert_threshold_percent: 80.0,
        }
    }

    pub fn with_alert_threshold(mut self, threshold: f64) -> Self {
        self.alert_threshold_percent = threshold;
        self
    }

    pub fn with_telemetry(mut self, tenant_id: String, store: std::sync::Arc<::server_harness::telemetry::ViolationStore>) -> Self {
        self.tenant_id = Some(tenant_id);
        self.telemetry_store = Some(store);
        self
    }

    pub fn record_spend(&self, amount: f64) -> Result<bool, String> {
        if amount < 0.0 {
            return Err("spend amount cannot be negative".to_string());
        }
        if amount == 0.0 {
            return Ok(self.get_remaining() >= 0.0);
        }

        if let (Some(_store), Some(tid)) = (&self.telemetry_store, &self.tenant_id) {
            tracing::info!("💰 Miser telemetry: Recording budget spend for tenant {}", tid); // pii-safe
        }

        let final_current_bits = self.current.fetch_update(
            Ordering::SeqCst,
            Ordering::Relaxed,
            |bits| {
                let current = f64::from_bits(bits);
                let next = current + amount;
                Some(next.to_bits())
            }
        ).unwrap(); // safe because the closure always returns Some

        let final_current = f64::from_bits(final_current_bits) + amount;

        if let (Some(store), Some(tid)) = (&self.telemetry_store, &self.tenant_id) {
            let cents = (amount * 100.0).round() as u64;
            if cents > 0 {
                store.llm_cost_counter.add(
                    cents,
                    &[opentelemetry::KeyValue::new("tenant_id", tid.to_string())],
                );
            }
        }

        if final_current > self.total_limit {
            Ok(false)
        } else {
            Ok(true)
        }
    }

    pub fn get_remaining(&self) -> f64 {
        let current = f64::from_bits(self.current.load(Ordering::SeqCst));
        self.total_limit - current
    }

    pub fn get_remaining_cents(&self) -> i64 {
        let current = f64::from_bits(self.current.load(Ordering::SeqCst));
        ((self.total_limit - current) * 100.0).round() as i64
    }

    pub fn record_spend_cents(&self, amount_cents: i64) -> Result<bool, String> {
        if amount_cents == 0 {
            return Ok(self.get_remaining() >= 0.0);
        }
        self.record_spend((amount_cents as f64) / 100.0)
    }

    pub fn check_alert_threshold(&self) -> bool {
        if self.total_limit <= 0.0 {
            return false;
        }
        let current = f64::from_bits(self.current.load(Ordering::SeqCst));
        let usage_percent = (current / self.total_limit) * 100.0;
        usage_percent >= self.alert_threshold_percent
    }

    pub fn is_projected_cost_over_threshold(&self, projected_cost_cents: i64) -> bool {
        if self.total_limit <= 0.0 {
            return projected_cost_cents > 0;
        }
        let limit_threshold_cents = ((self.total_limit * 100.0) * (self.alert_threshold_percent / 100.0)).round() as i64;
        projected_cost_cents >= limit_threshold_cents
    }

    pub fn check_alert_threshold_cents(&self, total_limit_cents: i64) -> bool {
        if total_limit_cents <= 0 {
            return false;
        }
        let current = f64::from_bits(self.current.load(Ordering::SeqCst));
        let current_cents = (current * 100.0).round() as i64;
        let limit_threshold_cents = ((total_limit_cents as f64) * (self.alert_threshold_percent / 100.0)).round() as i64;
        current_cents >= limit_threshold_cents
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_budget_manager() {
        let manager = BudgetManager::new(100.0);
        
        assert_eq!(manager.get_remaining(), 100.0);
        
        assert!(manager.record_spend(50.0).expect("failed to unwrap"));
        assert_eq!(manager.get_remaining(), 50.0);
        
        // Exceed budget, it's a soft limit so it returns false but updates current
        assert!(!(manager.record_spend(60.0).expect("failed to unwrap")));
        assert_eq!(manager.get_remaining(), -10.0);
        
        let err = manager.record_spend(-10.0).unwrap_err();
        assert_eq!(err, "spend amount cannot be negative");

        // test cents (soft limit still applies)
        assert!(!(manager.record_spend_cents(1000).expect("failed to unwrap"))); // spend $10
        assert_eq!(manager.get_remaining(), -20.0);
        assert_eq!(manager.get_remaining_cents(), -2000);
    }

    #[test]
    fn test_budget_manager_exact_limit() {
        let manager = BudgetManager::new(100.0);
        assert_eq!(manager.get_remaining(), 100.0);

        // Spend exactly the limit
        assert!(manager.record_spend(100.0).expect("failed to unwrap"));
        assert_eq!(manager.get_remaining(), 0.0);
        assert_eq!(manager.get_remaining_cents(), 0);

        // Even an epsilon more should be over the limit (soft limit handled as false)
        assert!(!(manager.record_spend(0.01).expect("failed to unwrap")));
        let rem = manager.get_remaining();
        assert!(rem < -0.009 && rem > -0.011);
    }

    #[test]
    fn test_budget_manager_with_telemetry() {
        let store = std::sync::Arc::new(::server_harness::telemetry::ViolationStore::new(None));

        let manager = BudgetManager::new(50.0).with_telemetry("tenant-123".to_string(), store);
        assert!(manager.telemetry_store.is_some());
        // Spend money to hit telemetry path without panic
        manager.record_spend_cents(1000).expect("failed to unwrap");

        // Ensure struct states updated correctly
        assert_eq!(manager.tenant_id, Some("tenant-123".to_string()));
        assert!(manager.telemetry_store.is_some());

        // Spend money to hit telemetry path without panic
        assert!(manager.record_spend(10.0).expect("failed to unwrap"));
        assert_eq!(manager.get_remaining(), 30.0);
    }

    #[test]
    fn test_record_spend_cents_zero() {
        let manager = BudgetManager::new(100.0);
        assert!(manager.record_spend_cents(0).expect("failed to unwrap"));
        assert_eq!(manager.get_remaining_cents(), 10000);
    }

    #[test]
    fn test_check_alert_threshold() {
        let manager = BudgetManager::new(100.0);

        // Not over threshold initially
        assert!(!manager.check_alert_threshold());

        // Spend 50%
        manager.record_spend(50.0).expect("failed to unwrap");
        assert!(!manager.check_alert_threshold());

        // Spend up to 80%
        manager.record_spend(30.0).expect("failed to unwrap");
        assert!(manager.check_alert_threshold()); // Default is 80.0

        // Custom threshold
        let custom_manager = BudgetManager::new(100.0).with_alert_threshold(90.0);
        custom_manager.record_spend(85.0).expect("failed to unwrap");
        assert!(!custom_manager.check_alert_threshold());

        custom_manager.record_spend(10.0).expect("failed to unwrap"); // 95%
        assert!(custom_manager.check_alert_threshold());
    }

    #[test]
    fn test_check_alert_threshold_zero_limit() {
        let manager = BudgetManager::new(0.0);
        assert!(!manager.check_alert_threshold());
        assert!(!manager.check_alert_threshold_cents(0));
    }

    #[test]
    fn test_check_alert_threshold_cents() {
        let manager = BudgetManager::new(100.0);

        // Not over threshold initially
        assert!(!manager.check_alert_threshold_cents(10000));

        // Spend 50%
        manager.record_spend_cents(5000).expect("failed to unwrap");
        assert!(!manager.check_alert_threshold_cents(10000));

        // Spend up to 80% (8000 cents)
        manager.record_spend_cents(3000).expect("failed to unwrap");
        assert!(manager.check_alert_threshold_cents(10000)); // Default is 80.0

        // Custom threshold using cents
        let custom_manager = BudgetManager::new(100.0).with_alert_threshold(90.0);
        custom_manager.record_spend_cents(8500).expect("failed to unwrap");
        assert!(!custom_manager.check_alert_threshold_cents(10000));

        custom_manager.record_spend_cents(1000).expect("failed to unwrap"); // 95%
        assert!(custom_manager.check_alert_threshold_cents(10000));

        // Exact threshold check
        let exact_manager = BudgetManager::new(100.0).with_alert_threshold(80.0);
        exact_manager.record_spend_cents(8000).expect("failed to unwrap");
        assert!(exact_manager.check_alert_threshold_cents(10000));
    }

    #[test]
    fn test_check_alert_threshold_with_projected_costs() {
        let manager = BudgetManager::new(10.0);
        // $10 limit, 80% threshold = $8 (800 cents)
        assert!(!manager.is_projected_cost_over_threshold(700)); // $7
        assert!(manager.is_projected_cost_over_threshold(800)); // $8
        assert!(manager.is_projected_cost_over_threshold(1500)); // $15

        let zero_manager = BudgetManager::new(0.0);
        assert!(zero_manager.is_projected_cost_over_threshold(800));
        assert!(zero_manager.is_projected_cost_over_threshold(1));
        assert!(!zero_manager.is_projected_cost_over_threshold(0));
    }

    #[test]
    fn test_record_spend_cents_negative() {
        let manager = BudgetManager::new(100.0);
        let err = manager.record_spend_cents(-1000).unwrap_err();
        assert_eq!(err, "spend amount cannot be negative");
    }

    #[test]
    fn test_budget_manager_with_telemetry_no_tenant() {
        let store = std::sync::Arc::new(::server_harness::telemetry::ViolationStore::new(None));
        let mut manager = BudgetManager::new(50.0);
        manager.telemetry_store = Some(store);
        assert!(manager.record_spend(10.0).expect("failed to unwrap"));
        assert_eq!(manager.get_remaining(), 40.0);
    }

    #[test]
    fn test_budget_manager_edge_cases() {
        let manager = BudgetManager::new(f64::MAX);
        assert!(manager.record_spend(1.0).expect("failed to unwrap"));

        // Check extremely small threshold values
        let threshold_manager = BudgetManager::new(100.0).with_alert_threshold(0.01);
        threshold_manager.record_spend(0.02).expect("failed to unwrap");
        assert!(threshold_manager.check_alert_threshold());
    }
}
