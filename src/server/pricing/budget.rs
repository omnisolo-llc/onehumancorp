use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

#[derive(Debug, Default)]
pub struct BudgetState {
    pub total_allocated_cents: i64,
    pub settled_cents: i64,
}

pub struct BudgetReservation {
    state: Arc<Mutex<BudgetState>>,
    amount_cents: i64,
    settled: AtomicBool,
    telemetry_store: Option<Arc<::server_harness::telemetry::ViolationStore>>,
    tenant_id: Option<String>,
}

impl BudgetReservation {
    pub fn settle(&self, final_amount_cents: i64) -> Result<(), String> {
        if self.settled.swap(true, Ordering::SeqCst) {
            return Err("Reservation already settled or released".to_string());
        }

        {
            let mut state = self.state.lock().unwrap();
            state.settled_cents = state
                .settled_cents
                .checked_add(final_amount_cents)
                .unwrap_or(i64::MAX);

            let diff = final_amount_cents - self.amount_cents;
            if diff != 0 {
                state.total_allocated_cents = state
                    .total_allocated_cents
                    .checked_add(diff)
                    .unwrap_or(i64::MAX);
            }
        }

        if let (Some(store), Some(tid)) = (&self.telemetry_store, &self.tenant_id) {
            if final_amount_cents > 0 {
                store.llm_cost_counter.add(
                    final_amount_cents as u64,
                    &[opentelemetry::KeyValue::new("tenant_id", tid.to_string())],
                );
                store.mission_cost_cents.add(
                    final_amount_cents as u64,
                    &[opentelemetry::KeyValue::new("tenant_id", tid.to_string())],
                );
            }
        }

        Ok(())
    }

    pub fn release(&self) -> Result<(), String> {
        if self.settled.swap(true, Ordering::SeqCst) {
            return Err("Reservation already settled or released".to_string());
        }

        let mut state = self.state.lock().unwrap();
        state.total_allocated_cents -= self.amount_cents;
        Ok(())
    }
}

impl Drop for BudgetReservation {
    fn drop(&mut self) {
        if !self.settled.load(Ordering::SeqCst) {
            let mut state = self.state.lock().unwrap();
            state.total_allocated_cents -= self.amount_cents;
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
        let total_limit_cents = if limit == f64::MAX {
            i64::MAX
        } else if !limit.is_finite() || limit < 0.0 || limit * 100.0 >= i64::MAX as f64 {
            0
        } else {
            (limit * 100.0).round() as i64
        };
        BudgetManager {
            total_limit: limit,
            total_limit_cents,
            state: Arc::new(Mutex::new(BudgetState::default())),
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

    pub fn reserve(&self, amount_cents: i64) -> Result<BudgetReservation, String> {
        if amount_cents < 0 {
            return Err("reserve amount cannot be negative".to_string());
        }

        let mut state = self.state.lock().unwrap();
        let new_allocated = match state.total_allocated_cents.checked_add(amount_cents) {
            Some(v) => v,
            None => return Err("overflow".to_string()),
        };

        if new_allocated > self.total_limit_cents {
            return Err("budget limit exceeded".to_string());
        }

        state.total_allocated_cents = new_allocated;

        Ok(BudgetReservation {
            state: self.state.clone(),
            amount_cents,
            settled: AtomicBool::new(false),
            telemetry_store: self.telemetry_store.clone(),
            tenant_id: self.tenant_id.clone(),
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
            );
        }

        match self.reserve(amount_cents) {
            Ok(reservation) => {
                let _ = reservation.settle(amount_cents);
                Ok(true)
            }
            Err(_) => Ok(false),
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
        projected_cost_cents >= limit_threshold_cents || current >= limit_threshold_cents
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
        current as f64 > expected_spend * 1.5
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn concurrent_admission_never_exceeds_limit() {
        let manager = Arc::new(BudgetManager::new(1.0));
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
    fn test_reserve_and_settle() {
        let manager = BudgetManager::new(1.0);

        let res = manager.reserve(50).unwrap();
        assert_eq!(manager.get_remaining_cents(), 50);

        res.settle(50).unwrap();
        assert_eq!(manager.get_remaining_cents(), 50);

        let state = manager.state.lock().unwrap();
        assert_eq!(state.total_allocated_cents, 50);
        assert_eq!(state.settled_cents, 50);
    }

    #[test]
    fn test_reserve_and_release() {
        let manager = BudgetManager::new(1.0);

        let res = manager.reserve(50).unwrap();
        assert_eq!(manager.get_remaining_cents(), 50);

        res.release().unwrap();
        assert_eq!(manager.get_remaining_cents(), 100);

        let state = manager.state.lock().unwrap();
        assert_eq!(state.total_allocated_cents, 0);
        assert_eq!(state.settled_cents, 0);
    }

    #[test]
    fn test_reserve_drop() {
        let manager = BudgetManager::new(1.0);

        {
            let _res = manager.reserve(50).unwrap();
            assert_eq!(manager.get_remaining_cents(), 50);
        }

        assert_eq!(manager.get_remaining_cents(), 100);
    }

    #[test]
    fn test_reserve_partial_settle() {
        let manager = BudgetManager::new(1.0);

        let res = manager.reserve(50).unwrap();
        res.settle(30).unwrap(); // Settle for less than reserved

        assert_eq!(manager.get_remaining_cents(), 70); // 100 - 30 = 70
        let state = manager.state.lock().unwrap();
        assert_eq!(state.total_allocated_cents, 30);
        assert_eq!(state.settled_cents, 30);
    }

    #[test]
    fn test_reserve_over_settle() {
        let manager = BudgetManager::new(1.0);

        let res = manager.reserve(50).unwrap();
        res.settle(70).unwrap(); // Settle for more than reserved

        assert_eq!(manager.get_remaining_cents(), 30); // 100 - 70 = 30
        let state = manager.state.lock().unwrap();
        assert_eq!(state.total_allocated_cents, 70);
        assert_eq!(state.settled_cents, 70);
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

        assert!(manager.record_spend(100.0).unwrap());
        assert_eq!(manager.get_remaining(), 0.0);
        assert_eq!(manager.get_remaining_cents(), 0);

        assert!(!manager.record_spend(0.01).unwrap());
        assert_eq!(manager.get_remaining_cents(), 0);
    }

    #[test]
    fn test_budget_manager_with_telemetry() {
        let store = Arc::new(::server_harness::telemetry::ViolationStore::new(None));

        let manager = BudgetManager::new(50.0).with_telemetry("tenant-123".to_string(), store);
        assert!(manager.telemetry_store.is_some());
        manager.record_spend_cents(1000).unwrap();

        assert_eq!(manager.tenant_id, Some("tenant-123".to_string()));
        assert!(manager.telemetry_store.is_some());

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
        assert!(manager.check_alert_threshold());

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
        assert!(!manager.is_projected_cost_over_threshold(700));
        assert!(manager.is_projected_cost_over_threshold(800));
        assert!(manager.is_projected_cost_over_threshold(1500));

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
        let store = Arc::new(::server_harness::telemetry::ViolationStore::new(None));
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
        let manager = BudgetManager::new(100.0);
        manager.record_spend(20.0).unwrap();
        let one_day = std::time::Duration::from_secs(86400);
        let thirty_days = std::time::Duration::from_secs(30 * 86400);
        assert!(manager.is_spend_rate_too_high(one_day, thirty_days));
        assert!(
            !manager
                .is_spend_rate_too_high(std::time::Duration::from_secs(10 * 86400), thirty_days)
        );
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
