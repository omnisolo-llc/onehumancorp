use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingContext {
    pub temperature_f: Option<f64>,
    pub is_raining: Option<bool>,
    pub inventory_velocity: Option<f64>, // items sold per hour
    pub current_hour: Option<u32>,
    pub inventory_remaining: Option<u32>,
    pub closing_in_hours: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicPricingConfig {
    pub enabled: bool,
    pub min_price_cents: i64,
    pub max_price_cents: i64,
    pub strategies: Vec<PricingStrategy>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PricingStrategy {
    ClearInventory, // Lower price when close to closing and inventory is high
    MaximizeRevenue, // Raise price when velocity is high
    FillSchedule,    // Lower price for empty slots soon
    WeatherDemand,   // Adjust based on weather (e.g. cold drinks on hot days)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdjustedPrice {
    pub original_price_cents: i64,
    pub adjusted_price_cents: i64,
    pub reason: Option<String>,
}

pub struct PricingEngine;

impl PricingEngine {
    pub fn calculate_price(
        base_price_cents: i64,
        config: &DynamicPricingConfig,
        context: &PricingContext,
    ) -> AdjustedPrice {
        if !config.enabled {
            return AdjustedPrice {
                original_price_cents: base_price_cents,
                adjusted_price_cents: base_price_cents,
                reason: None,
            };
        }

        let mut current_price = base_price_cents as f64;
        let mut reasons = Vec::new();

        for strategy in &config.strategies {
            match strategy {
                PricingStrategy::ClearInventory => {
                    if let (Some(remaining), Some(closing_in)) = (context.inventory_remaining, context.closing_in_hours) {
                        if closing_in <= 2 && remaining > 5 {
                            // 20% discount to clear inventory
                            current_price *= 0.8;
                            reasons.push("Last Minute Discount");
                        }
                    }
                }
                PricingStrategy::MaximizeRevenue => {
                    if let Some(velocity) = context.inventory_velocity {
                        if velocity > 10.0 {
                            // High demand, increase price by 15%
                            current_price *= 1.15;
                            reasons.push("High Demand");
                        }
                    }
                }
                PricingStrategy::FillSchedule => {
                    if let Some(closing_in) = context.closing_in_hours {
                        if closing_in <= 3 {
                           // 10% discount for last minute
                           current_price *= 0.9;
                           reasons.push("Happy Hour");
                        }
                    }
                }
                PricingStrategy::WeatherDemand => {
                    if let Some(temp) = context.temperature_f {
                        if temp >= 90.0 {
                            // Hot weather surge
                            current_price *= 1.1;
                            reasons.push("Hot Weather Surge");
                        }
                    }
                }
            }
        }

        let mut adjusted_price = current_price as i64;

        // Apply bounds
        if adjusted_price < config.min_price_cents {
            adjusted_price = config.min_price_cents;
        } else if adjusted_price > config.max_price_cents {
            adjusted_price = config.max_price_cents;
        }

        let reason_str = if reasons.is_empty() {
            None
        } else {
            Some(reasons.join(", "))
        };

        AdjustedPrice {
            original_price_cents: base_price_cents,
            adjusted_price_cents: adjusted_price,
            reason: reason_str,
        }
    }
}
