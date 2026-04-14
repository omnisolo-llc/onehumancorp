package pricing

import (
	"testing"
)

func TestNewTokenAuditor(t *testing.T) {
	_, err := NewTokenAuditor(-1.0)
	if err == nil {
		t.Error("expected error for negative cost")
	}

	auditor, err := NewTokenAuditor(0.01)
	if err != nil {
		t.Errorf("unexpected error: %v", err)
	}
	if auditor.CostPerThousandTokens != 0.01 {
		t.Errorf("expected 0.01, got %v", auditor.CostPerThousandTokens)
	}
}

func TestCalculateCost(t *testing.T) {
	auditor, _ := NewTokenAuditor(0.01)

	tests := []struct {
		tokens   int
		expected float64
	}{
		{0, 0},
		{500, 0.005},
		{1000, 0.01},
		{1500, 0.015},
		{-100, 0},
	}

	for _, test := range tests {
		cost := auditor.CalculateCost(test.tokens)
		if cost != test.expected {
			t.Errorf("for %d tokens, expected %v, got %v", test.tokens, test.expected, cost)
		}
	}
}
