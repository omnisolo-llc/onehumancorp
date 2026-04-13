package pricing

// Pricing values per 1 million tokens (in USD)
type ModelPricing struct {
	InputCost  float64
	OutputCost float64
	CachedCost float64
}

var ModelRegistry = map[string]ModelPricing{
	"claude-3-5-sonnet-20241022": {
		InputCost:  3.00,
		OutputCost: 15.00,
		CachedCost: 0.30,
	},
	"claude-3-5-sonnet-20240620": {
		InputCost:  3.00,
		OutputCost: 15.00,
		CachedCost: 0.30,
	},
	"gpt-4o": {
		InputCost:  5.00,
		OutputCost: 15.00,
		CachedCost: 2.50,
	},
}

// CalculateCost returns the estimated cost in USD for a given API usage.
func CalculateCost(model string, promptTokens, completionTokens, cachedTokens int) float64 {
	pricing, ok := ModelRegistry[model]
	if !ok {
		// Fallback to average pricing if unknown
		pricing = ModelPricing{
			InputCost:  3.00,
			OutputCost: 15.00,
			CachedCost: 1.50,
		}
	}

	cost := (float64(promptTokens) * pricing.InputCost / 1000000.0) +
		(float64(completionTokens) * pricing.OutputCost / 1000000.0) +
		(float64(cachedTokens) * pricing.CachedCost / 1000000.0)

	return cost
}
