package pricing

import (
	"testing"
)

func TestCalculateCost(t *testing.T) {
	tests := []struct {
		model            string
		promptTokens     int64
		completionTokens int64
		cachedTokens     int64
		expected         float64
	}{
		{
			model:            "claude-3-5-sonnet-20241022",
			promptTokens:     1000000,
			completionTokens: 1000000,
			cachedTokens:     1000000,
			expected:         18.30, // 3.00 + 15.00 + 0.30
		},
		{
			model:            "gpt-4o",
			promptTokens:     1000000,
			completionTokens: 1000000,
			cachedTokens:     1000000,
			expected:         13.75, // 2.50 + 10.00 + 1.25
		},
		{
			model:            "unknown-model",
			promptTokens:     1000000,
			completionTokens: 1000000,
			cachedTokens:     1000000,
			expected:         19.50, // 3.00 + 15.00 + 1.50 (fallback)
		},
		{
			model:            "claude-3-haiku",
			promptTokens:     4000000,
			completionTokens: 1000000,
			cachedTokens:     0,
			expected:         2.25, // (4 * 0.25) + (1 * 1.25) = 1.0 + 1.25 = 2.25
		},
	}

	for _, tt := range tests {
		t.Run(tt.model, func(t *testing.T) {
			cost := CalculateCost(tt.model, tt.promptTokens, tt.completionTokens, tt.cachedTokens)
			if cost != tt.expected {
				t.Errorf("CalculateCost(%s) = %f, expected %f", tt.model, cost, tt.expected)
			}
		})
	}
}

func TestGetPricing(t *testing.T) {
	pricing, ok := GetPricing("claude-3.5-sonnet")
	if !ok {
		t.Fatal("Expected pricing for claude-3.5-sonnet to be found")
	}
	if pricing.InputCost != 3.00 || pricing.OutputCost != 15.00 || pricing.CachedCost != 0.30 {
		t.Errorf("Unexpected pricing for claude-3.5-sonnet: %+v", pricing)
	}

	_, ok = GetPricing("non-existent-model")
	if ok {
		t.Error("Expected GetPricing to return ok=false for non-existent model")
	}
}
