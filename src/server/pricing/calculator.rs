/// Represents the per-model pricing configuration for input, output, and cached tokens.
/// This struct is fundamentally responsible for holding the USD cost per 1,000,000 tokens
/// as specified by the various LLM providers (Anthropic, OpenAI, Google, etc.).
///
/// The pricing architecture in OHC is designed to ensure maximum cost predictability
/// for small business owners. To achieve this, the `ModelPricing` struct strictly separates
/// `input_cost`, `output_cost`, and `cached_cost` because prompt caching can drastically
/// reduce costs for recurring queries (e.g., when Maya the baker repeatedly queries the same
/// product catalog context).
///
/// # Fields
/// * `input_cost` - The cost in USD per 1M input tokens.
/// * `output_cost` - The cost in USD per 1M output tokens.
/// * `cached_cost` - The cost in USD per 1M cached input tokens.
pub struct ModelPricing {
    pub input_cost: f64,
    pub output_cost: f64,
    pub cached_cost: f64,
}

/// Retrieves the predefined pricing configuration for a given LLM model identifier.
///
/// This function acts as the central repository for static pricing tiers within the OHC
/// ecosystem. It uses a comprehensive match statement to evaluate the provided model string
/// against known configurations.
///
/// OHC relies on this function to estimate costs before routing agent requests. For example,
/// if a Free Tier tenant attempts to execute a high-cost task, the system can use this function
/// to determine if the task should be routed to a cheaper model (like `claude-3-haiku` instead
/// of `claude-3-opus`) to preserve their budget.
///
/// # Arguments
/// * `model` - A string slice representing the unique model identifier (e.g., "gpt-4o").
///
/// # Returns
/// A `ModelPricing` struct containing the relevant costs.
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

#[cfg(test)]
mod exhaustive_tests {
    use super::*;

    #[test]
    fn test_pricing_retrieval_exact_for_claude_3_opus() {
        let pricing = get_pricing("claude-3-opus");
        assert_eq!(pricing.input_cost, 15.0);
        assert_eq!(pricing.output_cost, 75.0);
        assert_eq!(pricing.cached_cost, 0.0);
    }

    #[test]
    fn test_cost_calculation_1m_tokens_for_claude_3_opus() {
        let cost = calculate_cost("claude-3-opus", 1_000_000, 1_000_000, 1_000_000);
        let expected = (15.0 as f64) + (75.0 as f64) + (0.0 as f64);
        assert!((cost - expected).abs() < 1e-9);
    }

    #[test]
    fn test_cost_calculation_fractional_for_claude_3_opus() {
        let prompt = 150_000;
        let completion = 25_000;
        let cached = 50_000;
        let cost = calculate_cost("claude-3-opus", prompt, completion, cached);

        let expected_input = (prompt as f64) * 15.0 / 1_000_000.0;
        let expected_output = (completion as f64) * 75.0 / 1_000_000.0;
        let expected_cached = (cached as f64) * 0.0 / 1_000_000.0;
        let expected = expected_input + expected_output + expected_cached;

        assert!((cost - expected).abs() < 1e-9);
    }

    #[test]
    fn test_cost_calculation_zero_tokens_for_claude_3_opus() {
        let cost = calculate_cost("claude-3-opus", 0, 0, 0);
        assert_eq!(cost, 0.0);
    }

    #[test]
    fn test_cost_cents_calculation_for_claude_3_opus() {
        // We will calculate for 1M tokens to avoid precision losses
        let cents = calculate_cost_cents("claude-3-opus", 1_000_000, 1_000_000, 1_000_000);
        let expected_dollars = (15.0 as f64) + (75.0 as f64) + (0.0 as f64);
        let expected_cents = (expected_dollars * 100.0).round() as i64;
        assert_eq!(cents, expected_cents);
    }

    #[test]
    fn test_pricing_retrieval_exact_for_claude_3_sonnet() {
        let pricing = get_pricing("claude-3-sonnet");
        assert_eq!(pricing.input_cost, 3.0);
        assert_eq!(pricing.output_cost, 15.0);
        assert_eq!(pricing.cached_cost, 0.0);
    }

    #[test]
    fn test_cost_calculation_1m_tokens_for_claude_3_sonnet() {
        let cost = calculate_cost("claude-3-sonnet", 1_000_000, 1_000_000, 1_000_000);
        let expected = (3.0 as f64) + (15.0 as f64) + (0.0 as f64);
        assert!((cost - expected).abs() < 1e-9);
    }

    #[test]
    fn test_cost_calculation_fractional_for_claude_3_sonnet() {
        let prompt = 150_000;
        let completion = 25_000;
        let cached = 50_000;
        let cost = calculate_cost("claude-3-sonnet", prompt, completion, cached);

        let expected_input = (prompt as f64) * 3.0 / 1_000_000.0;
        let expected_output = (completion as f64) * 15.0 / 1_000_000.0;
        let expected_cached = (cached as f64) * 0.0 / 1_000_000.0;
        let expected = expected_input + expected_output + expected_cached;

        assert!((cost - expected).abs() < 1e-9);
    }

    #[test]
    fn test_cost_calculation_zero_tokens_for_claude_3_sonnet() {
        let cost = calculate_cost("claude-3-sonnet", 0, 0, 0);
        assert_eq!(cost, 0.0);
    }

    #[test]
    fn test_cost_cents_calculation_for_claude_3_sonnet() {
        // We will calculate for 1M tokens to avoid precision losses
        let cents = calculate_cost_cents("claude-3-sonnet", 1_000_000, 1_000_000, 1_000_000);
        let expected_dollars = (3.0 as f64) + (15.0 as f64) + (0.0 as f64);
        let expected_cents = (expected_dollars * 100.0).round() as i64;
        assert_eq!(cents, expected_cents);
    }

    #[test]
    fn test_pricing_retrieval_exact_for_claude_3_haiku() {
        let pricing = get_pricing("claude-3-haiku");
        assert_eq!(pricing.input_cost, 0.25);
        assert_eq!(pricing.output_cost, 1.25);
        assert_eq!(pricing.cached_cost, 0.0);
    }

    #[test]
    fn test_cost_calculation_1m_tokens_for_claude_3_haiku() {
        let cost = calculate_cost("claude-3-haiku", 1_000_000, 1_000_000, 1_000_000);
        let expected = (0.25 as f64) + (1.25 as f64) + (0.0 as f64);
        assert!((cost - expected).abs() < 1e-9);
    }

    #[test]
    fn test_cost_calculation_fractional_for_claude_3_haiku() {
        let prompt = 150_000;
        let completion = 25_000;
        let cached = 50_000;
        let cost = calculate_cost("claude-3-haiku", prompt, completion, cached);

        let expected_input = (prompt as f64) * 0.25 / 1_000_000.0;
        let expected_output = (completion as f64) * 1.25 / 1_000_000.0;
        let expected_cached = (cached as f64) * 0.0 / 1_000_000.0;
        let expected = expected_input + expected_output + expected_cached;

        assert!((cost - expected).abs() < 1e-9);
    }

    #[test]
    fn test_cost_calculation_zero_tokens_for_claude_3_haiku() {
        let cost = calculate_cost("claude-3-haiku", 0, 0, 0);
        assert_eq!(cost, 0.0);
    }

    #[test]
    fn test_cost_cents_calculation_for_claude_3_haiku() {
        // We will calculate for 1M tokens to avoid precision losses
        let cents = calculate_cost_cents("claude-3-haiku", 1_000_000, 1_000_000, 1_000_000);
        let expected_dollars = (0.25 as f64) + (1.25 as f64) + (0.0 as f64);
        let expected_cents = (expected_dollars * 100.0).round() as i64;
        assert_eq!(cents, expected_cents);
    }

    #[test]
    fn test_pricing_retrieval_exact_for_claude_3_5_sonnet() {
        let pricing = get_pricing("claude-3.5-sonnet");
        assert_eq!(pricing.input_cost, 3.0);
        assert_eq!(pricing.output_cost, 15.0);
        assert_eq!(pricing.cached_cost, 0.3);
    }

    #[test]
    fn test_cost_calculation_1m_tokens_for_claude_3_5_sonnet() {
        let cost = calculate_cost("claude-3.5-sonnet", 1_000_000, 1_000_000, 1_000_000);
        let expected = (3.0 as f64) + (15.0 as f64) + (0.3 as f64);
        assert!((cost - expected).abs() < 1e-9);
    }

    #[test]
    fn test_cost_calculation_fractional_for_claude_3_5_sonnet() {
        let prompt = 150_000;
        let completion = 25_000;
        let cached = 50_000;
        let cost = calculate_cost("claude-3.5-sonnet", prompt, completion, cached);

        let expected_input = (prompt as f64) * 3.0 / 1_000_000.0;
        let expected_output = (completion as f64) * 15.0 / 1_000_000.0;
        let expected_cached = (cached as f64) * 0.3 / 1_000_000.0;
        let expected = expected_input + expected_output + expected_cached;

        assert!((cost - expected).abs() < 1e-9);
    }

    #[test]
    fn test_cost_calculation_zero_tokens_for_claude_3_5_sonnet() {
        let cost = calculate_cost("claude-3.5-sonnet", 0, 0, 0);
        assert_eq!(cost, 0.0);
    }

    #[test]
    fn test_cost_cents_calculation_for_claude_3_5_sonnet() {
        // We will calculate for 1M tokens to avoid precision losses
        let cents = calculate_cost_cents("claude-3.5-sonnet", 1_000_000, 1_000_000, 1_000_000);
        let expected_dollars = (3.0 as f64) + (15.0 as f64) + (0.3 as f64);
        let expected_cents = (expected_dollars * 100.0).round() as i64;
        assert_eq!(cents, expected_cents);
    }

    #[test]
    fn test_pricing_retrieval_exact_for_claude_3_5_sonnet_20241022() {
        let pricing = get_pricing("claude-3-5-sonnet-20241022");
        assert_eq!(pricing.input_cost, 3.0);
        assert_eq!(pricing.output_cost, 15.0);
        assert_eq!(pricing.cached_cost, 0.3);
    }

    #[test]
    fn test_cost_calculation_1m_tokens_for_claude_3_5_sonnet_20241022() {
        let cost = calculate_cost("claude-3-5-sonnet-20241022", 1_000_000, 1_000_000, 1_000_000);
        let expected = (3.0 as f64) + (15.0 as f64) + (0.3 as f64);
        assert!((cost - expected).abs() < 1e-9);
    }

    #[test]
    fn test_cost_calculation_fractional_for_claude_3_5_sonnet_20241022() {
        let prompt = 150_000;
        let completion = 25_000;
        let cached = 50_000;
        let cost = calculate_cost("claude-3-5-sonnet-20241022", prompt, completion, cached);

        let expected_input = (prompt as f64) * 3.0 / 1_000_000.0;
        let expected_output = (completion as f64) * 15.0 / 1_000_000.0;
        let expected_cached = (cached as f64) * 0.3 / 1_000_000.0;
        let expected = expected_input + expected_output + expected_cached;

        assert!((cost - expected).abs() < 1e-9);
    }

    #[test]
    fn test_cost_calculation_zero_tokens_for_claude_3_5_sonnet_20241022() {
        let cost = calculate_cost("claude-3-5-sonnet-20241022", 0, 0, 0);
        assert_eq!(cost, 0.0);
    }

    #[test]
    fn test_cost_cents_calculation_for_claude_3_5_sonnet_20241022() {
        // We will calculate for 1M tokens to avoid precision losses
        let cents = calculate_cost_cents("claude-3-5-sonnet-20241022", 1_000_000, 1_000_000, 1_000_000);
        let expected_dollars = (3.0 as f64) + (15.0 as f64) + (0.3 as f64);
        let expected_cents = (expected_dollars * 100.0).round() as i64;
        assert_eq!(cents, expected_cents);
    }

    #[test]
    fn test_pricing_retrieval_exact_for_claude_3_5_sonnet_20240620() {
        let pricing = get_pricing("claude-3-5-sonnet-20240620");
        assert_eq!(pricing.input_cost, 3.0);
        assert_eq!(pricing.output_cost, 15.0);
        assert_eq!(pricing.cached_cost, 0.3);
    }

    #[test]
    fn test_cost_calculation_1m_tokens_for_claude_3_5_sonnet_20240620() {
        let cost = calculate_cost("claude-3-5-sonnet-20240620", 1_000_000, 1_000_000, 1_000_000);
        let expected = (3.0 as f64) + (15.0 as f64) + (0.3 as f64);
        assert!((cost - expected).abs() < 1e-9);
    }

    #[test]
    fn test_cost_calculation_fractional_for_claude_3_5_sonnet_20240620() {
        let prompt = 150_000;
        let completion = 25_000;
        let cached = 50_000;
        let cost = calculate_cost("claude-3-5-sonnet-20240620", prompt, completion, cached);

        let expected_input = (prompt as f64) * 3.0 / 1_000_000.0;
        let expected_output = (completion as f64) * 15.0 / 1_000_000.0;
        let expected_cached = (cached as f64) * 0.3 / 1_000_000.0;
        let expected = expected_input + expected_output + expected_cached;

        assert!((cost - expected).abs() < 1e-9);
    }

    #[test]
    fn test_cost_calculation_zero_tokens_for_claude_3_5_sonnet_20240620() {
        let cost = calculate_cost("claude-3-5-sonnet-20240620", 0, 0, 0);
        assert_eq!(cost, 0.0);
    }

    #[test]
    fn test_cost_cents_calculation_for_claude_3_5_sonnet_20240620() {
        // We will calculate for 1M tokens to avoid precision losses
        let cents = calculate_cost_cents("claude-3-5-sonnet-20240620", 1_000_000, 1_000_000, 1_000_000);
        let expected_dollars = (3.0 as f64) + (15.0 as f64) + (0.3 as f64);
        let expected_cents = (expected_dollars * 100.0).round() as i64;
        assert_eq!(cents, expected_cents);
    }

    #[test]
    fn test_pricing_retrieval_exact_for_claude_3_5_haiku() {
        let pricing = get_pricing("claude-3.5-haiku");
        assert_eq!(pricing.input_cost, 0.8);
        assert_eq!(pricing.output_cost, 4.0);
        assert_eq!(pricing.cached_cost, 0.08);
    }

    #[test]
    fn test_cost_calculation_1m_tokens_for_claude_3_5_haiku() {
        let cost = calculate_cost("claude-3.5-haiku", 1_000_000, 1_000_000, 1_000_000);
        let expected = (0.8 as f64) + (4.0 as f64) + (0.08 as f64);
        assert!((cost - expected).abs() < 1e-9);
    }

    #[test]
    fn test_cost_calculation_fractional_for_claude_3_5_haiku() {
        let prompt = 150_000;
        let completion = 25_000;
        let cached = 50_000;
        let cost = calculate_cost("claude-3.5-haiku", prompt, completion, cached);

        let expected_input = (prompt as f64) * 0.8 / 1_000_000.0;
        let expected_output = (completion as f64) * 4.0 / 1_000_000.0;
        let expected_cached = (cached as f64) * 0.08 / 1_000_000.0;
        let expected = expected_input + expected_output + expected_cached;

        assert!((cost - expected).abs() < 1e-9);
    }

    #[test]
    fn test_cost_calculation_zero_tokens_for_claude_3_5_haiku() {
        let cost = calculate_cost("claude-3.5-haiku", 0, 0, 0);
        assert_eq!(cost, 0.0);
    }

    #[test]
    fn test_cost_cents_calculation_for_claude_3_5_haiku() {
        // We will calculate for 1M tokens to avoid precision losses
        let cents = calculate_cost_cents("claude-3.5-haiku", 1_000_000, 1_000_000, 1_000_000);
        let expected_dollars = (0.8 as f64) + (4.0 as f64) + (0.08 as f64);
        let expected_cents = (expected_dollars * 100.0).round() as i64;
        assert_eq!(cents, expected_cents);
    }

    #[test]
    fn test_pricing_retrieval_exact_for_claude_3_7_sonnet() {
        let pricing = get_pricing("claude-3.7-sonnet");
        assert_eq!(pricing.input_cost, 3.0);
        assert_eq!(pricing.output_cost, 15.0);
        assert_eq!(pricing.cached_cost, 0.3);
    }

    #[test]
    fn test_cost_calculation_1m_tokens_for_claude_3_7_sonnet() {
        let cost = calculate_cost("claude-3.7-sonnet", 1_000_000, 1_000_000, 1_000_000);
        let expected = (3.0 as f64) + (15.0 as f64) + (0.3 as f64);
        assert!((cost - expected).abs() < 1e-9);
    }

    #[test]
    fn test_cost_calculation_fractional_for_claude_3_7_sonnet() {
        let prompt = 150_000;
        let completion = 25_000;
        let cached = 50_000;
        let cost = calculate_cost("claude-3.7-sonnet", prompt, completion, cached);

        let expected_input = (prompt as f64) * 3.0 / 1_000_000.0;
        let expected_output = (completion as f64) * 15.0 / 1_000_000.0;
        let expected_cached = (cached as f64) * 0.3 / 1_000_000.0;
        let expected = expected_input + expected_output + expected_cached;

        assert!((cost - expected).abs() < 1e-9);
    }

    #[test]
    fn test_cost_calculation_zero_tokens_for_claude_3_7_sonnet() {
        let cost = calculate_cost("claude-3.7-sonnet", 0, 0, 0);
        assert_eq!(cost, 0.0);
    }

    #[test]
    fn test_cost_cents_calculation_for_claude_3_7_sonnet() {
        // We will calculate for 1M tokens to avoid precision losses
        let cents = calculate_cost_cents("claude-3.7-sonnet", 1_000_000, 1_000_000, 1_000_000);
        let expected_dollars = (3.0 as f64) + (15.0 as f64) + (0.3 as f64);
        let expected_cents = (expected_dollars * 100.0).round() as i64;
        assert_eq!(cents, expected_cents);
    }

    #[test]
    fn test_pricing_retrieval_exact_for_gpt_4() {
        let pricing = get_pricing("gpt-4");
        assert_eq!(pricing.input_cost, 30.0);
        assert_eq!(pricing.output_cost, 60.0);
        assert_eq!(pricing.cached_cost, 0.0);
    }

    #[test]
    fn test_cost_calculation_1m_tokens_for_gpt_4() {
        let cost = calculate_cost("gpt-4", 1_000_000, 1_000_000, 1_000_000);
        let expected = (30.0 as f64) + (60.0 as f64) + (0.0 as f64);
        assert!((cost - expected).abs() < 1e-9);
    }

    #[test]
    fn test_cost_calculation_fractional_for_gpt_4() {
        let prompt = 150_000;
        let completion = 25_000;
        let cached = 50_000;
        let cost = calculate_cost("gpt-4", prompt, completion, cached);

        let expected_input = (prompt as f64) * 30.0 / 1_000_000.0;
        let expected_output = (completion as f64) * 60.0 / 1_000_000.0;
        let expected_cached = (cached as f64) * 0.0 / 1_000_000.0;
        let expected = expected_input + expected_output + expected_cached;

        assert!((cost - expected).abs() < 1e-9);
    }

    #[test]
    fn test_cost_calculation_zero_tokens_for_gpt_4() {
        let cost = calculate_cost("gpt-4", 0, 0, 0);
        assert_eq!(cost, 0.0);
    }

    #[test]
    fn test_cost_cents_calculation_for_gpt_4() {
        // We will calculate for 1M tokens to avoid precision losses
        let cents = calculate_cost_cents("gpt-4", 1_000_000, 1_000_000, 1_000_000);
        let expected_dollars = (30.0 as f64) + (60.0 as f64) + (0.0 as f64);
        let expected_cents = (expected_dollars * 100.0).round() as i64;
        assert_eq!(cents, expected_cents);
    }

    #[test]
    fn test_pricing_retrieval_exact_for_gpt_4_turbo() {
        let pricing = get_pricing("gpt-4-turbo");
        assert_eq!(pricing.input_cost, 10.0);
        assert_eq!(pricing.output_cost, 30.0);
        assert_eq!(pricing.cached_cost, 0.0);
    }

    #[test]
    fn test_cost_calculation_1m_tokens_for_gpt_4_turbo() {
        let cost = calculate_cost("gpt-4-turbo", 1_000_000, 1_000_000, 1_000_000);
        let expected = (10.0 as f64) + (30.0 as f64) + (0.0 as f64);
        assert!((cost - expected).abs() < 1e-9);
    }

    #[test]
    fn test_cost_calculation_fractional_for_gpt_4_turbo() {
        let prompt = 150_000;
        let completion = 25_000;
        let cached = 50_000;
        let cost = calculate_cost("gpt-4-turbo", prompt, completion, cached);

        let expected_input = (prompt as f64) * 10.0 / 1_000_000.0;
        let expected_output = (completion as f64) * 30.0 / 1_000_000.0;
        let expected_cached = (cached as f64) * 0.0 / 1_000_000.0;
        let expected = expected_input + expected_output + expected_cached;

        assert!((cost - expected).abs() < 1e-9);
    }

    #[test]
    fn test_cost_calculation_zero_tokens_for_gpt_4_turbo() {
        let cost = calculate_cost("gpt-4-turbo", 0, 0, 0);
        assert_eq!(cost, 0.0);
    }

    #[test]
    fn test_cost_cents_calculation_for_gpt_4_turbo() {
        // We will calculate for 1M tokens to avoid precision losses
        let cents = calculate_cost_cents("gpt-4-turbo", 1_000_000, 1_000_000, 1_000_000);
        let expected_dollars = (10.0 as f64) + (30.0 as f64) + (0.0 as f64);
        let expected_cents = (expected_dollars * 100.0).round() as i64;
        assert_eq!(cents, expected_cents);
    }

    #[test]
    fn test_pricing_retrieval_exact_for_gpt_4o() {
        let pricing = get_pricing("gpt-4o");
        assert_eq!(pricing.input_cost, 5.0);
        assert_eq!(pricing.output_cost, 15.0);
        assert_eq!(pricing.cached_cost, 2.5);
    }

    #[test]
    fn test_cost_calculation_1m_tokens_for_gpt_4o() {
        let cost = calculate_cost("gpt-4o", 1_000_000, 1_000_000, 1_000_000);
        let expected = (5.0 as f64) + (15.0 as f64) + (2.5 as f64);
        assert!((cost - expected).abs() < 1e-9);
    }

    #[test]
    fn test_cost_calculation_fractional_for_gpt_4o() {
        let prompt = 150_000;
        let completion = 25_000;
        let cached = 50_000;
        let cost = calculate_cost("gpt-4o", prompt, completion, cached);

        let expected_input = (prompt as f64) * 5.0 / 1_000_000.0;
        let expected_output = (completion as f64) * 15.0 / 1_000_000.0;
        let expected_cached = (cached as f64) * 2.5 / 1_000_000.0;
        let expected = expected_input + expected_output + expected_cached;

        assert!((cost - expected).abs() < 1e-9);
    }

    #[test]
    fn test_cost_calculation_zero_tokens_for_gpt_4o() {
        let cost = calculate_cost("gpt-4o", 0, 0, 0);
        assert_eq!(cost, 0.0);
    }

    #[test]
    fn test_cost_cents_calculation_for_gpt_4o() {
        // We will calculate for 1M tokens to avoid precision losses
        let cents = calculate_cost_cents("gpt-4o", 1_000_000, 1_000_000, 1_000_000);
        let expected_dollars = (5.0 as f64) + (15.0 as f64) + (2.5 as f64);
        let expected_cents = (expected_dollars * 100.0).round() as i64;
        assert_eq!(cents, expected_cents);
    }

    #[test]
    fn test_pricing_retrieval_exact_for_gpt_4o_mini() {
        let pricing = get_pricing("gpt-4o-mini");
        assert_eq!(pricing.input_cost, 0.15);
        assert_eq!(pricing.output_cost, 0.6);
        assert_eq!(pricing.cached_cost, 0.075);
    }

    #[test]
    fn test_cost_calculation_1m_tokens_for_gpt_4o_mini() {
        let cost = calculate_cost("gpt-4o-mini", 1_000_000, 1_000_000, 1_000_000);
        let expected = (0.15 as f64) + (0.6 as f64) + (0.075 as f64);
        assert!((cost - expected).abs() < 1e-9);
    }

    #[test]
    fn test_cost_calculation_fractional_for_gpt_4o_mini() {
        let prompt = 150_000;
        let completion = 25_000;
        let cached = 50_000;
        let cost = calculate_cost("gpt-4o-mini", prompt, completion, cached);

        let expected_input = (prompt as f64) * 0.15 / 1_000_000.0;
        let expected_output = (completion as f64) * 0.6 / 1_000_000.0;
        let expected_cached = (cached as f64) * 0.075 / 1_000_000.0;
        let expected = expected_input + expected_output + expected_cached;

        assert!((cost - expected).abs() < 1e-9);
    }

    #[test]
    fn test_cost_calculation_zero_tokens_for_gpt_4o_mini() {
        let cost = calculate_cost("gpt-4o-mini", 0, 0, 0);
        assert_eq!(cost, 0.0);
    }

    #[test]
    fn test_cost_cents_calculation_for_gpt_4o_mini() {
        // We will calculate for 1M tokens to avoid precision losses
        let cents = calculate_cost_cents("gpt-4o-mini", 1_000_000, 1_000_000, 1_000_000);
        let expected_dollars = (0.15 as f64) + (0.6 as f64) + (0.075 as f64);
        let expected_cents = (expected_dollars * 100.0).round() as i64;
        assert_eq!(cents, expected_cents);
    }

    #[test]
    fn test_pricing_retrieval_exact_for_gpt_4_1() {
        let pricing = get_pricing("gpt-4.1");
        assert_eq!(pricing.input_cost, 2.0);
        assert_eq!(pricing.output_cost, 8.0);
        assert_eq!(pricing.cached_cost, 0.0);
    }

    #[test]
    fn test_cost_calculation_1m_tokens_for_gpt_4_1() {
        let cost = calculate_cost("gpt-4.1", 1_000_000, 1_000_000, 1_000_000);
        let expected = (2.0 as f64) + (8.0 as f64) + (0.0 as f64);
        assert!((cost - expected).abs() < 1e-9);
    }

    #[test]
    fn test_cost_calculation_fractional_for_gpt_4_1() {
        let prompt = 150_000;
        let completion = 25_000;
        let cached = 50_000;
        let cost = calculate_cost("gpt-4.1", prompt, completion, cached);

        let expected_input = (prompt as f64) * 2.0 / 1_000_000.0;
        let expected_output = (completion as f64) * 8.0 / 1_000_000.0;
        let expected_cached = (cached as f64) * 0.0 / 1_000_000.0;
        let expected = expected_input + expected_output + expected_cached;

        assert!((cost - expected).abs() < 1e-9);
    }

    #[test]
    fn test_cost_calculation_zero_tokens_for_gpt_4_1() {
        let cost = calculate_cost("gpt-4.1", 0, 0, 0);
        assert_eq!(cost, 0.0);
    }

    #[test]
    fn test_cost_cents_calculation_for_gpt_4_1() {
        // We will calculate for 1M tokens to avoid precision losses
        let cents = calculate_cost_cents("gpt-4.1", 1_000_000, 1_000_000, 1_000_000);
        let expected_dollars = (2.0 as f64) + (8.0 as f64) + (0.0 as f64);
        let expected_cents = (expected_dollars * 100.0).round() as i64;
        assert_eq!(cents, expected_cents);
    }

    #[test]
    fn test_pricing_retrieval_exact_for_gpt_4_1_mini() {
        let pricing = get_pricing("gpt-4.1-mini");
        assert_eq!(pricing.input_cost, 0.4);
        assert_eq!(pricing.output_cost, 1.6);
        assert_eq!(pricing.cached_cost, 0.0);
    }

    #[test]
    fn test_cost_calculation_1m_tokens_for_gpt_4_1_mini() {
        let cost = calculate_cost("gpt-4.1-mini", 1_000_000, 1_000_000, 1_000_000);
        let expected = (0.4 as f64) + (1.6 as f64) + (0.0 as f64);
        assert!((cost - expected).abs() < 1e-9);
    }

    #[test]
    fn test_cost_calculation_fractional_for_gpt_4_1_mini() {
        let prompt = 150_000;
        let completion = 25_000;
        let cached = 50_000;
        let cost = calculate_cost("gpt-4.1-mini", prompt, completion, cached);

        let expected_input = (prompt as f64) * 0.4 / 1_000_000.0;
        let expected_output = (completion as f64) * 1.6 / 1_000_000.0;
        let expected_cached = (cached as f64) * 0.0 / 1_000_000.0;
        let expected = expected_input + expected_output + expected_cached;

        assert!((cost - expected).abs() < 1e-9);
    }

    #[test]
    fn test_cost_calculation_zero_tokens_for_gpt_4_1_mini() {
        let cost = calculate_cost("gpt-4.1-mini", 0, 0, 0);
        assert_eq!(cost, 0.0);
    }

    #[test]
    fn test_cost_cents_calculation_for_gpt_4_1_mini() {
        // We will calculate for 1M tokens to avoid precision losses
        let cents = calculate_cost_cents("gpt-4.1-mini", 1_000_000, 1_000_000, 1_000_000);
        let expected_dollars = (0.4 as f64) + (1.6 as f64) + (0.0 as f64);
        let expected_cents = (expected_dollars * 100.0).round() as i64;
        assert_eq!(cents, expected_cents);
    }

    #[test]
    fn test_pricing_retrieval_exact_for_gpt_4_1_nano() {
        let pricing = get_pricing("gpt-4.1-nano");
        assert_eq!(pricing.input_cost, 0.1);
        assert_eq!(pricing.output_cost, 0.4);
        assert_eq!(pricing.cached_cost, 0.0);
    }

    #[test]
    fn test_cost_calculation_1m_tokens_for_gpt_4_1_nano() {
        let cost = calculate_cost("gpt-4.1-nano", 1_000_000, 1_000_000, 1_000_000);
        let expected = (0.1 as f64) + (0.4 as f64) + (0.0 as f64);
        assert!((cost - expected).abs() < 1e-9);
    }

    #[test]
    fn test_cost_calculation_fractional_for_gpt_4_1_nano() {
        let prompt = 150_000;
        let completion = 25_000;
        let cached = 50_000;
        let cost = calculate_cost("gpt-4.1-nano", prompt, completion, cached);

        let expected_input = (prompt as f64) * 0.1 / 1_000_000.0;
        let expected_output = (completion as f64) * 0.4 / 1_000_000.0;
        let expected_cached = (cached as f64) * 0.0 / 1_000_000.0;
        let expected = expected_input + expected_output + expected_cached;

        assert!((cost - expected).abs() < 1e-9);
    }

    #[test]
    fn test_cost_calculation_zero_tokens_for_gpt_4_1_nano() {
        let cost = calculate_cost("gpt-4.1-nano", 0, 0, 0);
        assert_eq!(cost, 0.0);
    }

    #[test]
    fn test_cost_cents_calculation_for_gpt_4_1_nano() {
        // We will calculate for 1M tokens to avoid precision losses
        let cents = calculate_cost_cents("gpt-4.1-nano", 1_000_000, 1_000_000, 1_000_000);
        let expected_dollars = (0.1 as f64) + (0.4 as f64) + (0.0 as f64);
        let expected_cents = (expected_dollars * 100.0).round() as i64;
        assert_eq!(cents, expected_cents);
    }

    #[test]
    fn test_pricing_retrieval_exact_for_o1() {
        let pricing = get_pricing("o1");
        assert_eq!(pricing.input_cost, 15.0);
        assert_eq!(pricing.output_cost, 60.0);
        assert_eq!(pricing.cached_cost, 0.0);
    }

    #[test]
    fn test_cost_calculation_1m_tokens_for_o1() {
        let cost = calculate_cost("o1", 1_000_000, 1_000_000, 1_000_000);
        let expected = (15.0 as f64) + (60.0 as f64) + (0.0 as f64);
        assert!((cost - expected).abs() < 1e-9);
    }

    #[test]
    fn test_cost_calculation_fractional_for_o1() {
        let prompt = 150_000;
        let completion = 25_000;
        let cached = 50_000;
        let cost = calculate_cost("o1", prompt, completion, cached);

        let expected_input = (prompt as f64) * 15.0 / 1_000_000.0;
        let expected_output = (completion as f64) * 60.0 / 1_000_000.0;
        let expected_cached = (cached as f64) * 0.0 / 1_000_000.0;
        let expected = expected_input + expected_output + expected_cached;

        assert!((cost - expected).abs() < 1e-9);
    }

    #[test]
    fn test_cost_calculation_zero_tokens_for_o1() {
        let cost = calculate_cost("o1", 0, 0, 0);
        assert_eq!(cost, 0.0);
    }

    #[test]
    fn test_cost_cents_calculation_for_o1() {
        // We will calculate for 1M tokens to avoid precision losses
        let cents = calculate_cost_cents("o1", 1_000_000, 1_000_000, 1_000_000);
        let expected_dollars = (15.0 as f64) + (60.0 as f64) + (0.0 as f64);
        let expected_cents = (expected_dollars * 100.0).round() as i64;
        assert_eq!(cents, expected_cents);
    }

    #[test]
    fn test_pricing_retrieval_exact_for_o1_mini() {
        let pricing = get_pricing("o1-mini");
        assert_eq!(pricing.input_cost, 3.0);
        assert_eq!(pricing.output_cost, 12.0);
        assert_eq!(pricing.cached_cost, 0.0);
    }

    #[test]
    fn test_cost_calculation_1m_tokens_for_o1_mini() {
        let cost = calculate_cost("o1-mini", 1_000_000, 1_000_000, 1_000_000);
        let expected = (3.0 as f64) + (12.0 as f64) + (0.0 as f64);
        assert!((cost - expected).abs() < 1e-9);
    }

    #[test]
    fn test_cost_calculation_fractional_for_o1_mini() {
        let prompt = 150_000;
        let completion = 25_000;
        let cached = 50_000;
        let cost = calculate_cost("o1-mini", prompt, completion, cached);

        let expected_input = (prompt as f64) * 3.0 / 1_000_000.0;
        let expected_output = (completion as f64) * 12.0 / 1_000_000.0;
        let expected_cached = (cached as f64) * 0.0 / 1_000_000.0;
        let expected = expected_input + expected_output + expected_cached;

        assert!((cost - expected).abs() < 1e-9);
    }

    #[test]
    fn test_cost_calculation_zero_tokens_for_o1_mini() {
        let cost = calculate_cost("o1-mini", 0, 0, 0);
        assert_eq!(cost, 0.0);
    }

    #[test]
    fn test_cost_cents_calculation_for_o1_mini() {
        // We will calculate for 1M tokens to avoid precision losses
        let cents = calculate_cost_cents("o1-mini", 1_000_000, 1_000_000, 1_000_000);
        let expected_dollars = (3.0 as f64) + (12.0 as f64) + (0.0 as f64);
        let expected_cents = (expected_dollars * 100.0).round() as i64;
        assert_eq!(cents, expected_cents);
    }

    #[test]
    fn test_pricing_retrieval_exact_for_o3_mini() {
        let pricing = get_pricing("o3-mini");
        assert_eq!(pricing.input_cost, 1.1);
        assert_eq!(pricing.output_cost, 4.4);
        assert_eq!(pricing.cached_cost, 0.0);
    }

    #[test]
    fn test_cost_calculation_1m_tokens_for_o3_mini() {
        let cost = calculate_cost("o3-mini", 1_000_000, 1_000_000, 1_000_000);
        let expected = (1.1 as f64) + (4.4 as f64) + (0.0 as f64);
        assert!((cost - expected).abs() < 1e-9);
    }

    #[test]
    fn test_cost_calculation_fractional_for_o3_mini() {
        let prompt = 150_000;
        let completion = 25_000;
        let cached = 50_000;
        let cost = calculate_cost("o3-mini", prompt, completion, cached);

        let expected_input = (prompt as f64) * 1.1 / 1_000_000.0;
        let expected_output = (completion as f64) * 4.4 / 1_000_000.0;
        let expected_cached = (cached as f64) * 0.0 / 1_000_000.0;
        let expected = expected_input + expected_output + expected_cached;

        assert!((cost - expected).abs() < 1e-9);
    }

    #[test]
    fn test_cost_calculation_zero_tokens_for_o3_mini() {
        let cost = calculate_cost("o3-mini", 0, 0, 0);
        assert_eq!(cost, 0.0);
    }

    #[test]
    fn test_cost_cents_calculation_for_o3_mini() {
        // We will calculate for 1M tokens to avoid precision losses
        let cents = calculate_cost_cents("o3-mini", 1_000_000, 1_000_000, 1_000_000);
        let expected_dollars = (1.1 as f64) + (4.4 as f64) + (0.0 as f64);
        let expected_cents = (expected_dollars * 100.0).round() as i64;
        assert_eq!(cents, expected_cents);
    }

    #[test]
    fn test_pricing_retrieval_exact_for_gemini_1_5_pro() {
        let pricing = get_pricing("gemini-1.5-pro");
        assert_eq!(pricing.input_cost, 3.5);
        assert_eq!(pricing.output_cost, 10.5);
        assert_eq!(pricing.cached_cost, 0.0);
    }

    #[test]
    fn test_cost_calculation_1m_tokens_for_gemini_1_5_pro() {
        let cost = calculate_cost("gemini-1.5-pro", 1_000_000, 1_000_000, 1_000_000);
        let expected = (3.5 as f64) + (10.5 as f64) + (0.0 as f64);
        assert!((cost - expected).abs() < 1e-9);
    }

    #[test]
    fn test_cost_calculation_fractional_for_gemini_1_5_pro() {
        let prompt = 150_000;
        let completion = 25_000;
        let cached = 50_000;
        let cost = calculate_cost("gemini-1.5-pro", prompt, completion, cached);

        let expected_input = (prompt as f64) * 3.5 / 1_000_000.0;
        let expected_output = (completion as f64) * 10.5 / 1_000_000.0;
        let expected_cached = (cached as f64) * 0.0 / 1_000_000.0;
        let expected = expected_input + expected_output + expected_cached;

        assert!((cost - expected).abs() < 1e-9);
    }

    #[test]
    fn test_cost_calculation_zero_tokens_for_gemini_1_5_pro() {
        let cost = calculate_cost("gemini-1.5-pro", 0, 0, 0);
        assert_eq!(cost, 0.0);
    }

    #[test]
    fn test_cost_cents_calculation_for_gemini_1_5_pro() {
        // We will calculate for 1M tokens to avoid precision losses
        let cents = calculate_cost_cents("gemini-1.5-pro", 1_000_000, 1_000_000, 1_000_000);
        let expected_dollars = (3.5 as f64) + (10.5 as f64) + (0.0 as f64);
        let expected_cents = (expected_dollars * 100.0).round() as i64;
        assert_eq!(cents, expected_cents);
    }

    #[test]
    fn test_pricing_retrieval_exact_for_gemini_1_5_flash() {
        let pricing = get_pricing("gemini-1.5-flash");
        assert_eq!(pricing.input_cost, 0.35);
        assert_eq!(pricing.output_cost, 1.05);
        assert_eq!(pricing.cached_cost, 0.0);
    }

    #[test]
    fn test_cost_calculation_1m_tokens_for_gemini_1_5_flash() {
        let cost = calculate_cost("gemini-1.5-flash", 1_000_000, 1_000_000, 1_000_000);
        let expected = (0.35 as f64) + (1.05 as f64) + (0.0 as f64);
        assert!((cost - expected).abs() < 1e-9);
    }

    #[test]
    fn test_cost_calculation_fractional_for_gemini_1_5_flash() {
        let prompt = 150_000;
        let completion = 25_000;
        let cached = 50_000;
        let cost = calculate_cost("gemini-1.5-flash", prompt, completion, cached);

        let expected_input = (prompt as f64) * 0.35 / 1_000_000.0;
        let expected_output = (completion as f64) * 1.05 / 1_000_000.0;
        let expected_cached = (cached as f64) * 0.0 / 1_000_000.0;
        let expected = expected_input + expected_output + expected_cached;

        assert!((cost - expected).abs() < 1e-9);
    }

    #[test]
    fn test_cost_calculation_zero_tokens_for_gemini_1_5_flash() {
        let cost = calculate_cost("gemini-1.5-flash", 0, 0, 0);
        assert_eq!(cost, 0.0);
    }

    #[test]
    fn test_cost_cents_calculation_for_gemini_1_5_flash() {
        // We will calculate for 1M tokens to avoid precision losses
        let cents = calculate_cost_cents("gemini-1.5-flash", 1_000_000, 1_000_000, 1_000_000);
        let expected_dollars = (0.35 as f64) + (1.05 as f64) + (0.0 as f64);
        let expected_cents = (expected_dollars * 100.0).round() as i64;
        assert_eq!(cents, expected_cents);
    }

    #[test]
    fn test_pricing_retrieval_exact_for_gemini_2_0_flash() {
        let pricing = get_pricing("gemini-2.0-flash");
        assert_eq!(pricing.input_cost, 0.1);
        assert_eq!(pricing.output_cost, 0.4);
        assert_eq!(pricing.cached_cost, 0.0);
    }

    #[test]
    fn test_cost_calculation_1m_tokens_for_gemini_2_0_flash() {
        let cost = calculate_cost("gemini-2.0-flash", 1_000_000, 1_000_000, 1_000_000);
        let expected = (0.1 as f64) + (0.4 as f64) + (0.0 as f64);
        assert!((cost - expected).abs() < 1e-9);
    }

    #[test]
    fn test_cost_calculation_fractional_for_gemini_2_0_flash() {
        let prompt = 150_000;
        let completion = 25_000;
        let cached = 50_000;
        let cost = calculate_cost("gemini-2.0-flash", prompt, completion, cached);

        let expected_input = (prompt as f64) * 0.1 / 1_000_000.0;
        let expected_output = (completion as f64) * 0.4 / 1_000_000.0;
        let expected_cached = (cached as f64) * 0.0 / 1_000_000.0;
        let expected = expected_input + expected_output + expected_cached;

        assert!((cost - expected).abs() < 1e-9);
    }

    #[test]
    fn test_cost_calculation_zero_tokens_for_gemini_2_0_flash() {
        let cost = calculate_cost("gemini-2.0-flash", 0, 0, 0);
        assert_eq!(cost, 0.0);
    }

    #[test]
    fn test_cost_cents_calculation_for_gemini_2_0_flash() {
        // We will calculate for 1M tokens to avoid precision losses
        let cents = calculate_cost_cents("gemini-2.0-flash", 1_000_000, 1_000_000, 1_000_000);
        let expected_dollars = (0.1 as f64) + (0.4 as f64) + (0.0 as f64);
        let expected_cents = (expected_dollars * 100.0).round() as i64;
        assert_eq!(cents, expected_cents);
    }

    #[test]
    fn test_pricing_retrieval_exact_for_gemini_2_0_flash_lite() {
        let pricing = get_pricing("gemini-2.0-flash-lite");
        assert_eq!(pricing.input_cost, 0.075);
        assert_eq!(pricing.output_cost, 0.3);
        assert_eq!(pricing.cached_cost, 0.0);
    }

    #[test]
    fn test_cost_calculation_1m_tokens_for_gemini_2_0_flash_lite() {
        let cost = calculate_cost("gemini-2.0-flash-lite", 1_000_000, 1_000_000, 1_000_000);
        let expected = (0.075 as f64) + (0.3 as f64) + (0.0 as f64);
        assert!((cost - expected).abs() < 1e-9);
    }

    #[test]
    fn test_cost_calculation_fractional_for_gemini_2_0_flash_lite() {
        let prompt = 150_000;
        let completion = 25_000;
        let cached = 50_000;
        let cost = calculate_cost("gemini-2.0-flash-lite", prompt, completion, cached);

        let expected_input = (prompt as f64) * 0.075 / 1_000_000.0;
        let expected_output = (completion as f64) * 0.3 / 1_000_000.0;
        let expected_cached = (cached as f64) * 0.0 / 1_000_000.0;
        let expected = expected_input + expected_output + expected_cached;

        assert!((cost - expected).abs() < 1e-9);
    }

    #[test]
    fn test_cost_calculation_zero_tokens_for_gemini_2_0_flash_lite() {
        let cost = calculate_cost("gemini-2.0-flash-lite", 0, 0, 0);
        assert_eq!(cost, 0.0);
    }

    #[test]
    fn test_cost_cents_calculation_for_gemini_2_0_flash_lite() {
        // We will calculate for 1M tokens to avoid precision losses
        let cents = calculate_cost_cents("gemini-2.0-flash-lite", 1_000_000, 1_000_000, 1_000_000);
        let expected_dollars = (0.075 as f64) + (0.3 as f64) + (0.0 as f64);
        let expected_cents = (expected_dollars * 100.0).round() as i64;
        assert_eq!(cents, expected_cents);
    }

    #[test]
    fn test_pricing_retrieval_exact_for_gemini_2_5_pro() {
        let pricing = get_pricing("gemini-2.5-pro");
        assert_eq!(pricing.input_cost, 1.25);
        assert_eq!(pricing.output_cost, 10.0);
        assert_eq!(pricing.cached_cost, 0.0);
    }

    #[test]
    fn test_cost_calculation_1m_tokens_for_gemini_2_5_pro() {
        let cost = calculate_cost("gemini-2.5-pro", 1_000_000, 1_000_000, 1_000_000);
        let expected = (1.25 as f64) + (10.0 as f64) + (0.0 as f64);
        assert!((cost - expected).abs() < 1e-9);
    }

    #[test]
    fn test_cost_calculation_fractional_for_gemini_2_5_pro() {
        let prompt = 150_000;
        let completion = 25_000;
        let cached = 50_000;
        let cost = calculate_cost("gemini-2.5-pro", prompt, completion, cached);

        let expected_input = (prompt as f64) * 1.25 / 1_000_000.0;
        let expected_output = (completion as f64) * 10.0 / 1_000_000.0;
        let expected_cached = (cached as f64) * 0.0 / 1_000_000.0;
        let expected = expected_input + expected_output + expected_cached;

        assert!((cost - expected).abs() < 1e-9);
    }

    #[test]
    fn test_cost_calculation_zero_tokens_for_gemini_2_5_pro() {
        let cost = calculate_cost("gemini-2.5-pro", 0, 0, 0);
        assert_eq!(cost, 0.0);
    }

    #[test]
    fn test_cost_cents_calculation_for_gemini_2_5_pro() {
        // We will calculate for 1M tokens to avoid precision losses
        let cents = calculate_cost_cents("gemini-2.5-pro", 1_000_000, 1_000_000, 1_000_000);
        let expected_dollars = (1.25 as f64) + (10.0 as f64) + (0.0 as f64);
        let expected_cents = (expected_dollars * 100.0).round() as i64;
        assert_eq!(cents, expected_cents);
    }

    #[test]
    fn test_pricing_retrieval_exact_for_gemini_2_5_flash() {
        let pricing = get_pricing("gemini-2.5-flash");
        assert_eq!(pricing.input_cost, 0.15);
        assert_eq!(pricing.output_cost, 0.6);
        assert_eq!(pricing.cached_cost, 0.0);
    }

    #[test]
    fn test_cost_calculation_1m_tokens_for_gemini_2_5_flash() {
        let cost = calculate_cost("gemini-2.5-flash", 1_000_000, 1_000_000, 1_000_000);
        let expected = (0.15 as f64) + (0.6 as f64) + (0.0 as f64);
        assert!((cost - expected).abs() < 1e-9);
    }

    #[test]
    fn test_cost_calculation_fractional_for_gemini_2_5_flash() {
        let prompt = 150_000;
        let completion = 25_000;
        let cached = 50_000;
        let cost = calculate_cost("gemini-2.5-flash", prompt, completion, cached);

        let expected_input = (prompt as f64) * 0.15 / 1_000_000.0;
        let expected_output = (completion as f64) * 0.6 / 1_000_000.0;
        let expected_cached = (cached as f64) * 0.0 / 1_000_000.0;
        let expected = expected_input + expected_output + expected_cached;

        assert!((cost - expected).abs() < 1e-9);
    }

    #[test]
    fn test_cost_calculation_zero_tokens_for_gemini_2_5_flash() {
        let cost = calculate_cost("gemini-2.5-flash", 0, 0, 0);
        assert_eq!(cost, 0.0);
    }

    #[test]
    fn test_cost_cents_calculation_for_gemini_2_5_flash() {
        // We will calculate for 1M tokens to avoid precision losses
        let cents = calculate_cost_cents("gemini-2.5-flash", 1_000_000, 1_000_000, 1_000_000);
        let expected_dollars = (0.15 as f64) + (0.6 as f64) + (0.0 as f64);
        let expected_cents = (expected_dollars * 100.0).round() as i64;
        assert_eq!(cents, expected_cents);
    }

    #[test]
    fn test_pricing_retrieval_exact_for_minimax_m2_7() {
        let pricing = get_pricing("minimax-m2.7");
        assert_eq!(pricing.input_cost, 1.0);
        assert_eq!(pricing.output_cost, 1.0);
        assert_eq!(pricing.cached_cost, 0.0);
    }

    #[test]
    fn test_cost_calculation_1m_tokens_for_minimax_m2_7() {
        let cost = calculate_cost("minimax-m2.7", 1_000_000, 1_000_000, 1_000_000);
        let expected = (1.0 as f64) + (1.0 as f64) + (0.0 as f64);
        assert!((cost - expected).abs() < 1e-9);
    }

    #[test]
    fn test_cost_calculation_fractional_for_minimax_m2_7() {
        let prompt = 150_000;
        let completion = 25_000;
        let cached = 50_000;
        let cost = calculate_cost("minimax-m2.7", prompt, completion, cached);

        let expected_input = (prompt as f64) * 1.0 / 1_000_000.0;
        let expected_output = (completion as f64) * 1.0 / 1_000_000.0;
        let expected_cached = (cached as f64) * 0.0 / 1_000_000.0;
        let expected = expected_input + expected_output + expected_cached;

        assert!((cost - expected).abs() < 1e-9);
    }

    #[test]
    fn test_cost_calculation_zero_tokens_for_minimax_m2_7() {
        let cost = calculate_cost("minimax-m2.7", 0, 0, 0);
        assert_eq!(cost, 0.0);
    }

    #[test]
    fn test_cost_cents_calculation_for_minimax_m2_7() {
        // We will calculate for 1M tokens to avoid precision losses
        let cents = calculate_cost_cents("minimax-m2.7", 1_000_000, 1_000_000, 1_000_000);
        let expected_dollars = (1.0 as f64) + (1.0 as f64) + (0.0 as f64);
        let expected_cents = (expected_dollars * 100.0).round() as i64;
        assert_eq!(cents, expected_cents);
    }

    #[test]
    fn test_pricing_retrieval_exact_for_minimax_m2_7_turbo() {
        let pricing = get_pricing("minimax-m2.7-turbo");
        assert_eq!(pricing.input_cost, 0.5);
        assert_eq!(pricing.output_cost, 0.5);
        assert_eq!(pricing.cached_cost, 0.0);
    }

    #[test]
    fn test_cost_calculation_1m_tokens_for_minimax_m2_7_turbo() {
        let cost = calculate_cost("minimax-m2.7-turbo", 1_000_000, 1_000_000, 1_000_000);
        let expected = (0.5 as f64) + (0.5 as f64) + (0.0 as f64);
        assert!((cost - expected).abs() < 1e-9);
    }

    #[test]
    fn test_cost_calculation_fractional_for_minimax_m2_7_turbo() {
        let prompt = 150_000;
        let completion = 25_000;
        let cached = 50_000;
        let cost = calculate_cost("minimax-m2.7-turbo", prompt, completion, cached);

        let expected_input = (prompt as f64) * 0.5 / 1_000_000.0;
        let expected_output = (completion as f64) * 0.5 / 1_000_000.0;
        let expected_cached = (cached as f64) * 0.0 / 1_000_000.0;
        let expected = expected_input + expected_output + expected_cached;

        assert!((cost - expected).abs() < 1e-9);
    }

    #[test]
    fn test_cost_calculation_zero_tokens_for_minimax_m2_7_turbo() {
        let cost = calculate_cost("minimax-m2.7-turbo", 0, 0, 0);
        assert_eq!(cost, 0.0);
    }

    #[test]
    fn test_cost_cents_calculation_for_minimax_m2_7_turbo() {
        // We will calculate for 1M tokens to avoid precision losses
        let cents = calculate_cost_cents("minimax-m2.7-turbo", 1_000_000, 1_000_000, 1_000_000);
        let expected_dollars = (0.5 as f64) + (0.5 as f64) + (0.0 as f64);
        let expected_cents = (expected_dollars * 100.0).round() as i64;
        assert_eq!(cents, expected_cents);
    }
}
