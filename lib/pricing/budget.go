package pricing

import (
	"fmt"
	"sync"
)

// BudgetManager tracks and limits token spending across sessions.
type BudgetManager struct {
	TotalLimit float64
	current    float64
	mu         sync.Mutex
}

// NewBudgetManager creates a new BudgetManager.
func NewBudgetManager(limit float64) *BudgetManager {
	return &BudgetManager{TotalLimit: limit}
}

// RecordSpend records a token transaction and returns whether it exceeds the budget.
func (b *BudgetManager) RecordSpend(amount float64) (bool, error) {
	b.mu.Lock()
	defer b.mu.Unlock()

	if amount < 0 {
		return false, fmt.Errorf("spend amount cannot be negative")
	}

	if b.current+amount > b.TotalLimit {
		return false, fmt.Errorf("budget exceeded: cannot spend %.2f, remaining budget is %.2f", amount, b.TotalLimit-b.current)
	}

	b.current += amount
	return true, nil
}

// GetRemaining returns the current remaining budget.
func (b *BudgetManager) GetRemaining() float64 {
	b.mu.Lock()
	defer b.mu.Unlock()
	return b.TotalLimit - b.current
}

// RefundSpend refunds previously spent tokens back to the budget.
func (b *BudgetManager) RefundSpend(amount float64) error {
	b.mu.Lock()
	defer b.mu.Unlock()

	if amount < 0 {
		return fmt.Errorf("refund amount cannot be negative")
	}

	b.current -= amount
	if b.current < 0 {
		b.current = 0
	}
	return nil
}
