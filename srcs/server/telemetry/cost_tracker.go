package telemetry

import (
	"sync"
)

// Price represents the cost for a given model
type Price struct {
	InputPerMillionUSD  float64
	OutputPerMillionUSD float64
	CachedPerMillionUSD float64
}

// CostTracker accumulates token usage and calculates costs based on a pricing model.
type CostTracker struct {
	mu            sync.Mutex
	catalog       map[string]Price
	PromptTokens  int64
	CompletionTokens int64
	CachedTokens  int64
	TotalTokens   int64
	CostUSD       float64
}

// DefaultCatalog provides a comprehensive list of LLM inference prices.
var DefaultCatalog = map[string]Price{
	// Anthropic — Claude 3 family
	"claude-3-opus":   {InputPerMillionUSD: 15.00, OutputPerMillionUSD: 75.00},
	"claude-3-sonnet": {InputPerMillionUSD: 3.00, OutputPerMillionUSD: 15.00},
	"claude-3-haiku":  {InputPerMillionUSD: 0.25, OutputPerMillionUSD: 1.25},
	// Anthropic — Claude 3.5 family
	"claude-3.5-sonnet": {InputPerMillionUSD: 3.00, OutputPerMillionUSD: 15.00, CachedPerMillionUSD: 0.30},
	"claude-3.5-haiku":  {InputPerMillionUSD: 0.80, OutputPerMillionUSD: 4.00, CachedPerMillionUSD: 0.08},
	// Anthropic — Claude 3.7 family
	"claude-3-7-sonnet-20250219": {InputPerMillionUSD: 3.00, OutputPerMillionUSD: 15.00, CachedPerMillionUSD: 0.30},
	// OpenAI — GPT-4 family
	"gpt-4":       {InputPerMillionUSD: 30.00, OutputPerMillionUSD: 60.00},
	"gpt-4-turbo": {InputPerMillionUSD: 10.00, OutputPerMillionUSD: 30.00},
	"gpt-4o":      {InputPerMillionUSD: 5.00, OutputPerMillionUSD: 15.00},
	"gpt-4o-mini": {InputPerMillionUSD: 0.15, OutputPerMillionUSD: 0.60},
	"o1-mini":     {InputPerMillionUSD: 3.00, OutputPerMillionUSD: 12.00},
	"o1-preview":  {InputPerMillionUSD: 15.00, OutputPerMillionUSD: 60.00},
	// Google — Gemini family
	"gemini-1.5-pro":   {InputPerMillionUSD: 3.50, OutputPerMillionUSD: 10.50},
	"gemini-1.5-flash": {InputPerMillionUSD: 0.075, OutputPerMillionUSD: 0.30},
	"gemini-2.0-flash": {InputPerMillionUSD: 0.10, OutputPerMillionUSD: 0.40},
}

// NewCostTracker creates a new CostTracker.
func NewCostTracker(catalog map[string]Price) *CostTracker {
	if catalog == nil {
		catalog = DefaultCatalog
	}
	copied := make(map[string]Price, len(catalog))
	for model, price := range catalog {
		copied[model] = price
	}
	return &CostTracker{
		catalog: copied,
	}
}

// AddUsage records token usage and calculates incremental cost.
func (c *CostTracker) AddUsage(model string, promptTokens, completionTokens, cachedTokens int64) {
	c.mu.Lock()
	defer c.mu.Unlock()

	c.PromptTokens += promptTokens
	c.CompletionTokens += completionTokens
	c.CachedTokens += cachedTokens
	c.TotalTokens += promptTokens + completionTokens + cachedTokens

	if price, ok := c.catalog[model]; ok {
		c.CostUSD += (float64(promptTokens)/1_000_000.0)*price.InputPerMillionUSD +
			(float64(completionTokens)/1_000_000.0)*price.OutputPerMillionUSD +
			(float64(cachedTokens)/1_000_000.0)*price.CachedPerMillionUSD
	}
}

// GetMetrics returns the current tracked metrics.
func (c *CostTracker) GetMetrics() (promptTokens, completionTokens, totalTokens int64, costUSD float64) {
	c.mu.Lock()
	defer c.mu.Unlock()
	return c.PromptTokens, c.CompletionTokens, c.TotalTokens, c.CostUSD
}
