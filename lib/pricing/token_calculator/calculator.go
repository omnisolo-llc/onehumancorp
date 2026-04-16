package token_calculator
import "math"
type CostConfig struct {
  CostPerInputToken       float64
  CostPerOutputToken      float64
  CostPerCachedInputToken float64
  CostPerLocalEmbedding   float64
	CostPerGBMonth          float64
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

func CalculateStorageSavings(originalBytes, compressedBytes int64, config CostConfig) float64 {
	if originalBytes <= compressedBytes {
		return 0.0
	}
	savedBytes := float64(originalBytes - compressedBytes)
	savedGB := savedBytes / (1024 * 1024 * 1024)
	savings := savedGB * config.CostPerGBMonth * (1.0 - config.DiscountFactor)
	return math.Round(savings*10000) / 10000
}
