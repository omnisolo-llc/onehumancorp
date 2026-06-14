use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, Timelike};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingBounds {
    pub min_price_cents: i64,
    pub max_price_cents: i64,
    pub base_price_cents: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextSignals {
    pub current_time: DateTime<Utc>,
    pub inventory_level: i32,
    pub inventory_velocity_7d: f64, // avg sales per day
    pub demand_score: f64, // 0.0 to 1.0
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "config")]
pub enum RuleType {
    TimeWindow {
        start_hour: u32,
        end_hour: u32,
        adjustment_percent: f64,
    },
    InventoryThreshold {
        threshold: i32,
        adjustment_percent: f64,
    },
    DemandSurge {
        threshold_score: f64,
        adjustment_percent: f64,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingRule {
    pub id: String,
    pub name: String,
    pub rule_type: RuleType,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicPriceResult {
    pub price_cents: i64,
    pub applied_rules: Vec<String>,
}

pub struct DynamicPricingEngine;

impl DynamicPricingEngine {
    pub fn calculate_price(
        bounds: &PricingBounds,
        rules: &[PricingRule],
        context: &ContextSignals,
    ) -> DynamicPriceResult {
        let mut total_adjustment_percent = 0.0;
        let mut applied_rules = Vec::new();

        for rule in rules {
            if !rule.is_active {
                continue;
            }

            match &rule.rule_type {
                RuleType::TimeWindow { start_hour, end_hour, adjustment_percent } => {
                    let hour = context.current_time.hour();
                    if hour >= *start_hour && hour < *end_hour {
                        total_adjustment_percent += adjustment_percent;
                        applied_rules.push(rule.name.clone());
                    }
                }
                RuleType::InventoryThreshold { threshold, adjustment_percent } => {
                    if context.inventory_level <= *threshold {
                        total_adjustment_percent += adjustment_percent;
                        applied_rules.push(rule.name.clone());
                    }
                }
                RuleType::DemandSurge { threshold_score, adjustment_percent } => {
                    if context.demand_score >= *threshold_score {
                        total_adjustment_percent += adjustment_percent;
                        applied_rules.push(rule.name.clone());
                    }
                }
            }
        }

        let mut adjusted_price = bounds.base_price_cents as f64 * (1.0 + total_adjustment_percent / 100.0);

        // Clamp to bounds
        adjusted_price = adjusted_price.clamp(bounds.min_price_cents as f64, bounds.max_price_cents as f64);

        DynamicPriceResult {
            price_cents: adjusted_price.round() as i64,
            applied_rules,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_dynamic_pricing_combined_rules() {
        let bounds = PricingBounds {
            base_price_cents: 1000, // $10.00
            min_price_cents: 800,
            max_price_cents: 1500,
        };

        let rules = vec![
            PricingRule {
                id: "1".into(),
                name: "Happy Hour".into(),
                rule_type: RuleType::TimeWindow { start_hour: 17, end_hour: 19, adjustment_percent: -10.0 },
                is_active: true,
            },
            PricingRule {
                id: "2".into(),
                name: "Low Stock Surge".into(),
                rule_type: RuleType::InventoryThreshold { threshold: 5, adjustment_percent: 20.0 },
                is_active: true,
            }
        ];

        // Scenario 1: Happy Hour (18:00) and Normal Stock (10)
        let context1 = ContextSignals {
            current_time: Utc.with_ymd_and_hms(2025, 1, 1, 18, 0, 0).unwrap(),
            inventory_level: 10,
            inventory_velocity_7d: 1.0,
            demand_score: 0.5,
        };
        let result1 = DynamicPricingEngine::calculate_price(&bounds, &rules, &context1);
        assert_eq!(result1.price_cents, 900);
        assert!(result1.applied_rules.contains(&"Happy Hour".to_string()));

        // Scenario 2: Happy Hour (18:00) and Low Stock (3)
        let context2 = ContextSignals {
            current_time: Utc.with_ymd_and_hms(2025, 1, 1, 18, 0, 0).unwrap(),
            inventory_level: 3,
            inventory_velocity_7d: 1.0,
            demand_score: 0.5,
        };
        let result2 = DynamicPricingEngine::calculate_price(&bounds, &rules, &context2);
        // 1000 * (1 - 0.10 + 0.20) = 1100
        assert_eq!(result2.price_cents, 1100);
        assert_eq!(result2.applied_rules.len(), 2);
    }

    #[test]
    fn test_dynamic_pricing_clamping() {
        let bounds = PricingBounds {
            base_price_cents: 1000,
            min_price_cents: 950,
            max_price_cents: 1050,
        };

        let rules = vec![
            PricingRule {
                id: "1".into(),
                name: "Big Discount".into(),
                rule_type: RuleType::InventoryThreshold { threshold: 100, adjustment_percent: -50.0 },
                is_active: true,
            }
        ];

        let context = ContextSignals {
            current_time: Utc::now(),
            inventory_level: 10,
            inventory_velocity_7d: 1.0,
            demand_score: 0.5,
        };

        let result = DynamicPricingEngine::calculate_price(&bounds, &rules, &context);
        assert_eq!(result.price_cents, 950); // Clamped to min_price
    }
}
