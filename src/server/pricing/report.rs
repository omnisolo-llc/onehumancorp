use std::collections::HashMap;
use crate::calculator::{CostConfig, ModelPricing};
use crate::steering::ModelTier;

pub struct CostOptimizationReport {
    pub organization_id: String,
    pub period_start: chrono::DateTime<chrono::Utc>,
    pub period_end: chrono::DateTime<chrono::Utc>,
    pub usage_stats: UsageStats,
    pub potential_savings: Vec<SavingsOpportunity>,
}

pub struct UsageStats {
    pub total_tokens: i64,
    pub total_cost_usd: f64,
    pub storage_usage_bytes: i64,
    pub model_distribution: HashMap<String, i64>,
}

pub struct SavingsOpportunity {
    pub category: String, // "LLM", "Storage", "Network", "Payments"
    pub description: String,
    pub annual_savings_usd: f64,
    pub effort_to_implement: String, // "Low", "Medium", "High"
}

impl CostOptimizationReport {
    pub fn generate(
        org_id: &str,
        usage: UsageStats,
        config: &CostConfig
    ) -> Self {
        let mut potential_savings = Vec::new();

        // LLM Savings
        if usage.total_cost_usd > 100.0 {
            potential_savings.push(SavingsOpportunity {
                category: "LLM".to_string(),
                description: "Enable prompt caching and context pruning to reduce token waste.".to_string(),
                annual_savings_usd: usage.total_cost_usd * 0.3 * 12.0,
                effort_to_implement: "Low".to_string(),
            });
        }

        // Model Steering
        let premium_usage = usage.model_distribution.get("o1").cloned().unwrap_or(0);
        if premium_usage > 100000 {
             potential_savings.push(SavingsOpportunity {
                category: "LLM".to_string(),
                description: "Automatic steering for simple tasks could offload 40% of premium model usage.".to_string(),
                annual_savings_usd: (premium_usage as f64 * 0.000015) * 12.0, // Rough estimate
                effort_to_implement: "Medium".to_string(),
            });
        }

        // Storage
        if usage.storage_usage_bytes > 1024 * 1024 * 1024 { // > 1GB
            potential_savings.push(SavingsOpportunity {
                category: "Storage".to_string(),
                description: "Convert all assets to WebP and enable aggressive TTL for transient data.".to_string(),
                annual_savings_usd: (usage.storage_usage_bytes as f64 / 1e9 * 0.10) * 12.0,
                effort_to_implement: "Low".to_string(),
            });
        }

        Self {
            organization_id: org_id.to_string(),
            period_start: chrono::Utc::now() - chrono::Duration::days(30),
            period_end: chrono::Utc::now(),
            usage_stats: usage,
            potential_savings,
        }
    }

    pub fn to_plain_language(&self) -> String {
        let mut s = format!("## Cost Health Report for {}\n\n", self.organization_id);
        s.push_str(&format!("In the last 30 days, your business consumed {} tokens at a cost of ${:.2}.\n",
            self.usage_stats.total_tokens, self.usage_stats.total_cost_usd));

        if self.potential_savings.is_empty() {
            s.push_str("Your infrastructure is currently running at peak efficiency. No optimizations recommended!\n");
        } else {
            s.push_str("We've identified several ways to make your business more profitable by reducing waste:\n\n");
            for opt in &self.potential_savings {
                s.push_str(&format!("### {}\n", opt.description));
                s.push_str(&format!("- **Potential Annual Savings**: ${:.2}\n", opt.annual_savings_usd));
                s.push_str(&format!("- **Effort**: {}\n\n", opt.effort_to_implement));
            }
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_report_generation() {
        let mut distribution = HashMap::new();
        distribution.insert("o1".to_string(), 200000);
        distribution.insert("gpt-4o".to_string(), 500000);

        let usage = UsageStats {
            total_tokens: 700000,
            total_cost_usd: 150.0,
            storage_usage_bytes: 2000000000, // 2GB
            model_distribution: distribution,
        };

        let config = CostConfig::default();
        let report = CostOptimizationReport::generate("maya-bakery", usage, &config);

        assert!(!report.potential_savings.is_empty());
        let text = report.to_plain_language();
        assert!(text.contains("maya-bakery"));
        assert!(text.contains("Annual Savings"));
    }
}
