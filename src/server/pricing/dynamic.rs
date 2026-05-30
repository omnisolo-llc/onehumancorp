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

pub struct DynamicPricingEngine;

impl DynamicPricingEngine {
    pub fn calculate_price(bounds: &PricingBounds, context: &ContextSignals) -> DynamicPriceResult {
        let mut adjusted_price = bounds.base_price;
        let mut reason = "Standard price".to_string();
        let mut adjustment_type = "none".to_string();

        // Very basic logic for now. We can make it more sophisticated later.

        // 1. Demand & Velocity (Surge)
        if context.demand_level == "high" || context.inventory_velocity == "fast" {
            adjusted_price = bounds.base_price * 1.15; // 15% surge
            reason = "High demand".to_string();
            adjustment_type = "surge".to_string();
        }
        // 2. Inventory Clearing (Discount)
        else if context.time_of_day == "closing_soon" || context.inventory_velocity == "slow" {
            adjusted_price = bounds.base_price * 0.85; // 15% discount
            reason = "Last minute deal!".to_string();
            adjustment_type = "discount".to_string();
        }
        // 3. Weather context (e.g. raining => want people to stick around or clear out depending on business)
        // For this basic version, let's say rain causes a slight discount to attract people if demand is normal
        else if context.weather == "raining" && context.demand_level == "normal" {
            adjusted_price = bounds.base_price * 0.90; // 10% discount
            reason = "Rainy day special".to_string();
            adjustment_type = "discount".to_string();
        }

        // Clamp to bounds
        if adjusted_price > bounds.max_price {
            adjusted_price = bounds.max_price;
        }
        if adjusted_price < bounds.min_price {
            adjusted_price = bounds.min_price;
        }

        // If clamped back to base price, reset reason
        if (adjusted_price - bounds.base_price).abs() < 0.01 {
            reason = "Standard price".to_string();
            adjustment_type = "none".to_string();
        }

        DynamicPriceResult {
            price: (adjusted_price * 100.0).round() / 100.0, // round to 2 decimal places
            reason,
            adjustment_type,
        }
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
