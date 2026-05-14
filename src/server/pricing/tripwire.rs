use crate::calculator::CostConfig;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum TripwireState {
    Safe,
    Warning,
    Locked,
}

pub struct BudgetTripwire {
    pub daily_limit_usd: f64,
    pub warning_threshold: f64, // e.g. 0.8 for 80%
}

impl BudgetTripwire {
    pub fn new(daily_limit_usd: f64, warning_threshold: f64) -> Self {
        Self { daily_limit_usd, warning_threshold }
    }

    pub fn evaluate(&self, current_spend_usd: f64) -> TripwireState {
        if current_spend_usd >= self.daily_limit_usd {
            TripwireState::Locked
        } else if current_spend_usd >= self.daily_limit_usd * self.warning_threshold {
            TripwireState::Warning
        } else {
            TripwireState::Safe
        }
    }

    pub fn get_nudge_message(&self, state: TripwireState) -> Option<String> {
        match state {
            TripwireState::Safe => None,
            TripwireState::Warning => Some("Your business has consumed 80% of its daily AI budget. Consider enabling 'Miser Mode' to extend your run rate.".to_string()),
            TripwireState::Locked => Some("Daily budget limit reached. To protect your business from overspending, AI actions are temporarily paused. Upgrade or increase your limit to continue.".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tripwire_states() {
        let tripwire = BudgetTripwire::new(10.0, 0.8);

        assert!(matches!(tripwire.evaluate(5.0), TripwireState::Safe));
        assert!(matches!(tripwire.evaluate(8.5), TripwireState::Warning));
        assert!(matches!(tripwire.evaluate(11.0), TripwireState::Locked));

        assert!(tripwire.get_nudge_message(TripwireState::Warning).unwrap().contains("80%"));
    }
}
