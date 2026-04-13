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
		{
			name:             "Bulk Discount test > 1M tokens",
			model:            "claude-3-5-sonnet-20240620",
			promptTokens:     2000000,
			completionTokens: 0,
			cachedTokens:     0,
			// 10% discount on 2M input tokens
			expectedCost:     (2000000.0 * 3.0 / 1000000.0) * 0.90,
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

	// Test without discount
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
	if math.Abs(details.VolumeDiscount-0.0) > 1e-9 {
		t.Errorf("expected volume discount 0.0, got %f", details.VolumeDiscount)
	}
	if math.Abs(details.TotalCost-expectedTotalCost) > 1e-9 {
		t.Errorf("expected total cost %f, got %f", expectedTotalCost, details.TotalCost)
	}

	// Test with discount (> 1,000,000 input tokens)
	detailsWithDiscount := CalculateCostDetails(ctx, "claude-3-opus-20240229", 1500000, 500, 1000)

	rawInputCost := 1500000.0 * 15.0 / 1000000.0
	expectedDiscount := rawInputCost * 0.10
	expectedInputCostDiscounted := rawInputCost - expectedDiscount
	expectedTotalCostDiscounted := expectedInputCostDiscounted + expectedOutputCost + expectedCachedCost

	if math.Abs(detailsWithDiscount.InputCost-expectedInputCostDiscounted) > 1e-9 {
		t.Errorf("expected discounted input cost %f, got %f", expectedInputCostDiscounted, detailsWithDiscount.InputCost)
	}
	if math.Abs(detailsWithDiscount.VolumeDiscount-expectedDiscount) > 1e-9 {
		t.Errorf("expected volume discount %f, got %f", expectedDiscount, detailsWithDiscount.VolumeDiscount)
	}
	if math.Abs(detailsWithDiscount.TotalCost-expectedTotalCostDiscounted) > 1e-9 {
		t.Errorf("expected total cost %f, got %f", expectedTotalCostDiscounted, detailsWithDiscount.TotalCost)
	}
}
