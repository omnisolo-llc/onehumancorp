package pricing

// ModelPricing represents the input, output, and cached costs per 1 million tokens (in USD).
type ModelPricing struct {
	InputCost  float64
	OutputCost float64
	CachedCost float64
}

// ModelRegistry provides a comprehensive list of LLM inference prices.
var ModelRegistry = map[string]ModelPricing{
	// Anthropic — Claude 3 family
	"claude-3-opus":   {InputCost: 15.00, OutputCost: 75.00},
	"claude-3-sonnet": {InputCost: 3.00, OutputCost: 15.00},
	"claude-3-haiku":  {InputCost: 0.25, OutputCost: 1.25},
	// Anthropic — Claude 3.5 family
	"claude-3.5-sonnet":          {InputCost: 3.00, OutputCost: 15.00, CachedCost: 0.30},
	"claude-3.5-haiku":           {InputCost: 0.80, OutputCost: 4.00, CachedCost: 0.08},
	"claude-3-5-sonnet-20241022": {InputCost: 3.00, OutputCost: 15.00, CachedCost: 0.30},
	"claude-3-5-sonnet-20240620": {InputCost: 3.00, OutputCost: 15.00, CachedCost: 0.30},
	// Anthropic — Claude 3.7 family
	"claude-3.7-sonnet": {InputCost: 3.00, OutputCost: 15.00, CachedCost: 0.30},
	// OpenAI — GPT-4 family
	"gpt-4":       {InputCost: 30.00, OutputCost: 60.00},
	"gpt-4-turbo": {InputCost: 10.00, OutputCost: 30.00},
	"gpt-4o":      {InputCost: 2.50, OutputCost: 10.00, CachedCost: 1.25},
	"gpt-4o-mini": {InputCost: 0.15, OutputCost: 0.60, CachedCost: 0.075},
	// OpenAI — GPT-4.1 family
	"gpt-4.1":      {InputCost: 2.00, OutputCost: 8.00},
	"gpt-4.1-mini": {InputCost: 0.40, OutputCost: 1.60},
	"gpt-4.1-nano": {InputCost: 0.10, OutputCost: 0.40},
	// OpenAI — o-series reasoning models
	"o1":      {InputCost: 15.00, OutputCost: 60.00},
	"o1-mini": {InputCost: 3.00, OutputCost: 12.00},
	"o3-mini": {InputCost: 1.10, OutputCost: 4.40},
	// Google — Gemini 1.5 family
	"gemini-1.5-pro":   {InputCost: 3.50, OutputCost: 10.50},
	"gemini-1.5-flash": {InputCost: 0.35, OutputCost: 1.05},
	// Google — Gemini 2.0 family
	"gemini-2.0-flash":      {InputCost: 0.10, OutputCost: 0.40},
	"gemini-2.0-flash-lite": {InputCost: 0.075, OutputCost: 0.30},
	// Google — Gemini 2.5 family
	"gemini-2.5-pro":   {InputCost: 1.25, OutputCost: 10.00},
	"gemini-2.5-flash": {InputCost: 0.15, OutputCost: 0.60},
	// MiniMax — M2.7 family
	"minimax-m2.7":       {InputCost: 1.00, OutputCost: 1.00},
	"minimax-m2.7-turbo": {InputCost: 0.50, OutputCost: 0.50},
}

// CalculateCost returns the estimated cost in USD for a given API usage.
func CalculateCost(model string, promptTokens, completionTokens, cachedTokens int64) float64 {
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

// GetPricing returns the pricing for a given model if it exists in the registry.
func GetPricing(model string) (ModelPricing, bool) {
	pricing, ok := ModelRegistry[model]
	return pricing, ok
}
