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
    DiscountFactor:          0.10,
  }
  cost := CalculateCost(1000, 500, 2000, 1000, config)
  expected := 0.0333
  if cost != expected {
    t.Errorf("expected %f, got %f", expected, cost)
  }
}
