use std::sync::Mutex;

pub struct BudgetManager {
    pub total_limit: f64,
    current: Mutex<f64>,
}

impl BudgetManager {
    pub fn new(limit: f64) -> Self {
        BudgetManager {
            total_limit: limit,
            current: Mutex::new(0.0),
        }
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
        Ok(true)
    }

    pub fn get_remaining(&self) -> f64 {
        let current = self.current.lock().unwrap();
        self.total_limit - *current
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
    }
}
