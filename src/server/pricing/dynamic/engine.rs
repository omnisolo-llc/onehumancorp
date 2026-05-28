use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicPrice {
    pub original_price_cents: i64,
    pub adjusted_price_cents: i64,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalContext {
    pub inventory_velocity: String, // e.g., "fast", "slow", "normal"
    pub time_to_close_hours: Option<f64>,
    pub weather_demand_multiplier: Option<f64>,
}

pub struct AutonomousPricingEngine;

impl AutonomousPricingEngine {
    pub fn evaluate_price(
        base_price_cents: i64,
        min_price_cents: i64,
        max_price_cents: i64,
        context: LocalContext,
    ) -> DynamicPrice {
        let mut adjusted_price = base_price_cents as f64;
        let mut reason = "Standard Pricing".to_string();

        // Evaluate Inventory Velocity
        match context.inventory_velocity.as_str() {
            "fast" => {
                adjusted_price *= 1.15; // +15%
                reason = "High Demand".to_string();
            }
            "slow" => {
                adjusted_price *= 0.85; // -15%
                reason = "Clearance".to_string();
            }
            _ => {}
        }

        // Evaluate Time to Close
        if let Some(hours) = context.time_to_close_hours {
            if hours <= 2.0 && context.inventory_velocity != "fast" {
                adjusted_price *= 0.80; // Additional -20%
                reason = "Last Minute Deal!".to_string();
            }
        }

        // Evaluate Weather Demand
        if let Some(multiplier) = context.weather_demand_multiplier {
            adjusted_price *= multiplier;
            if multiplier > 1.0 {
                reason = "Surge Pricing".to_string();
            } else if multiplier < 1.0 {
                reason = "Happy Hour!".to_string();
            }
        }

        // Ensure price is within bounds
        let mut final_price_cents = adjusted_price.round() as i64;
        if final_price_cents < min_price_cents {
            final_price_cents = min_price_cents;
            reason = format!("{} (Min Price Reached)", reason);
        } else if final_price_cents > max_price_cents {
            final_price_cents = max_price_cents;
            reason = format!("{} (Max Price Reached)", reason);
        }

        // Telemetry
        if final_price_cents != base_price_cents {
            tracing::info!(
                original_price_cents = base_price_cents,
                adjusted_price_cents = final_price_cents,
                reason = %reason,
                "Dynamic pricing adjusted price"
            );
        }

        DynamicPrice {
            original_price_cents: base_price_cents,
            adjusted_price_cents: final_price_cents,
            reason,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_high_demand_surge() {
        let context = LocalContext {
            inventory_velocity: "fast".to_string(),
            time_to_close_hours: None,
            weather_demand_multiplier: Some(1.2),
        };
        let result = AutonomousPricingEngine::evaluate_price(1000, 800, 1500, context);
        // Base: 1000. Velocity "fast" -> 1150. Weather "1.2" -> 1380. Reason: Surge Pricing
        assert_eq!(result.adjusted_price_cents, 1380);
        assert!(result.reason.contains("Surge"));
    }

    #[test]
    fn test_clearance_discount() {
        let context = LocalContext {
            inventory_velocity: "slow".to_string(),
            time_to_close_hours: Some(1.0),
            weather_demand_multiplier: None,
        };
        let result = AutonomousPricingEngine::evaluate_price(1000, 500, 1500, context);
        // Base: 1000. Velocity "slow" -> 850. Time to close < 2 -> 850 * 0.8 = 680. Reason: Last Minute Deal!
        assert_eq!(result.adjusted_price_cents, 680);
        assert!(result.reason.contains("Last Minute Deal!"));
    }

    #[test]
    fn test_min_price_boundary_enforcement() {
        let context = LocalContext {
            inventory_velocity: "slow".to_string(),
            time_to_close_hours: Some(1.0),
            weather_demand_multiplier: None,
        };
        // Price would go down to 680, but min is 800
        let result = AutonomousPricingEngine::evaluate_price(1000, 800, 1500, context);
        assert_eq!(result.adjusted_price_cents, 800);
        assert!(result.reason.contains("Min Price Reached"));
    }

    #[test]
    fn test_max_price_boundary_enforcement() {
        let context = LocalContext {
            inventory_velocity: "fast".to_string(),
            time_to_close_hours: None,
            weather_demand_multiplier: Some(2.0),
        };
        // Price would go up to 2300, but max is 1500
        let result = AutonomousPricingEngine::evaluate_price(1000, 800, 1500, context);
        assert_eq!(result.adjusted_price_cents, 1500);
        assert!(result.reason.contains("Max Price Reached"));
    }
}
