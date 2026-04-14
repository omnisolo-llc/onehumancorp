package pricing

import (
	"math"
	"strings"
)

// TokenEstimator provides heuristic-based token estimation.
// It assumes an average of 4 characters per token for English text.
type TokenEstimator struct {
	CharsPerToken float64
}

// NewTokenEstimator creates a new TokenEstimator with a default ratio.
func NewTokenEstimator() *TokenEstimator {
	return &TokenEstimator{
		CharsPerToken: 4.0,
	}
}

// EstimateTokens provides a rough estimate of the token count for a given text.
func (e *TokenEstimator) EstimateTokens(text string) int {
	text = strings.TrimSpace(text)
	if text == "" {
		return 0
	}
	return int(math.Ceil(float64(len(text)) / e.CharsPerToken))
}
