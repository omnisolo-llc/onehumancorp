package pricing

import (
	"crypto/sha256"
	"encoding/hex"
	"fmt"
	"strings"
	"sync"
)

// CostOptimizer provides utilities for analyzing and optimizing costs,
// including a thread-safe token cache.
type CostOptimizer struct {
	BaseCost float64
	cache    sync.Map
}

// NewCostOptimizer creates a new CostOptimizer.
func NewCostOptimizer(baseCost float64) *CostOptimizer {
	return &CostOptimizer{BaseCost: baseCost}
}

// generateCacheKey creates a consistent hash for caching
func generateCacheKey(prompt string) string {
	hash := sha256.Sum256([]byte(strings.TrimSpace(prompt)))
	return hex.EncodeToString(hash[:])
}

// GetCachedPrompt checks the local memory cache for an existing prompt response
// to avoid unnecessary LLM calls and reduce token cost.
func (c *CostOptimizer) GetCachedPrompt(prompt string) (string, bool) {
	key := generateCacheKey(prompt)
	if val, ok := c.cache.Load(key); ok {
		return val.(string), true
	}
	return "", false
}

// SetCachedPrompt stores a response in the cache for future reuse.
func (c *CostOptimizer) SetCachedPrompt(prompt, response string) {
	key := generateCacheKey(prompt)
	c.cache.Store(key, response)
}

// AnalyzeCost calculates the projected cost after applying compression optimizations.
func (c *CostOptimizer) AnalyzeCost(tokens int) float64 {
	// Simple cost per token reduction strategy
	discountedRate := 0.0001
	return c.BaseCost + (float64(tokens) * discountedRate)
}

// GetTokenEfficiency returns the efficiency metric.
func (c *CostOptimizer) GetTokenEfficiency() string {
	return fmt.Sprintf("Efficiency calculated for base cost: %.2f", c.BaseCost)
}

// OptimizePrompt unifies the prompt compression pipeline:
// 1. Minifies JSON if applicable
// 2. Reduces English stop words
// 3. Truncates the total word count to the maximum boundary limit
func (c *CostOptimizer) OptimizePrompt(prompt string, maxWords int) string {
	prompt = MinifyJSONPrompt(prompt)
	prompt = ReduceTokens(prompt)
	prompt = TruncateByWordCount(prompt, maxWords)
	return prompt
}
