package pricing

import (
	"testing"
)

func TestCalculateCost(t *testing.T) {
	tests := []struct {
		name             string
		model            string
		promptTokens     int
		completionTokens int
		cachedTokens     int
		expectedCost     float64
	}{
		{
			name:             "Claude 3.5 Sonnet normal usage",
			model:            "claude-3-5-sonnet-20240620",
			promptTokens:     1000000,
			completionTokens: 1000000,
			cachedTokens:     0,
			expectedCost:     18.0, // 3.0 + 15.0
		},
		{
			name:             "GPT-4o normal usage",
			model:            "gpt-4o",
			promptTokens:     1000000,
			completionTokens: 1000000,
			cachedTokens:     1000000,
			expectedCost:     22.5, // 5.0 + 15.0 + 2.5
		},
		{
			name:             "Minimax usage",
			model:            "minimax-abab6.5s-chat",
			promptTokens:     1000000,
			completionTokens: 1000000,
			cachedTokens:     1000000,
			expectedCost:     4.2, // 2.0 + 2.0 + 0.2
		},
		{
			name:             "Unknown model",
			model:            "unknown-model",
			promptTokens:     1000,
			completionTokens: 1000,
			cachedTokens:     0,
			expectedCost:     0.0,
		},
	}

	for _, tc := range tests {
		t.Run(tc.name, func(t *testing.T) {
			cost := CalculateCost(tc.model, tc.promptTokens, tc.completionTokens, tc.cachedTokens)
			// Simple float comparison; for real apps we'd use an epsilon
			if cost != tc.expectedCost {
				t.Errorf("expected cost %v, got %v", tc.expectedCost, cost)
			}
		})
	}
}
