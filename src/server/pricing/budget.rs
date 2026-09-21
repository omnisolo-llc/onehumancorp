use std::collections::HashMap;
use std::sync::Mutex;

struct BudgetState {
    current: i64,
    allocated: i64,
    reservations: HashMap<u64, i64>,
    next_reservation_id: u64,
}

pub struct BudgetManager {
    pub total_limit: f64,
    pub total_limit_cents: i64,
    state: Mutex<BudgetState>,
    pub telemetry_store: Option<std::sync::Arc<::server_harness::telemetry::ViolationStore>>,
    tenant_id: Option<String>,
    pub alert_threshold_percent: f64,
}

impl BudgetManager {
    pub fn new(limit: f64) -> Self {
        // Invalid limits fail closed; retain the explicit legacy MAX sentinel.
        let total_limit_cents = if limit == f64::MAX {
            i64::MAX
        } else if !limit.is_finite() || limit < 0.0 || limit * 100.0 >= i64::MAX as f64 {
            0
        } else {
            (limit * 100.0).round() as i64
        };
        BudgetManager {
            total_limit: limit,
            state: Mutex::new(BudgetState {
                current: 0,
                allocated: 0,
                reservations: HashMap::new(),
                next_reservation_id: 1,
            }),
            total_limit_cents,
            telemetry_store: None,
            tenant_id: None,
            alert_threshold_percent: 80.0,
        }
    }

    pub fn with_alert_threshold(mut self, threshold: f64) -> Self {
        self.alert_threshold_percent = threshold;
        self
    }

    pub fn with_telemetry(
        mut self,
        tenant_id: String,
        store: std::sync::Arc<::server_harness::telemetry::ViolationStore>,
    ) -> Self {
        self.tenant_id = Some(tenant_id);
        self.telemetry_store = Some(store);
        self
    }

    pub fn record_spend(&self, amount: f64) -> Result<bool, String> {
        if amount < 0.0 {
            return Err("spend amount cannot be negative".to_string());
        }
        if !amount.is_finite() || amount * 100.0 >= i64::MAX as f64 {
            return Err("spend amount must be finite and bounded".to_string());
        }
        let amount_cents = (amount * 100.0).round() as i64;
        self.record_spend_cents(amount_cents)
    }

    // Retained for compatibility. Routes through immediate reserve+settle.
    pub fn record_spend_cents(&self, amount_cents: i64) -> Result<bool, String> {
        if amount_cents < 0 {
            return Err("spend amount cannot be negative".to_string());
        }
        if amount_cents == 0 {
            return Ok(self.get_remaining_cents() >= 0);
        }

        if let (Some(_store), Some(tid)) = (&self.telemetry_store, &self.tenant_id) {
            tracing::info!(
                "💰 Miser telemetry: Recording budget spend for tenant {}",
                tid
            ); // pii-safe
        }

        match self.reserve(amount_cents) {
            Ok(Some(id)) => {
                self.settle(id, amount_cents)?;
                Ok(true)
            }
            Ok(None) => Ok(true), // amount_cents == 0
            Err(_) => Ok(false),
        }
    }

    pub fn reserve(&self, amount_cents: i64) -> Result<Option<u64>, String> {
        if amount_cents < 0 {
            return Err("reserve amount cannot be negative".to_string());
        }
        if amount_cents == 0 {
            return Ok(None);
        }

        let mut state = self.state.lock().unwrap();

        let new_total = match state
            .current
            .checked_add(state.allocated)
            .and_then(|t| t.checked_add(amount_cents))
        {
            Some(t) => t,
            None => return Err("reservation would overflow".to_string()),
        };

        if new_total > self.total_limit_cents {
            return Err("reservation exceeds budget limit".to_string());
        }

        state.allocated += amount_cents;
        let id = state.next_reservation_id;
        state.next_reservation_id += 1;
        state.reservations.insert(id, amount_cents);

        Ok(Some(id))
    }

    pub fn settle(&self, reservation_id: u64, actual_cents: i64) -> Result<(), String> {
        if actual_cents < 0 {
            return Err("settle amount cannot be negative".to_string());
        }

        {
            let mut state = self.state.lock().unwrap();
            let reserved = state
                .reservations
                .remove(&reservation_id)
                .ok_or_else(|| "invalid reservation ID".to_string())?;

            state.allocated -= reserved;
            state.current += actual_cents;
        }

        if let (Some(store), Some(tid)) = (&self.telemetry_store, &self.tenant_id)
            && actual_cents > 0
        {
            store.llm_cost_counter.add(
                actual_cents as u64,
                &[opentelemetry::KeyValue::new("tenant_id", tid.to_string())],
            );
            store.mission_cost_cents.add(
                actual_cents as u64,
                &[opentelemetry::KeyValue::new("tenant_id", tid.to_string())],
            );
        }

        Ok(())
    }

    pub fn release(&self, reservation_id: u64) -> Result<(), String> {
        let mut state = self.state.lock().unwrap();
        let reserved = state
            .reservations
            .remove(&reservation_id)
            .ok_or_else(|| "invalid reservation ID".to_string())?;
        state.allocated -= reserved;
        Ok(())
    }

    pub fn get_remaining(&self) -> f64 {
        let current = {
            let state = self.state.lock().unwrap();
            state.current + state.allocated
        };
        (self.total_limit_cents - current) as f64 / 100.0
    }

    pub fn get_remaining_cents(&self) -> i64 {
        let current = {
            let state = self.state.lock().unwrap();
            state.current + state.allocated
        };
        self.total_limit_cents - current
    }

    pub fn check_alert_threshold(&self) -> bool {
        if self.total_limit_cents <= 0 {
            return false;
        }
        let current = {
            let state = self.state.lock().unwrap();
            state.current + state.allocated
        };
        let usage_percent = (current as f64 / self.total_limit_cents as f64) * 100.0;
        usage_percent >= self.alert_threshold_percent
    }

    pub fn is_projected_cost_over_threshold(&self, projected_cost_cents: i64) -> bool {
        if self.total_limit_cents <= 0 {
            return projected_cost_cents > 0 || {
                let state = self.state.lock().unwrap();
                state.current + state.allocated
            } > 0;
        }
        let limit_threshold_cents = ((self.total_limit_cents as f64)
            * (self.alert_threshold_percent / 100.0))
            .round() as i64;
        projected_cost_cents >= limit_threshold_cents || {
            let state = self.state.lock().unwrap();
            state.current + state.allocated
        } >= limit_threshold_cents
    }

    pub fn check_alert_threshold_cents(&self, total_limit_cents: i64) -> bool {
        if total_limit_cents <= 0 {
            return false;
        }
        let current = {
            let state = self.state.lock().unwrap();
            state.current + state.allocated
        };
        let limit_threshold_cents =
            ((total_limit_cents as f64) * (self.alert_threshold_percent / 100.0)).round() as i64;
        current >= limit_threshold_cents
    }

    pub fn is_spend_rate_too_high(
        &self,
        time_elapsed: std::time::Duration,
        total_duration: std::time::Duration,
    ) -> bool {
        if self.total_limit_cents <= 0 || total_duration.as_secs() == 0 {
            return false;
        }
        let current = {
            let state = self.state.lock().unwrap();
            state.current + state.allocated
        };
        let expected_spend = (self.total_limit_cents as f64)
            * (time_elapsed.as_secs() as f64 / total_duration.as_secs() as f64);
        current as f64 > expected_spend * 1.5 // 50% higher than expected rate
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partial_settlement_and_duplicates() {
        let manager = BudgetManager::new(100.0); // 10000 cents

        let id1 = manager.reserve(5000).unwrap().unwrap();
        // Settle with less than reserved
        manager.settle(id1, 2000).unwrap();
        assert_eq!(manager.get_remaining_cents(), 8000); // 10000 - 2000 settled - 0 allocated

        // Cannot settle or release an already settled ID
        assert!(manager.settle(id1, 1000).is_err());
        assert!(manager.release(id1).is_err());

        // Test release
        let id2 = manager.reserve(3000).unwrap().unwrap();
        assert_eq!(manager.get_remaining_cents(), 5000); // 8000 - 3000 allocated
        manager.release(id2).unwrap();
        assert_eq!(manager.get_remaining_cents(), 8000); // Back to 8000

        // Cannot release or settle an already released ID
        assert!(manager.release(id2).is_err());
        assert!(manager.settle(id2, 1000).is_err());
    }

    #[test]
    fn concurrent_admission_never_exceeds_limit() {
        let manager = std::sync::Arc::new(BudgetManager::new(1.0));
        let workers: Vec<_> = (0..32)
            .map(|_| {
                let manager = manager.clone();
                std::thread::spawn(move || manager.record_spend_cents(10).unwrap())
            })
            .collect();
        let admitted = workers
            .into_iter()
            .map(|worker| usize::from(worker.join().unwrap()))
            .sum::<usize>();
        assert_eq!(admitted, 10);
        assert_eq!(manager.get_remaining_cents(), 0);
    }

    #[test]
    fn invalid_amounts_and_overflow_fail_closed() {
        for limit in [f64::NAN, f64::INFINITY, -1.0] {
            assert!(!BudgetManager::new(limit).record_spend_cents(1).unwrap());
        }
        let manager = BudgetManager::new(f64::MAX);
        assert!(manager.record_spend_cents(i64::MAX).unwrap());
        assert!(!manager.record_spend_cents(1).unwrap());
        assert_eq!(manager.get_remaining_cents(), 0);
        for amount in [f64::NAN, f64::INFINITY, f64::MAX] {
            assert!(manager.record_spend(amount).is_err());
        }
        assert_eq!(manager.get_remaining_cents(), 0);
    }

    #[test]
    fn test_budget_manager() {
        let manager = BudgetManager::new(100.0);

        assert_eq!(manager.get_remaining(), 100.0);

        let id1 = manager.reserve(5000).unwrap().unwrap();
        manager.settle(id1, 5000).unwrap();
        assert_eq!(manager.get_remaining(), 50.0);

        // Rejected admission must not debit the remaining balance.
        assert!(manager.reserve(6000).is_err());
        assert_eq!(manager.get_remaining(), 50.0);

        let err = manager.reserve(-1000).unwrap_err();
        assert_eq!(err, "reserve amount cannot be negative");

        let id2 = manager.reserve(1000).unwrap().unwrap(); // spend $10
        manager.settle(id2, 1000).unwrap();
        assert_eq!(manager.get_remaining(), 40.0);
        assert_eq!(manager.get_remaining_cents(), 4000);
    }

    #[test]
    fn test_budget_manager_exact_limit() {
        let manager = BudgetManager::new(100.0);
        assert_eq!(manager.get_remaining(), 100.0);

        // Spend exactly the limit
        let id = manager.reserve(10000).unwrap().unwrap();
        manager.settle(id, 10000).unwrap();
        assert_eq!(manager.get_remaining(), 0.0);
        assert_eq!(manager.get_remaining_cents(), 0);

        // One extra cent must be rejected without changing the balance.
        assert!(manager.reserve(1).is_err());
        assert_eq!(manager.get_remaining_cents(), 0);
    }

    #[test]
    fn test_concurrent_reserve_settle_release() {
        let manager = std::sync::Arc::new(BudgetManager::new(100.0)); // 10000 cents
        let mut handles = vec![];

        for _ in 0..5 {
            let m = manager.clone();
            handles.push(std::thread::spawn(move || {
                if let Ok(Some(id)) = m.reserve(2000) {
                    m.settle(id, 2000).unwrap();
                }
            }));
        }

        for h in handles {
            h.join().unwrap();
        }

        // At this point 10000 is consumed, check that further reservations fail
        assert_eq!(manager.get_remaining_cents(), 0);
        assert!(manager.reserve(1).is_err());

        // Add more money to test release
        let manager_release = std::sync::Arc::new(BudgetManager::new(100.0));
        let mut release_handles = vec![];

        for _ in 0..5 {
            let m = manager_release.clone();
            release_handles.push(std::thread::spawn(move || {
                if let Ok(Some(id)) = m.reserve(2000) {
                    m.release(id).unwrap();
                }
            }));
        }

        for h in release_handles {
            h.join().unwrap();
        }

        // All money should be back
        assert_eq!(manager_release.get_remaining_cents(), 10000);
    }

    #[test]
    fn test_budget_manager_with_telemetry() {
        let store = std::sync::Arc::new(::server_harness::telemetry::ViolationStore::new(None));

        let manager = BudgetManager::new(50.0).with_telemetry("tenant-123".to_string(), store);
        assert!(manager.telemetry_store.is_some());
        // Spend money to hit telemetry path without panic
        manager.record_spend_cents(1000).unwrap();

        // Ensure struct states updated correctly
        assert_eq!(manager.tenant_id, Some("tenant-123".to_string()));
        assert!(manager.telemetry_store.is_some());

        // Spend money to hit telemetry path without panic
        assert!(manager.record_spend(10.0).unwrap());
        assert_eq!(manager.get_remaining(), 30.0);
    }

    #[test]
    fn test_record_spend_cents_zero() {
        let manager = BudgetManager::new(100.0);
        assert!(manager.record_spend_cents(0).unwrap());
        assert_eq!(manager.get_remaining_cents(), 10000);
    }

    #[test]
    fn test_check_alert_threshold() {
        let manager = BudgetManager::new(100.0);

        // Not over threshold initially
        assert!(!manager.check_alert_threshold());

        // Spend 50%
        manager.record_spend(50.0).unwrap();
        assert!(!manager.check_alert_threshold());

        // Spend up to 80%
        manager.record_spend(30.0).unwrap();
        assert!(manager.check_alert_threshold()); // Default is 80.0

        // Custom threshold
        let custom_manager = BudgetManager::new(100.0).with_alert_threshold(90.0);
        custom_manager.record_spend(85.0).unwrap();
        assert!(!custom_manager.check_alert_threshold());

        custom_manager.record_spend(10.0).unwrap(); // 95%
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
        manager.record_spend_cents(5000).unwrap();
        assert!(!manager.check_alert_threshold_cents(10000));

        // Spend up to 80% (8000 cents)
        manager.record_spend_cents(3000).unwrap();
        assert!(manager.check_alert_threshold_cents(10000)); // Default is 80.0

        // Custom threshold using cents
        let custom_manager = BudgetManager::new(100.0).with_alert_threshold(90.0);
        custom_manager.record_spend_cents(8500).unwrap();
        assert!(!custom_manager.check_alert_threshold_cents(10000));

        custom_manager.record_spend_cents(1000).unwrap(); // 95%
        assert!(custom_manager.check_alert_threshold_cents(10000));

        // Exact threshold check
        let exact_manager = BudgetManager::new(100.0).with_alert_threshold(80.0);
        exact_manager.record_spend_cents(8000).unwrap();
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
        assert!(manager.record_spend(10.0).unwrap());
        assert_eq!(manager.get_remaining(), 40.0);
    }

    #[test]
    fn test_budget_manager_edge_cases() {
        let manager = BudgetManager::new(f64::MAX);
        assert!(manager.record_spend(1.0).unwrap());

        // Check extremely small threshold values
        let threshold_manager = BudgetManager::new(100.0).with_alert_threshold(0.01);
        threshold_manager.record_spend(0.02).unwrap();
        assert!(threshold_manager.check_alert_threshold());
    }

    #[test]
    fn test_is_spend_rate_too_high() {
        let manager = BudgetManager::new(100.0); // $100 limit, 10000 cents
        manager.record_spend(20.0).unwrap(); // 2000 cents
        let one_day = std::time::Duration::from_secs(86400);
        let thirty_days = std::time::Duration::from_secs(30 * 86400);
        assert!(manager.is_spend_rate_too_high(one_day, thirty_days)); // 20% in 1 day is way too high
        assert!(
            !manager
                .is_spend_rate_too_high(std::time::Duration::from_secs(10 * 86400), thirty_days)
        ); // 20% in 10 days is fine
    }

    #[test]
    fn test_is_spend_rate_too_high_edge_cases() {
        let zero_manager = BudgetManager::new(0.0);
        let one_day = std::time::Duration::from_secs(86400);
        assert!(!zero_manager.is_spend_rate_too_high(one_day, one_day));

        let manager = BudgetManager::new(100.0);
        assert!(!manager.is_spend_rate_too_high(one_day, std::time::Duration::from_secs(0)));
    }
}
