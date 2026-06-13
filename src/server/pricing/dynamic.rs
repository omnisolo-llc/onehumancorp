use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingBounds {
    pub min_price: f64,
    pub max_price: f64,
    pub base_price: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextSignals {
    pub time_of_day: String, // e.g., "morning", "afternoon", "evening", "closing_soon"
    pub weather: String, // e.g., "sunny", "raining", "hot", "cold"
    pub inventory_velocity: String, // e.g., "fast", "normal", "slow"
    pub demand_level: String, // e.g., "high", "normal", "low"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicPriceResult {
    pub price: f64,
    pub reason: String,
    pub adjustment_type: String, // "surge", "discount", "none"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicRuleCondition {
    pub field: String, // e.g., "time_of_day", "inventory_velocity", "demand_level"
    pub operator: String, // e.g., "==", "!=", "in"
    pub value: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicRuleAction {
    pub adjustment_type: String, // e.g., "surge", "discount", "fixed"
    pub value: f64, // percentage (e.g., 15.0 for 15%) or fixed amount
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicPricingRule {
    pub name: String,
    pub condition: DynamicRuleCondition,
    pub action: DynamicRuleAction,
}

pub struct DynamicPricingEngine;

impl DynamicPricingEngine {
    fn evaluate_condition(condition: &DynamicRuleCondition, context: &ContextSignals) -> bool {
        let actual_val = match condition.field.as_str() {
            "time_of_day" => &context.time_of_day,
            "weather" => &context.weather,
            "inventory_velocity" => &context.inventory_velocity,
            "demand_level" => &context.demand_level,
            _ => return false,
        };

        match condition.operator.as_str() {
            "==" => {
                if let Some(v) = condition.value.as_str() {
                    actual_val == v
                } else {
                    false
                }
            }
            "!=" => {
                if let Some(v) = condition.value.as_str() {
                    actual_val != v
                } else {
                    false
                }
            }
            "in" => {
                if let Some(arr) = condition.value.as_array() {
                    arr.iter().any(|item| item.as_str() == Some(actual_val.as_str()))
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    pub fn evaluate_rules(bounds: &PricingBounds, context: &ContextSignals, rules: &[DynamicPricingRule]) -> DynamicPriceResult {
        let mut final_price = bounds.base_price;
        let mut final_reason = "Standard price".to_string();
        let mut final_adjustment = "none".to_string();

        for rule in rules {
            if Self::evaluate_condition(&rule.condition, context) {
                let multiplier = match rule.action.adjustment_type.as_str() {
                    "surge" => 1.0 + (rule.action.value / 100.0),
                    "discount" => 1.0 - (rule.action.value / 100.0),
                    _ => 1.0,
                };

                final_price = bounds.base_price * multiplier;
                final_reason = rule.name.clone();
                final_adjustment = rule.action.adjustment_type.clone();
                break; // Apply first matching rule
            }
        }

        final_price = final_price.clamp(bounds.min_price, bounds.max_price);

        if (final_price - bounds.base_price).abs() < 0.01 {
            final_reason = "Standard price".to_string();
            final_adjustment = "none".to_string();
        }

        DynamicPriceResult {
            price: (final_price * 100.0).round() / 100.0,
            reason: final_reason,
            adjustment_type: final_adjustment,
        }
    }

    pub fn calculate_price(bounds: &PricingBounds, context: &ContextSignals) -> DynamicPriceResult {
        // Fallback to old behavior via rules to pass existing tests
        let rules = vec![
            DynamicPricingRule {
                name: "High demand".to_string(),
                condition: DynamicRuleCondition { field: "demand_level".to_string(), operator: "==".to_string(), value: serde_json::json!("high") },
                action: DynamicRuleAction { adjustment_type: "surge".to_string(), value: 15.0 },
            },
            DynamicPricingRule {
                name: "High demand".to_string(),
                condition: DynamicRuleCondition { field: "inventory_velocity".to_string(), operator: "==".to_string(), value: serde_json::json!("fast") },
                action: DynamicRuleAction { adjustment_type: "surge".to_string(), value: 15.0 },
            },
            DynamicPricingRule {
                name: "Last minute deal!".to_string(),
                condition: DynamicRuleCondition { field: "time_of_day".to_string(), operator: "==".to_string(), value: serde_json::json!("closing_soon") },
                action: DynamicRuleAction { adjustment_type: "discount".to_string(), value: 15.0 },
            },
            DynamicPricingRule {
                name: "Last minute deal!".to_string(),
                condition: DynamicRuleCondition { field: "inventory_velocity".to_string(), operator: "==".to_string(), value: serde_json::json!("slow") },
                action: DynamicRuleAction { adjustment_type: "discount".to_string(), value: 15.0 },
            },
            DynamicPricingRule {
                name: "Rainy day special".to_string(),
                condition: DynamicRuleCondition { field: "weather".to_string(), operator: "==".to_string(), value: serde_json::json!("raining") },
                action: DynamicRuleAction { adjustment_type: "discount".to_string(), value: 10.0 },
            },
        ];

        Self::evaluate_rules(bounds, context, &rules)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dynamic_pricing_surge() {
        let bounds = PricingBounds { base_price: 10.0, min_price: 8.0, max_price: 12.0 };
        let context = ContextSignals {
            time_of_day: "afternoon".to_string(),
            weather: "sunny".to_string(),
            inventory_velocity: "fast".to_string(),
            demand_level: "high".to_string(),
        };
        let result = DynamicPricingEngine::calculate_price(&bounds, &context);
        assert!(result.price > 10.0);
        assert_eq!(result.adjustment_type, "surge");
    }

    #[test]
    fn test_dynamic_pricing_discount() {
        let bounds = PricingBounds { base_price: 10.0, min_price: 8.0, max_price: 12.0 };
        let context = ContextSignals {
            time_of_day: "closing_soon".to_string(),
            weather: "sunny".to_string(),
            inventory_velocity: "slow".to_string(),
            demand_level: "low".to_string(),
        };
        let result = DynamicPricingEngine::calculate_price(&bounds, &context);
        assert!(result.price < 10.0);
        assert_eq!(result.adjustment_type, "discount");
    }

    #[test]
    fn test_dynamic_pricing_bounds() {
        // Surge would normally be 11.5, but max is 11.0
        let bounds = PricingBounds { base_price: 10.0, min_price: 8.0, max_price: 11.0 };
        let context = ContextSignals {
            time_of_day: "afternoon".to_string(),
            weather: "sunny".to_string(),
            inventory_velocity: "fast".to_string(),
            demand_level: "high".to_string(),
        };
        let result = DynamicPricingEngine::calculate_price(&bounds, &context);
        assert_eq!(result.price, 11.0);
    }
}
// verified by miser
