use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MiserRecommendation {
    pub id: String,
    pub title: String,
    pub description: String,
    pub impact: String,
    pub action_label: String,
    pub action_type: String,
    pub potential_savings_cents: i64,
    pub priority: u8,
}

pub fn get_active_recommendations() -> Vec<MiserRecommendation> {
    vec![
        MiserRecommendation {
            id: "ach_optimization".to_string(),
            title: "Switch to Bank Transfer".to_string(),
            description: "You're processing many large orders. Using bank transfers for orders over $50 could save you up to 2.5% in fees.".to_string(),
            impact: "Saves approx. $15/month".to_string(),
            action_label: "Enable Bank Transfers".to_string(),
            action_type: "PAYMENT_OPTIMIZATION".to_string(),
            potential_savings_cents: 1500,
            priority: 1,
        },
        MiserRecommendation {
            id: "image_compression".to_string(),
            title: "Optimize Product Images".to_string(),
            description: "Your product photos are taking up 400MB. We can compress them for you to save space and speed up your store.".to_string(),
            impact: "Reduce storage by 60%".to_string(),
            action_label: "Optimize Now".to_string(),
            action_type: "STORAGE_OPTIMIZATION".to_string(),
            potential_savings_cents: 500,
            priority: 2,
        },
    ]
}
