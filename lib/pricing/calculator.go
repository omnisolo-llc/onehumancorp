package pricing

import (
	"context"
	"strings"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
)

var (
	meter          = otel.Meter("github.com/onehumancorp/mono/ohc")
	costCounter    metric.Float64Counter
)

func init() {
	var err error
	costCounter, err = meter.Float64Counter("llm.cost.usd", metric.WithDescription("Accumulated LLM cost in USD"))
	if err != nil {
		// Log the error but do not panic to avoid crashing the whole system
		_ = err
	}
}

// PricingRates defines the cost per 1M tokens in USD.
type PricingRates struct {
	Input  float64
	Output float64
	Cached float64
}

var ModelPricing = map[string]PricingRates{
	"claude-3-5-sonnet-20240620": {Input: 3.0, Output: 15.0, Cached: 0.30}, // Example rates
	"claude-3-haiku-20240307":    {Input: 0.25, Output: 1.25, Cached: 0.025},
	"claude-3-opus-20240229":     {Input: 15.0, Output: 75.0, Cached: 1.50},
	"gpt-4o":                     {Input: 5.0, Output: 15.0, Cached: 2.50},
	"gpt-4o-mini":                {Input: 0.15, Output: 0.60, Cached: 0.075},
	"o1-preview":                 {Input: 15.0, Output: 60.0, Cached: 7.50},
	"o1-mini":                    {Input: 3.0, Output: 12.0, Cached: 1.50},
}

// EmbeddingPricing defines the cost per 1M tokens in USD for embedding models.
var EmbeddingPricing = map[string]float64{
	"text-embedding-3-small": 0.02,
	"text-embedding-3-large": 0.13,
}

// CalculateCost computes the cost of an LLM request in USD.
func CalculateCost(ctx context.Context, model string, promptTokens, completionTokens, cachedTokens int) float64 {
	rates, ok := ModelPricing[strings.ToLower(model)]
	if !ok {
		// Fallback to average or zero if model unknown
		return 0.0
	}

	inputCost := float64(promptTokens) * (rates.Input / 1000000.0)
	outputCost := float64(completionTokens) * (rates.Output / 1000000.0)
	cachedCost := float64(cachedTokens) * (rates.Cached / 1000000.0)

	totalCost := inputCost + outputCost + cachedCost

	if costCounter != nil {
		costCounter.Add(ctx, totalCost)
	}

	return totalCost
}

// CalculateEmbeddingCost computes the cost of an embedding request in USD.
func CalculateEmbeddingCost(ctx context.Context, model string, tokens int) float64 {
	rate, ok := EmbeddingPricing[strings.ToLower(model)]
	if !ok {
		// Fallback if model unknown
		return 0.0
	}

	cost := float64(tokens) * (rate / 1000000.0)

	if costCounter != nil {
		costCounter.Add(ctx, cost)
	}

	return cost
}

// ExceedsBudget returns true if the current spend exceeds the limit.
func ExceedsBudget(currentSpend, limit float64) bool {
	return currentSpend > limit
}
