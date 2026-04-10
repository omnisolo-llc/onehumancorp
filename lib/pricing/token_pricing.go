package pricing

import (
	"errors"
)

// TokenCostCalculator calculates the cost of LLM tokens for various models.
type TokenCostCalculator struct {
	modelRates map[string]ModelRates
}

// ModelRates defines the cost per 1M tokens for a specific model.
type ModelRates struct {
	InputCostPer1M       float64
	OutputCostPer1M      float64
	CachedInputCostPer1M float64 // Some models (like Claude 3.5) support discounted prompt caching
}

// NewTokenCostCalculator creates a new calculator with default rates.
func NewTokenCostCalculator() *TokenCostCalculator {
	return &TokenCostCalculator{
		modelRates: map[string]ModelRates{
			"claude-3-5-sonnet-20240620": {
				InputCostPer1M:       3.00,
				OutputCostPer1M:      15.00,
				CachedInputCostPer1M: 0.30, // 90% discount for cached tokens
			},
			"gpt-4o": {
				InputCostPer1M:       5.00,
				OutputCostPer1M:      15.00,
				CachedInputCostPer1M: 2.50, // 50% discount for cached tokens
			},
		},
	}
}

// CalculateCost computes the total cost for a given request.
func (c *TokenCostCalculator) CalculateCost(model string, inputTokens, outputTokens, cachedInputTokens int) (float64, error) {
	rates, exists := c.modelRates[model]
	if !exists {
		return 0, errors.New("unsupported model")
	}

	inputCost := (float64(inputTokens) / 1000000.0) * rates.InputCostPer1M
	outputCost := (float64(outputTokens) / 1000000.0) * rates.OutputCostPer1M
	cachedInputCost := (float64(cachedInputTokens) / 1000000.0) * rates.CachedInputCostPer1M

	return inputCost + outputCost + cachedInputCost, nil
}
