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
