package pricing

import (
	"testing"
)

func TestBudgetManager(t *testing.T) {
	budget := NewBudgetManager(100.0)

	ok, err := budget.RecordSpend(50.0)
	if err != nil || !ok {
		t.Fatalf("Failed to record valid spend: %v", err)
	}

	if budget.GetRemaining() != 50.0 {
		t.Fatalf("Expected remaining 50.0, got %f", budget.GetRemaining())
	}

	ok, err = budget.RecordSpend(60.0)
	if err == nil || ok {
		t.Fatalf("Expected spend to be rejected for exceeding budget")
	}

	if budget.GetRemaining() != 50.0 {
		t.Fatalf("Expected remaining to be unchanged (50.0), got %f", budget.GetRemaining())
	}
}

func TestBudgetManager_NegativeSpend(t *testing.T) {
	budget := NewBudgetManager(100.0)

	ok, err := budget.RecordSpend(-10.0)
	if err == nil || ok {
		t.Fatalf("Expected negative spend to be rejected")
	}
}

func TestBudgetManager_RefundSpend(t *testing.T) {
	budget := NewBudgetManager(100.0)

	budget.RecordSpend(50.0)
	err := budget.RefundSpend(25.0)
	if err != nil {
		t.Fatalf("Unexpected error: %v", err)
	}

	if budget.GetRemaining() != 75.0 {
		t.Fatalf("Expected remaining 75.0, got %f", budget.GetRemaining())
	}

	err = budget.RefundSpend(-10.0)
	if err == nil {
		t.Fatalf("Expected negative refund to be rejected")
	}
}
