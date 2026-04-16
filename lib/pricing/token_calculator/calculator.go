package token_calculator
import "math"
type CostConfig struct {
  CostPerInputToken  float64
  CostPerOutputToken float64
  DiscountFactor     float64
}
func CalculateCost(inputTokens, outputTokens int, config CostConfig) float64 {
  inputCost := float64(inputTokens) * config.CostPerInputToken
  outputCost := float64(outputTokens) * config.CostPerOutputToken
  total := (inputCost + outputCost) * (1.0 - config.DiscountFactor)
  return math.Round(total*10000) / 10000
}
