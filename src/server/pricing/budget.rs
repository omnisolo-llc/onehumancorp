use std::sync::Mutex;
use uuid::Uuid;

pub struct Reservation {
    pub id: Uuid,
    pub amount_micros: i64,
}

struct BudgetState {
    current_micros: i64,
    reserved_micros: i64,
    reservations: std::collections::HashMap<Uuid, i64>,
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
            state: Mutex::new(BudgetState {
                current_micros: 0,
                reserved_micros: 0,
                reservations: std::collections::HashMap::new(),
            }),
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

    pub fn reserve_micros(&self, amount_micros: i64) -> Result<Reservation, String> {
        if amount_micros < 0 {
            return Err("reserve amount cannot be negative".to_string());
        }

        let res = Reservation {
            id: Uuid::new_v4(),
            amount_micros,
        };

        let mut state = self.state.lock().unwrap();

        let current = state.current_micros;
        let reserved = state.reserved_micros;

        let next_r = match reserved.checked_add(amount_micros) {
            Some(v) => v,
            None => return Err("budget limit exceeded (overflow)".to_string()),
        };

        let total_projected = match current.checked_add(next_r) {
            Some(v) => v,
            None => return Err("budget limit exceeded (overflow)".to_string()),
        };

        if total_projected <= self.total_limit_micros {
            state.reserved_micros = next_r;
            state.reservations.insert(res.id, amount_micros);
            Ok(res)
        } else {
            Err("budget limit exceeded".to_string())
        }
    }

    pub fn settle(
        &self,
        reservation: Reservation,
        final_amount_micros: i64,
    ) -> Result<bool, String> {
        if final_amount_micros < 0 {
            return Err("settle amount cannot be negative".to_string());
        }

        let mut state = self.state.lock().unwrap();

        if let Some(reserved_amount) = state.reservations.remove(&reservation.id) {
            // Fast fail on overflow
            let next_current = match state.current_micros.checked_add(final_amount_micros) {
                Some(v) => v,
                None => {
                    // Restore reservation
                    state.reservations.insert(reservation.id, reserved_amount);
                    return Err("budget limit exceeded (overflow)".to_string());
                }
            };

            // Note: we check the total limit *after* replacing the reservation with actual spend
            let next_reserved = state.reserved_micros - reserved_amount;

            if next_current.saturating_add(next_reserved) <= self.total_limit_micros {
                state.reserved_micros = next_reserved;
                state.current_micros = next_current;

                // Drop lock before emitting telemetry
                drop(state);

                if let (Some(store), Some(tid)) = (&self.telemetry_store, &self.tenant_id)
                    && final_amount_micros > 0
                {
                    tracing::info!(
                        "💰 Miser telemetry: Recording budget spend for tenant {}",
                        tid
                    ); // pii-safe

                    let amount_cents = (final_amount_micros as f64 / 10_000.0).round() as u64;
                    store.llm_cost_counter.add(
                        amount_cents,
                        &[opentelemetry::KeyValue::new("tenant_id", tid.to_string())],
                    );
                    store.mission_cost_cents.add(
                        amount_cents,
                        &[opentelemetry::KeyValue::new("tenant_id", tid.to_string())],
                    );
                }

                Ok(true)
            } else {
                // Restore reservation
                state.reservations.insert(reservation.id, reserved_amount);
                Ok(false)
            }
        } else {
            Err("reservation not found".to_string())
        }
    }

    pub fn release(&self, reservation: Reservation) -> Result<(), String> {
        let mut state = self.state.lock().unwrap();
        if let Some(reserved_amount) = state.reservations.remove(&reservation.id) {
            state.reserved_micros -= reserved_amount;
            Ok(())
        } else {
            Err("reservation not found".to_string())
        }
    }

    pub fn record_spend_micros(&self, amount_micros: i64) -> Result<bool, String> {
        if amount_micros < 0 {
            return Err("spend amount cannot be negative".to_string());
        }
        if amount_micros == 0 {
            return Ok(self.get_remaining_cents() >= 0);
        }

        let mut state = self.state.lock().unwrap();

        let next_current = match state.current_micros.checked_add(amount_micros) {
            Some(v) => v,
            None => return Ok(false),
        };

        let reserved = state.reserved_micros;

        let total_projected = match next_current.checked_add(reserved) {
            Some(v) => v,
            None => return Ok(false),
        };

        if total_projected <= self.total_limit_micros {
            state.current_micros = next_current;

            // Drop lock before telemetry
            drop(state);

            if let (Some(store), Some(tid)) = (&self.telemetry_store, &self.tenant_id) {
                tracing::info!(
                    "💰 Miser telemetry: Recording budget spend for tenant {}",
                    tid
                ); // pii-safe
                let amount_cents = (amount_micros as f64 / 10_000.0).round() as u64;
                store.llm_cost_counter.add(
                    amount_cents,
                    &[opentelemetry::KeyValue::new("tenant_id", tid.to_string())],
                );
                store.mission_cost_cents.add(
                    amount_cents,
                    &[opentelemetry::KeyValue::new("tenant_id", tid.to_string())],
                );
            }
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn record_spend_cents(&self, amount_cents: i64) -> Result<bool, String> {
        self.record_spend_micros(amount_cents.saturating_mul(10_000))
    }

    pub fn get_remaining(&self) -> f64 {
        let state = self.state.lock().unwrap();
        let current = state.current_micros;
        let reserved = state.reserved_micros;
        (self.total_limit_micros - current - reserved) as f64 / 1_000_000.0
    }

    pub fn get_remaining_cents(&self) -> i64 {
        let state = self.state.lock().unwrap();
        let current = state.current_micros;
        let reserved = state.reserved_micros;
        (self.total_limit_micros - current - reserved) / 10_000
    }

    pub fn check_alert_threshold(&self) -> bool {
        if self.total_limit_micros <= 0 {
            return false;
        }
        let state = self.state.lock().unwrap();
        let current = state.current_micros;
        let reserved = state.reserved_micros;
        let usage_percent = ((current + reserved) as f64 / self.total_limit_micros as f64) * 100.0;
        usage_percent >= self.alert_threshold_percent
    }

    pub fn is_projected_cost_over_threshold(&self, projected_cost_cents: i64) -> bool {
        let projected_cost_micros = projected_cost_cents.saturating_mul(10_000);
        let state = self.state.lock().unwrap();
        if self.total_limit_micros <= 0 {
            return projected_cost_micros > 0
                || (state.current_micros + state.reserved_micros) > 0;
        }
        let limit_threshold_micros = ((self.total_limit_micros as f64)
            * (self.alert_threshold_percent / 100.0))
            .round() as i64;
        let current = state.current_micros;
        let reserved = state.reserved_micros;
        projected_cost_micros >= limit_threshold_micros
            || (current + reserved) >= limit_threshold_micros
    }

    pub fn check_alert_threshold_cents(&self, total_limit_cents: i64) -> bool {
        if total_limit_cents <= 0 {
            return false;
        }
        let total_limit_micros = total_limit_cents.saturating_mul(10_000);
        let state = self.state.lock().unwrap();
        let current = state.current_micros;
        let reserved = state.reserved_micros;
        let limit_threshold_micros =
            ((total_limit_micros as f64) * (self.alert_threshold_percent / 100.0)).round() as i64;
        (current + reserved) >= limit_threshold_micros
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
        let current = state.current_micros;
        let reserved = state.reserved_micros;
        let expected_spend = (self.total_limit_micros as f64)
            * (time_elapsed.as_secs() as f64 / total_duration.as_secs() as f64);
        (current + reserved) as f64 > expected_spend * 1.5 // 50% higher than expected rate
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
        let manager = BudgetManager::new(10.0); // $10 limit = 10,000,000 micros

        // 1. Reserve some budget
        let res = manager.reserve_micros(5_000_000).unwrap(); // $5
        assert_eq!(manager.get_remaining(), 5.0);
        assert_eq!(manager.get_remaining_cents(), 500);

        // 2. Try to reserve beyond remaining limit
        assert!(manager.reserve_micros(6_000_000).is_err());

        // 3. Settle with a smaller amount
        assert!(manager.settle(res, 4_000_000).unwrap());
        assert_eq!(manager.get_remaining(), 6.0); // 10.0 - 4.0 = 6.0

        // 4. Reserve and release
        let res2 = manager.reserve_micros(6_000_000).unwrap();
        assert_eq!(manager.get_remaining(), 0.0);
        assert!(manager.reserve_micros(1).is_err()); // Cannot reserve more

        manager.release(res2).unwrap();
        assert_eq!(manager.get_remaining(), 6.0);
    }

    #[test]
    fn test_reserve_micros_concurrency() {
        let manager = std::sync::Arc::new(BudgetManager::new(1.0)); // $1.0 limit
        let workers: Vec<_> = (0..32)
            .map(|_| {
                let manager = manager.clone();
                std::thread::spawn(move || manager.reserve_micros(100_000).is_ok()) // Reserve 10 cents
            })
            .collect();
        let admitted = workers
            .into_iter()
            .map(|worker| usize::from(worker.join().unwrap()))
            .sum::<usize>();

        // Only 10 workers should succeed (10 * 10 cents = $1.0)
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
