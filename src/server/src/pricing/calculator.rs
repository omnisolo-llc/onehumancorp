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
    }
}
