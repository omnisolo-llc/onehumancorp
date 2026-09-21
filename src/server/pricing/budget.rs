use std::sync::{Arc, Mutex};

pub struct BudgetState {
    pub total_allocated_cents: i64,
    pub settled_cents: i64,
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
    reserved_cents: i64,
    settled: bool,
    tenant_id: Option<String>,
    telemetry_store: Option<std::sync::Arc<::server_harness::telemetry::ViolationStore>>,
}

impl BudgetReservation {
    pub fn settle(mut self, actual_spent_cents: i64) {
        self.settled = true;

        {
            let mut guard = self.state.lock().unwrap();
            guard.settled_cents += actual_spent_cents;

            // Adjust allocated cents based on what was actually spent.
            // If we spent less than reserved, we free up the difference.
            // If we spent more than reserved, we allocate the extra.
            let diff = actual_spent_cents - self.reserved_cents;
            guard.total_allocated_cents += diff;
        }

        // Drop the lock before telemetry to avoid deadlocks
        if let (Some(store), Some(tid)) = (&self.telemetry_store, &self.tenant_id)
            && actual_spent_cents > 0
        {
            store.llm_cost_counter.add(
                actual_spent_cents as u64,
                &[opentelemetry::KeyValue::new("tenant_id", tid.to_string())],
            );
            store.mission_cost_cents.add(
                actual_spent_cents as u64,
                &[opentelemetry::KeyValue::new("tenant_id", tid.to_string())],
            );
        }
    }
}

impl Drop for BudgetReservation {
    fn drop(&mut self) {
        if !self.settled {
            let mut guard = self.state.lock().unwrap();
            guard.total_allocated_cents -= self.reserved_cents;
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


    pub fn reserve_cents(&self, amount_cents: i64) -> Result<BudgetReservation, String> {
        if amount_cents < 0 {
            return Err("spend amount cannot be negative".to_string());
        }

        let mut guard = self.state.lock().unwrap();

        // If amount_cents > 0, check if we exceed limit.
        if amount_cents > 0 {
            if guard.total_allocated_cents.checked_add(amount_cents).is_none_or(|next| next > self.total_limit_cents) {
                return Err("budget limit exceeded".to_string());
            }
            guard.total_allocated_cents += amount_cents;
        }

        if let (Some(_store), Some(tid)) = (&self.telemetry_store, &self.tenant_id) {
            tracing::info!(
                "💰 Miser telemetry: Recording budget spend for tenant {}",
                tid
            ); // pii-safe
        }

        Ok(BudgetReservation {
            state: Arc::clone(&self.state),
            reserved_cents: amount_cents,
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
        if amount_cents == 0 {
            return Ok(self.get_remaining_cents() >= 0);
        }
        match self.reserve_cents(amount_cents) {
            Ok(reservation) => {
                reservation.settle(amount_cents);
                Ok(true)
            },
            Err(e) if e == "budget limit exceeded" => Ok(false),
            Err(e) => Err(e),
        }
    }

    pub fn get_remaining(&self) -> f64 {
        let current = self.state.lock().unwrap().total_allocated_cents;
        (self.total_limit_cents - current) as f64 / 100.0
    }

    pub fn get_remaining_cents(&self) -> i64 {
        let current = self.state.lock().unwrap().total_allocated_cents;
        self.total_limit_cents - current
    }

    pub fn check_alert_threshold(&self) -> bool {
        if self.total_limit_cents <= 0 {
            return false;
        }
        let current = self.state.lock().unwrap().total_allocated_cents;
        let usage_percent = (current as f64 / self.total_limit_cents as f64) * 100.0;
        usage_percent >= self.alert_threshold_percent
    }

    pub fn is_projected_cost_over_threshold(&self, projected_cost_cents: i64) -> bool {
        let current = self.state.lock().unwrap().total_allocated_cents;
        if self.total_limit_cents <= 0 {
            return projected_cost_cents > 0 || current > 0;
        }
        let limit_threshold_cents = ((self.total_limit_cents as f64)
            * (self.alert_threshold_percent / 100.0))
            .round() as i64;
        projected_cost_cents >= limit_threshold_cents
            || current >= limit_threshold_cents
    }

    pub fn check_alert_threshold_cents(&self, total_limit_cents: i64) -> bool {
        if total_limit_cents <= 0 {
            return false;
        }
        let current = self.state.lock().unwrap().total_allocated_cents;
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
        let current = self.state.lock().unwrap().total_allocated_cents;
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

    #[test]
    fn test_budget_reservation_reserve_and_settle() {
        let manager = BudgetManager::new(100.0); // 10000 cents
        let res = manager.reserve_cents(2000).unwrap();
        assert_eq!(manager.get_remaining_cents(), 8000);

        // Settle for less
        res.settle(1500);
        assert_eq!(manager.get_remaining_cents(), 8500);
        assert_eq!(manager.state.lock().unwrap().settled_cents, 1500);
        assert_eq!(manager.state.lock().unwrap().total_allocated_cents, 1500);
    }

    #[test]
    fn test_budget_reservation_drop() {
        let manager = BudgetManager::new(100.0);
        {
            let _res = manager.reserve_cents(3000).unwrap();
            assert_eq!(manager.get_remaining_cents(), 7000);
            // Drop without settle
        }
        assert_eq!(manager.get_remaining_cents(), 10000);
        assert_eq!(manager.state.lock().unwrap().total_allocated_cents, 0);
        assert_eq!(manager.state.lock().unwrap().settled_cents, 0);
    }

    #[test]
    fn test_concurrent_reservation() {
        let manager = std::sync::Arc::new(BudgetManager::new(100.0)); // 10000 cents

        let workers: Vec<_> = (0..10)
            .map(|i| {
                let m = manager.clone();
                std::thread::spawn(move || {
                    let res = m.reserve_cents(500).unwrap();
                    std::thread::sleep(std::time::Duration::from_millis(10));
                    // Even threads settle exact, odd threads drop
                    if i % 2 == 0 {
                        res.settle(500);
                    }
                    // Else implicitly drop
                })
            })
            .collect();

        for worker in workers {
            worker.join().unwrap();
        }

        // 5 threads settled 500 each = 2500 cents
        // 5 threads dropped 500 each = 0 cents added
        assert_eq!(manager.get_remaining_cents(), 7500);
        assert_eq!(manager.state.lock().unwrap().settled_cents, 2500);
        assert_eq!(manager.state.lock().unwrap().total_allocated_cents, 2500);
    }

}
