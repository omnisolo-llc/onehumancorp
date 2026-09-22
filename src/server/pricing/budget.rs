use std::sync::{Arc, Mutex};

pub struct BudgetState {
    pub total_allocated_cents: i64,
    pub settled_cents: i64,
}

pub struct BudgetManager {
    pub total_limit: f64,
    pub total_limit_cents: i64,
    pub state: Arc<Mutex<BudgetState>>,
    pub telemetry_store: Option<std::sync::Arc<::server_harness::telemetry::ViolationStore>>,
    pub tenant_id: Option<String>,
    pub alert_threshold_percent: f64,
}

pub struct BudgetReservation {
    state: Arc<Mutex<BudgetState>>,
    amount_cents: i64,
    settled: bool,
    tenant_id: Option<String>,
    telemetry_store: Option<std::sync::Arc<::server_harness::telemetry::ViolationStore>>,
}

impl Drop for BudgetReservation {
    fn drop(&mut self) {
        if !self.settled
            && let Ok(mut state) = self.state.lock()
        {
            state.total_allocated_cents = state
                .total_allocated_cents
                .saturating_sub(self.amount_cents);
        }
    }
}

impl BudgetReservation {
    pub fn settle(&mut self, final_amount_cents: i64) -> Result<(), String> {
        if self.settled {
            return Err("Reservation already settled".to_string());
        }
        if final_amount_cents < 0 {
            return Err("Settle amount cannot be negative".to_string());
        }

        if let (Some(store), Some(tid)) = (&self.telemetry_store, &self.tenant_id)
            && final_amount_cents > 0
        {
            store.llm_cost_counter.add(
                final_amount_cents as u64,
                &[opentelemetry::KeyValue::new("tenant_id", tid.to_string())],
            );
            store.mission_cost_cents.add(
                final_amount_cents as u64,
                &[opentelemetry::KeyValue::new("tenant_id", tid.to_string())],
            );
        }

        if let Ok(mut state) = self.state.lock() {
            state.settled_cents = state.settled_cents.saturating_add(final_amount_cents);
            state.total_allocated_cents = state
                .total_allocated_cents
                .saturating_sub(self.amount_cents);
        }
        self.settled = true;
        Ok(())
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
                total_allocated_cents: 0,
                settled_cents: 0,
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

    pub fn reserve(&self, amount_cents: i64) -> Result<BudgetReservation, String> {
        if amount_cents < 0 {
            return Err("reserve amount cannot be negative".to_string());
        }

        let mut state = self
            .state
            .lock()
            .map_err(|_| "Failed to acquire budget lock".to_string())?;

        let current_total = state
            .total_allocated_cents
            .checked_add(state.settled_cents)
            .ok_or_else(|| "Budget arithmetic overflow".to_string())?;

        let projected_total = current_total
            .checked_add(amount_cents)
            .ok_or_else(|| "Budget arithmetic overflow".to_string())?;

        if projected_total > self.total_limit_cents {
            return Err("Budget limit exceeded".to_string());
        }

        state.total_allocated_cents = state
            .total_allocated_cents
            .checked_add(amount_cents)
            .ok_or_else(|| "Budget arithmetic overflow".to_string())?;

        Ok(BudgetReservation {
            state: self.state.clone(),
            amount_cents,
            settled: false,
            tenant_id: self.tenant_id.clone(),
            telemetry_store: self.telemetry_store.clone(),
        })
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

        if let (Some(_store), Some(tid)) = (&self.telemetry_store, &self.tenant_id) {
            tracing::info!(
                "💰 Miser telemetry: Recording budget spend for tenant {}",
                tid
            ); // pii-safe
        }

        match self.reserve(amount_cents) {
            Ok(mut reservation) => {
                let _ = reservation.settle(amount_cents);
                Ok(true)
            }
            Err(_) => Ok(false),
        }
    }

    pub fn get_remaining(&self) -> f64 {
        (self.get_remaining_cents() as f64) / 100.0
    }

    pub fn get_remaining_cents(&self) -> i64 {
        if let Ok(state) = self.state.lock() {
            let used = state
                .total_allocated_cents
                .saturating_add(state.settled_cents);
            self.total_limit_cents.saturating_sub(used)
        } else {
            0
        }
    }

    pub fn get_total_allocated_cents(&self) -> i64 {
        if let Ok(state) = self.state.lock() {
            state.total_allocated_cents
        } else {
            0
        }
    }

    pub fn get_settled_cents(&self) -> i64 {
        if let Ok(state) = self.state.lock() {
            state.settled_cents
        } else {
            0
        }
    }

    pub fn check_alert_threshold(&self) -> bool {
        if self.total_limit_cents <= 0 {
            return false;
        }
        let current = if let Ok(state) = self.state.lock() {
            state
                .total_allocated_cents
                .saturating_add(state.settled_cents)
        } else {
            0
        };
        let usage_percent = (current as f64 / self.total_limit_cents as f64) * 100.0;
        usage_percent >= self.alert_threshold_percent
    }

    pub fn is_projected_cost_over_threshold(&self, projected_cost_cents: i64) -> bool {
        let current = if let Ok(state) = self.state.lock() {
            state
                .total_allocated_cents
                .saturating_add(state.settled_cents)
        } else {
            0
        };

        if self.total_limit_cents <= 0 {
            return projected_cost_cents > 0 || current > 0;
        }
        let limit_threshold_cents = ((self.total_limit_cents as f64)
            * (self.alert_threshold_percent / 100.0))
            .round() as i64;
        projected_cost_cents >= limit_threshold_cents || current >= limit_threshold_cents
    }

    pub fn check_alert_threshold_cents(&self, total_limit_cents: i64) -> bool {
        if total_limit_cents <= 0 {
            return false;
        }
        let current = if let Ok(state) = self.state.lock() {
            state
                .total_allocated_cents
                .saturating_add(state.settled_cents)
        } else {
            0
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
        let current = if let Ok(state) = self.state.lock() {
            state
                .total_allocated_cents
                .saturating_add(state.settled_cents)
        } else {
            0
        };
        let expected_spend = (self.total_limit_cents as f64)
            * (time_elapsed.as_secs() as f64 / total_duration.as_secs() as f64);
        current as f64 > expected_spend * 1.5 // 50% higher than expected rate
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn concurrent_admission_never_exceeds_limit() {
        let manager = std::sync::Arc::new(BudgetManager::new(1.0)); // 100 cents
        let workers: Vec<_> = (0..32)
            .map(|_| {
                let manager = manager.clone();
                thread::spawn(move || {
                    if let Ok(mut reservation) = manager.reserve(10) {
                        // Keep hold of reservation for a bit
                        let _ = reservation.settle(10);
                        true
                    } else {
                        false
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

        assert!(!manager.check_alert_threshold());

        manager.record_spend(50.0).unwrap();
        assert!(!manager.check_alert_threshold());

        manager.record_spend(30.0).unwrap();
        assert!(manager.check_alert_threshold()); // Default is 80.0

        let custom_manager = BudgetManager::new(100.0).with_alert_threshold(90.0);
        custom_manager.record_spend(85.0).unwrap();
        assert!(!custom_manager.check_alert_threshold());

        custom_manager.record_spend(10.0).unwrap();
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

        assert!(!manager.check_alert_threshold_cents(10000));

        manager.record_spend_cents(5000).unwrap();
        assert!(!manager.check_alert_threshold_cents(10000));

        manager.record_spend_cents(3000).unwrap();
        assert!(manager.check_alert_threshold_cents(10000));

        let custom_manager = BudgetManager::new(100.0).with_alert_threshold(90.0);
        custom_manager.record_spend_cents(8500).unwrap();
        assert!(!custom_manager.check_alert_threshold_cents(10000));

        custom_manager.record_spend_cents(1000).unwrap();
        assert!(custom_manager.check_alert_threshold_cents(10000));

        let exact_manager = BudgetManager::new(100.0).with_alert_threshold(80.0);
        exact_manager.record_spend_cents(8000).unwrap();
        assert!(exact_manager.check_alert_threshold_cents(10000));
    }

    #[test]
    fn test_check_alert_threshold_with_projected_costs() {
        let manager = BudgetManager::new(10.0);
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

    #[test]
    fn test_budget_reservation_settle() {
        let manager = BudgetManager::new(10.0); // 1000 cents

        let mut reservation = manager.reserve(500).unwrap();
        assert_eq!(manager.get_total_allocated_cents(), 500);
        assert_eq!(manager.get_settled_cents(), 0);
        assert_eq!(manager.get_remaining_cents(), 500);

        // Settle for less than reserved
        assert!(reservation.settle(300).is_ok());

        assert_eq!(manager.get_total_allocated_cents(), 0);
        assert_eq!(manager.get_settled_cents(), 300);
        assert_eq!(manager.get_remaining_cents(), 700);

        // Can't settle twice
        assert!(reservation.settle(100).is_err());
    }

    #[test]
    fn test_budget_reservation_drop() {
        let manager = BudgetManager::new(10.0); // 1000 cents

        {
            let _reservation = manager.reserve(600).unwrap();
            assert_eq!(manager.get_total_allocated_cents(), 600);
            assert_eq!(manager.get_remaining_cents(), 400);
            // goes out of scope without settle
        }

        // Funds should be returned
        assert_eq!(manager.get_total_allocated_cents(), 0);
        assert_eq!(manager.get_settled_cents(), 0);
        assert_eq!(manager.get_remaining_cents(), 1000);
    }
}
