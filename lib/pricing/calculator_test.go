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

func TestCalculateBatchCost(t *testing.T) {
	ctx := context.Background()

	tests := []struct {
		name             string
		model            string
		promptTokens     int
		completionTokens int
		expectedCost     float64
	}{
		{
			name:             "Claude 3.5 Sonnet Batch",
			model:            "claude-3-5-sonnet-20240620",
			promptTokens:     1000,
			completionTokens: 500,
			expectedCost:     (1000.0 * 1.5 / 1000000.0) + (500.0 * 7.5 / 1000000.0),
		},
		{
			name:             "GPT-4o Batch",
			model:            "gpt-4o",
			promptTokens:     1000,
			completionTokens: 1000,
			expectedCost:     (1000.0 * 2.5 / 1000000.0) + (1000.0 * 7.5 / 1000000.0),
		},
		{
			name:             "Unknown model Batch",
			model:            "unknown-model",
			promptTokens:     1000,
			completionTokens: 1000,
			expectedCost:     0.0,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			cost := CalculateBatchCost(ctx, tt.model, tt.promptTokens, tt.completionTokens)
			if math.Abs(cost-tt.expectedCost) > 1e-9 {
				t.Errorf("expected %f, got %f", tt.expectedCost, cost)
			}
		})
	}
}
