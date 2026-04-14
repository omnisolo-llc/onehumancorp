package hybridfsmcp

import (
	"context"
	"strings"
)

type Escalator interface {
	AnalyzeComplexity(ctx context.Context, query string) bool
}

type ComplexityAnalyzer struct {
	TokenThreshold int
}

func NewComplexityAnalyzer(threshold int) *ComplexityAnalyzer {
	return &ComplexityAnalyzer{TokenThreshold: threshold}
}

func (a *ComplexityAnalyzer) AnalyzeComplexity(ctx context.Context, query string) bool {
	// Simple heuristic: count words to estimate tokens.
	words := len(strings.Fields(query))
	return words > a.TokenThreshold
}
