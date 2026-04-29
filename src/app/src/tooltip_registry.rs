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
