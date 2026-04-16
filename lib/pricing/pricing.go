package pricing

import (
	"context"
	"math"
)

type CostAnalysis struct {
	TotalTokens    int64
	PromptTokens   int64
	CompletionTokens int64
	EstimatedCost  float64
}

// CalculateCost takes the prompt and completion tokens and returns an estimated cost
// using an optimized model strategy.
func CalculateCost(ctx context.Context, promptTokens, completionTokens int64) CostAnalysis {
	// Dummy cost calculation based on arbitrary rates
	promptRate := 0.000001
	completionRate := 0.000002

	cost := float64(promptTokens)*promptRate + float64(completionTokens)*completionRate

	return CostAnalysis{
		TotalTokens:      promptTokens + completionTokens,
		PromptTokens:     promptTokens,
		CompletionTokens: completionTokens,
		EstimatedCost:    math.Round(cost*1000000) / 1000000,
	}
}
