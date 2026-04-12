package pricing

import (
	"strings"
)

// ModelPricing represents the cost per 1 million tokens.
type ModelPricing struct {
	InputTokens  float64
	OutputTokens float64
	CachedTokens float64
}

// Registry of known models and their pricing per 1M tokens.
var Registry = map[string]ModelPricing{
	"claude-3-5-sonnet-20240620": {InputTokens: 3.0, OutputTokens: 15.0, CachedTokens: 0.3},
	"gpt-4o":                     {InputTokens: 5.0, OutputTokens: 15.0, CachedTokens: 2.5},
	"minimax-abab6.5s-chat":      {InputTokens: 2.0, OutputTokens: 2.0, CachedTokens: 0.2}, // Added Minimax pricing
}

// CalculateCost calculates the total cost of an LLM request in USD.
// If the model is not found, it returns 0.0.
func CalculateCost(model string, promptTokens, completionTokens, cachedTokens int) float64 {
	// Normalize model name (optional, depends on how strictly we match)
	model = strings.ToLower(strings.TrimSpace(model))

	pricing, ok := Registry[model]
	if !ok {
		// Fallback for unknown models
		return 0.0
	}

	cost := (float64(promptTokens) / 1000000.0) * pricing.InputTokens
	cost += (float64(completionTokens) / 1000000.0) * pricing.OutputTokens
	cost += (float64(cachedTokens) / 1000000.0) * pricing.CachedTokens

	return cost
}
