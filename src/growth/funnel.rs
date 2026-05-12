// Free Tier and Upgrade Funnel
pub struct SubscriptionTier {
    pub name: String,
    pub price: u32,
}
pub fn show_upgrade_prompt() -> String {
    "Upgrade to Pro!".to_string()
}
