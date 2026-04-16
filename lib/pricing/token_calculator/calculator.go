package token_calculator
import "math"
type CostConfig struct {
  CostPerInputToken       float64
  CostPerOutputToken      float64
  CostPerCachedInputToken float64
  CostPerLocalEmbedding   float64
  DiscountFactor          float64
}
func CalculateCost(inputTokens, outputTokens, cachedInputTokens, localEmbeddingTokens int, config CostConfig) float64 {
  inputCost := float64(inputTokens) * config.CostPerInputToken
  outputCost := float64(outputTokens) * config.CostPerOutputToken
  cachedCost := float64(cachedInputTokens) * config.CostPerCachedInputToken
  embeddingCost := float64(localEmbeddingTokens) * config.CostPerLocalEmbedding
  total := (inputCost + outputCost + cachedCost + embeddingCost) * (1.0 - config.DiscountFactor)
  return math.Round(total*10000) / 10000
}
