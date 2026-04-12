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
	"gpt-4o":                     {Input: 5.0, Output: 15.0, Cached: 2.50},
	"gpt-4o-mini":                {Input: 0.15, Output: 0.60, Cached: 0.075},
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
