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
        self.tooltips.insert(
            "dashboard_active_agents".to_string(),
            "The number of AI agents currently working on tasks for your business.".to_string(),
        );
        self.tooltips.insert(
            "dashboard_active_tasks".to_string(),
            "Tasks that your agents are actively processing right now.".to_string(),
        );
        self.tooltips.insert(
            "dashboard_scheduled_calls".to_string(),
            "Upcoming phone calls or meetings your agents will handle.".to_string(),
        );
        self.tooltips.insert(
            "dashboard_team_members".to_string(),
            "Human teammates who have access to manage this business.".to_string(),
        );
    }

    pub fn get_tooltip(&self, key: &str) -> Option<String> {
        self.tooltips.get(key).cloned()
    }
}
