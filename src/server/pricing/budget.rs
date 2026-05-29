use std::sync::Mutex;

pub struct BudgetManager {
    pub total_limit: f64,
    current: Mutex<f64>,
    pub telemetry_store: Option<std::sync::Arc<::server_harness::telemetry::ViolationStore>>,
    tenant_id: Option<String>,
}

impl BudgetManager {
    pub fn new(limit: f64) -> Self {
        BudgetManager {
            total_limit: limit,
            current: Mutex::new(0.0),
            telemetry_store: None,
            tenant_id: None,
        }
    }

    pub fn with_telemetry(mut self, tenant_id: String, store: std::sync::Arc<::server_harness::telemetry::ViolationStore>) -> Self {
        self.tenant_id = Some(tenant_id);
        self.telemetry_store = Some(store);
        self
    }

    pub fn record_spend(&self, amount: f64) -> Result<bool, String> {
        let mut current = self.current.lock().unwrap();

        if amount < 0.0 {
            return Err("spend amount cannot be negative".to_string());
        }

        if *current + amount > self.total_limit {
            return Err(format!(
                "budget exceeded: cannot spend {:.2}, remaining budget is {:.2}",
                amount,
                self.total_limit - *current
            ));
        }

        *current += amount;

        if let (Some(store), Some(tid)) = (&self.telemetry_store, &self.tenant_id) {
            let cents = (amount * 100.0).round() as u64;
            if cents > 0 {
                store.llm_cost_counter.add(
                    cents,
                    &[opentelemetry::KeyValue::new("tenant_id", tid.to_string())],
                );
            }
        }

        Ok(true)
    }

    pub fn get_remaining(&self) -> f64 {
        let current = self.current.lock().unwrap();
        self.total_limit - *current
    }

    pub fn get_remaining_cents(&self) -> i64 {
        let current = self.current.lock().unwrap();
        ((self.total_limit - *current) * 100.0).round() as i64
    }

    pub fn record_spend_cents(&self, amount_cents: i64) -> Result<bool, String> {
        self.record_spend((amount_cents as f64) / 100.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_budget_manager() {
        let manager = BudgetManager::new(100.0);
        
        assert_eq!(manager.get_remaining(), 100.0);
        
        assert!(manager.record_spend(50.0).is_ok());
        assert_eq!(manager.get_remaining(), 50.0);
        
        let err = manager.record_spend(60.0).unwrap_err();
        assert!(err.contains("budget exceeded"));

        assert_eq!(manager.get_remaining(), 50.0); // Should not change!
        
        let err = manager.record_spend(-10.0).unwrap_err();
        assert_eq!(err, "spend amount cannot be negative");

        // test cents
        assert!(manager.record_spend_cents(1000).is_ok()); // spend $10
        assert_eq!(manager.get_remaining(), 40.0);
        assert_eq!(manager.get_remaining_cents(), 4000);
    }
}
