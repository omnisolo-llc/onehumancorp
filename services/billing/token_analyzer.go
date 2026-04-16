package billing

import "github.com/onehumancorp/mono/lib/pricing"

func AnalyzeTokens(original int, cached int, baseRate float64) float64 {
	optimizer := pricing.NewCacheOptimizer(baseRate)
	return optimizer.CalculateSavings(original, cached)
}
