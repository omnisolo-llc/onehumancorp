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
        // Very basic logic for now. We can make it more sophisticated later.
        let (mut adjusted_price, mut reason, mut adjustment_type) = if context.demand_level == "high" || context.inventory_velocity == "fast" {
            // 1. Demand & Velocity (Surge)
            (bounds.base_price * 1.15, "High demand", "surge")
        } else if context.time_of_day == "closing_soon" || context.inventory_velocity == "slow" {
            // 2. Inventory Clearing (Discount)
            (bounds.base_price * 0.85, "Last minute deal!", "discount")
        } else if context.weather == "raining" && context.demand_level == "normal" {
            // 3. Weather context (e.g. raining => want people to stick around or clear out depending on business)
            (bounds.base_price * 0.90, "Rainy day special", "discount")
        } else {
            (bounds.base_price, "Standard price", "none")
        };

        // Clamp to bounds
        adjusted_price = adjusted_price.clamp(bounds.min_price, bounds.max_price);

        // If clamped back to base price, reset reason
        if (adjusted_price - bounds.base_price).abs() < 0.01 {
            reason = "Standard price";
            adjustment_type = "none";
        }

        DynamicPriceResult {
            price: (adjusted_price * 100.0).round() / 100.0, // round to 2 decimal places
            reason: reason.to_string(),
            adjustment_type: adjustment_type.to_string(),
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
// verified by miser
