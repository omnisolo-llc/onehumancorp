use std::sync::Mutex;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PayerMode {
    #[default]
    Managed,
    Byok,
}

pub struct BudgetState {
    pub total_allocated: i64,
}

pub struct BudgetManager {
    pub total_limit_cents: i64,
    pub alert_threshold_percent: f64,
    pub state: Mutex<BudgetState>,
    pub tenant_id: Option<String>,
    pub telemetry_store: Option<std::sync::Arc<::server_harness::telemetry::ViolationStore>>,
}

impl BudgetManager {
    pub fn new(total_limit: f64) -> Self {
        let total_limit_cents =
            if total_limit.is_nan() || total_limit.is_infinite() || total_limit < 0.0 {
                -1 // Fail closed for invalid limits
            } else if total_limit > (i64::MAX as f64) / 100.0 {
                i64::MAX
            } else {
                (total_limit * 100.0).round() as i64
            };

        Self {
            total_limit_cents,
            alert_threshold_percent: 80.0,
            state: Mutex::new(BudgetState { total_allocated: 0 }),
            tenant_id: None,
            telemetry_store: None,
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

    pub fn get_remaining(&self) -> f64 {
        self.get_remaining_cents() as f64 / 100.0
    }

    pub fn get_remaining_cents(&self) -> i64 {
        if self.total_limit_cents <= 0 {
            return 0;
        }
        let allocated = self.state.lock().unwrap().total_allocated;
        self.total_limit_cents.saturating_sub(allocated).max(0)
    }

    /// Record spend with the default PayerMode::Managed.
    pub fn record_spend(&self, amount: f64) -> Result<bool, String> {
        self.record_spend_with_mode(amount, PayerMode::Managed)
    }

    /// Record spend with a specific PayerMode. BYOK spends are not deducted from the OHC budget.
    pub fn record_spend_with_mode(
        &self,
        amount: f64,
        payer_mode: PayerMode,
    ) -> Result<bool, String> {
        if amount.is_nan() || amount.is_infinite() {
            return Err("invalid spend amount".to_string());
        }
        if amount < 0.0 {
            return Err("spend amount cannot be negative".to_string());
        }

        if amount > (i64::MAX as f64) / 100.0 {
            return Err("spend amount too large".to_string());
        }

        let amount_cents = (amount * 100.0).round() as i64;
        self.record_spend_cents_with_mode(amount_cents, payer_mode)
    }

    /// Record spend in cents with the default PayerMode::Managed.
    pub fn record_spend_cents(&self, amount_cents: i64) -> Result<bool, String> {
        self.record_spend_cents_with_mode(amount_cents, PayerMode::Managed)
    }

    /// Record spend in cents with a specific PayerMode.
    pub fn record_spend_cents_with_mode(
        &self,
        amount_cents: i64,
        payer_mode: PayerMode,
    ) -> Result<bool, String> {
        if amount_cents < 0 {
            return Err("spend amount cannot be negative".to_string());
        }

        // Emit telemetry if configured, regardless of payer mode.
        // We include a unique idempotency key for every event.
        if let Some(store) = &self.telemetry_store {
            let tenant = self.tenant_id.as_deref().unwrap_or("unknown");
            let idempotency_key = Uuid::new_v4().to_string();
            let mode_str = match payer_mode {
                PayerMode::Managed => "MANAGED",
                PayerMode::Byok => "BYOK",
            };
            store.record_spend(tenant, amount_cents, mode_str, &idempotency_key);
        }

        // BYOK inference costs are visually tracked but explicitly zeroed out in OHC's internal revenue ledger.
        // Therefore, they do not consume the OHC-managed budget.
        if payer_mode == PayerMode::Byok {
            return Ok(true); // Always allow BYOK spend as it doesn't hit OHC budget limits.
        }

        if self.total_limit_cents <= 0 {
            return Ok(amount_cents == 0); // Only allow 0 if limit is <= 0
        }

        let mut state = self.state.lock().unwrap();

        // Check if we can allocate without overflowing or exceeding limit
        if let Some(new_total) = state.total_allocated.checked_add(amount_cents)
            && new_total <= self.total_limit_cents
        {
            state.total_allocated = new_total;
            return Ok(true);
        }

        Ok(false)
    }

    pub fn check_alert_threshold(&self) -> bool {
        if self.total_limit_cents <= 0 {
            return false;
        }
        let current = self.state.lock().unwrap().total_allocated;
        let usage_percent = (current as f64) / (self.total_limit_cents as f64) * 100.0;
        usage_percent >= self.alert_threshold_percent
    }

    pub fn is_projected_cost_over_threshold(&self, projected_cost_cents: i64) -> bool {
        let current = self.state.lock().unwrap().total_allocated;
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
    fn test_budget_manager_byok() {
        let manager = BudgetManager::new(100.0);
        assert_eq!(manager.get_remaining(), 100.0);

        // BYOK spend should not decrease remaining balance
        assert!(
            manager
                .record_spend_cents_with_mode(1000, PayerMode::Byok)
                .unwrap()
        );
        assert_eq!(manager.get_remaining(), 100.0);

        assert!(
            manager
                .record_spend_with_mode(50.0, PayerMode::Byok)
                .unwrap()
        );
        assert_eq!(manager.get_remaining(), 100.0);
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
        );
        // 20% in 10 days is fine
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
