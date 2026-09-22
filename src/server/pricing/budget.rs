use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct BudgetState {
    pub total_allocated: i64,
    pub settled: i64,
}

pub struct BudgetReservation {
    state: Arc<Mutex<BudgetState>>,
    reserved_cents: i64,
    telemetry_store: Option<Arc<::server_harness::telemetry::ViolationStore>>,
    tenant_id: Option<String>,
    is_settled: bool,
}

impl Drop for BudgetReservation {
    fn drop(&mut self) {
        if !self.is_settled {
            if let Ok(mut state) = self.state.lock() {
                state.total_allocated = state.total_allocated.saturating_sub(self.reserved_cents);
            }
        }
    }
}


impl std::fmt::Debug for BudgetReservation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BudgetReservation")
            .field("reserved_cents", &self.reserved_cents)
            .field("is_settled", &self.is_settled)
            .field("tenant_id", &self.tenant_id)
            .finish()
    }
}

impl BudgetReservation {
    pub fn settle(mut self) {
        if self.is_settled {
            return;
        }
        if let Ok(mut state) = self.state.lock() {
            state.settled = state.settled.saturating_add(self.reserved_cents);
            state.total_allocated = state.total_allocated.saturating_sub(self.reserved_cents);
        }
        self.is_settled = true;

        if let (Some(store), Some(tid)) = (&self.telemetry_store, &self.tenant_id) {
            if self.reserved_cents > 0 {
                store.llm_cost_counter.add(
                    self.reserved_cents as u64,
                    &[opentelemetry::KeyValue::new("tenant_id", tid.to_string())],
                );
                store.mission_cost_cents.add(
                    self.reserved_cents as u64,
                    &[opentelemetry::KeyValue::new("tenant_id", tid.to_string())],
                );
            }
        }
    }
}

pub struct BudgetManager {
    pub total_limit: f64,
    pub total_limit_cents: i64,
    state: Arc<Mutex<BudgetState>>,
    pub telemetry_store: Option<Arc<::server_harness::telemetry::ViolationStore>>,
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
            state: Arc::new(Mutex::new(BudgetState { total_allocated: 0, settled: 0 })),
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
        store: Arc<::server_harness::telemetry::ViolationStore>,
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
        if let Some(reservation) = self.reserve_cents(amount_cents)? {
            reservation.settle();
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
                    reserved_cents: 0,
                    telemetry_store: self.telemetry_store.clone(),
                    tenant_id: self.tenant_id.clone(),
                    is_settled: false,
                }));
            } else {
                return Ok(None);
            }
        }

        if let (Some(_store), Some(tid)) = (&self.telemetry_store, &self.tenant_id) {
            tracing::info!(
                "💰 Miser telemetry: Recording budget spend for tenant {}",
                tid
            ); // pii-safe
        }

        let mut lock = self.state.lock().map_err(|_| "PoisonError".to_string())?;

        let total = lock.total_allocated.saturating_add(lock.settled);
        if let Some(new_total) = total.checked_add(amount_cents) {
            if new_total <= self.total_limit_cents {
                lock.total_allocated = lock.total_allocated.saturating_add(amount_cents);
                drop(lock);
                return Ok(Some(BudgetReservation {
                    state: self.state.clone(),
                    reserved_cents: amount_cents,
                    telemetry_store: self.telemetry_store.clone(),
                    tenant_id: self.tenant_id.clone(),
                    is_settled: false,
                }));
            }
        }

        Ok(None)
    }

    pub fn get_remaining(&self) -> f64 {
        self.get_remaining_cents() as f64 / 100.0
    }

    pub fn get_remaining_cents(&self) -> i64 {
        if let Ok(lock) = self.state.lock() {
            self.total_limit_cents - lock.total_allocated.saturating_add(lock.settled)
        } else {
            0
        }
    }

    pub fn check_alert_threshold(&self) -> bool {
        if self.total_limit_cents <= 0 {
            return false;
        }
        let current = if let Ok(lock) = self.state.lock() {
            lock.total_allocated.saturating_add(lock.settled)
        } else {
            0
        };
        let usage_percent = (current as f64 / self.total_limit_cents as f64) * 100.0;
        usage_percent >= self.alert_threshold_percent
    }

    pub fn is_projected_cost_over_threshold(&self, projected_cost_cents: i64) -> bool {
        let current = if let Ok(lock) = self.state.lock() {
            lock.total_allocated.saturating_add(lock.settled)
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
        let current = if let Ok(lock) = self.state.lock() {
            lock.total_allocated.saturating_add(lock.settled)
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
        let current = if let Ok(lock) = self.state.lock() {
            lock.total_allocated.saturating_add(lock.settled)
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

    #[test]
    fn concurrent_admission_never_exceeds_limit() {
        let manager = std::sync::Arc::new(BudgetManager::new(1.0));
        let workers: Vec<_> = (0..32)
            .map(|_| {
                let manager = manager.clone();
                std::thread::spawn(move || {
                    if let Ok(Some(reservation)) = manager.reserve_cents(10) {
                        reservation.settle();
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
            assert!(BudgetManager::new(limit).reserve_cents(1).unwrap().is_none());
        }
        let manager = BudgetManager::new(f64::MAX);
        let _res = manager.reserve_cents(i64::MAX).unwrap().unwrap();
        assert!(manager.reserve_cents(1).unwrap().is_none());
        assert_eq!(manager.get_remaining_cents(), 0);
        for amount in [f64::NAN, f64::INFINITY, f64::MAX] {
            assert!(manager.reserve(amount).is_err());
        }
        assert_eq!(manager.get_remaining_cents(), 0);
    }

    #[test]
    fn test_budget_manager() {
        let manager = BudgetManager::new(100.0);

        assert_eq!(manager.get_remaining(), 100.0);

        let reservation1 = manager.reserve(50.0).unwrap().unwrap();
        reservation1.settle();
        assert_eq!(manager.get_remaining(), 50.0);

        // Rejected admission must not debit the remaining balance.
        assert!(manager.reserve(60.0).unwrap().is_none());
        assert_eq!(manager.get_remaining(), 50.0);

        let err = manager.reserve(-10.0).unwrap_err();
        assert_eq!(err, "spend amount cannot be negative");

        let reservation2 = manager.reserve_cents(1000).unwrap().unwrap(); // spend $10
        reservation2.settle();
        assert_eq!(manager.get_remaining(), 40.0);
        assert_eq!(manager.get_remaining_cents(), 4000);
    }

    #[test]
    fn test_budget_manager_exact_limit() {
        let manager = BudgetManager::new(100.0);
        assert_eq!(manager.get_remaining(), 100.0);

        // Spend exactly the limit
        let res = manager.reserve(100.0).unwrap().unwrap();
        res.settle();
        assert_eq!(manager.get_remaining(), 0.0);
        assert_eq!(manager.get_remaining_cents(), 0);

        // One extra cent must be rejected without changing the balance.
        assert!(manager.reserve(0.01).unwrap().is_none());
        assert_eq!(manager.get_remaining_cents(), 0);
    }

    #[test]
    fn test_budget_manager_with_telemetry() {
        let store = std::sync::Arc::new(::server_harness::telemetry::ViolationStore::new(None));

        let manager = BudgetManager::new(50.0).with_telemetry("tenant-123".to_string(), store);
        assert!(manager.telemetry_store.is_some());
        // Spend money to hit telemetry path without panic
        manager.reserve_cents(1000).unwrap().unwrap().settle();

        // Ensure struct states updated correctly
        assert_eq!(manager.tenant_id, Some("tenant-123".to_string()));
        assert!(manager.telemetry_store.is_some());

        // Spend money to hit telemetry path without panic
        manager.reserve(10.0).unwrap().unwrap().settle();
        assert_eq!(manager.get_remaining(), 30.0);
    }

    #[test]
    fn test_reserve_cents_zero() {
        let manager = BudgetManager::new(100.0);
        let res = manager.reserve_cents(0).unwrap().unwrap();
        res.settle();
        assert_eq!(manager.get_remaining_cents(), 10000);
    }

    #[test]
    fn test_check_alert_threshold() {
        let manager = BudgetManager::new(100.0);

        // Not over threshold initially
        assert!(!manager.check_alert_threshold());

        // Spend 50%
        manager.reserve(50.0).unwrap().unwrap().settle();
        assert!(!manager.check_alert_threshold());

        // Spend up to 80%
        manager.reserve(30.0).unwrap().unwrap().settle();
        assert!(manager.check_alert_threshold()); // Default is 80.0

        // Custom threshold
        let custom_manager = BudgetManager::new(100.0).with_alert_threshold(90.0);
        custom_manager.reserve(85.0).unwrap().unwrap().settle();
        assert!(!custom_manager.check_alert_threshold());

        custom_manager.reserve(10.0).unwrap().unwrap().settle(); // 95%
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
        manager.reserve_cents(5000).unwrap().unwrap().settle();
        assert!(!manager.check_alert_threshold_cents(10000));

        // Spend up to 80% (8000 cents)
        manager.reserve_cents(3000).unwrap().unwrap().settle();
        assert!(manager.check_alert_threshold_cents(10000)); // Default is 80.0

        // Custom threshold using cents
        let custom_manager = BudgetManager::new(100.0).with_alert_threshold(90.0);
        custom_manager.reserve_cents(8500).unwrap().unwrap().settle();
        assert!(!custom_manager.check_alert_threshold_cents(10000));

        custom_manager.reserve_cents(1000).unwrap().unwrap().settle(); // 95%
        assert!(custom_manager.check_alert_threshold_cents(10000));

        // Exact threshold check
        let exact_manager = BudgetManager::new(100.0).with_alert_threshold(80.0);
        exact_manager.reserve_cents(8000).unwrap().unwrap().settle();
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
    fn test_reserve_cents_negative() {
        let manager = BudgetManager::new(100.0);
        let err = manager.reserve_cents(-1000).unwrap_err();
        assert_eq!(err, "spend amount cannot be negative");
    }

    #[test]
    fn test_budget_manager_with_telemetry_no_tenant() {
        let store = std::sync::Arc::new(::server_harness::telemetry::ViolationStore::new(None));
        let mut manager = BudgetManager::new(50.0);
        manager.telemetry_store = Some(store);
        manager.reserve(10.0).unwrap().unwrap().settle();
        assert_eq!(manager.get_remaining(), 40.0);
    }

    #[test]
    fn test_budget_manager_edge_cases() {
        let manager = BudgetManager::new(f64::MAX);
        manager.reserve(1.0).unwrap().unwrap().settle();

        // Check extremely small threshold values
        let threshold_manager = BudgetManager::new(100.0).with_alert_threshold(0.01);
        threshold_manager.reserve(0.02).unwrap().unwrap().settle();
        assert!(threshold_manager.check_alert_threshold());
    }

    #[test]
    fn test_is_spend_rate_too_high() {
        let manager = BudgetManager::new(100.0); // $100 limit, 10000 cents
        manager.reserve(20.0).unwrap().unwrap().settle(); // 2000 cents
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
    fn test_budget_reservation_drop() {
        let manager = BudgetManager::new(100.0);
        {
            let _res = manager.reserve(50.0).unwrap().unwrap();
            assert_eq!(manager.get_remaining(), 50.0);
            // Drop without settle
        }
        // Balance should be restored
        assert_eq!(manager.get_remaining(), 100.0);
    }
}
