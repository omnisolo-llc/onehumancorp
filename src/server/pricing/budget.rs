use std::sync::Mutex;

#[derive(Debug, Default)]
struct BudgetState {
    settled_micros: i64,
    in_flight_micros: i64,
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
            state: Mutex::new(BudgetState::default()),
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

    pub fn reserve(&self, amount_micros: i64) -> Result<String, String> {
        if amount_micros < 0 {
            return Err("reservation amount cannot be negative".to_string());
        }
        if amount_micros == 0 {
            return Ok("res_0".to_string());
        }

        let mut state = self.state.lock().unwrap();

        // Use checked_add for all additions to correctly identify overflow as exceeding the limit
        let current_total_micros = state.settled_micros.checked_add(state.in_flight_micros);

        let total_limit_micros = if self.total_limit_cents == i64::MAX {
            i64::MAX
        } else {
            self.total_limit_cents.saturating_mul(10_000)
        };

        let new_total = current_total_micros.and_then(|t| t.checked_add(amount_micros));

        match new_total {
            Some(total) if total <= total_limit_micros => {
                // Inside limit
            }
            _ => {
                // Either overflowed or exceeded limit
                // Except if total_limit_micros is i64::MAX and we overflowed, it still exceeds
                // Actually if total_limit_micros is i64::MAX, any overflow means we exceeded it or reached it.
                // But if amount is > 0 and we are at MAX, we should reject.
                // Wait, if it overflows, it definitely exceeds any valid limit.
                return Err("reservation exceeds budget limit".to_string());
            }
        }

        state.in_flight_micros = state
            .in_flight_micros
            .checked_add(amount_micros)
            .unwrap_or(i64::MAX);

        let reservation_id = uuid::Uuid::new_v4().to_string();
        Ok(reservation_id)
    }

    pub fn settle(
        &self,
        _reservation_id: &str,
        actual_amount_micros: i64,
        original_reservation_micros: i64,
    ) -> Result<(), String> {
        if actual_amount_micros < 0 {
            return Err("settlement amount cannot be negative".to_string());
        }

        let mut state = self.state.lock().unwrap();
        // Decrease in-flight by the original reservation amount, ensuring we don't go below 0
        state.in_flight_micros = state
            .in_flight_micros
            .saturating_sub(original_reservation_micros)
            .max(0);

        // Increase settled amount by the actual amount
        state.settled_micros = state.settled_micros.saturating_add(actual_amount_micros);

        let actual_cents = (actual_amount_micros as f64 / 10_000.0).round() as i64;

        if let (Some(store), Some(tid)) = (&self.telemetry_store, &self.tenant_id)
            && actual_cents > 0
        {
            tracing::info!(
                "💰 Miser telemetry: Recording budget spend for tenant {}",
                tid
            );
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

    pub fn release(
        &self,
        _reservation_id: &str,
        original_reservation_micros: i64,
    ) -> Result<(), String> {
        let mut state = self.state.lock().unwrap();
        state.in_flight_micros = state
            .in_flight_micros
            .saturating_sub(original_reservation_micros)
            .max(0);
        Ok(())
    }

    pub fn record_spend_cents(&self, amount_cents: i64) -> Result<bool, String> {
        if amount_cents < 0 {
            return Err("spend amount cannot be negative".to_string());
        }
        if amount_cents == 0 {
            return Ok(self.get_remaining_cents() >= 0);
        }

        let amount_micros = amount_cents.saturating_mul(10_000);
        match self.reserve(amount_micros) {
            Ok(res_id) => {
                self.settle(&res_id, amount_micros, amount_micros)?;
                Ok(true)
            }
            Err(_) => Ok(false),
        }
    }

    pub fn get_remaining(&self) -> f64 {
        let state = self.state.lock().unwrap();
        let current_cents = if state.settled_micros == i64::MAX {
            i64::MAX
        } else {
            (state.settled_micros as f64 / 10_000.0).round() as i64
        };
        (self.total_limit_cents.saturating_sub(current_cents)) as f64 / 100.0
    }

    pub fn get_remaining_cents(&self) -> i64 {
        let state = self.state.lock().unwrap();
        let current_cents = if state.settled_micros == i64::MAX {
            i64::MAX
        } else {
            (state.settled_micros as f64 / 10_000.0).round() as i64
        };
        self.total_limit_cents.saturating_sub(current_cents)
    }

    pub fn check_alert_threshold(&self) -> bool {
        if self.total_limit_cents <= 0 {
            return false;
        }
        let state = self.state.lock().unwrap();
        let current_cents = if state.settled_micros == i64::MAX {
            i64::MAX
        } else {
            (state.settled_micros as f64 / 10_000.0).round() as i64
        };
        let usage_percent = (current_cents as f64 / self.total_limit_cents as f64) * 100.0;
        usage_percent >= self.alert_threshold_percent
    }

    pub fn is_projected_cost_over_threshold(&self, projected_cost_cents: i64) -> bool {
        let state = self.state.lock().unwrap();
        let current_cents = if state.settled_micros == i64::MAX {
            i64::MAX
        } else {
            (state.settled_micros as f64 / 10_000.0).round() as i64
        };

        if self.total_limit_cents <= 0 {
            return projected_cost_cents > 0 || current_cents > 0;
        }
        let limit_threshold_cents = ((self.total_limit_cents as f64)
            * (self.alert_threshold_percent / 100.0))
            .round() as i64;
        projected_cost_cents >= limit_threshold_cents || current_cents >= limit_threshold_cents
    }

    pub fn check_alert_threshold_cents(&self, total_limit_cents: i64) -> bool {
        if total_limit_cents <= 0 {
            return false;
        }
        let state = self.state.lock().unwrap();
        let current_cents = if state.settled_micros == i64::MAX {
            i64::MAX
        } else {
            (state.settled_micros as f64 / 10_000.0).round() as i64
        };
        let limit_threshold_cents =
            ((total_limit_cents as f64) * (self.alert_threshold_percent / 100.0)).round() as i64;
        current_cents >= limit_threshold_cents
    }

    pub fn is_spend_rate_too_high(
        &self,
        time_elapsed: std::time::Duration,
        total_duration: std::time::Duration,
    ) -> bool {
        if self.total_limit_cents <= 0 || total_duration.as_secs() == 0 {
            return false;
        }
        let state = self.state.lock().unwrap();
        let current_cents = if state.settled_micros == i64::MAX {
            i64::MAX
        } else {
            (state.settled_micros as f64 / 10_000.0).round() as i64
        };
        let expected_spend = (self.total_limit_cents as f64)
            * (time_elapsed.as_secs() as f64 / total_duration.as_secs() as f64);
        current_cents as f64 > expected_spend * 1.5 // 50% higher than expected rate
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
    fn concurrent_reserve_settle_never_exceeds_limit() {
        let manager = std::sync::Arc::new(BudgetManager::new(1.0));
        let workers: Vec<_> = (0..32)
            .map(|_| {
                let manager = manager.clone();
                std::thread::spawn(move || {
                    let amount_micros = 10 * 10_000;
                    match manager.reserve(amount_micros) {
                        Ok(res_id) => {
                            manager
                                .settle(&res_id, amount_micros, amount_micros)
                                .unwrap();
                            true
                        }
                        Err(_) => false,
                    }
                })
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
    fn concurrent_reserve_release_never_exceeds_limit() {
        let manager = std::sync::Arc::new(BudgetManager::new(1.0));
        let workers: Vec<_> = (0..32)
            .map(|_| {
                let manager = manager.clone();
                std::thread::spawn(move || {
                    let amount_micros = 10 * 10_000;
                    // Keep the reservation active to ensure we hit the limit
                    manager.reserve(amount_micros)
                })
            })
            .collect();
        let results: Vec<_> = workers
            .into_iter()
            .map(|worker| worker.join().unwrap())
            .collect();

        let admitted = results.iter().filter(|r| r.is_ok()).count();
        assert_eq!(admitted, 10);

        // Now release them
        for res_id in results.into_iter().flatten() {
            manager.release(&res_id, 10 * 10_000).unwrap();
        }
        assert_eq!(manager.get_remaining_cents(), 100);
    }

    #[test]
    fn reserve_and_partial_settle() {
        let manager = BudgetManager::new(1.0);
        let res_id = manager.reserve(100 * 10_000).unwrap(); // Reserve $1.00

        assert_eq!(manager.get_remaining_cents(), 100); // In flight doesn't affect reported cents directly, but...
        // Actually, get_remaining_cents looks at settled.
        // Wait, let's verify remaining logic: it checks settled_micros.

        manager.settle(&res_id, 50 * 10_000, 100 * 10_000).unwrap();
        assert_eq!(manager.get_remaining_cents(), 50);

        let err = manager.reserve(60 * 10_000);
        assert!(err.is_err()); // Exceeds budget (50 + 60 = 110 > 100)
    }

    #[test]
    fn invalid_amounts_and_overflow_fail_closed() {
        for limit in [f64::NAN, f64::INFINITY, -1.0] {
            assert!(!BudgetManager::new(limit).record_spend_cents(1).unwrap());
        }
        let manager = BudgetManager::new(f64::MAX);
        // Reserving MAX cents will be clamped or allowed. Since limit is i64::MAX, it has MAX micros.
        // Wait, if amount_cents is i64::MAX, saturating_mul(10_000) will be i64::MAX.
        assert!(manager.record_spend_cents(i64::MAX).unwrap());
        // Since we settled i64::MAX micros, the next reservation should fail.
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
