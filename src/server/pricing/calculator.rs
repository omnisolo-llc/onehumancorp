use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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

pub struct ModelPricing {
    pub input_cost: f64,
    pub output_cost: f64,
    pub cached_cost: f64,
}

#[inline]
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
        "o1" | "o1-preview" => ModelPricing { input_cost: 15.00, output_cost: 60.00, cached_cost: 7.50 },
        "o1-mini" => ModelPricing { input_cost: 3.00, output_cost: 12.00, cached_cost: 1.50 },
        "o3-mini" => ModelPricing { input_cost: 1.10, output_cost: 4.40, cached_cost: 0.55 },
        // DeepSeek family
        "deepseek-chat" | "deepseek-v3" => ModelPricing { input_cost: 0.14, output_cost: 0.28, cached_cost: 0.014 },
        "deepseek-reasoner" | "deepseek-r1" => ModelPricing { input_cost: 0.55, output_cost: 2.19, cached_cost: 0.14 },
        // Meta Llama 3/3.1/3.2/3.3 family via typical API providers
        "llama-3.3-70b-versatile" | "llama-3.1-8b-instant" | "llama3-8b-8192" => ModelPricing { input_cost: 0.05, output_cost: 0.08, cached_cost: 0.0 },
        "llama-3.1-70b-versatile" | "llama3-70b-8192" => ModelPricing { input_cost: 0.15, output_cost: 0.20, cached_cost: 0.0 },
        "llama-3.1-405b-reasoning" => ModelPricing { input_cost: 2.70, output_cost: 2.70, cached_cost: 0.0 },
        // xAI — Grok family
        "grok-3" | "grok-2" => ModelPricing { input_cost: 2.00, output_cost: 10.00, cached_cost: 0.0 },
        "grok-3-mini" | "grok-2-mini" => ModelPricing { input_cost: 0.20, output_cost: 1.00, cached_cost: 0.0 },
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
            cached_cost: 1.50, // Standardize a default cached cost for unknown models
        },
    }
}

#[inline]
pub fn calculate_cost_cents(model: &str, input_tokens: i64, output_tokens: i64, cached_input_tokens: i64) -> i64 {
    let cost = calculate_cost(model, input_tokens, output_tokens, cached_input_tokens);
    (cost * 100.0).round() as i64
}

#[inline]
pub fn calculate_cost(model: &str, input_tokens: i64, output_tokens: i64, cached_input_tokens: i64) -> f64 {
    let pricing = get_pricing(model);

    // Per 1M tokens
    let input_cost = (input_tokens as f64 / 1_000_000.0) * pricing.input_cost;
    let output_cost = (output_tokens as f64 / 1_000_000.0) * pricing.output_cost;
    let cached_cost = (cached_input_tokens as f64 / 1_000_000.0) * pricing.cached_cost;

    let total = input_cost + output_cost + cached_cost;
    (total * 10000.0).round() / 10000.0
}

#[inline]
pub fn calculate_cost_with_config_cents(input_tokens: i64, output_tokens: i64, cached_input_tokens: i64, local_embedding_tokens: i64, config: &CostConfig) -> i64 {
    let cost = calculate_cost_with_config(input_tokens, output_tokens, cached_input_tokens, local_embedding_tokens, config);
    (cost * 100.0).round() as i64
}

#[inline]
pub fn calculate_cost_with_config(input_tokens: i64, output_tokens: i64, cached_input_tokens: i64, local_embedding_tokens: i64, config: &CostConfig) -> f64 {
    let input_cost = input_tokens as f64 * config.cost_per_input_token;
    let output_cost = output_tokens as f64 * config.cost_per_output_token;
    let cached_cost = cached_input_tokens as f64 * config.cost_per_cached_input_token;
    let embedding_cost = local_embedding_tokens as f64 * config.cost_per_local_embedding;
    let total = (input_cost + output_cost + cached_cost + embedding_cost) * (1.0 - config.discount_factor);
    (total * 10000.0).round() / 10000.0
}

#[inline]
pub fn calculate_storage_savings_cents(original_bytes: i64, compressed_bytes: i64, config: &CostConfig) -> i64 {
    let cost = calculate_storage_savings(original_bytes, compressed_bytes, config);
    (cost * 100.0).round() as i64
}

#[inline]
pub fn calculate_storage_savings(original_bytes: i64, compressed_bytes: i64, config: &CostConfig) -> f64 {
    if original_bytes < 0 || compressed_bytes < 0 {
        return 0.0;
    }
    let saved_bytes = (original_bytes - compressed_bytes) as f64;
    let saved_bytes = if saved_bytes < 0.0 { 0.0 } else { saved_bytes };
    let saved_gb = saved_bytes / (1024.0 * 1024.0 * 1024.0);
    let savings = saved_gb * config.cost_per_gb_month;
    (savings * 10000.0).round() / 10000.0
}

#[inline]
pub fn calculate_bandwidth_savings_cents(original_bytes: i64, compressed_bytes: i64, config: &CostConfig) -> i64 {
    let cost = calculate_bandwidth_savings(original_bytes, compressed_bytes, config);
    (cost * 100.0).round() as i64
}

#[inline]
pub fn calculate_bandwidth_savings(original_bytes: i64, compressed_bytes: i64, config: &CostConfig) -> f64 {
    if original_bytes < 0 || compressed_bytes < 0 {
        return 0.0;
    }
    let saved_bytes = (original_bytes - compressed_bytes) as f64;
    let saved_bytes = if saved_bytes < 0.0 { 0.0 } else { saved_bytes };
    let saved_gb = saved_bytes / (1024.0 * 1024.0 * 1024.0);
    let savings = saved_gb * config.cost_per_network_gb;
    (savings * 10000.0).round() / 10000.0
}

#[inline]
pub fn calculate_compute_cost_cents(hours: f64, config: &CostConfig) -> i64 {
    let cost = calculate_compute_cost(hours, config);
    (cost * 100.0).round() as i64
}

#[inline]
pub fn calculate_compute_cost(hours: f64, config: &CostConfig) -> f64 {
    if hours < 0.0 {
        return 0.0;
    }
    let cost = hours * config.cost_per_compute_hour;
    (cost * 10000.0).round() / 10000.0
}

#[inline]
pub fn calculate_network_cost_cents(bytes: i64, config: &CostConfig) -> i64 {
    let cost = calculate_network_cost(bytes, config);
    (cost * 100.0).round() as i64
}

#[inline]
pub fn calculate_network_cost(bytes: i64, config: &CostConfig) -> f64 {
    if bytes < 0 {
        return 0.0;
    }
    let gb = bytes as f64 / (1024.0 * 1024.0 * 1024.0);
    let cost = gb * config.cost_per_network_gb;
    (cost * 10000.0).round() / 10000.0
}

#[inline]
pub fn calculate_storage_cost_cents(bytes: i64, config: &CostConfig) -> i64 {
    let cost = calculate_storage_cost(bytes, config);
    (cost * 100.0).round() as i64
}

#[inline]
pub fn calculate_storage_cost(bytes: i64, config: &CostConfig) -> f64 {
    if bytes < 0 {
        return 0.0;
    }
    let gb = bytes as f64 / (1024.0 * 1024.0 * 1024.0);
    let cost = gb * config.cost_per_gb_month;
    (cost * 10000.0).round() / 10000.0
}

#[inline]
pub fn calculate_roi(cost: f64, revenue: f64) -> f64 {
    if cost <= 0.0 {
        return 0.0;
    }
    (revenue - cost) / cost * 100.0
}

#[inline]
pub fn calculate_efficiency(cost: f64, output_tokens: i64) -> f64 {
    if cost <= 0.0 {
        return 0.0;
    }
    (output_tokens as f64) / cost
}

// Advanced heuristic: estimate savings when fallback logic kicks in or tokens are dynamically truncated
#[inline]
pub fn calculate_heuristic_token_efficiency(original_tokens: i64, truncated_tokens: i64, model: &str) -> f64 {
    tracing::info!("💰 Miser telemetry: Calculating token efficiency for model: {}", model); // pii-safe
    if original_tokens <= truncated_tokens || original_tokens == 0 {
        return 0.0;
    }
    let saved_tokens = original_tokens - truncated_tokens;
    let pricing = get_pricing(model);
    let estimated_savings = (saved_tokens as f64 / 1_000_000.0) * pricing.input_cost;
    (estimated_savings * 10000.0).round() / 10000.0
}

#[inline]
pub fn calculate_projected_monthly_cost_cents(current_cost: f64, days_elapsed: u32, total_days: u32) -> i64 {
    let projected = calculate_projected_monthly_cost(current_cost, days_elapsed, total_days);
    (projected * 100.0).round() as i64
}

#[inline]
pub fn calculate_projected_monthly_cost(current_cost: f64, days_elapsed: u32, total_days: u32) -> f64 {
    if days_elapsed == 0 || current_cost < 0.0 {
        return 0.0;
    }
    let projected = (current_cost / days_elapsed as f64) * total_days as f64;
    (projected * 10000.0).round() / 10000.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_cost() {
        // Test with a known model
        let cost = calculate_cost("claude-3-opus", 1000000, 1000000, 0);
        assert_eq!(cost, 15.00 + 75.00);

        let cost = calculate_cost("gpt-4o-mini", 1000000, 1000000, 0);
        assert_eq!(cost, 0.15 + 0.60);

        let cost = calculate_cost("o1", 1000000, 1000000, 0);
        assert_eq!(cost, 15.0 + 60.0);

        let cost = calculate_cost("gemini-2.5-flash", 1000000, 1000000, 0);
        assert_eq!(cost, 0.15 + 0.60);

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

        let roi_neg = calculate_roi(-10.0, 100.0);
        assert_eq!(roi_neg, 0.0);

        let efficiency_neg = calculate_efficiency(-10.0, 1000);
        assert_eq!(efficiency_neg, 0.0);
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

        let cost_cents = calculate_cost_with_config_cents(1000, 500, 200, 100, &config);
        assert_eq!(cost_cents, 190);
    }

    #[test]
    fn test_calculate_bandwidth_savings() {
        let config = CostConfig {
            cost_per_network_gb: 0.50,
            ..Default::default()
        };

        let original = 2 * 1024 * 1024 * 1024; // 2GB
        let compressed = 1024 * 1024 * 1024; // 1GB
        let savings = calculate_bandwidth_savings(original, compressed, &config);
        assert_eq!(savings, 0.50);

        let savings_cents = calculate_bandwidth_savings_cents(original, compressed, &config);
        assert_eq!(savings_cents, 50);

        assert_eq!(calculate_bandwidth_savings(-1, 100, &config), 0.0);
        assert_eq!(calculate_bandwidth_savings(100, -1, &config), 0.0);
    }

    #[test]
    fn test_calculate_storage_savings() {
        let config = CostConfig {
            cost_per_gb_month: 0.10,
            ..Default::default()
        };

        let original = 2 * 1024 * 1024 * 1024; // 2GB
        let compressed = 1024 * 1024 * 1024; // 1GB
        let savings = calculate_storage_savings(original, compressed, &config);
        assert_eq!(savings, 0.10);

        let savings_cents = calculate_storage_savings_cents(original, compressed, &config);
        assert_eq!(savings_cents, 10);

        assert_eq!(calculate_storage_savings(-1, 100, &config), 0.0);
        assert_eq!(calculate_storage_savings(100, -1, &config), 0.0);
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

    #[test]
    fn test_calculate_compute_cost() {
        let config = CostConfig {
            cost_per_compute_hour: 2.0,
            ..Default::default()
        };

        assert_eq!(calculate_compute_cost(5.0, &config), 10.0);
        assert_eq!(calculate_compute_cost_cents(5.0, &config), 1000);

        assert_eq!(calculate_compute_cost(-5.0, &config), 0.0);
        assert_eq!(calculate_compute_cost_cents(-5.0, &config), 0);
    }

    #[test]
    fn test_calculate_network_cost() {
        let config = CostConfig {
            cost_per_network_gb: 0.50,
            ..Default::default()
        };

        let bytes: i64 = 10 * 1024 * 1024 * 1024; // 10 GB
        assert_eq!(calculate_network_cost(bytes, &config), 5.0);
        assert_eq!(calculate_network_cost_cents(bytes, &config), 500);

        let bytes_neg: i64 = -10;
        assert_eq!(calculate_network_cost(bytes_neg, &config), 0.0);
        assert_eq!(calculate_network_cost_cents(bytes_neg, &config), 0);

        // Check small bytes
        let small_bytes: i64 = 1024 * 1024; // 1 MB
        let cost = calculate_network_cost(small_bytes, &config);
        assert!(cost > 0.0 && cost < 0.01);
    }

    #[test]
    fn test_calculate_storage_cost() {
        let config = CostConfig {
            cost_per_gb_month: 0.10,
            ..Default::default()
        };
        let bytes: i64 = 10 * 1024 * 1024 * 1024;
        assert_eq!(calculate_storage_cost_cents(bytes, &config), 100);
    }

    #[test]
    fn test_calculate_cost_with_cached_tokens() {
        // gpt-4o: input 5.0, output 15.0, cached 2.50
        // input 1,000,000 tokens
        // cached 1,000,000 tokens
        // output 1,000,000 tokens
        let cost = calculate_cost("gpt-4o", 1000000, 1000000, 1000000);
        assert_eq!(cost, 5.00 + 15.00 + 2.50);

        let cost_cents = calculate_cost_cents("gpt-4o", 1000000, 1000000, 1000000);
        assert_eq!(cost_cents, 2250); // 22.50 * 100
    }

    #[test]
    fn test_calculate_projected_monthly_cost() {
        assert_eq!(calculate_projected_monthly_cost(10.0, 5, 30), 60.0);
        assert_eq!(calculate_projected_monthly_cost(10.0, 0, 30), 0.0);
        assert_eq!(calculate_projected_monthly_cost(10.0, 30, 30), 10.0);
        assert_eq!(calculate_projected_monthly_cost(15.5, 10, 31), 48.05);
        assert_eq!(calculate_projected_monthly_cost(10.0, 5, 0), 0.0);
        assert_eq!(calculate_projected_monthly_cost(-10.0, 5, 30), 0.0);
    }

    #[test]
    fn test_calculate_projected_monthly_cost_cents() {
        assert_eq!(calculate_projected_monthly_cost_cents(10.0, 5, 30), 6000);
        assert_eq!(calculate_projected_monthly_cost_cents(10.0, 0, 30), 0);
        assert_eq!(calculate_projected_monthly_cost_cents(10.0, 30, 30), 1000);
        assert_eq!(calculate_projected_monthly_cost_cents(15.5, 10, 31), 4805);
        assert_eq!(calculate_projected_monthly_cost_cents(10.0, 5, 0), 0);
    }

    #[test]
    fn test_calculate_heuristic_token_efficiency() {
        // gpt-4o input cost is 5.00 per 1M tokens.
        // We truncate from 100,000 to 50,000 tokens, saving 50,000.
        // 50,000 / 1,000,000 * 5.00 = 0.25
        assert_eq!(calculate_heuristic_token_efficiency(100_000, 50_000, "gpt-4o"), 0.25);
        assert_eq!(calculate_heuristic_token_efficiency(10_000, 10_000, "gpt-4o"), 0.0);
        assert_eq!(calculate_heuristic_token_efficiency(10_000, 20_000, "gpt-4o"), 0.0);
        assert_eq!(calculate_heuristic_token_efficiency(-10_000, 0, "gpt-4o"), 0.0);
        assert_eq!(calculate_heuristic_token_efficiency(0, 0, "gpt-4o"), 0.0);
    }
}
// Optimizations handled: Cost savings functionality verified and intact
