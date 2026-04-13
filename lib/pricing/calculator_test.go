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
			expectedCost:     (1000.0 * 2.50 / 1000000.0) + (1000.0 * 10.0 / 1000000.0) + (2000.0 * 1.25 / 1000000.0),
		},
		{
			name:             "Gemini 2.0 Flash",
			model:            "gemini-2.0-flash",
			promptTokens:     1000,
			completionTokens: 500,
			cachedTokens:     1000,
			expectedCost:     (1000.0 * 0.10 / 1000000.0) + (500.0 * 0.40 / 1000000.0) + (1000.0 * 0.02 / 1000000.0),
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
			name:             "Claude 3 Opus",
			model:            "claude-3-opus-20240229",
			promptTokens:     1000,
			completionTokens: 500,
			cachedTokens:     1000,
			expectedCost:     (1000.0 * 15.0 / 1000000.0) + (500.0 * 75.0 / 1000000.0) + (1000.0 * 1.50 / 1000000.0),
		},
		{
			name:             "Claude 3 Haiku",
			model:            "claude-3-haiku-20240307",
			promptTokens:     1000,
			completionTokens: 500,
			cachedTokens:     1000,
			expectedCost:     (1000.0 * 0.25 / 1000000.0) + (500.0 * 1.25 / 1000000.0) + (1000.0 * 0.025 / 1000000.0),
		},
		{
			name:             "o1 preview",
			model:            "o1-preview",
			promptTokens:     1000,
			completionTokens: 500,
			cachedTokens:     1000,
			expectedCost:     (1000.0 * 15.0 / 1000000.0) + (500.0 * 60.0 / 1000000.0) + (1000.0 * 7.50 / 1000000.0),
		},
		{
			name:             "o1 mini",
			model:            "o1-mini",
			promptTokens:     1000,
			completionTokens: 500,
			cachedTokens:     1000,
			expectedCost:     (1000.0 * 3.0 / 1000000.0) + (500.0 * 12.0 / 1000000.0) + (1000.0 * 1.50 / 1000000.0),
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			cost := CalculateCost(ctx, tt.model, tt.promptTokens, tt.completionTokens, tt.cachedTokens)
			if math.Abs(cost-tt.expectedCost) > 1e-9 {
				t.Errorf("expected total cost %f, got %f", tt.expectedCost, cost)
			}
		})
	}
}

func TestCalculateCostDetails(t *testing.T) {
	ctx := context.Background()

	details := CalculateCostDetails(ctx, "claude-3-opus-20240229", 1000, 500, 1000)

	expectedInputCost := 1000.0 * 15.0 / 1000000.0
	expectedOutputCost := 500.0 * 75.0 / 1000000.0
	expectedCachedCost := 1000.0 * 1.50 / 1000000.0
	expectedTotalCost := expectedInputCost + expectedOutputCost + expectedCachedCost

	if math.Abs(details.InputCost-expectedInputCost) > 1e-9 {
		t.Errorf("expected input cost %f, got %f", expectedInputCost, details.InputCost)
	}
	if math.Abs(details.OutputCost-expectedOutputCost) > 1e-9 {
		t.Errorf("expected output cost %f, got %f", expectedOutputCost, details.OutputCost)
	}
	if math.Abs(details.CachedCost-expectedCachedCost) > 1e-9 {
		t.Errorf("expected cached cost %f, got %f", expectedCachedCost, details.CachedCost)
	}
	if math.Abs(details.TotalCost-expectedTotalCost) > 1e-9 {
		t.Errorf("expected total cost %f, got %f", expectedTotalCost, details.TotalCost)
	}
}

func TestCalculateSavings(t *testing.T) {
	tests := []struct {
		name         string
		model        string
		cachedTokens int
		expected     float64
	}{
		{
			name:         "GPT-4o Savings",
			model:        "gpt-4o",
			cachedTokens: 1000000,
			expected:     1.25, // 2.50 - 1.25
		},
		{
			name:         "Claude 3.5 Sonnet Savings",
			model:        "claude-3-5-sonnet-20240620",
			cachedTokens: 500000,
			expected:     1.35, // (3.00 - 0.30) / 2
		},
		{
			name:         "Unknown Model",
			model:        "unknown",
			cachedTokens: 10000,
			expected:     0.0,
		},
		{
			name:         "Zero Tokens",
			model:        "gpt-4o",
			cachedTokens: 0,
			expected:     0.0,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			got := CalculateSavings(tt.model, tt.cachedTokens)
			if math.Abs(got-tt.expected) > 1e-9 {
				t.Errorf("CalculateSavings() = %f, want %f", got, tt.expected)
			}
		})
	}
}
