use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicI64, Ordering};

/// Represents the isolated repayment state for a single tenant's capital advance.
pub struct TenantLedger {
    pub tenant_id: String,
    pub merchant_balance_cents: AtomicI64,
    pub capital_repayment_cents: AtomicI64,
    pub total_repayment_target_cents: AtomicI64,
    pub split_percentage: f64,
}

impl TenantLedger {
    pub fn new(tenant_id: String, merchant_balance: i64, repayment_target: i64, split_percentage: f64) -> Self {
        Self {
            tenant_id,
            merchant_balance_cents: AtomicI64::new(merchant_balance),
            capital_repayment_cents: AtomicI64::new(0),
            total_repayment_target_cents: AtomicI64::new(repayment_target),
            split_percentage,
        }
    }

    /// Processes an incoming payment, splitting it into merchant balance and capital repayment.
    /// Mathematically guarantees correct split and atomic updates.
    pub fn process_payment(&self, payment_cents: i64) -> (i64, i64) {
        if payment_cents <= 0 {
            return (0, 0);
        }

        let target = self.total_repayment_target_cents.load(Ordering::SeqCst);

        // Loop to safely atomic Compare-and-Swap
        let mut current_repayment = self.capital_repayment_cents.load(Ordering::SeqCst);
        let final_repayment;

        loop {
            if current_repayment >= target {
                final_repayment = 0;
                break;
            }

            let mut repayment = (payment_cents as f64 * self.split_percentage).round() as i64;

            if current_repayment + repayment > target {
                repayment = target - current_repayment;
            }

            let new_repayment = current_repayment + repayment;

            match self.capital_repayment_cents.compare_exchange_weak(
                current_repayment,
                new_repayment,
                Ordering::SeqCst,
                Ordering::SeqCst,
            ) {
                Ok(_) => {
                    final_repayment = repayment;
                    break;
                }, // Successfully updated
                Err(updated) => current_repayment = updated, // Retry with the updated value
            }
        }

        let merchant_cut = payment_cents - final_repayment;

        // Atomically update merchant balance
        self.merchant_balance_cents.fetch_add(merchant_cut, Ordering::SeqCst);

        (merchant_cut, final_repayment)
    }

    pub fn get_merchant_balance(&self) -> i64 {
        self.merchant_balance_cents.load(Ordering::SeqCst)
    }

    pub fn get_repayment_balance(&self) -> i64 {
        self.capital_repayment_cents.load(Ordering::SeqCst)
    }

    pub fn get_repayment_target(&self) -> i64 {
        self.total_repayment_target_cents.load(Ordering::SeqCst)
    }
}

/// Manages multiple tenant ledgers with Zero-Trust isolation.
pub struct LedgerManager {
    tenants: Mutex<HashMap<String, Arc<TenantLedger>>>,
}

impl LedgerManager {
    pub fn new() -> Self {
        Self {
            tenants: Mutex::new(HashMap::new()),
        }
    }

    pub fn register_tenant(&self, tenant_id: String, initial_balance: i64, repayment_target: i64, split_percentage: f64) {
        let mut tenants = self.tenants.lock().unwrap();
        if !tenants.contains_key(&tenant_id) {
            tenants.insert(tenant_id.clone(), Arc::new(TenantLedger::new(tenant_id, initial_balance, repayment_target, split_percentage)));
        }
    }

    pub fn process_tenant_payment(&self, tenant_id: &str, payment_cents: i64) -> Result<(i64, i64), String> {
        let tenants = self.tenants.lock().unwrap();
        if let Some(ledger) = tenants.get(tenant_id) {
            Ok(ledger.process_payment(payment_cents))
        } else {
            Err(format!("Tenant {} not found in ledger", tenant_id))
        }
    }

    pub fn get_tenant_ledger(&self, tenant_id: &str) -> Option<Arc<TenantLedger>> {
        let tenants = self.tenants.lock().unwrap();
        tenants.get(tenant_id).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_mathematical_split() {
        let ledger = TenantLedger::new("tenant_1".to_string(), 0, 132000, 0.10);
        let (merchant, repay) = ledger.process_payment(10000); // $100

        assert_eq!(merchant, 9000); // $90
        assert_eq!(repay, 1000); // $10
        assert_eq!(ledger.get_merchant_balance(), 9000);
        assert_eq!(ledger.get_repayment_balance(), 1000);
    }

    #[test]
    fn test_atomic_concurrent_payments() {
        let ledger = Arc::new(TenantLedger::new("tenant_2".to_string(), 0, 132000, 0.10));
        let mut handles = vec![];

        for _ in 0..100 {
            let ledger_clone = Arc::clone(&ledger);
            handles.push(thread::spawn(move || {
                ledger_clone.process_payment(1000); // $10 payments
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        // 100 payments of 1000 cents = 100,000 cents total
        // 90% merchant = 90,000 cents
        // 10% repayment = 10,000 cents
        assert_eq!(ledger.get_merchant_balance(), 90000);
        assert_eq!(ledger.get_repayment_balance(), 10000);
    }

    #[test]
    fn test_ledger_manager_isolation() {
        let manager = LedgerManager::new();
        manager.register_tenant("tenant_a".to_string(), 0, 1000, 0.10);
        manager.register_tenant("tenant_b".to_string(), 0, 2000, 0.20);

        manager.process_tenant_payment("tenant_a", 1000).unwrap();
        manager.process_tenant_payment("tenant_b", 1000).unwrap();

        let ledger_a = manager.get_tenant_ledger("tenant_a").unwrap();
        let ledger_b = manager.get_tenant_ledger("tenant_b").unwrap();

        assert_eq!(ledger_a.get_merchant_balance(), 900);
        assert_eq!(ledger_a.get_repayment_balance(), 100);

        assert_eq!(ledger_b.get_merchant_balance(), 800);
        assert_eq!(ledger_b.get_repayment_balance(), 200);
    }

    #[test]
    fn test_mathematical_split_cap() {
        let ledger = TenantLedger::new("tenant_1".to_string(), 0, 100, 0.10);
        let (merchant, repay) = ledger.process_payment(900); // 90 to repayment, 810 to merchant

        assert_eq!(merchant, 810);
        assert_eq!(repay, 90);
        assert_eq!(ledger.get_merchant_balance(), 810);
        assert_eq!(ledger.get_repayment_balance(), 90);

        let (merchant2, repay2) = ledger.process_payment(200); // 10 to repayment, 190 to merchant
        assert_eq!(merchant2, 190);
        assert_eq!(repay2, 10);
        assert_eq!(ledger.get_merchant_balance(), 1000);
        assert_eq!(ledger.get_repayment_balance(), 100);

        let (merchant3, repay3) = ledger.process_payment(500); // 0 to repayment, 500 to merchant
        assert_eq!(merchant3, 500);
        assert_eq!(repay3, 0);
        assert_eq!(ledger.get_merchant_balance(), 1500);
        assert_eq!(ledger.get_repayment_balance(), 100);
    }
}
