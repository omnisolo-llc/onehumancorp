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
		{
			name:             "Claude 3 Haiku",
			model:            "claude-3-haiku-20240307",
			promptTokens:     2000,
			completionTokens: 500,
			cachedTokens:     0,
			expectedCost:     (2000.0 * 0.25 / 1000000.0) + (500.0 * 1.25 / 1000000.0),
		},
		{
			name:             "Claude 3 Opus",
			model:            "claude-3-opus-20240229",
			promptTokens:     100,
			completionTokens: 200,
			cachedTokens:     50,
			expectedCost:     (100.0 * 15.0 / 1000000.0) + (200.0 * 75.0 / 1000000.0) + (50.0 * 1.50 / 1000000.0),
		},
		{
			name:             "o1-preview",
			model:            "o1-preview",
			promptTokens:     1000,
			completionTokens: 1000,
			cachedTokens:     0,
			expectedCost:     (1000.0 * 15.0 / 1000000.0) + (1000.0 * 60.0 / 1000000.0),
		},
		{
			name:             "o1-mini",
			model:            "o1-mini",
			promptTokens:     500,
			completionTokens: 500,
			cachedTokens:     1000,
			expectedCost:     (500.0 * 3.0 / 1000000.0) + (500.0 * 12.0 / 1000000.0) + (1000.0 * 1.50 / 1000000.0),
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

func TestCalculateEmbeddingCost(t *testing.T) {
	ctx := context.Background()

	tests := []struct {
		name         string
		model        string
		tokens       int
		expectedCost float64
	}{
		{
			name:         "text-embedding-3-small",
			model:        "text-embedding-3-small",
			tokens:       1000000,
			expectedCost: 0.02,
		},
		{
			name:         "text-embedding-3-large",
			model:        "text-embedding-3-large",
			tokens:       500000,
			expectedCost: 0.065,
		},
		{
			name:         "Unknown embedding model",
			model:        "unknown-embedding",
			tokens:       10000,
			expectedCost: 0.0,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			cost := CalculateEmbeddingCost(ctx, tt.model, tt.tokens)
			if math.Abs(cost-tt.expectedCost) > 1e-9 {
				t.Errorf("expected %f, got %f", tt.expectedCost, cost)
			}
		})
	}
}

func TestExceedsBudget(t *testing.T) {
	tests := []struct {
		name         string
		currentSpend float64
		limit        float64
		expected     bool
	}{
		{
			name:         "Under budget",
			currentSpend: 10.5,
			limit:        20.0,
			expected:     false,
		},
		{
			name:         "Equal to budget",
			currentSpend: 20.0,
			limit:        20.0,
			expected:     false,
		},
		{
			name:         "Over budget",
			currentSpend: 20.5,
			limit:        20.0,
			expected:     true,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := ExceedsBudget(tt.currentSpend, tt.limit)
			if result != tt.expected {
				t.Errorf("expected %v, got %v", tt.expected, result)
			}
		})
	}
}
