use std::sync::{Arc, Mutex};

pub struct BudgetState {
    pub total_allocated: i64,
    pub settled: i64,
}

pub struct BudgetManager {
    pub total_limit: f64,
    pub total_limit_cents: i64,
    state: Arc<Mutex<BudgetState>>,
    pub telemetry_store: Option<std::sync::Arc<::server_harness::telemetry::ViolationStore>>,
    tenant_id: Option<String>,
    pub alert_threshold_percent: f64,
}

pub struct BudgetReservation {
    state: Arc<Mutex<BudgetState>>,
    amount_cents: i64,
    is_settled: bool,
    telemetry_store: Option<std::sync::Arc<::server_harness::telemetry::ViolationStore>>,
    tenant_id: Option<String>,
}

impl BudgetReservation {
    pub fn settle(&mut self) {
        if self.is_settled {
            return;
        }

        let amount_cents = self.amount_cents;
        let mut emit_telemetry = false;

        {
            let mut state = self.state.lock().unwrap();
            if let Some(next_settled) = state.settled.checked_add(amount_cents) {
                state.settled = next_settled;
                // Note: total_allocated remains elevated to reflect the spent funds.
            } else {
                // Handle overflow if needed, but in theory total_allocated bounds this
                state.settled = state.settled.saturating_add(amount_cents);
            }
            self.is_settled = true;
            if self.telemetry_store.is_some() && self.tenant_id.is_some() {
                emit_telemetry = true;
            }
        }

        if emit_telemetry && amount_cents > 0 {
            let tid = self.tenant_id.as_ref().unwrap();
            let store = self.telemetry_store.as_ref().unwrap();

            tracing::info!(
                "💰 Miser telemetry: Recording budget spend for tenant {}",
                tid
            ); // pii-safe

            store.llm_cost_counter.add(
                amount_cents as u64,
                &[opentelemetry::KeyValue::new("tenant_id", tid.to_string())],
            );
            store.mission_cost_cents.add(
                amount_cents as u64,
                &[opentelemetry::KeyValue::new("tenant_id", tid.to_string())],
            );
        }
    }

    pub fn release(&mut self) {
        if self.is_settled {
            return; // Cannot release if already settled
        }
        if let Ok(mut state) = self.state.lock() {
            state.total_allocated = state.total_allocated.saturating_sub(self.amount_cents);
            self.is_settled = true; // Prevents Drop from releasing it again
            self.amount_cents = 0; // Prevent further operations
        }
    }
}

#[allow(clippy::collapsible_if)]
impl Drop for BudgetReservation {
    fn drop(&mut self) {
        if !self.is_settled {
            if let Ok(mut state) = self.state.lock() {
                state.total_allocated = state.total_allocated.saturating_sub(self.amount_cents);
            }
        }
    }
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
            state: Arc::new(Mutex::new(BudgetState {
                total_allocated: 0,
                settled: 0,
            })),
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

    pub fn record_spend_cents(&self, amount_cents: i64) -> Result<bool, String> {
        if amount_cents < 0 {
            return Err("spend amount cannot be negative".to_string());
        }
        if amount_cents == 0 {
            return Ok(self.get_remaining_cents() >= 0);
        }

        let reservation = self.reserve_cents(amount_cents)?;
        if let Some(mut r) = reservation {
            r.settle();
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn reserve(&self, amount: f64) -> Result<Option<BudgetReservation>, String> {
        if amount < 0.0 {
            return Err("spend amount cannot be negative".to_string());
        }
        if !amount.is_finite() || amount * 100.0 >= i64::MAX as f64 {
            return Err("spend amount must be finite and bounded".to_string());
        }
        let amount_cents = (amount * 100.0).round() as i64;
        self.reserve_cents(amount_cents)
    }

    pub fn reserve_cents(&self, amount_cents: i64) -> Result<Option<BudgetReservation>, String> {
        if amount_cents < 0 {
            return Err("spend amount cannot be negative".to_string());
        }
        if amount_cents == 0 {
            if self.get_remaining_cents() >= 0 {
                return Ok(Some(BudgetReservation {
                    state: self.state.clone(),
                    amount_cents: 0,
                    is_settled: false,
                    telemetry_store: self.telemetry_store.clone(),
                    tenant_id: self.tenant_id.clone(),
                }));
            } else {
                return Ok(None);
            }
        }

        let mut state = self.state.lock().unwrap();
        if let Some(next) = state.total_allocated.checked_add(amount_cents).filter(|&next| next <= self.total_limit_cents) {
            state.total_allocated = next;
            return Ok(Some(BudgetReservation {
                state: self.state.clone(),
                amount_cents,
                is_settled: false,
                telemetry_store: self.telemetry_store.clone(),
                tenant_id: self.tenant_id.clone(),
            }));
        }
        Ok(None)
    }

    pub fn get_remaining(&self) -> f64 {
        let current = self.state.lock().unwrap().total_allocated;
        (self.total_limit_cents - current) as f64 / 100.0
    }

    pub fn get_remaining_cents(&self) -> i64 {
        let current = self.state.lock().unwrap().total_allocated;
        self.total_limit_cents - current
    }

    pub fn check_alert_threshold(&self) -> bool {
        if self.total_limit_cents <= 0 {
            return false;
        }
        let current = self.state.lock().unwrap().total_allocated;
        let usage_percent = (current as f64 / self.total_limit_cents as f64) * 100.0;
        usage_percent >= self.alert_threshold_percent
    }

    pub fn is_projected_cost_over_threshold(&self, projected_cost_cents: i64) -> bool {
        if self.total_limit_cents <= 0 {
            return projected_cost_cents > 0 || self.state.lock().unwrap().total_allocated > 0;
        }
        let limit_threshold_cents = ((self.total_limit_cents as f64)
            * (self.alert_threshold_percent / 100.0))
            .round() as i64;
        projected_cost_cents >= limit_threshold_cents
            || self.state.lock().unwrap().total_allocated >= limit_threshold_cents
    }

    pub fn check_alert_threshold_cents(&self, total_limit_cents: i64) -> bool {
        if total_limit_cents <= 0 {
            return false;
        }
        let current = self.state.lock().unwrap().total_allocated;
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
        let current = self.state.lock().unwrap().total_allocated;
        let expected_spend = (self.total_limit_cents as f64)
            * (time_elapsed.as_secs() as f64 / total_duration.as_secs() as f64);
        current as f64 > expected_spend * 1.5 // 50% higher than expected rate
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_budget_reservation_settle_release() {
        let manager = BudgetManager::new(100.0);

        // Reserve 50
        let mut res1 = manager.reserve(50.0).unwrap().unwrap();
        assert_eq!(manager.get_remaining(), 50.0);

        // Release it
        res1.release();
        assert_eq!(manager.get_remaining(), 100.0);

        // Reserve 60 and settle
        let mut res2 = manager.reserve(60.0).unwrap().unwrap();
        assert_eq!(manager.get_remaining(), 40.0);
        res2.settle();

        // Attempt to release after settle does nothing
        res2.release();
        assert_eq!(manager.get_remaining(), 40.0);

        // Ensure state contains correct values
        let state = manager.state.lock().unwrap();
        assert_eq!(state.total_allocated, 6000);
        assert_eq!(state.settled, 6000);
    }

    #[test]
    fn test_budget_reservation_drop() {
        let manager = BudgetManager::new(100.0);

        {
            let _res1 = manager.reserve(50.0).unwrap().unwrap();
            assert_eq!(manager.get_remaining(), 50.0);
            // Drops here
        }
        assert_eq!(manager.get_remaining(), 100.0);

        {
            let mut res2 = manager.reserve(60.0).unwrap().unwrap();
            assert_eq!(manager.get_remaining(), 40.0);
            res2.settle();
            // Drops here but is settled
        }
        assert_eq!(manager.get_remaining(), 40.0);
    }

    #[test]
    fn test_concurrent_budget_reservations() {
        let manager = std::sync::Arc::new(BudgetManager::new(100.0));
        let mut workers = Vec::new();

        for i in 0..150 {
            let manager = manager.clone();
            workers.push(std::thread::spawn(move || {
                if let Ok(Some(mut res)) = manager.reserve_cents(100) {
                    if i % 2 == 0 {
                        res.settle();
                    } else {
                        res.release(); // explicitly released or dropped
                    }
                    true
                } else {
                    false
                }
            }));
        }

        let mut _success_count = 0;
        for w in workers {
            if w.join().unwrap() {
                _success_count += 1;
            }
        }

        // It shouldn't admit all 150 because we have 100 limit initially
        // But with release, we might be able to admit more depending on concurrency.
        // What is guaranteed is that state.total_allocated <= 10000.
        let state = manager.state.lock().unwrap();
        assert!(state.total_allocated <= 10000);
    }

    use super::*;

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

        assert!(manager.record_spend(50.0).unwrap());
        assert_eq!(manager.get_remaining(), 50.0);

        // Rejected admission must not debit the remaining balance.
        assert!(!manager.record_spend(60.0).unwrap());
        assert_eq!(manager.get_remaining(), 50.0);

        let err = manager.record_spend(-10.0).unwrap_err();
        assert_eq!(err, "spend amount cannot be negative");

        assert!(manager.record_spend_cents(1000).unwrap()); // spend $10
        assert_eq!(manager.get_remaining(), 40.0);
        assert_eq!(manager.get_remaining_cents(), 4000);
    }

    #[test]
    fn test_budget_manager_exact_limit() {
        let manager = BudgetManager::new(100.0);
        assert_eq!(manager.get_remaining(), 100.0);

        // Spend exactly the limit
        assert!(manager.record_spend(100.0).unwrap());
        assert_eq!(manager.get_remaining(), 0.0);
        assert_eq!(manager.get_remaining_cents(), 0);

        // One extra cent must be rejected without changing the balance.
        assert!(!manager.record_spend(0.01).unwrap());
        assert_eq!(manager.get_remaining_cents(), 0);
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
