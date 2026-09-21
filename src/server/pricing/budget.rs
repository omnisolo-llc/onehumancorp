use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Default)]
pub struct BudgetState {
    pub settled_micros: i64,
    pub reservations: HashMap<String, i64>,
}

impl BudgetState {
    pub fn current_exposure_micros(&self) -> i64 {
        self.settled_micros + self.reservations.values().copied().sum::<i64>()
    }
}

pub struct BudgetManager {
    pub total_limit: f64,
    pub total_limit_cents: i64,
    pub total_limit_micros: i64,
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
        let total_limit_micros = if total_limit_cents == i64::MAX {
            i64::MAX
        } else {
            total_limit_cents.saturating_mul(10_000)
        };
        BudgetManager {
            total_limit: limit,
            total_limit_cents,
            total_limit_micros,
            state: Mutex::new(BudgetState::default()),
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

    pub fn reserve_micros(&self, reservation_id: &str, amount_micros: i64) -> Result<bool, String> {
        if amount_micros < 0 {
            return Err("reserve amount cannot be negative".to_string());
        }
        let mut state = self.state.lock().map_err(|_| "lock poisoned".to_string())?;

        // Replay check: if already reserved with this ID, do nothing (idempotent).
        // Wait, if it exists, should we update the amount? Or just keep it? We keep it or update it?
        // Usually, idempotency means same ID = same transaction. Let's just return true if it already exists.
        if state.reservations.contains_key(reservation_id) {
            return Ok(true);
        }

        let new_exposure = state.current_exposure_micros().checked_add(amount_micros);
        if let Some(exposure) = new_exposure
            && exposure <= self.total_limit_micros
        {
            state
                .reservations
                .insert(reservation_id.to_string(), amount_micros);
            return Ok(true);
        }
        Ok(false)
    }

    pub fn settle_micros(
        &self,
        reservation_id: &str,
        final_cost_micros: i64,
    ) -> Result<bool, String> {
        if final_cost_micros < 0 {
            return Err("settle amount cannot be negative".to_string());
        }
        let mut state = self.state.lock().map_err(|_| "lock poisoned".to_string())?;

        // If the reservation exists, we remove it and add the final cost to settled.
        if state.reservations.remove(reservation_id).is_some() {
            // Check overflow just in case, but it shouldn't normally happen.
            if let Some(new_settled) = state.settled_micros.checked_add(final_cost_micros) {
                // Should we enforce total_limit on settlement? Even if final_cost > reserved,
                // we might exceed total limit, but we must settle the actual cost. Let's just add it.
                state.settled_micros = new_settled;
            } else {
                state.settled_micros = i64::MAX;
            }
            if let (Some(store), Some(tid)) = (&self.telemetry_store, &self.tenant_id)
                && final_cost_micros > 0
            {
                let amount_cents = (final_cost_micros / 10_000) as u64; // rough translation
                store.llm_cost_counter.add(
                    amount_cents,
                    &[opentelemetry::KeyValue::new("tenant_id", tid.to_string())],
                );
                store.mission_cost_cents.add(
                    amount_cents,
                    &[opentelemetry::KeyValue::new("tenant_id", tid.to_string())],
                );
            }
            return Ok(true);
        }

        // If not found, it might already be settled (replay). So we just return Ok(true).
        Ok(true)
    }

    pub fn release_micros(&self, reservation_id: &str) -> Result<bool, String> {
        let mut state = self.state.lock().map_err(|_| "lock poisoned".to_string())?;
        state.reservations.remove(reservation_id);
        Ok(true)
    }

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

        let amount_micros = amount_cents.saturating_mul(10_000);

        let mut state = self.state.lock().map_err(|_| "lock poisoned".to_string())?;

        let new_exposure = state.current_exposure_micros().checked_add(amount_micros);
        if let Some(exposure) = new_exposure
            && exposure <= self.total_limit_micros
        {
            state.settled_micros += amount_micros;

            if let (Some(store), Some(tid)) = (&self.telemetry_store, &self.tenant_id) {
                store.llm_cost_counter.add(
                    amount_cents as u64,
                    &[opentelemetry::KeyValue::new("tenant_id", tid.to_string())],
                );
                store.mission_cost_cents.add(
                    amount_cents as u64,
                    &[opentelemetry::KeyValue::new("tenant_id", tid.to_string())],
                );
            }
            return Ok(true);
        }
        Ok(false)
    }

    pub fn get_remaining(&self) -> f64 {
        let state = self.state.lock().unwrap();
        (self.total_limit_micros - state.current_exposure_micros()) as f64 / 1_000_000.0
    }

    pub fn get_remaining_cents(&self) -> i64 {
        let state = self.state.lock().unwrap();
        (self.total_limit_micros - state.current_exposure_micros()) / 10_000
    }

    pub fn check_alert_threshold(&self) -> bool {
        if self.total_limit_micros <= 0 {
            return false;
        }
        let state = self.state.lock().unwrap();
        let exposure = state.current_exposure_micros();
        let usage_percent = (exposure as f64 / self.total_limit_micros as f64) * 100.0;
        usage_percent >= self.alert_threshold_percent
    }

    pub fn is_projected_cost_over_threshold(&self, projected_cost_cents: i64) -> bool {
        let state = self.state.lock().unwrap();
        let exposure = state.current_exposure_micros();
        let projected_micros = projected_cost_cents.saturating_mul(10_000);
        if self.total_limit_micros <= 0 {
            return projected_micros > 0 || exposure > 0;
        }
        let limit_threshold_micros = ((self.total_limit_micros as f64)
            * (self.alert_threshold_percent / 100.0))
            .round() as i64;
        projected_micros >= limit_threshold_micros || exposure >= limit_threshold_micros
    }

    pub fn check_alert_threshold_cents(&self, total_limit_cents: i64) -> bool {
        if total_limit_cents <= 0 {
            return false;
        }
        let state = self.state.lock().unwrap();
        let exposure = state.current_exposure_micros();
        let limit_threshold_micros = ((total_limit_cents as f64 * 10_000.0)
            * (self.alert_threshold_percent / 100.0))
            .round() as i64;
        exposure >= limit_threshold_micros
    }

    pub fn is_spend_rate_too_high(
        &self,
        time_elapsed: std::time::Duration,
        total_duration: std::time::Duration,
    ) -> bool {
        if self.total_limit_micros <= 0 || total_duration.as_secs() == 0 {
            return false;
        }
        let state = self.state.lock().unwrap();
        let exposure = state.current_exposure_micros();
        let expected_spend = (self.total_limit_micros as f64)
            * (time_elapsed.as_secs() as f64 / total_duration.as_secs() as f64);
        exposure as f64 > expected_spend * 1.5 // 50% higher than expected rate
    }
}

#[cfg(test)]
mod tests {
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
    fn test_reserve_settle_release() {
        let manager = BudgetManager::new(1.0); // 100 cents, 1,000,000 micros
        // reserve 50 cents
        assert!(manager.reserve_micros("task1", 500_000).unwrap());
        assert_eq!(manager.get_remaining_cents(), 50);

        // reserve another 60 cents should fail
        assert!(!manager.reserve_micros("task2", 600_000).unwrap());
        assert_eq!(manager.get_remaining_cents(), 50);

        // settle task1 for 40 cents
        assert!(manager.settle_micros("task1", 400_000).unwrap());
        assert_eq!(manager.get_remaining_cents(), 60);

        // reserve 60 cents should pass now
        assert!(manager.reserve_micros("task2", 600_000).unwrap());
        assert_eq!(manager.get_remaining_cents(), 0);

        // release task2
        assert!(manager.release_micros("task2").unwrap());
        assert_eq!(manager.get_remaining_cents(), 60);
    }

    #[test]
    fn test_replay_restart_idempotency() {
        let manager = BudgetManager::new(1.0); // 1,000,000 micros

        assert!(manager.reserve_micros("task1", 500_000).unwrap());

        // Try reserving same ID again
        assert!(manager.reserve_micros("task1", 500_000).unwrap());
        assert_eq!(manager.get_remaining_cents(), 50); // Did not double-reserve

        // Try settling same ID twice
        assert!(manager.settle_micros("task1", 500_000).unwrap());
        assert_eq!(manager.get_remaining_cents(), 50);

        assert!(manager.settle_micros("task1", 500_000).unwrap());
        assert_eq!(manager.get_remaining_cents(), 50); // Idempotent settlement (returns true, changes nothing)
    }

    #[test]
    fn test_concurrent_reserve_settle() {
        let manager = std::sync::Arc::new(BudgetManager::new(10.0)); // 10,000,000 micros
        let workers: Vec<_> = (0..100)
            .map(|i| {
                let manager = manager.clone();
                std::thread::spawn(move || {
                    let task_id = format!("task_{}", i);
                    if manager.reserve_micros(&task_id, 200_000).unwrap() {
                        // reserve 20 cents
                        // settle for 10 cents
                        manager.settle_micros(&task_id, 100_000).unwrap();
                        1
                    } else {
                        0
                    }
                })
            })
            .collect();
        let admitted = workers
            .into_iter()
            .map(|worker| worker.join().unwrap())
            .sum::<usize>();
        // Total limit 1000 cents.
        // Even if 100 threads try to reserve 20 cents (which takes 2000 cents),
        // at most 50 can be in-flight at once, but they also settle immediately.
        // We know that exactly 100 threads can complete if they settle fast enough,
        // or fewer if they overlap.
        // We just ensure we don't violate limits.
        let remaining = manager.get_remaining_cents();
        // Since each successful task settles for 10 cents, total settled is admitted * 10.
        // Limit is 1000. So admitted <= 100.
        assert!(admitted <= 100);
        assert_eq!(remaining, 1000 - (admitted as i64 * 10));
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
