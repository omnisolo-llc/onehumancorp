package pricing

type CacheOptimizer struct {
	BaseCostPerToken float64
}

func NewCacheOptimizer(base float64) *CacheOptimizer {
	return &CacheOptimizer{BaseCostPerToken: base}
}

func (co *CacheOptimizer) CalculateSavings(originalTokens int, cachedTokens int) float64 {
	savedTokens := originalTokens - cachedTokens
	if savedTokens <= 0 {
		return 0
	}
	return float64(savedTokens) * co.BaseCostPerToken
}
