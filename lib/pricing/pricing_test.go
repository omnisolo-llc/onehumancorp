package pricing

import (
	"context"
	"testing"
)

func TestCalculateCost(t *testing.T) {
	ctx := context.Background()
	analysis := CalculateCost(ctx, 1000, 500)

	if analysis.TotalTokens != 1500 {
		t.Errorf("Expected 1500 total tokens, got %d", analysis.TotalTokens)
	}

	expectedCost := 0.002 // 1000 * 0.000001 + 500 * 0.000002
	if analysis.EstimatedCost != expectedCost {
		t.Errorf("Expected cost %f, got %f", expectedCost, analysis.EstimatedCost)
	}
}
