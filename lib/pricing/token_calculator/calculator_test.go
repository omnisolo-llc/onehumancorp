package token_calculator
import (
  "testing"
)
func TestCalculateCost(t *testing.T) {
  config := CostConfig{
    CostPerInputToken:       0.00001,
    CostPerOutputToken:      0.00003,
    CostPerCachedInputToken: 0.000005,
    CostPerLocalEmbedding:   0.000002,
		CostPerGBMonth:          0.02,
    DiscountFactor:          0.10,
  }
  cost := CalculateCost(1000, 500, 2000, 1000, config)
  expected := 0.0333
  if cost != expected {
    t.Errorf("expected %f, got %f", expected, cost)
  }
}

func TestCalculateStorageSavings(t *testing.T) {
	config := CostConfig{
		CostPerGBMonth: 0.02,
		DiscountFactor: 0.10,
	}
	// 10 GB original, 2 GB compressed -> 8 GB saved
	// Savings: 8 * 0.02 * 0.9 = 0.144
	savings := CalculateStorageSavings(10*1024*1024*1024, 2*1024*1024*1024, config)
	expected := 0.144
	if savings != expected {
		t.Errorf("expected %f, got %f", expected, savings)
	}
}
