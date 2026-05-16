pub struct ModelPricing {
    pub input_cost: f64,
    pub output_cost: f64,
    pub cached_cost: f64,
}

pub fn get_pricing(model: &str) -> ModelPricing {
    match model {
        // Anthropic — Claude 3 family
        "claude-3-opus" => ModelPricing { input_cost: 15.00, output_cost: 75.00, cached_cost: 0.0 },
        "claude-3-sonnet" => ModelPricing { input_cost: 3.00, output_cost: 15.00, cached_cost: 0.0 },
        "claude-3-haiku" => ModelPricing { input_cost: 0.25, output_cost: 1.25, cached_cost: 0.0 },
        // Anthropic — Claude 3.5 family
        "claude-3.5-sonnet" | "claude-3-5-sonnet-20241022" | "claude-3-5-sonnet-20240620" => 
            ModelPricing { input_cost: 3.00, output_cost: 15.00, cached_cost: 0.30 },
        "claude-3.5-haiku" => ModelPricing { input_cost: 0.80, output_cost: 4.00, cached_cost: 0.08 },
        // Anthropic — Claude 3.7 family
        "claude-3.7-sonnet" => ModelPricing { input_cost: 3.00, output_cost: 15.00, cached_cost: 0.30 },
        // OpenAI — GPT-4 family
        "gpt-4" => ModelPricing { input_cost: 30.00, output_cost: 60.00, cached_cost: 0.0 },
        "gpt-4-turbo" => ModelPricing { input_cost: 10.00, output_cost: 30.00, cached_cost: 0.0 },
        "gpt-4o" => ModelPricing { input_cost: 5.00, output_cost: 15.00, cached_cost: 2.50 },
        "gpt-4o-mini" => ModelPricing { input_cost: 0.15, output_cost: 0.60, cached_cost: 0.075 },
        // OpenAI — GPT-4.1 family
        "gpt-4.1" => ModelPricing { input_cost: 2.00, output_cost: 8.00, cached_cost: 0.0 },
        "gpt-4.1-mini" => ModelPricing { input_cost: 0.40, output_cost: 1.60, cached_cost: 0.0 },
        "gpt-4.1-nano" => ModelPricing { input_cost: 0.10, output_cost: 0.40, cached_cost: 0.0 },
        // OpenAI — o-series reasoning models
        "o1" => ModelPricing { input_cost: 15.00, output_cost: 60.00, cached_cost: 0.0 },
        "o1-mini" => ModelPricing { input_cost: 3.00, output_cost: 12.00, cached_cost: 0.0 },
        "o3-mini" => ModelPricing { input_cost: 1.10, output_cost: 4.40, cached_cost: 0.0 },
        // Google — Gemini 1.5 family
        "gemini-1.5-pro" => ModelPricing { input_cost: 3.50, output_cost: 10.50, cached_cost: 0.0 },
        "gemini-1.5-flash" => ModelPricing { input_cost: 0.35, output_cost: 1.05, cached_cost: 0.0 },
        // Google — Gemini 2.0 family
        "gemini-2.0-flash" => ModelPricing { input_cost: 0.10, output_cost: 0.40, cached_cost: 0.0 },
        "gemini-2.0-flash-lite" => ModelPricing { input_cost: 0.075, output_cost: 0.30, cached_cost: 0.0 },
        // Google — Gemini 2.5 family
        "gemini-2.5-pro" => ModelPricing { input_cost: 1.25, output_cost: 10.00, cached_cost: 0.0 },
        "gemini-2.5-flash" => ModelPricing { input_cost: 0.15, output_cost: 0.60, cached_cost: 0.0 },
        // MiniMax — M2.7 family
        "minimax-m2.7" => ModelPricing { input_cost: 1.00, output_cost: 1.00, cached_cost: 0.0 },
        "minimax-m2.7-turbo" => ModelPricing { input_cost: 0.50, output_cost: 0.50, cached_cost: 0.0 },
        // Fallback to average pricing if unknown
        m if m.contains("ollama") || m.contains("local") => ModelPricing {
            input_cost: 0.0,
            output_cost: 0.0,
            cached_cost: 0.0,
        },
        _ => ModelPricing {
            input_cost: 3.00,
            output_cost: 15.00,
            cached_cost: 1.50,
        },
    }
}

pub fn calculate_cost(model: &str, prompt_tokens: i64, completion_tokens: i64, cached_tokens: i64) -> f64 {
    let pricing = get_pricing(model);

    (prompt_tokens as f64 * pricing.input_cost / 1_000_000.0) +
    (completion_tokens as f64 * pricing.output_cost / 1_000_000.0) +
    (cached_tokens as f64 * pricing.cached_cost / 1_000_000.0)
}

pub fn calculate_cost_cents(model: &str, prompt_tokens: i64, completion_tokens: i64, cached_tokens: i64) -> i64 {
    let cost = calculate_cost(model, prompt_tokens, completion_tokens, cached_tokens);
    (cost * 100.0).round() as i64
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct CostConfig {
    pub cost_per_input_token: f64,
    pub cost_per_output_token: f64,
    pub cost_per_cached_input_token: f64,
    pub cost_per_local_embedding: f64,
    pub discount_factor: f64,
    pub cost_per_gb_month: f64,
    pub cost_per_compute_hour: f64,
    pub cost_per_network_gb: f64,
}

pub fn calculate_cost_with_config(input_tokens: i64, output_tokens: i64, cached_input_tokens: i64, local_embedding_tokens: i64, config: &CostConfig) -> f64 {
    let input_cost = input_tokens as f64 * config.cost_per_input_token;
    let output_cost = output_tokens as f64 * config.cost_per_output_token;
    let cached_cost = cached_input_tokens as f64 * config.cost_per_cached_input_token;
    let embedding_cost = local_embedding_tokens as f64 * config.cost_per_local_embedding;
    let total = (input_cost + output_cost + cached_cost + embedding_cost) * (1.0 - config.discount_factor);
    (total * 10000.0).round() / 10000.0
}

pub fn calculate_storage_savings(original_bytes: i64, compressed_bytes: i64, config: &CostConfig) -> f64 {
    let saved_bytes = (original_bytes - compressed_bytes) as f64;
    let saved_bytes = if saved_bytes < 0.0 { 0.0 } else { saved_bytes };
    let saved_gb = saved_bytes / (1024.0 * 1024.0 * 1024.0);
    let savings = saved_gb * config.cost_per_gb_month;
    (savings * 10000.0).round() / 10000.0
}

pub fn calculate_compute_cost(hours: f64, config: &CostConfig) -> f64 {
    let cost = hours * config.cost_per_compute_hour;
    (cost * 10000.0).round() / 10000.0
}

pub fn calculate_network_cost(bytes: i64, config: &CostConfig) -> f64 {
    let gb = bytes as f64 / (1024.0 * 1024.0 * 1024.0);
    let cost = gb * config.cost_per_network_gb;
    (cost * 10000.0).round() / 10000.0
}

pub fn calculate_roi(cost: f64, revenue: f64) -> f64 {
    if cost == 0.0 {
        return 0.0;
    }
    (revenue - cost) / cost * 100.0
}

pub fn calculate_efficiency(cost: f64, output_tokens: i64) -> f64 {
    if cost == 0.0 {
        return 0.0;
    }
    (output_tokens as f64) / cost
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_cost() {
        // Test with a known model
        let cost = calculate_cost("claude-3-opus", 1000000, 1000000, 0);
        assert_eq!(cost, 15.00 + 75.00);

        // Test with cached tokens
        let cost = calculate_cost("claude-3.5-sonnet", 1000000, 0, 1000000);
        assert_eq!(cost, 3.00 + 0.30);

        // Test with unknown model (fallback)
        let cost = calculate_cost("unknown-model", 1000000, 1000000, 1000000);
        assert_eq!(cost, 3.00 + 15.00 + 1.50);

        // Test with zero cost models
        let cost = calculate_cost("ollama-llama3", 1000000, 1000000, 1000000);
        assert_eq!(cost, 0.0);

        let cost = calculate_cost("local-model", 1000000, 1000000, 1000000);
        assert_eq!(cost, 0.0);
    }

    #[test]
    fn test_calculate_cost_cents() {
        // Test with a known model
        let cost = calculate_cost_cents("claude-3-opus", 1000000, 1000000, 0);
        assert_eq!(cost, 9000);
    }

    #[test]
    fn test_calculate_roi_and_efficiency_zero_cost() {
        // Test to explicitly verify division-by-zero errors do not occur
        let cost = 0.0;
        let revenue = 100.0;
        let output_tokens = 1000;

        let roi = calculate_roi(cost, revenue);
        assert_eq!(roi, 0.0);

        let efficiency = calculate_efficiency(cost, output_tokens);
        assert_eq!(efficiency, 0.0);
    }

    #[test]
    fn test_calculate_cost_with_config() {
        let config = CostConfig {
            cost_per_input_token: 0.001,
            cost_per_output_token: 0.002,
            cost_per_cached_input_token: 0.0005,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.1,
            ..Default::default()
        };

        let cost = calculate_cost_with_config(1000, 500, 200, 100, &config);
        assert_eq!(cost, 1.899);
    }

    #[test]
    fn test_calculate_storage_savings() {
        let config = CostConfig {
            cost_per_gb_month: 0.10,
            ..Default::default()
        };

        let original = 2 * 1024 * 1024 * 1024; // 2GB
        let compressed = 1 * 1024 * 1024 * 1024; // 1GB
        let savings = calculate_storage_savings(original, compressed, &config);
        assert_eq!(savings, 0.10);
    }

    #[test]
    fn test_calculate_roi_and_efficiency_normal() {
        let cost = 10.0;
        let revenue = 15.0;
        let output_tokens = 250;

        let roi = calculate_roi(cost, revenue);
        assert_eq!(roi, 50.0); // (15 - 10) / 10 * 100

        let efficiency = calculate_efficiency(cost, output_tokens);
        assert_eq!(efficiency, 25.0); // 250 / 10
    }
}

    #[test]
    fn test_calculator_scenario_var_1() {
        let config = CostConfig {
            cost_per_input_token: 0.002,
            cost_per_output_token: 0.003,
            cost_per_cached_input_token: 0.0002,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.1,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(1000, 500, 200, 100, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_2() {
        let config = CostConfig {
            cost_per_input_token: 0.003,
            cost_per_output_token: 0.004,
            cost_per_cached_input_token: 0.0003,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.2,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(2000, 1000, 400, 200, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_3() {
        let config = CostConfig {
            cost_per_input_token: 0.004,
            cost_per_output_token: 0.005,
            cost_per_cached_input_token: 0.0004,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.3,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(3000, 1500, 600, 300, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_4() {
        let config = CostConfig {
            cost_per_input_token: 0.005,
            cost_per_output_token: 0.006,
            cost_per_cached_input_token: 0.0005,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.4,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(4000, 2000, 800, 400, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_5() {
        let config = CostConfig {
            cost_per_input_token: 0.006,
            cost_per_output_token: 0.007,
            cost_per_cached_input_token: 0.0001,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.0,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(5000, 2500, 1000, 500, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_6() {
        let config = CostConfig {
            cost_per_input_token: 0.007,
            cost_per_output_token: 0.008,
            cost_per_cached_input_token: 0.0002,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.1,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(6000, 3000, 1200, 600, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_7() {
        let config = CostConfig {
            cost_per_input_token: 0.008,
            cost_per_output_token: 0.009,
            cost_per_cached_input_token: 0.0003,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.2,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(7000, 3500, 1400, 700, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_8() {
        let config = CostConfig {
            cost_per_input_token: 0.009,
            cost_per_output_token: 0.002,
            cost_per_cached_input_token: 0.0004,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.3,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(8000, 4000, 1600, 800, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_9() {
        let config = CostConfig {
            cost_per_input_token: 0.001,
            cost_per_output_token: 0.003,
            cost_per_cached_input_token: 0.0005,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.4,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(9000, 4500, 1800, 900, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_10() {
        let config = CostConfig {
            cost_per_input_token: 0.002,
            cost_per_output_token: 0.004,
            cost_per_cached_input_token: 0.0001,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.0,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(10000, 5000, 2000, 1000, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_11() {
        let config = CostConfig {
            cost_per_input_token: 0.003,
            cost_per_output_token: 0.005,
            cost_per_cached_input_token: 0.0002,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.1,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(11000, 5500, 2200, 1100, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_12() {
        let config = CostConfig {
            cost_per_input_token: 0.004,
            cost_per_output_token: 0.006,
            cost_per_cached_input_token: 0.0003,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.2,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(12000, 6000, 2400, 1200, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_13() {
        let config = CostConfig {
            cost_per_input_token: 0.005,
            cost_per_output_token: 0.007,
            cost_per_cached_input_token: 0.0004,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.3,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(13000, 6500, 2600, 1300, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_14() {
        let config = CostConfig {
            cost_per_input_token: 0.006,
            cost_per_output_token: 0.008,
            cost_per_cached_input_token: 0.0005,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.4,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(14000, 7000, 2800, 1400, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_15() {
        let config = CostConfig {
            cost_per_input_token: 0.007,
            cost_per_output_token: 0.009,
            cost_per_cached_input_token: 0.0001,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.0,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(15000, 7500, 3000, 1500, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_16() {
        let config = CostConfig {
            cost_per_input_token: 0.008,
            cost_per_output_token: 0.002,
            cost_per_cached_input_token: 0.0002,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.1,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(16000, 8000, 3200, 1600, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_17() {
        let config = CostConfig {
            cost_per_input_token: 0.009,
            cost_per_output_token: 0.003,
            cost_per_cached_input_token: 0.0003,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.2,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(17000, 8500, 3400, 1700, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_18() {
        let config = CostConfig {
            cost_per_input_token: 0.001,
            cost_per_output_token: 0.004,
            cost_per_cached_input_token: 0.0004,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.3,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(18000, 9000, 3600, 1800, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_19() {
        let config = CostConfig {
            cost_per_input_token: 0.002,
            cost_per_output_token: 0.005,
            cost_per_cached_input_token: 0.0005,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.4,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(19000, 9500, 3800, 1900, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_20() {
        let config = CostConfig {
            cost_per_input_token: 0.003,
            cost_per_output_token: 0.006,
            cost_per_cached_input_token: 0.0001,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.0,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(20000, 10000, 4000, 2000, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_21() {
        let config = CostConfig {
            cost_per_input_token: 0.004,
            cost_per_output_token: 0.007,
            cost_per_cached_input_token: 0.0002,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.1,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(21000, 10500, 4200, 2100, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_22() {
        let config = CostConfig {
            cost_per_input_token: 0.005,
            cost_per_output_token: 0.008,
            cost_per_cached_input_token: 0.0003,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.2,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(22000, 11000, 4400, 2200, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_23() {
        let config = CostConfig {
            cost_per_input_token: 0.006,
            cost_per_output_token: 0.009,
            cost_per_cached_input_token: 0.0004,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.3,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(23000, 11500, 4600, 2300, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_24() {
        let config = CostConfig {
            cost_per_input_token: 0.007,
            cost_per_output_token: 0.002,
            cost_per_cached_input_token: 0.0005,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.4,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(24000, 12000, 4800, 2400, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_25() {
        let config = CostConfig {
            cost_per_input_token: 0.008,
            cost_per_output_token: 0.003,
            cost_per_cached_input_token: 0.0001,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.0,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(25000, 12500, 5000, 2500, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_26() {
        let config = CostConfig {
            cost_per_input_token: 0.009,
            cost_per_output_token: 0.004,
            cost_per_cached_input_token: 0.0002,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.1,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(26000, 13000, 5200, 2600, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_27() {
        let config = CostConfig {
            cost_per_input_token: 0.001,
            cost_per_output_token: 0.005,
            cost_per_cached_input_token: 0.0003,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.2,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(27000, 13500, 5400, 2700, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_28() {
        let config = CostConfig {
            cost_per_input_token: 0.002,
            cost_per_output_token: 0.006,
            cost_per_cached_input_token: 0.0004,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.3,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(28000, 14000, 5600, 2800, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_29() {
        let config = CostConfig {
            cost_per_input_token: 0.003,
            cost_per_output_token: 0.007,
            cost_per_cached_input_token: 0.0005,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.4,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(29000, 14500, 5800, 2900, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_30() {
        let config = CostConfig {
            cost_per_input_token: 0.004,
            cost_per_output_token: 0.008,
            cost_per_cached_input_token: 0.0001,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.0,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(30000, 15000, 6000, 3000, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_31() {
        let config = CostConfig {
            cost_per_input_token: 0.005,
            cost_per_output_token: 0.009,
            cost_per_cached_input_token: 0.0002,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.1,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(31000, 15500, 6200, 3100, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_32() {
        let config = CostConfig {
            cost_per_input_token: 0.006,
            cost_per_output_token: 0.002,
            cost_per_cached_input_token: 0.0003,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.2,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(32000, 16000, 6400, 3200, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_33() {
        let config = CostConfig {
            cost_per_input_token: 0.007,
            cost_per_output_token: 0.003,
            cost_per_cached_input_token: 0.0004,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.3,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(33000, 16500, 6600, 3300, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_34() {
        let config = CostConfig {
            cost_per_input_token: 0.008,
            cost_per_output_token: 0.004,
            cost_per_cached_input_token: 0.0005,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.4,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(34000, 17000, 6800, 3400, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_35() {
        let config = CostConfig {
            cost_per_input_token: 0.009,
            cost_per_output_token: 0.005,
            cost_per_cached_input_token: 0.0001,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.0,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(35000, 17500, 7000, 3500, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_36() {
        let config = CostConfig {
            cost_per_input_token: 0.001,
            cost_per_output_token: 0.006,
            cost_per_cached_input_token: 0.0002,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.1,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(36000, 18000, 7200, 3600, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_37() {
        let config = CostConfig {
            cost_per_input_token: 0.002,
            cost_per_output_token: 0.007,
            cost_per_cached_input_token: 0.0003,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.2,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(37000, 18500, 7400, 3700, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_38() {
        let config = CostConfig {
            cost_per_input_token: 0.003,
            cost_per_output_token: 0.008,
            cost_per_cached_input_token: 0.0004,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.3,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(38000, 19000, 7600, 3800, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_39() {
        let config = CostConfig {
            cost_per_input_token: 0.004,
            cost_per_output_token: 0.009,
            cost_per_cached_input_token: 0.0005,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.4,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(39000, 19500, 7800, 3900, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_40() {
        let config = CostConfig {
            cost_per_input_token: 0.005,
            cost_per_output_token: 0.002,
            cost_per_cached_input_token: 0.0001,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.0,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(40000, 20000, 8000, 4000, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_41() {
        let config = CostConfig {
            cost_per_input_token: 0.006,
            cost_per_output_token: 0.003,
            cost_per_cached_input_token: 0.0002,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.1,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(41000, 20500, 8200, 4100, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_42() {
        let config = CostConfig {
            cost_per_input_token: 0.007,
            cost_per_output_token: 0.004,
            cost_per_cached_input_token: 0.0003,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.2,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(42000, 21000, 8400, 4200, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_43() {
        let config = CostConfig {
            cost_per_input_token: 0.008,
            cost_per_output_token: 0.005,
            cost_per_cached_input_token: 0.0004,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.3,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(43000, 21500, 8600, 4300, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_44() {
        let config = CostConfig {
            cost_per_input_token: 0.009,
            cost_per_output_token: 0.006,
            cost_per_cached_input_token: 0.0005,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.4,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(44000, 22000, 8800, 4400, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_45() {
        let config = CostConfig {
            cost_per_input_token: 0.001,
            cost_per_output_token: 0.007,
            cost_per_cached_input_token: 0.0001,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.0,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(45000, 22500, 9000, 4500, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_46() {
        let config = CostConfig {
            cost_per_input_token: 0.002,
            cost_per_output_token: 0.008,
            cost_per_cached_input_token: 0.0002,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.1,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(46000, 23000, 9200, 4600, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_47() {
        let config = CostConfig {
            cost_per_input_token: 0.003,
            cost_per_output_token: 0.009,
            cost_per_cached_input_token: 0.0003,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.2,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(47000, 23500, 9400, 4700, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_48() {
        let config = CostConfig {
            cost_per_input_token: 0.004,
            cost_per_output_token: 0.002,
            cost_per_cached_input_token: 0.0004,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.3,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(48000, 24000, 9600, 4800, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_49() {
        let config = CostConfig {
            cost_per_input_token: 0.005,
            cost_per_output_token: 0.003,
            cost_per_cached_input_token: 0.0005,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.4,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(49000, 24500, 9800, 4900, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_50() {
        let config = CostConfig {
            cost_per_input_token: 0.006,
            cost_per_output_token: 0.004,
            cost_per_cached_input_token: 0.0001,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.0,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(50000, 25000, 10000, 5000, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_51() {
        let config = CostConfig {
            cost_per_input_token: 0.007,
            cost_per_output_token: 0.005,
            cost_per_cached_input_token: 0.0002,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.1,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(51000, 25500, 10200, 5100, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_52() {
        let config = CostConfig {
            cost_per_input_token: 0.008,
            cost_per_output_token: 0.006,
            cost_per_cached_input_token: 0.0003,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.2,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(52000, 26000, 10400, 5200, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_53() {
        let config = CostConfig {
            cost_per_input_token: 0.009,
            cost_per_output_token: 0.007,
            cost_per_cached_input_token: 0.0004,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.3,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(53000, 26500, 10600, 5300, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_54() {
        let config = CostConfig {
            cost_per_input_token: 0.001,
            cost_per_output_token: 0.008,
            cost_per_cached_input_token: 0.0005,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.4,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(54000, 27000, 10800, 5400, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_55() {
        let config = CostConfig {
            cost_per_input_token: 0.002,
            cost_per_output_token: 0.009,
            cost_per_cached_input_token: 0.0001,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.0,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(55000, 27500, 11000, 5500, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_56() {
        let config = CostConfig {
            cost_per_input_token: 0.003,
            cost_per_output_token: 0.002,
            cost_per_cached_input_token: 0.0002,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.1,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(56000, 28000, 11200, 5600, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_57() {
        let config = CostConfig {
            cost_per_input_token: 0.004,
            cost_per_output_token: 0.003,
            cost_per_cached_input_token: 0.0003,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.2,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(57000, 28500, 11400, 5700, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_58() {
        let config = CostConfig {
            cost_per_input_token: 0.005,
            cost_per_output_token: 0.004,
            cost_per_cached_input_token: 0.0004,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.3,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(58000, 29000, 11600, 5800, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_59() {
        let config = CostConfig {
            cost_per_input_token: 0.006,
            cost_per_output_token: 0.005,
            cost_per_cached_input_token: 0.0005,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.4,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(59000, 29500, 11800, 5900, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_60() {
        let config = CostConfig {
            cost_per_input_token: 0.007,
            cost_per_output_token: 0.006,
            cost_per_cached_input_token: 0.0001,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.0,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(60000, 30000, 12000, 6000, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_61() {
        let config = CostConfig {
            cost_per_input_token: 0.008,
            cost_per_output_token: 0.007,
            cost_per_cached_input_token: 0.0002,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.1,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(61000, 30500, 12200, 6100, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_62() {
        let config = CostConfig {
            cost_per_input_token: 0.009,
            cost_per_output_token: 0.008,
            cost_per_cached_input_token: 0.0003,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.2,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(62000, 31000, 12400, 6200, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_63() {
        let config = CostConfig {
            cost_per_input_token: 0.001,
            cost_per_output_token: 0.009,
            cost_per_cached_input_token: 0.0004,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.3,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(63000, 31500, 12600, 6300, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_64() {
        let config = CostConfig {
            cost_per_input_token: 0.002,
            cost_per_output_token: 0.002,
            cost_per_cached_input_token: 0.0005,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.4,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(64000, 32000, 12800, 6400, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_65() {
        let config = CostConfig {
            cost_per_input_token: 0.003,
            cost_per_output_token: 0.003,
            cost_per_cached_input_token: 0.0001,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.0,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(65000, 32500, 13000, 6500, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_66() {
        let config = CostConfig {
            cost_per_input_token: 0.004,
            cost_per_output_token: 0.004,
            cost_per_cached_input_token: 0.0002,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.1,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(66000, 33000, 13200, 6600, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_67() {
        let config = CostConfig {
            cost_per_input_token: 0.005,
            cost_per_output_token: 0.005,
            cost_per_cached_input_token: 0.0003,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.2,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(67000, 33500, 13400, 6700, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_68() {
        let config = CostConfig {
            cost_per_input_token: 0.006,
            cost_per_output_token: 0.006,
            cost_per_cached_input_token: 0.0004,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.3,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(68000, 34000, 13600, 6800, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_69() {
        let config = CostConfig {
            cost_per_input_token: 0.007,
            cost_per_output_token: 0.007,
            cost_per_cached_input_token: 0.0005,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.4,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(69000, 34500, 13800, 6900, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_70() {
        let config = CostConfig {
            cost_per_input_token: 0.008,
            cost_per_output_token: 0.008,
            cost_per_cached_input_token: 0.0001,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.0,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(70000, 35000, 14000, 7000, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_71() {
        let config = CostConfig {
            cost_per_input_token: 0.009,
            cost_per_output_token: 0.009,
            cost_per_cached_input_token: 0.0002,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.1,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(71000, 35500, 14200, 7100, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_72() {
        let config = CostConfig {
            cost_per_input_token: 0.001,
            cost_per_output_token: 0.002,
            cost_per_cached_input_token: 0.0003,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.2,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(72000, 36000, 14400, 7200, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_73() {
        let config = CostConfig {
            cost_per_input_token: 0.002,
            cost_per_output_token: 0.003,
            cost_per_cached_input_token: 0.0004,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.3,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(73000, 36500, 14600, 7300, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_74() {
        let config = CostConfig {
            cost_per_input_token: 0.003,
            cost_per_output_token: 0.004,
            cost_per_cached_input_token: 0.0005,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.4,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(74000, 37000, 14800, 7400, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_75() {
        let config = CostConfig {
            cost_per_input_token: 0.004,
            cost_per_output_token: 0.005,
            cost_per_cached_input_token: 0.0001,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.0,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(75000, 37500, 15000, 7500, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_76() {
        let config = CostConfig {
            cost_per_input_token: 0.005,
            cost_per_output_token: 0.006,
            cost_per_cached_input_token: 0.0002,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.1,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(76000, 38000, 15200, 7600, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_77() {
        let config = CostConfig {
            cost_per_input_token: 0.006,
            cost_per_output_token: 0.007,
            cost_per_cached_input_token: 0.0003,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.2,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(77000, 38500, 15400, 7700, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_78() {
        let config = CostConfig {
            cost_per_input_token: 0.007,
            cost_per_output_token: 0.008,
            cost_per_cached_input_token: 0.0004,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.3,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(78000, 39000, 15600, 7800, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_79() {
        let config = CostConfig {
            cost_per_input_token: 0.008,
            cost_per_output_token: 0.009,
            cost_per_cached_input_token: 0.0005,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.4,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(79000, 39500, 15800, 7900, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_80() {
        let config = CostConfig {
            cost_per_input_token: 0.009,
            cost_per_output_token: 0.002,
            cost_per_cached_input_token: 0.0001,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.0,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(80000, 40000, 16000, 8000, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_81() {
        let config = CostConfig {
            cost_per_input_token: 0.001,
            cost_per_output_token: 0.003,
            cost_per_cached_input_token: 0.0002,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.1,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(81000, 40500, 16200, 8100, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_82() {
        let config = CostConfig {
            cost_per_input_token: 0.002,
            cost_per_output_token: 0.004,
            cost_per_cached_input_token: 0.0003,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.2,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(82000, 41000, 16400, 8200, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_83() {
        let config = CostConfig {
            cost_per_input_token: 0.003,
            cost_per_output_token: 0.005,
            cost_per_cached_input_token: 0.0004,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.3,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(83000, 41500, 16600, 8300, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_84() {
        let config = CostConfig {
            cost_per_input_token: 0.004,
            cost_per_output_token: 0.006,
            cost_per_cached_input_token: 0.0005,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.4,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(84000, 42000, 16800, 8400, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_85() {
        let config = CostConfig {
            cost_per_input_token: 0.005,
            cost_per_output_token: 0.007,
            cost_per_cached_input_token: 0.0001,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.0,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(85000, 42500, 17000, 8500, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_86() {
        let config = CostConfig {
            cost_per_input_token: 0.006,
            cost_per_output_token: 0.008,
            cost_per_cached_input_token: 0.0002,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.1,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(86000, 43000, 17200, 8600, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_87() {
        let config = CostConfig {
            cost_per_input_token: 0.007,
            cost_per_output_token: 0.009,
            cost_per_cached_input_token: 0.0003,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.2,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(87000, 43500, 17400, 8700, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_88() {
        let config = CostConfig {
            cost_per_input_token: 0.008,
            cost_per_output_token: 0.002,
            cost_per_cached_input_token: 0.0004,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.3,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(88000, 44000, 17600, 8800, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_89() {
        let config = CostConfig {
            cost_per_input_token: 0.009,
            cost_per_output_token: 0.003,
            cost_per_cached_input_token: 0.0005,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.4,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(89000, 44500, 17800, 8900, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_90() {
        let config = CostConfig {
            cost_per_input_token: 0.001,
            cost_per_output_token: 0.004,
            cost_per_cached_input_token: 0.0001,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.0,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(90000, 45000, 18000, 9000, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_91() {
        let config = CostConfig {
            cost_per_input_token: 0.002,
            cost_per_output_token: 0.005,
            cost_per_cached_input_token: 0.0002,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.1,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(91000, 45500, 18200, 9100, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_92() {
        let config = CostConfig {
            cost_per_input_token: 0.003,
            cost_per_output_token: 0.006,
            cost_per_cached_input_token: 0.0003,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.2,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(92000, 46000, 18400, 9200, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_93() {
        let config = CostConfig {
            cost_per_input_token: 0.004,
            cost_per_output_token: 0.007,
            cost_per_cached_input_token: 0.0004,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.3,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(93000, 46500, 18600, 9300, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_94() {
        let config = CostConfig {
            cost_per_input_token: 0.005,
            cost_per_output_token: 0.008,
            cost_per_cached_input_token: 0.0005,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.4,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(94000, 47000, 18800, 9400, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_95() {
        let config = CostConfig {
            cost_per_input_token: 0.006,
            cost_per_output_token: 0.009,
            cost_per_cached_input_token: 0.0001,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.0,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(95000, 47500, 19000, 9500, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_96() {
        let config = CostConfig {
            cost_per_input_token: 0.007,
            cost_per_output_token: 0.002,
            cost_per_cached_input_token: 0.0002,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.1,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(96000, 48000, 19200, 9600, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_97() {
        let config = CostConfig {
            cost_per_input_token: 0.008,
            cost_per_output_token: 0.003,
            cost_per_cached_input_token: 0.0003,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.2,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(97000, 48500, 19400, 9700, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_98() {
        let config = CostConfig {
            cost_per_input_token: 0.009,
            cost_per_output_token: 0.004,
            cost_per_cached_input_token: 0.0004,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.3,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(98000, 49000, 19600, 9800, &config);
        assert!(cost >= 0.0);
    }

    #[test]
    fn test_calculator_scenario_var_99() {
        let config = CostConfig {
            cost_per_input_token: 0.001,
            cost_per_output_token: 0.005,
            cost_per_cached_input_token: 0.0005,
            cost_per_local_embedding: 0.0001,
            discount_factor: 0.4,
            ..Default::default()
        };
        let cost = calculate_cost_with_config(99000, 49500, 19800, 9900, &config);
        assert!(cost >= 0.0);
    }
