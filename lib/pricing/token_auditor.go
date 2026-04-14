package pricing

import (
	"errors"
	"math"
)

// TokenAuditor calculates the cost of token usage.
type TokenAuditor struct {
	CostPerThousandTokens float64
}

// NewTokenAuditor creates a new TokenAuditor.
func NewTokenAuditor(costPerThousandTokens float64) (*TokenAuditor, error) {
	if costPerThousandTokens < 0 {
		return nil, errors.New("cost per thousand tokens cannot be negative")
	}
	return &TokenAuditor{
		CostPerThousandTokens: costPerThousandTokens,
	}, nil
}

// CalculateCost calculates the cost for a given number of tokens.
func (a *TokenAuditor) CalculateCost(tokens int) float64 {
	if tokens < 0 {
		return 0
	}
	cost := (float64(tokens) / 1000.0) * a.CostPerThousandTokens
	// Round to 4 decimal places
	return math.Round(cost*10000) / 10000
}
