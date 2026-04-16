package token_calculator
import "math"
type CostConfig struct {
  CostPerInputToken  float64
  CostPerOutputToken float64
  DiscountFactor     float64
  CostPerCachedInputToken float64
}
func CalculateCost(inputTokens, outputTokens int, config CostConfig) float64 {
  inputCost := float64(inputTokens) * config.CostPerInputToken
  outputCost := float64(outputTokens) * config.CostPerOutputToken
  total := (inputCost + outputCost) * (1.0 - config.DiscountFactor)
  return math.Round(total*10000) / 10000
}
func CalculateCostWithCache(inputTokens, cachedInputTokens, outputTokens int, config CostConfig) float64 {
  inputCost := float64(inputTokens) * config.CostPerInputToken
  cachedCost := float64(cachedInputTokens) * config.CostPerCachedInputToken
  outputCost := float64(outputTokens) * config.CostPerOutputToken
  total := (inputCost + cachedCost + outputCost) * (1.0 - config.DiscountFactor)
  return math.Round(total*10000) / 10000
}
