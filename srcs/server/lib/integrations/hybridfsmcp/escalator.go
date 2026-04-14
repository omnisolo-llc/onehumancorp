package hybridfsmcp

import (
	"context"
)

// Escalator defines the interface for evaluating query complexity
type Escalator interface {
	ShouldEscalate(ctx context.Context, query string) bool
}

// ComplexityAnalyzer is an implementation of Escalator
type ComplexityAnalyzer struct {
	TokenThreshold int
}

// NewComplexityAnalyzer creates a new ComplexityAnalyzer
func NewComplexityAnalyzer(threshold int) *ComplexityAnalyzer {
	return &ComplexityAnalyzer{
		TokenThreshold: threshold,
	}
}

// ShouldEscalate returns true if the query exceeds the complexity threshold
func (c *ComplexityAnalyzer) ShouldEscalate(ctx context.Context, query string) bool {
	// A simple heuristic: 1 token ~= 4 characters. We escalate if character count > 4 * TokenThreshold
	return len(query) > (c.TokenThreshold * 4)
}
