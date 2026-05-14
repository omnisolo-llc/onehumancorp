use crate::miser::MiserRecommendation;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MiserReport {
    pub tenant_id: String,
    pub total_spend_cents: i64,
    pub remaining_budget_cents: i64,
    pub recommendations: Vec<MiserRecommendation>,
}

impl MiserReport {
    pub fn generate_summary(&self) -> String {
        let mut summary = format!("--- Miser Economic Report for {} ---\n", self.tenant_id);
        summary.push_str(&format!("Monthly Spend: ${:.2}\n", self.total_spend_cents as f64 / 100.0));
        summary.push_str(&format!("Remaining Budget: ${:.2}\n", self.remaining_budget_cents as f64 / 100.0));
        summary.push_str("\nRecommendations:\n");

        for (i, rec) in self.recommendations.iter().enumerate() {
            summary.push_str(&format!("{}. {} - {}\n", i + 1, rec.title, rec.impact));
        }

        summary
    }
}
