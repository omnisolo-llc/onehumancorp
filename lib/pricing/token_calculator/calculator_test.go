package token_calculator
import (
  "testing"
)
func TestCalculateCost(t *testing.T) {
  config := CostConfig{
    CostPerInputToken:  0.00001,
    CostPerOutputToken: 0.00003,
    DiscountFactor:     0.10,
  }
  cost := CalculateCost(1000, 500, config)
  expected := 0.0225
  if cost != expected {
    t.Errorf("expected %f, got %f", expected, cost)
  }
}
func TestCalculateCostWithCache(t *testing.T) {
  config := CostConfig{
    CostPerInputToken:       0.00001,
    CostPerCachedInputToken: 0.000005,
    CostPerOutputToken:      0.00003,
    DiscountFactor:          0.0,
  }
  cost := CalculateCostWithCache(500, 500, 500, config)
  expected := 0.0225
  if cost != expected {
    t.Errorf("expected %f, got %f", expected, cost)
  }
}
