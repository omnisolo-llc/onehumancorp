package pricing

import (
	"context"
	"math"
	"testing"
)

func TestCalculateCost(t *testing.T) {
	ctx := context.Background()

	tests := []struct {
		name             string
		model            string
		promptTokens     int
		completionTokens int
		cachedTokens     int
		expectedCost     float64
	}{
		{
			name:             "Claude 3.5 Sonnet",
			model:            "claude-3-5-sonnet-20240620",
			promptTokens:     1000,
			completionTokens: 500,
			cachedTokens:     0,
			expectedCost:     (1000.0 * 3.0 / 1000000.0) + (500.0 * 15.0 / 1000000.0),
		},
		{
			name:             "GPT-4o with caching",
			model:            "gpt-4o",
			promptTokens:     1000,
			completionTokens: 1000,
			cachedTokens:     2000,
			expectedCost:     (1000.0 * 5.0 / 1000000.0) + (1000.0 * 15.0 / 1000000.0) + (2000.0 * 2.50 / 1000000.0),
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

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			cost := CalculateCost(ctx, tt.model, tt.promptTokens, tt.completionTokens, tt.cachedTokens)
			if math.Abs(cost-tt.expectedCost) > 1e-9 {
				t.Errorf("expected %f, got %f", tt.expectedCost, cost)
			}
		})
	}
}

func TestCalculateSavings(t *testing.T) {
	ctx := context.Background()

	tests := []struct {
		name         string
		model        string
		cachedTokens int
		expectedSave float64
	}{
		{
			name:         "Claude 3.5 Sonnet",
			model:        "claude-3-5-sonnet-20240620",
			cachedTokens: 1000,
			expectedSave: (1000.0 * 3.0 / 1000000.0) - (1000.0 * 0.30 / 1000000.0),
		},
		{
			name:         "Claude 3 Haiku",
			model:        "claude-3-haiku-20240307",
			cachedTokens: 2000,
			expectedSave: (2000.0 * 0.25 / 1000000.0) - (2000.0 * 0.025 / 1000000.0),
		},
		{
			name:         "Unknown model",
			model:        "unknown-model",
			cachedTokens: 1000,
			expectedSave: 0.0,
		},
		{
			name:         "Zero cached tokens",
			model:        "gpt-4o",
			cachedTokens: 0,
			expectedSave: 0.0,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			savings := CalculateSavings(ctx, tt.model, tt.cachedTokens)
			if math.Abs(savings-tt.expectedSave) > 1e-9 {
				t.Errorf("expected %f, got %f", tt.expectedSave, savings)
			}
		})
	}
}
