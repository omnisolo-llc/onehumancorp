package token_calculator

import "math"

type CostConfig struct {
	CostPerInputToken       float64
	CostPerOutputToken      float64
	CostPerCachedInputToken float64
	CostPerLocalEmbedding   float64
	DiscountFactor          float64
	CostPerGBMonth          float64
	CostPerComputeHour      float64
	CostPerNetworkGB        float64
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
	savedBytes := float64(originalBytes - compressedBytes)
	if savedBytes < 0 {
		savedBytes = 0
	}
	savedGB := savedBytes / (1024 * 1024 * 1024)
	savings := savedGB * config.CostPerGBMonth
	return math.Round(savings*10000) / 10000
}

func CalculateComputeCost(hours float64, config CostConfig) float64 {
	cost := hours * config.CostPerComputeHour
	return math.Round(cost*10000) / 10000
}

func CalculateNetworkCost(bytes int64, config CostConfig) float64 {
	gb := float64(bytes) / (1024 * 1024 * 1024)
	cost := gb * config.CostPerNetworkGB
	return math.Round(cost*10000) / 10000
}

func CalculateContextCompressionSavings(originalTokens, compressedTokens int, config CostConfig) float64 {
	savedTokens := float64(originalTokens - compressedTokens)
	if savedTokens < 0 {
		savedTokens = 0
	}
	savings := savedTokens * config.CostPerInputToken
	return math.Round(savings*10000) / 10000
}
