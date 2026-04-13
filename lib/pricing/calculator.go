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
	"claude-3-opus-20240229":     {Input: 15.0, Output: 75.0, Cached: 1.50},
	"claude-3-5-sonnet-20240620": {Input: 3.0, Output: 15.0, Cached: 0.30}, // Example rates
	"claude-3-haiku-20240307":    {Input: 0.25, Output: 1.25, Cached: 0.025},
	"gpt-4o":                     {Input: 5.0, Output: 15.0, Cached: 2.50},
	"gpt-4o-mini":                {Input: 0.15, Output: 0.60, Cached: 0.075},
	"o1-preview":                 {Input: 15.0, Output: 60.0, Cached: 7.50},
	"o1-mini":                    {Input: 3.0, Output: 12.0, Cached: 1.50},
}

// CostDetails represents the breakdown of costs for an LLM request.
type CostDetails struct {
	InputCost  float64
	OutputCost float64
	CachedCost float64
	TotalCost  float64
}

// CalculateCostDetails computes the granular costs of an LLM request in USD.
func CalculateCostDetails(ctx context.Context, model string, promptTokens, completionTokens, cachedTokens int) CostDetails {
	rates, ok := ModelPricing[strings.ToLower(model)]
	if !ok {
		// Fallback to average or zero if model unknown
		return CostDetails{}
	}

	details := CostDetails{
		InputCost:  float64(promptTokens) * (rates.Input / 1000000.0),
		OutputCost: float64(completionTokens) * (rates.Output / 1000000.0),
		CachedCost: float64(cachedTokens) * (rates.Cached / 1000000.0),
	}
	details.TotalCost = details.InputCost + details.OutputCost + details.CachedCost

	if costCounter != nil {
		costCounter.Add(ctx, details.TotalCost)
	}

	return details
}

// CalculateCost computes the total cost of an LLM request in USD.
func CalculateCost(ctx context.Context, model string, promptTokens, completionTokens, cachedTokens int) float64 {
	return CalculateCostDetails(ctx, model, promptTokens, completionTokens, cachedTokens).TotalCost
}
