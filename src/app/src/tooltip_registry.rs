use std::collections::HashMap;

pub struct TooltipRegistry {
    tooltips: HashMap<String, String>,
}

impl TooltipRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            tooltips: HashMap::new(),
        };
        registry.register_default_tooltips();
        registry
    }

    fn register_default_tooltips(&mut self) {
        self.tooltips.insert(
            "dashboard_revenue".to_string(),
            "Total money earned before any fees or taxes.".to_string(),
        );
        self.tooltips.insert(
            "agent_status_active".to_string(),
            "This AI agent is currently working for your business.".to_string(),
        );
        self.tooltips.insert(
            "settings_domain".to_string(),
            "The web address where customers can find your business online.".to_string(),
        );
        self.tooltips.insert(
            "payment_stripe".to_string(),
            "We use Stripe to securely process all your payments.".to_string(),
        );
    }

    pub fn get_tooltip(&self, key: &str) -> Option<String> {
        self.tooltips.get(key).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tooltip_registry_new() {
        let registry = TooltipRegistry::new();
        assert!(registry.get_tooltip("dashboard_revenue").is_some());
        assert!(registry.get_tooltip("non_existent").is_none());
    }

    #[test]
    fn test_register_default_tooltips() {
        let registry = TooltipRegistry::new();
        assert_eq!(
            registry.get_tooltip("dashboard_revenue"),
            Some("Total money earned before any fees or taxes.".to_string())
        );
        assert_eq!(
            registry.get_tooltip("agent_status_active"),
            Some("This AI agent is currently working for your business.".to_string())
        );
        assert_eq!(
            registry.get_tooltip("settings_domain"),
            Some("The web address where customers can find your business online.".to_string())
        );
        assert_eq!(
            registry.get_tooltip("payment_stripe"),
            Some("We use Stripe to securely process all your payments.".to_string())
        );
    }
}
