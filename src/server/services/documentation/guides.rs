pub struct GuidesManager;

impl GuidesManager {
    pub fn get_available_guides() -> Vec<&'static str> {
        vec![
            "advanced_shipping",
            "team_collaboration",
            "taxes",
            "custom_domain",
            "marketing_integrations",
            "seo_basics",
            "pos_system",
            "inventory_management",
        ]
    }
}
