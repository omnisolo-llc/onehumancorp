package pricing

import (
	"strings"
)

// CostAwareRouter routes prompts to appropriate models based on estimated cost and available budget.
type CostAwareRouter struct {
	PremiumModel string
	CheaperModel string
	Budget       *BudgetManager
	PremiumPrice float64 // Cost per estimated token for premium model
	CheaperPrice float64 // Cost per estimated token for cheaper model
	ComplexityTh int     // Minimum length to consider a prompt "complex"
}

// NewCostAwareRouter initializes a new CostAwareRouter.
func NewCostAwareRouter(budget *BudgetManager, premium, cheaper string, premiumPrice, cheaperPrice float64) *CostAwareRouter {
	return &CostAwareRouter{
		PremiumModel: premium,
		CheaperModel: cheaper,
		Budget:       budget,
		PremiumPrice: premiumPrice,
		CheaperPrice: cheaperPrice,
		ComplexityTh: 100, // Default complexity threshold
	}
}

// estimateTokens provides a very rough token estimation based on word count.
func estimateTokens(prompt string) int {
	words := len(strings.Fields(prompt))
	return words + (words / 3) // Rough estimate: 1 word ~ 1.33 tokens
}

// Route decides whether to use the premium or cheaper model.
// Returns the chosen model, estimated cost, and whether the prompt was considered complex.
func (r *CostAwareRouter) Route(prompt string) (string, float64, bool) {
	tokens := estimateTokens(prompt)
	isComplex := tokens >= r.ComplexityTh

	premiumCost := float64(tokens) * r.PremiumPrice
	cheaperCost := float64(tokens) * r.CheaperPrice
	remaining := r.Budget.GetRemaining()

	// If it's a complex prompt and we have enough budget for the premium model, use it.
	if isComplex && remaining >= premiumCost {
		return r.PremiumModel, premiumCost, true
	}

	// If we don't have enough budget for the cheaper model either, we might fail downstream,
	// but we'll return the cheaper model and cost anyway to let the caller handle it.
	return r.CheaperModel, cheaperCost, isComplex
}
