package pricing

import (
	"testing"
)

func TestCalculateCost(t *testing.T) {
	cost := CalculateCost("claude-3-5-sonnet-20241022", 1000000, 1000000, 1000000)
	expected := 18.30 // 3.00 + 15.00 + 0.30
	if cost != expected {
		t.Errorf("Expected cost %f, got %f", expected, cost)
	}

	// Unknown model fallback
	cost = CalculateCost("unknown-model", 1000000, 1000000, 1000000)
	expectedFallback := 19.50 // 3.00 + 15.00 + 1.50
	if cost != expectedFallback {
		t.Errorf("Expected fallback cost %f, got %f", expectedFallback, cost)
	}
}
